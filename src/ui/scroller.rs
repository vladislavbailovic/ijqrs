pub struct Scroller {
    position: usize,
    max: usize,
}

impl Scroller {
    pub fn new(max: usize) -> Scroller {
        Scroller { position: 0, max }
    }

    pub fn prev(&mut self) {
        if self.position > 0 {
            self.position -= 1;
        }
    }

    pub fn next(&mut self) {
        if self.position < self.max {
            self.position += 1;
        }
    }

    pub fn set_max(&mut self, max: usize) {
        self.max = max;
        if self.position > self.max {
            self.position = self.max
        }
    }

    pub fn set_position(&mut self, pos: usize) {
        if pos <= self.max {
            self.position = pos;
        }
    }

    pub fn max(&self) -> usize {
        self.max
    }

    pub fn get(&self) -> usize {
        self.position
    }
}
