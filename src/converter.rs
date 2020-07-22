use crate::command::Command;
use crate::settings::Settings;
use clt::confirm;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process;

pub struct Converter {
    list: Vec<PathBuf>,
}

impl Converter {
    pub fn new() -> Result<Converter, String> {
        match Converter::check_if_asciidoctor_is_installed() {
            Ok(_) => {
                let mut conv = Converter { list: Vec::new() };
                match conv.load() {
                    Ok(_) => return Ok(conv),
                    Err(e) => {
                        if e.kind() == std::io::ErrorKind::NotFound {
                            return Ok(conv);
                        }
                        return Err(format!("Error while loading settings file\n{:?}", e));
                    }
                }
            }
            Err(msg) => Err(msg),
        }
    }

    pub fn execute(&mut self, command: &Command) -> Result<(), String> {
        return match command {
            // TODO
            // complete rewrite of that
            // look at that https://rust-cli.github.io/book/in-depth/docs.html
            /*
            Command::Help => {
                // write help to the command line or show man page
                // following command works:
                // cargo run help > /tmp/ascii-tool.man; man /tmp/ascii-tool.man
                process::Command::new("sh")
                    .arg("-c")
                    .arg("echo hello")
                    .output()
                    .expect("failed to execute process");
                return Ok(Helper::get_help_text());
            }
            */
            Command::RemoveAll => self.remove_all_files(),
            Command::Remove(path) => self.remove_file(path, true),
            Command::Add(file) => self.add_file(file),
            Command::List => self.list_files(),
            Command::Convert(path) => self.convert(path),
            Command::ConvertAll => self.convert_all(),
            _ => Err("Command not implemented yet".to_string()),
        };
    }

    fn remove_all_files(&mut self) -> Result<(), String> {
        // confirm operation
        if !confirm("You really want to delete all files?", false, "\n", true) {
            return Ok(());
        }

        // delete all files
        self.list.clear();

        // save changes
        match self.save() {
            Ok(_) => (),
            Err(e) => return Err(format!("Error when writing file\n{:?}", e))
        }

        // tell user what happened
        println!("All items were removed");

        return Ok(());
    }

    fn remove_file(&mut self, path: &String, confirmation_required: bool) -> Result<(), String> {
        // check if file is in list
        let mut index: i32 = -1;
        let mut ind = 0;
        for item in &self.list.clone() {
            if item.as_path().to_str().unwrap() == path {
                index = ind;
            }
            ind += 1;
        }
        // file not found in list
        if index == -1 {
            return Err(format!("{} is not in list\ncheck if you wrote it correctly", path));
        }

        // check if user sure he wants to delete the file 
        if confirmation_required {
            if !confirm(format!("You really want to delete {}?", path).as_str(), false, "\n", true) {
                println!("delete process was interrupted");
                return Ok(());
            }
        }

        // remove it from list
        self.list.remove(index as usize);

        // save changes
        match self.save() {
            Ok(_) => (),
            Err(e) => return Err(format!("Error when writing file\n{:?}", e))
        }

        // tell user path was successfully removed
        println!("{} was removed successfully", path);

        return Ok(());
    }

    fn convert_all(&mut self) -> Result<(), String> {
        println!("Converting all files");
        for item in &self.list.clone() {
            let path = String::from(item.as_path().to_str().unwrap());

            match self.convert(&path) {
                Ok(_) => (),
                Err(msg) => return Err(msg),
            }
        }
        return Ok(());
    }

    fn convert(&mut self, path: &String) -> Result<(), String> {
        // create object
        let mut file = PathBuf::new();
        file.push(path);

        // check if file even exists
        if !file.is_file() {
            return Err(format!("File not found\n check path: {}", path));
        }

        // check if file is already in stored list
        let mut found = false;
        for item in &self.list {
            if item.as_path().to_str().unwrap() == path {
                found = true;
                break;
            }
        }
        if !found {
            if confirm(
                "File is not in the current list of stored files should it be added",
                false,
                "\n",
                true,
            ) {
                match self.add_file(path) {
                    Ok(()) => (),
                    Err(e) => return Err(e),
                }
            }
        }

        // convert file
        process::Command::new("asciidoctor")
            .arg(path)
            .output()
            .expect("error while converting");

        // inform user
        println!("\t - {} was converted", path);
        return Ok(());
    }
    fn check_if_asciidoctor_is_installed() -> Result<(), String> {
        if std::process::Command::new("inxi").spawn().is_ok() {
            return Ok(());
        }
        return Err("asciidoctor is not installed\ninstallation instructions are here: https://asciidoctor.org/docs/install-toolchain/".to_string());
    }

    fn load(&mut self) -> Result<(), std::io::Error> {
        let mut file = match File::open("settings.json") {
            Ok(file) => file,
            Err(e) => {
                return Err(e);
            }
        };
        let mut buffer: Vec<u8> = Vec::new();
        match file.read_to_end(&mut buffer) {
            Ok(_size) => (),
            Err(e) => {
                return Err(std::io::Error::new(std::io::ErrorKind::Interrupted, e));
            }
        }

        // parse read content
        let content = match std::str::from_utf8(&buffer) {
            Ok(content) => content,
            Err(e) => {
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e));
            }
        };
        let settings: Settings = match serde_json::from_str(&content) {
            Ok(s) => s,
            Err(e) => {
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e));
            }
        };
        // update prop
        self.list = settings.files.clone();
        Ok(())
    }

    fn save(&self) -> Result<(), std::io::Error> {
        // create file
        let mut file = File::create("settings.json").unwrap();

        // create json object
        let settings = Settings {
            files: self.list.clone(),
        };
        let j = serde_json::to_string(&settings)?;

        // write settings
        match file.write_all(j.as_bytes()) {
            Ok(_) => return Ok(()),
            Err(e) => return Err(e),
        }
    }

    fn add_file(&mut self, path: &String) -> Result<(), String> {
        let mut file = PathBuf::new();
        file.push(path);

        if !file.exists() {
            return Err(format!(
                "File or Path {} does not exist\npls check if really exists",
                path
            ));
        }
        // add file and save file list
        self.list.push(file);
        match self.save() {
            Ok(_) => {
                println!("{} wurde hinzugefügt", path.as_str());
                return Ok(());
            }
            Err(e) => return Err(format!("Error when writing file\n{:?}", e)),
        }
    }

    fn list_files(&self) -> Result<(), String> {
        if self.list.len() == 0 {
            println!("No path stored yet");
            return Ok(());
        }
        let mut st = String::from("Here are all currently stored files:");
        for item in &self.list {
            st += "\n\t - ";
            st += item.as_path().to_str().unwrap();
        }
        println!("{}", st);
        return Ok(());
    }
}

// TODO https://doc.rust-lang.org/book/ch12-06-writing-to-stderr-instead-of-stdout.html
