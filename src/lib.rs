use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}
/// Configuration struct for the command-line search utility.
///
/// Holds the search parameters and options for the text search operation.
/// 
/// # Fields
/// * `query` - The search term to look for in the file
/// * `file_path` - The path to the file to be searched
/// * `ignore_case` - Whether the search should be case-sensitive or case-insensitive
///
/// # Examples
/// ```
/// // Typical usage with case-sensitive search
/// let config = Config {
///     query: "rust".to_string(),
///     file_path: "example.txt".to_string(),
///     ignore_case: false
/// };
/// 
/// // Case-insensitive search configuration
/// let config_insensitive = Config {
///     query: "Rust".to_string(),
///     file_path: "example.txt".to_string(),
///     ignore_case: true
/// };
/// ```
impl Config {
    /// Builds a `Config` instance from command-line arguments.
    ///
    /// # Arguments
    /// * `args` - An iterator of command-line arguments
    ///
    /// # Returns
    /// * `Ok(Config)` if arguments are successfully parsed
    /// * `Err(&'static str)` if required arguments are missing
    ///
    /// # Errors
    /// * Returns an error if no query string or file path is provided
    ///
    /// # Environment Variables
    /// * `NO_IGNORE_CASE` - If set, enables case-insensitive search
    ///
    /// # Examples
    /// ```
    /// // Typical usage with valid arguments
    /// let args = vec!["program_name".to_string(), "query".to_string(), "file.txt".to_string()];
    /// let config = Config::build(args.into_iter()).unwrap();
    /// 
    /// // Example showing error handling with insufficient arguments
    /// let incomplete_args = vec!["program_name".to_string()];
    /// let result = Config::build(incomplete_args.into_iter());
    /// assert!(result.is_err());
    /// ```
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        //ignore the first item on the iterator because is the name of the program
        args.next();

        let query: String = match args.next() {
            Some(args) => args,
            None => return Err("didn't get query string"),
        };
        let file_path: String = match args.next() {
            Some(args) => args,
            None => return Err("no file path passed"),
        };

        let ignore_case = env::var("NO_IGNORE_CASE").is_ok();
        // This make the enviroment variable persistend along the command line session.
        // to remove the signed enviroment variable use Remove-Item Env:NO_IGNORE_CASE

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

/// Runs the text search operation based on the provided configuration.
///
/// # Arguments
/// * `config` - Configuration specifying search parameters
///
/// # Returns
/// * `Ok(())` if the search completes successfully
/// * `Err` if there are issues reading the file
///
/// # Errors
/// * Fails if the specified file cannot be read
///
/// # Examples
/// ```no_run
/// // Typical usage with a valid configuration
/// let config = Config {
///     query: "rust".to_string(),
///     file_path: "example.txt".to_string(),
///     ignore_case: false
/// };
/// run(config).expect("Search operation failed");
/// 
/// // Example with a non-existent file (will return an error)
/// let config_error = Config {
///     query: "rust".to_string(),
///     file_path: "non_existent.txt".to_string(),
///     ignore_case: false
/// };
/// assert!(run(config_error).is_err());
/// ```
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}
/// Performs a case-sensitive search for a query within file contents.
///
/// # Arguments
/// * `query` - The search term to find
/// * `contents` - The full text content to search through
///
/// # Returns
/// A vector of lines that contain the search query
///
/// # Examples
/// ```
/// let contents = "Rust is a systems programming language.\nRust is safe and fast.";
/// let results = search("Rust", contents);
/// assert_eq!(results, vec![
///     "Rust is a systems programming language.",
///     "Rust is safe and fast."
/// ]);
/// 
/// // Case-sensitive search (note the difference)
/// let case_sensitive_results = search("rust", contents);
/// assert_eq!(case_sensitive_results, Vec::<&str>::new());
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
/// Performs a case-insensitive search for a query within file contents.
///
/// # Arguments
/// * `query` - The search term to find
/// * `contents` - The full text content to search through
///
/// # Returns
/// A vector of lines that contain the search query, ignoring case
///
/// # Examples
/// ```
/// let contents = "Rust is a systems programming language.\nrust is safe and fast.";
/// let results = search_case_insensitive("rust", contents);
/// assert_eq!(results, vec![
///     "Rust is a systems programming language.",
///     "rust is safe and fast."
/// ]);
/// 
/// // Different case variations are matched
/// let mixed_case_results = search_case_insensitive("RuSt", contents);
/// assert_eq!(mixed_case_results, vec![
///     "Rust is a systems programming language.",
///      "rust is safe and fast."
/// ]);
/// ```
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duck tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick Three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
