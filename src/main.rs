mod file;
mod listener;
// mod settings;

use std::collections::HashMap;

fn main() {
  let mut settings = config::Config::default();

  settings
    .merge(config::File::with_name("Settings")).unwrap()
    .merge(config::Environment::with_prefix("APP")).unwrap();

  settings.set("files", "test");


  println!("{:?}", settings.try_into::<HashMap<String, String>>().unwrap());

}
