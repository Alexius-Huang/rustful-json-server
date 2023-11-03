use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub jsondb_dir: PathBuf,
    pub pool_capacity: Option<usize>,
    pub port: Option<usize>,
    pub verbose: bool
}

impl Config {
    pub fn from(args: Vec<String>) -> Result<Self, String> {
        let len = args.len();
        if len < 2 {
            return Err("Please specify the entry folder for all json db files".to_owned());
        }

        let mut config = Self {
            jsondb_dir: PathBuf::from(args[1].to_owned()),
            pool_capacity: None,
            port: None,
            verbose: false
        };

        for i in 2..len {
            config.parse_option(args[i].clone())?;
        }

        Ok(config)
    }

    fn parse_option(&mut self, arg: String) -> Result<(), String> {
        let mut option = arg.splitn(2, '=');
        let key = option.next().unwrap();

        let value = option.next();
        if value.is_none() {
            return Err("Expect option to be of format --<option>=<value>".to_owned());
        }
        let value = value.unwrap();

        match key {
            "--pool" => {
                self.pool_capacity = Some(value.parse::<usize>().unwrap());
                Ok(())
            },
            "--port" => {
                self.port = Some(value.parse::<usize>().unwrap());
                Ok(())
            },
            "--verbose" => {
                let value = match value {
                    "true" => true,
                    "false" => false,
                    _ => return Err(format!(r#"The option "--verbose" only accepts "true" or "false" value"#))
                };

                self.verbose = value;
                Ok(())
            },
            _ => {
                Err(format!("Unrecognized option: {key}"))
            }
        }
    }
}
