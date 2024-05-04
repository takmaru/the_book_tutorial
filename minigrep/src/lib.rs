use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

#[derive(PartialEq, Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        /*
        let mut case_sensitive = false;
        let mut is_decide_cs = false;
        if args.len() >= 4 {
            for arg in &args[3..] {
                match &arg.to_lowercase()[..] {
                    "-i" => {
                        case_sensitive = true;
                        is_decide_cs = true;
                        break;
                    },
                    "-c" => {
                        is_decide_cs = true;
                        break;
                    },
                    _ => (),
                }
            }
        }
        if !is_decide_cs {
            if env::var("CASE_INSENSITIVE").is_err() {
                case_sensitive = true;
            }
        }
        */

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

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
mod test {
    use super::*;

    #[test]
    fn config_new_just_args() {
        let exe_path = String::from("minigrep.exe");
        let query = String::from("search");
        let filename = String::from("filename");
        let case_sensitive = true;
        let args = vec![exe_path.clone(), query.clone(), filename.clone()];
        assert_eq!(Config { query, filename, case_sensitive }, Config::new(&args).unwrap());
    }
        
    #[test]
    fn config_new_over_args() {
        let exe_path = String::from("minigrep.exe");
        let query = String::from("keyword");
        let filename = String::from("filepath");
        let case_sensitive = true;
        let args = vec![exe_path.clone(), query.clone(), filename.clone(), String::from("arg4"), String::from("arg5")];
        assert_eq!(Config { query, filename, case_sensitive }, Config::new(&args).unwrap());
    }

    #[test]
    fn config_new_not_enough_args() {
        let exe_path = String::from("minigrep.exe");
        let query = String::from("nofile");
        let args = vec![exe_path.clone(), query.clone()];
        assert!(Config::new(args.iter_mut()).is_err());
    }

    #[test]
    fn run_not_exists_file() {
        let exe_path = String::from("minigrep.exe");
        let query = String::from("keyword");
        let filename = String::from("not_exists.txt");
        let args = vec![exe_path.clone(), query.clone(), filename.clone()];
        assert!(run(Config::new(&args).unwrap()).is_err());
    }

    #[test]
    fn run_invalid_utf8() {
        let exe_path = String::from("minigrep.exe");
        let query = String::from("keyword");
        let filename = String::from("invalid_utf8.txt");
        let args = vec![exe_path.clone(), query.clone(), filename.clone()];
        assert!(run(Config::new(&args).unwrap()).is_err());
    }

    #[test]
    fn run_ok() {
        let exe_path = String::from("minigrep.exe");
        let query = String::from("keyword");
        let filename = String::from("poem.txt");
        let args = vec![exe_path.clone(), query.clone(), filename.clone()];
        assert!(run(Config::new(&args).unwrap()).is_ok());
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}