mod settings;

use settings::Settings;

pub fn main() {
  let settings = Settings::new();

  let array = [1,2,3];

  settings.set();
}
