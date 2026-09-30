/// A Python module implemented in Rust. The name of this module must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pyo3::pymodule]
mod iron_pgn_parser {
    use pyo3::prelude::*;
    use regex::Regex;

    /// Formats the sum of two numbers as string.
    #[pyfunction]
    fn get_number_of_moves(pgn: String) -> PyResult<usize> {
        let pgn_vec: Vec<String> = pgn.lines().map(|x| String::from(x)).collect();
        let filtered_pgn: Vec<&String> = pgn_vec
            .iter()
            .filter(|x| {
                if x.len() < 1 {
                    return false;
                };
                if x.starts_with("[") {
                    return false;
                };
                return true;
            })
            .collect();
        if filtered_pgn.len() != 1 {
            println!("{:?}", filtered_pgn);
            panic!("Filtered not unique");
        }
        let game = filtered_pgn.last().unwrap();
        // let pair_number_of_moves = Regex::new(r"(\d+\.{3}[[:space:]])").unwrap();
        let odd_number_of_moves = Regex::new(r"(\d+\.{1}[[:space:]])").unwrap();
        // let mut get_last_pair: String = pair_number_of_moves
        //     .find_iter(game)
        //     .last()
        //     .unwrap()
        //     .as_str()
        //     .into();

        let mut get_last_odd: String = odd_number_of_moves
            .find_iter(game)
            // .map(|x| {
            //     println!("{:?}", x);
            //     return x;
            // })
            .last()
            .unwrap()
            .as_str()
            .into();
        // get_last_pair.truncate(3);
        // let max_pair = get_last_pair.parse::<u32>().unwrap();
        //
        let find_last_good = Regex::new(r"\d+").unwrap();
        let last: String = find_last_good
            .find(get_last_odd.as_str())
            .unwrap()
            .as_str()
            .into();
        println!("{}", last);
        let max_odd = last
            .parse::<u32>()
            .expect(format!("{} provided cannot be converted to a u32", get_last_odd).as_str());

        return Ok(max_odd as usize);
        // if max_pair < max_odd {
        // } else {
        //     return Ok(max_pair as usize);
        // }
    }
}
