use std::mem::discriminant;

#[derive(Clone, Copy)]
pub enum Value {
    Num(usize),
    Skip,
    Reverse,
    Draw(usize),

    /// No value
    None,
}

impl Value {
    fn match_with(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::None, _) | (_, Value::None) => false,
            (Value::Num(a), Value::Num(b)) => a == b,
            (a, b) => discriminant(a) == discriminant(b),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Color {
    Red,
    Yellow,
    Green,
    Blue,

    /// All Color
    Wild,
}

impl Color {
    fn match_with(&self, other: &Self) -> bool {
        match (self, other) {
            (Color::Wild, _) | (_, Color::Wild) => true,
            (a, b) => discriminant(a) == discriminant(b),
        }
    }
}

impl Color {}

pub struct Card {
    pub value: Value,
    pub color: Color,
}

impl Card {
    pub fn new(value: Value, color: Color) -> Self {
        Card { value, color }
    }
    pub fn match_with(&self, other: Self) -> bool {
        self.value.match_with(&other.value) || self.color.match_with(&other.color)
    }
}
