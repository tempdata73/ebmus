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
    difficulty: u8,
}

impl Puzzle {
    pub fn new(template: String, difficulty: u8) -> Self {
        let mut puzzle = Puzzle {
            template,
            nonce: 0,
            difficulty: 0,
        };

        puzzle.set_difficulty(difficulty).unwrap();
        puzzle
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

    pub fn set_difficulty(&mut self, difficulty: u8) -> Result<(), &'static str> {
        if difficulty > 64 {
            return Err("Difficulty cannot be greater than 64.");
        }

        self.difficulty = difficulty;
        Ok(())
    }
}

impl ToString for Puzzle {
    fn to_string(&self) -> String {
        format!("{}:{}", self.template, self.nonce)
    }
}
