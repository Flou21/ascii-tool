use std::env;
use std::process;
mod command;
mod converter;
mod helper;
mod settings;
use command::Command;
use converter::Converter;


fn main() {
    let arguments: Vec<String> = env::args().collect();
    let command = match arguments[1].as_str() {
        "help" => Command::Help,
        "list" => Command::List,
        "add" => {
            if arguments.len() != 3 {
                println!("Falsche Anzahl an Argumenten\nSee help for more information");
                process::exit(1);
            }
            Command::Add(arguments[2].clone())
        }
        "convert" => Command::Convert(arguments[2].clone()),
        "convertAll" => Command::ConvertAll,
        "remove" => Command::Remove(arguments[2].clone()),
        "removeAll" => Command::RemoveAll,
        _ => Command::Help,
    };

    let mut conv = match Converter::new() {
        Ok(c) => c,
        Err(msg) => {
            println!("{}", msg);
            process::exit(1);
        }
    };

    match conv.execute(&command) {
        Ok(())  => (),
        Err(msg) => {
            println!("Error occured\n{}", msg);
            process::exit(1);
        }
    }

    // just to prevent warinings for now
    helper::Helper::get_help_text();
}
