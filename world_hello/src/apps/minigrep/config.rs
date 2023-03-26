use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    #[allow(dead_code)]
    fn new(args: &[String]) -> Self {
        if args.len() < 3 {
            panic!("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        Config {
            query: query,
            file_path: file_path,
            ignore_case: true,
        }
    }

    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("didn't get a query string"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        return Ok(Config {
            query: query,
            file_path: file_path,
            ignore_case: ignore_case,
        });
    }

    #[allow(dead_code)]
    fn build_deprecated(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        return Ok(Config {
            query: query,
            file_path: file_path,
            ignore_case: ignore_case,
        });
    }
}

#[allow(dead_code)]
fn parse_config_deprecated(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];
    (query, file_path)
}
