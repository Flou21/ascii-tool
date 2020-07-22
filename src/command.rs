pub enum Command {
    List,
    Help,
    Add(String),
    Convert(String),
    ConvertAll,
    Remove(String),
    RemoveAll
}