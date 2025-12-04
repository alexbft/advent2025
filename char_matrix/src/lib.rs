pub struct CharMatrix {
    chars: Vec<Vec<char>>,
}

impl CharMatrix {
    pub fn new(s: &str) -> CharMatrix {
        let chars: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();
        CharMatrix { chars }
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        y >= 0 && y < self.chars.len() as i32 && x >= 0 && x < self.chars[y as usize].len() as i32
    }

    pub fn get(&self, x: i32, y: i32) -> char {
        self.chars[y as usize][x as usize]
    }

    pub fn safe_get(&self, x: i32, y: i32) -> Option<char> {
        if self.in_bounds(x, y) {
            Some(self.get(x, y))
        } else {
            None
        }
    }

    pub fn width(&self) -> i32 {
        if self.height() == 0 {
            0
        } else {
            self.chars[0].len() as i32
        }
    }

    pub fn height(&self) -> i32 {
        self.chars.len() as i32
    }
}
