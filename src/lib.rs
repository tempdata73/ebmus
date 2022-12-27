#[cfg(feature = "template")]
pub mod template;

const MAX_DIFFICULTY: u8 = 64;

#[derive(Debug)]
pub struct Puzzle {
    template: String,
    pub nonce: usize,
}

impl Puzzle {
    pub fn new(template: &str) -> Self {
        Puzzle {
            template: template.to_string(),
            nonce: 0,
        }
    }

    fn is_solved(&self, difficulty: u8) -> bool {
        let dump = self.hash();
        let mut dump = dump.chars();

        for _ in 0..difficulty {
            if dump.next().unwrap() != '0' {
                return false;
            }
        }

        true
    }

    pub fn solve(&mut self, difficulty: u8) -> Result<(), &'static str> {
        if difficulty > MAX_DIFFICULTY {
            return Err("Difficulty cannot be greater than 64.");
        }

        while !self.is_solved(difficulty) {
            self.nonce += 1;
        }

        Ok(())
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
