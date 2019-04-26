pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        let expected_result: Vec<&str> = vec!["safe, fast, productive."];

        assert_eq!(
            expected_result,
            search(query, contents)
        );
    }

    #[test]
    fn search_no_result() {
        let query = "slow";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        let expected_result: Vec<String> = vec![];

        assert_eq!(
            expected_result,
            search(query, contents)
        );
    }

    #[test]
    fn search_insensitive() {
        let query = "DuCt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        let expected_result: Vec<&str> = vec!["safe, fast, productive."];

        assert_eq!(
            expected_result,
            search_case_insensitive(query, contents)
        );
    }
}