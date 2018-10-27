use std::str::FromStr;
use std::string::String;

#[derive(Debug, Eq, PartialEq)]
pub enum Cmd {
    GET(String),
    SET(String, i64),
    ADD(String, i64),
    DELETE(String),
    COUNT,
}

impl FromStr for Cmd {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.trim().split(' ');
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
            (Some("ADD"), Some(key)) => {
                if let Some(value) = it.next() {
                    let value = match value.trim().parse::<i64>() {
                        Ok(v) => v,
                        Err(e) => return Err(e.to_string()),
                    };
                    let key = key.trim().to_string();
                    return Ok(Cmd::ADD(key, value));
                }
                return Err("ADD command expect value".to_string());
            }
            (Some("DELETE"), Some(key)) => {
                let key = key.trim().to_string();
                return Ok(Cmd::DELETE(key));
            }
            (Some("COUNT"), None) => return Ok(Cmd::COUNT),
            _ => return Err("invalid command".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use cmd::Cmd;
    use std::str::FromStr;
    #[test]
    fn cmd_from_str_ok() {
        assert_eq!(Cmd::from_str("GET a"), Ok(Cmd::GET("a".to_string())));
        assert_eq!(Cmd::from_str("GET 1"), Ok(Cmd::GET("1".to_string())));
        assert_eq!(
            Cmd::from_str(r#"GET "錆""#),
            Ok(Cmd::GET(r#""錆""#.to_string()))
        );
        assert_eq!(Cmd::from_str("GET a b c"), Ok(Cmd::GET("a".to_string())));
        assert_eq!(
            Cmd::from_str("SET abc 1"),
            Ok(Cmd::SET("abc".to_string(), 1)),
        );
        assert_eq!(Cmd::from_str("SET 1 1"), Ok(Cmd::SET("1".to_string(), 1)),);
        assert_eq!(
            Cmd::from_str("SET キー -999"),
            Ok(Cmd::SET("キー".to_string(), -999)),
        );
        assert_eq!(
            Cmd::from_str("ADD key 53"),
            Ok(Cmd::ADD("key".to_string(), 53)),
        );
        assert_eq!(Cmd::from_str("DELETE a"), Ok(Cmd::DELETE("a".to_string())));
        assert_eq!(Cmd::from_str("COUNT"), Ok(Cmd::COUNT));
        assert_eq!(Cmd::from_str("COUNT\n"), Ok(Cmd::COUNT));
    }
    #[test]
    fn cmd_from_str_err() {
        assert!(Cmd::from_str("NOT COMMAND").is_err());
        assert!(Cmd::from_str("GET").is_err());
        assert!(Cmd::from_str("SET").is_err());
        assert!(Cmd::from_str("ADD").is_err());
        assert!(Cmd::from_str("DELETE").is_err());
        assert!(Cmd::from_str("SET key").is_err());
        assert!(Cmd::from_str("SET key1 key2").is_err());
        assert!(Cmd::from_str("ADD key").is_err());
        assert!(Cmd::from_str("ADD key1 key2").is_err());
    }
}
