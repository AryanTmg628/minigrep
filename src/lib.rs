use std::fs;

pub struct Config {
    pub query_string: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &Vec<String>) -> Result<Config, &str> {
        if args.len() != 3 {
            return Err("Invalid arguments.");
        }

        let query_string = args.get(1).expect("Invalid arguments.");
        let file_path = args.get(2).expect("Invalid arguments.");

        return Ok(Config {
            query_string: query_string.to_string(),
            file_path: file_path.to_string(),
        });
    }

    pub fn search(query: &String, file: &String) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();

        for i in file.lines() {
            if i.contains(query) {
                result.push(i.to_string());
            }
        }

        return result;
    }

    pub fn run(config: Config) -> Result<(), String> {
        // opening the file
        let file = fs::read_to_string(config.file_path);

        match file {
            Ok(file) => {
                // searching fro the query in the file

                for line in Self::search(&config.query_string, &file) {
                    println!("{}", line);
                }
            }
            Err(_err) => return Err("Couldn't open or read from  file".to_string()),
        }

        Ok(())
    }
}
