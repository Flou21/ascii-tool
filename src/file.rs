
pub struct File {
  path: String
}

impl File {
  // Constructor
  pub fn new(p: &str) -> File {
    File {
      path: p.to_string()
    }
  }
}
