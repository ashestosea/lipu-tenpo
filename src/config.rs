use std::path::PathBuf;

use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

use crate::files;

pub const DEFAULT_STR: &str = r#"virtual_midnight = 2"#;

#[derive(Deserialize, Serialize, Default)]
pub struct Config {
    #[serde(with = "naive_time")]
    pub virtual_midnight: NaiveTime,
}

pub fn read_config(path: String) -> Config {
    let config_path = if path.is_empty() {
        files::config_path()
    } else {
        PathBuf::from(path)
    };

    let res =
        toml::from_str::<Config>(std::fs::read_to_string(config_path).unwrap().as_str()).unwrap();
    res
}

mod naive_time {
    use super::*;
    use chrono::Timelike;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(time: &NaiveTime, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u32(time.hour())
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<NaiveTime, D::Error> {
        let time = Deserialize::deserialize(deserializer)?;
        Ok(NaiveTime::from_hms_opt(time, 0, 0).unwrap_or_default())
    }
}

#[cfg(test)]
mod test {
    use chrono::NaiveTime;

    use super::read_config;
    use std::str::FromStr;

    #[test]
    fn test_read_good_file() {
        let config = read_config(String::from_str("./test/config.toml").unwrap());

        assert_eq!(
            config.virtual_midnight,
            NaiveTime::from_hms_opt(2, 0, 0).unwrap()
        );
    }
}
