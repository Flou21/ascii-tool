use std::fs::File;
use std::io;
use std::fs;
use std::fs::OpenOptions;
use std::io::ErrorKind;
use std::collections::HashMap;
use std::io::Read;

pub struct Settings {
  map: HashMap<String, String>
}

impl Settings {
    pub fn new() -> Settings {
        let settings = Settings {
          map: HashMap::new()
        };
        settings.load_config().unwrap();
        return settings;
    }

    fn load_config(&self) -> std::io::Result<()> {
        let mut file = self.load_readable_config().unwrap();

        let mut content = String::new();
        
        file.read_to_string(&mut content)?;
        println!("{:?}", content);

        Ok(())
    }

    fn save_config(&self) -> std::io::Result<()> {
        let mut _file = &self.load_writeable_config().unwrap();
        Ok(())
    }
    

    fn load_config_file(&self, _opt: &OpenOptions) -> Result<File, &'static str> {
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
