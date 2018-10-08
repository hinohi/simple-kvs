use std::string::String;

#[derive(Debug)]
pub enum Cmd {
    GET(String),
    SET(String, i64),
}

impl Cmd {
    pub fn from_string(s: String) -> Result<Self, String> {
        let mut it = s.split(' ');
        match (it.next(), it.next()) {
            (Some("GET"), Some(key)) => {
                let key = key.trim().to_string();
                return Ok(Cmd::GET(key));
            }
            (Some("SET"), Some(key)) => {
                if let Some(value) = it.next() {
                    let value = match value.trim().parse::<i64>() {
                        Ok(v) => v,
                        Err(e) => return Err(e.to_string()),
                    };
                    let key = key.trim().to_string();
                    return Ok(Cmd::SET(key, value));
                }
                return Err("SET command expect value".to_string());
            }
            _ => return Err("invalid command".to_string()),
        }
    }
}
