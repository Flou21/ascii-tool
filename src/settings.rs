use std::fs::File;
use std::fs::OpenOptions;
use std::io::ErrorKind;
use std::collections::HashMap;

pub struct Settings {
  map: HashMap<String, String>
}

impl Settings {
    pub fn new() -> Settings {
        let settings = Settings {
          map: HashMap::new()
        };
        settings.load_config();
        return settings;
    }

    fn load_config(&self) {
        let file = match self.load_readable_config() {
            Ok(file) => file,
            Err(error) => panic!(error),
        };

        serde_json::to_string(self.map);
        println!("{:?}", file);
    }

    fn load_config_file(&self, opt: &OpenOptions) -> Result<File, &'static str> {
        let file = match File::open("settings.json") {
            Ok(file) => file,
            Err(err) => match err.kind() {
                ErrorKind::NotFound => match File::create("settings.json") {
                    Ok(file) => file,
                    Err(e) => panic!("Problem while creating file settings.json {:?}", e),
                },
                other_error => panic!("Unknown error occured {:?}", other_error),
            },
        };
        return Ok(file);
    }

    fn load_readable_config(&self) -> Result<File, &'static str> {
        return self.load_config_file(OpenOptions::new().read(true));
    }

    fn load_writeable_config(&self) -> Result<File, &'static str> {
        return self.load_config_file(OpenOptions::new().read(true).write(true));
    }
}
