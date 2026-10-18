/// A Python module implemented in Rust. The name of this module must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pyo3::pymodule]
#[pyo3(name = "native")]
mod iron_pgn_parser {
    #[pyclass]
    struct ChessData {
        #[pyo3(get)]
        headers: HashMap<String, String>,
        #[pyo3(get)]
        moves: Vec<String>,
    }
    use pyo3::{PyResult, prelude::*, pyfunction};
    use regex::Regex;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{BufReader, Read};
    use std::sync::LazyLock;
    use zstd::Decoder;

    static BRACKETS_REGEX: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"\[([^]]+)\]").expect("This should always return a good REGEX.")
    });
    static BRACES_REGEX: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"\{.*?\}").expect("This should always return a good REGEX."));

    // Stole and slightly modified from https://stackoverflow.com/questions/40007937/regex-help-for-chess-moves-san
    static MOVE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"([PNBRQK]?[a-h]?[1-8]?[xX-]?[a-h][1-8](=[NBRQ]| ?e\.p\.)?|O-O(?:-O)?)[+#$]?")
            .expect("This should always return a good REGEX.")
    });
    #[pyfunction]
    fn parse_san(pgn: String) -> PyResult<Vec<String>> {
        let brackets_removed: String = String::from(BRACKETS_REGEX.replace_all(pgn.as_str(), ""));
        let braces_and_bracket_removed: String =
            String::from(BRACES_REGEX.replace_all(brackets_removed.as_str(), ""));
        let moves_as_vec: Vec<String> = MOVE_REGEX
            .find_iter(&braces_and_bracket_removed)
            .map(|m| m.as_str().to_string())
            .collect();

        return Ok(moves_as_vec);
    }

    #[pyfunction]
    fn parse_headers(pgn: &str) -> PyResult<HashMap<String, String>> {
        // https://stackoverflow.com/a/2403148/32714486
        let brace_remove = BRACES_REGEX.replace_all(pgn, "");
        let all_brackets: Vec<&str> = BRACKETS_REGEX
            .find_iter(&brace_remove)
            .map(|m| m.as_str())
            .collect();

        let zipped = all_brackets
            .iter()
            .map(|x| x.trim_matches(|c| c == '[' || c == ']'))
            .map(|x| x.splitn(2, " ").collect::<Vec<&str>>())
            .filter(|x| x[0] != "%eval")
            .map(|x| {
                (
                    String::from(x[0]),
                    String::from(x[1].trim_matches(|c| c == '"')),
                )
            })
            .collect::<HashMap<String, String>>();
        if &zipped["Site"] == r"https:// [%eval -89.04" {
            println!("{}", pgn);
        }
        Ok(zipped)
    }
    #[pyfunction]
    fn stream_file(file_path: &str) -> PyResult<Vec<ChessData>> {
        fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
            haystack
                .windows(needle.len())
                .position(|window| window == needle)
        }
        let mut global_moves: Vec<ChessData> = Vec::new();
        const BUF_SIZE: usize = 2_usize.pow(24);
        let bufreader = BufReader::with_capacity(BUF_SIZE, File::open(file_path)?);
        let mut decoder = zstd::Decoder::with_buffer(bufreader)?;
        const READER_SIZE: usize = 2_usize.pow(12);
        let buffer: &mut [u8] = &mut [0; READER_SIZE as usize];
        let mut at_eof = false;
        let mut rest: Vec<u8> = Vec::new();
        let mut counter = 0u32;
        while !at_eof {
            let read_size = decoder.read(buffer).unwrap();
            if read_size != READER_SIZE as usize {
                at_eof = true;
            }
            let mut tmp = rest.clone();
            tmp.extend(buffer.as_ref());
            let mut data = tmp;
            while let Some(found) = find_subsequence(data.as_slice(), b"\n\n[") {
                // println!("{}", counter);
                rest = data.split_off(found + 1);
                let single_pgn = str::from_utf8(data.as_slice()).unwrap();
                let headers = parse_headers(single_pgn).unwrap();
                let moves = parse_san(single_pgn.to_string()).unwrap();
                // passes back to `data` because `rest` may contain more fully formed
                // PGNs
                let moves_and_headers = ChessData { headers, moves };
                global_moves.push(moves_and_headers);
                data = rest.clone();
                counter += 1;
            }
        }

        Ok(global_moves)
    }
}
