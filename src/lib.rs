// template.rs
pub struct DefaultTemplate {
    name: String,
    timestamp: i64,
    token: String,
}

impl ToString for DefaultTemplate {
    fn to_string(&self) -> String {
        todo!();
    }
}

// lib.rs
#[derive(Debug)]
pub struct Puzzle {
    template: String,
    nonce: usize,
    pub difficulty: usize,
}

impl Puzzle {
    pub fn new(template: &str, difficulty: usize) -> Self {
        Puzzle {
            template: template.to_owned(),
            nonce: 0,
            difficulty,
        }
    }

    fn is_solved(&self) -> bool {
        let dump = self.hash();
        let mut dump = dump.chars();

        for _ in 0..self.difficulty {
            if dump.next().unwrap() != '0' {
                return false;
            }
        }

        true
    }

    pub fn solve(&mut self) {
        while !self.is_solved() {
            self.nonce += 1;
        }
    }

    pub fn hash(&self) -> String {
        let stream = self.to_string();
        let dump = blake3::hash(stream.as_bytes());
        dump.to_string()
    }
}

impl ToString for Puzzle {
    fn to_string(&self) -> String {
        format!("{}:{}", self.template, self.nonce)
    }
}
