use chrono::Utc;
use rand::distributions::{Alphanumeric, DistString};

pub struct Template {
    name: String,
    timestamp: i64,
    token: String,
}

impl Template {
    pub fn new(name: &str) -> Self {
        Template {
            name: name.to_owned(),
            timestamp: Utc::now().timestamp(),
            token: Alphanumeric.sample_string(&mut rand::thread_rng(), 16),
        }
    }
}

impl ToString for Template {
    fn to_string(&self) -> String {
        format!("{}:  {}:{}:::", self.name, self.timestamp, self.token)
    }
}
