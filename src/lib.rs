///Is - relative relation
pub enum Is {
    Left,
    On,
    Right,
}

///Side Relation
pub struct Side {
    s: Is
}

impl Side {
    ///New
    pub fn new() -> Side {
        Side { s: Is::On }
    }

    ///Is left
    pub fn is_left(&self) -> bool {
        match self.s {
            Is::Left => true,
            _ => false,
        }
    }

    ///As left
    pub fn as_left(&mut self) -> &mut Self {
        self.s = Is::Left;
        self
    }

    ///Is on
    pub fn is_on(&self) -> bool {
        match self.s {
            Is::On => true,
            _ => false,
        }
    }

    ///As on
    pub fn as_on(&mut self) -> &mut Self {
        self.s = Is::On;
        self
    }

    ///Is right
    pub fn is_right(&self) -> bool {
        match self.s {
            Is::Right => true,
            _ => false,
        }
    }

    ///As right
    pub fn as_right(&mut self) -> &mut Self {
        self.s = Is::Right;
        self
    }

    ///Is on or left
    pub fn is_on_or_left(&self) -> bool {
        return self.is_on() || self.is_left();
    }

    ///Is on or right
    pub fn is_on_or_right(&self) -> bool {
        return self.is_on() || self.is_right();
    }

    ///Is on the same side
    pub fn is_same_side(&self, other: &Side) -> bool {
        return (self.is_left() && other.is_left()) ||
            (self.is_on() && other.is_on()) ||
            (self.is_right() && other.is_right());
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sidedness() {
        let mut s = Side::new();
        assert!(s.as_on().is_on());

        assert!(s.as_left().is_left());
        assert!(s.is_on_or_left());
        assert!(!s.as_left().is_on());

        assert!(s.as_on().is_on());
        assert!(s.is_on_or_right());
        assert!(s.is_on_or_left());

        assert!(s.as_right().is_right());
        assert!(s.is_on_or_right());
        assert!(!s.is_on());

        let mut o = Side::new();
        s.as_left();
        o.as_left();

        assert!(s.is_same_side(&o));
        o.as_on();
        assert!(!s.is_same_side(&o));
        s.as_on();
        assert!(s.is_same_side(&o));
    }
}
