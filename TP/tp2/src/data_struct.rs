use std::str::FromStr;
use std::fmt::{self, Display};

#[derive(Debug, Clone)]
pub struct DataStruct {
    pub key1: f64,
    pub key2: f64,
    pub key3: String,
}

pub struct DataStructParseError;

// (:key1 50.0d:key2 5.45e-2:key3 "Data":)
impl Display for DataStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(:key1 {:.1}d:key2 {:.2e}:key3 \"{}\":)", self.key1, self.key2, self.key3)
    }
}

impl FromStr for DataStruct {
    type Err = DataStructParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if !s.starts_with('(') || !s.ends_with(')') {
            return Err(DataStructParseError);
        }

        let parts = s[1..s.len()-1]
            .split(':').filter(|p| !p.trim().is_empty());

        let mut key1: Option<f64> = None;
        let mut key2: Option<f64> = None;
        let mut key3: Option<String> = None;

        for part in parts {
            let part = part.trim();

            if let Some(rest) = part.strip_prefix("key1 ") {
                if let Some(num_str) = rest.strip_suffix('d') {
                    let val = num_str.trim().parse::<f64>().map_err(|_| DataStructParseError)?;
                    key1 = Some(val);
                } else {
                    return Err(DataStructParseError);
                }
            } else if let Some(rest) = part.strip_prefix("key2 ") {
                let val = rest.trim().parse::<f64>().map_err(|_| DataStructParseError)?;
                key2 = Some(val);
            } else if let Some(rest) = part.strip_prefix("key3 ") {
                let rest = rest.trim();
                if rest.starts_with('"') && rest.ends_with('"') && rest.len() >= 2 {
                    let val = &rest[1..rest.len()-1];
                    key3 = Some(val.to_string());
                } else {
                    return Err(DataStructParseError);
                }
            }
        }

        match (key1, key2, key3) {
            (Some(k1), Some(k2), Some(k3)) => Ok(DataStruct { key1: k1, key2: k2, key3: k3 }),
            _ => Err(DataStructParseError),
        }
    }
}