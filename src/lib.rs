/// A Python module implemented in Rust. The name of this module must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pyo3::pymodule]
#[pyo3(name = "native")]
mod iron_pgn_parser {
    use pyo3::{PyResult, prelude::*, pyfunction};
    use regex::Regex;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{BufReader, Read};
    use zstd::Decoder;

    /// Formats the sum of two numbers as string.
    #[pyfunction]
    fn parse_san(pgn: String) -> PyResult<Vec<String>> {
        let brackets_regex =
            Regex::new(r"\[([^]]+)\]").expect("This should always return a good REGEX.");
        let braces_regex = Regex::new(r"\{.*\}").expect("This should always return a good REGEX.");
        // Stole and slightly modified from https://stackoverflow.com/questions/40007937/regex-help-for-chess-moves-san
        let move_regex = Regex::new(
            r"([PNBRQK]?[a-h]?[1-8]?[xX-]?[a-h][1-8](=[NBRQ]| ?e\.p\.)?|O-O(?:-O)?)[+#$]?",
        )
        .expect("This should always return a good REGEX.");
        let brackets_removed: String = String::from(brackets_regex.replace_all(pgn.as_str(), ""));
        let braces_and_bracket_removed: String =
            String::from(braces_regex.replace_all(brackets_removed.as_str(), ""));
        let moves_as_vec: Vec<String> = move_regex
            .find_iter(&braces_and_bracket_removed)
            .map(|m| m.as_str().to_string())
            .collect();

        return Ok(moves_as_vec);
    }

    #[pyfunction]
    fn parse_headers(pgn: &str) -> PyResult<HashMap<&str, &str>> {
        // https://stackoverflow.com/a/2403148/32714486
        // println!("{:?}", pgn);
        let brackets_regex =
            Regex::new(r"\[([^]]+)\]").expect("This should always return a good REGEX.");
        let all_brackets: Vec<&str> = brackets_regex.find_iter(pgn).map(|m| m.as_str()).collect();

        let zipped = all_brackets
            .iter()
            .map(|x| x.trim_matches(|c| c == '[' || c == ']'))
            .map(|x| x.splitn(2, " ").collect::<Vec<&str>>())
            .map(|x| (x[0], x[1]))
            .collect::<HashMap<&str, &str>>();
        Ok(zipped)
    }
    #[pyfunction]
    fn stream_file(file_path: &str) -> PyResult<()> {
        fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
            haystack
                .windows(needle.len())
                .position(|window| window == needle)
        }
        const BUF_SIZE: usize = 2_usize.pow(24);
        let bufreader = BufReader::with_capacity(BUF_SIZE, File::open(file_path)?);
        let mut decoder = zstd::Decoder::with_buffer(bufreader)?;
        const READER_SIZE: usize = 2_usize.pow(12);
        let buffer: &mut [u8] = &mut [0; READER_SIZE as usize];
        let mut at_eof = false;
        let mut rest: Vec<u8> = Vec::new();
        let mut counter = 0;
        while !at_eof {
            let read_size = decoder.read(buffer).unwrap();
            if read_size != READER_SIZE as usize {
                at_eof = true;
            }
            if counter < 5 {
                println!("value of readsize is {} is at eof {}", read_size, at_eof);
            }
            let mut tmp = rest.clone();
            tmp.extend(buffer.as_ref());
            let mut data = tmp;
            while let Some(found) = find_subsequence(data.as_slice(), b"\n\n[") {
                // if is_first {
                //     println!("{}", found);
                //     println!("{:?}", str::from_utf8(data.get(..found).unwrap()));
                //     is_first = false;
                // }
                println!("{}", counter);
                rest = data.split_off(found + 1);

                if counter == 1 {
                    // println!("{}", str::from_utf8(rest.as_slice()).unwrap());
                }
                let single_pgn = str::from_utf8(data.as_slice()).unwrap();
                if counter == 0 {
                    // println!("{}", single_pgn);
                }
                // println!("{}", str::from_utf8(rest.as_slice()).unwrap());
                parse_headers(single_pgn).unwrap();
                parse_san(single_pgn.to_string()).unwrap();
                // passes back to `data` because `rest` may contain more fully formed
                // PGNs
                data = rest.clone();
                counter += 1;
            }
        }

        Ok(())
    }
}
