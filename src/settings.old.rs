  
use std::collections::HashMap;
// use std::fmt;
// use std::fmt::Display;
// use std::error::*;
// use serde::de::{Deserialize, Deserializer, Visitor};


pub type Res<T> = std::result::Result<T, ConfigError>;
pub type Array = Vec<Value>;
pub type Table = HashMap<String, Value>;

pub struct Settings {
  map: HashMap<String, String>
}

impl Settings {
  pub fn new() -> Self {
    Settings {
      map: HashMap::new(),
    }
  }

  pub fn loadConfig(&self) -> Result<bool, std::io::Error> {
    // load config
    
    // config loaded
    Ok(true)
  }

  pub fn saveConfig(&self) -> Result<bool, std::io::Error> {
    // save config
    
    // config saved
    Ok(true)
  }
  
  pub fn get(&self, key: &str) -> Result<ValueType, ConfigError> {

    if key.is_empty() {
      return Err(ConfigError::NotFound { key: key.to_string() } );
    }
    
    Ok(ValueType::String(key.to_string()))
    
  }

  pub fn set(&self, key: String, value: ValueType) -> Result<bool, ConfigError> {
    // check parameters
    
    // update hashmap 
    let value = match value {
      ValueType::String(val) => val,
      ValueType::String(val) => json::stringify(val),
    }
    self.map.insert(key, value);
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValueType {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Array(Array),
}

pub enum ConfigError {
  Frozen,
  NotFound { key: String }
}

impl ConfigError {
}

