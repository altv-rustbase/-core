
// CODE

pub struct Vitality {
    value:usize,
    max:usize,
    min:usize
}

impl Vitality {
    pub fn new(
        value:usize,
        max:usize,
        min:usize
    ) -> Vitality {
        Vitality {
            value,
            max,
            min
        }
    }

    // SETTERS

    pub fn set(&mut self, value:usize) {
        self.value = value.clamp(self.min, self.max);
    }

    // GETTERS

    pub fn get(&mut self) -> usize {
        return self.value;
    }

    // OTHERS
}