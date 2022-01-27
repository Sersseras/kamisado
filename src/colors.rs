use bevy::prelude::Color;

#[derive(Clone, Copy)]
pub enum Colors {
    Orange,
    Blue,
    Purple,
    Pink,
    Yellow,
    Red,
    Green,
    Brown,
}

impl Colors {
    pub fn color(&self) -> Color {
        match *self {
            Colors::Orange => Color::rgb(0.839, 0.458, 0.129),
            Colors::Blue => Color::rgb(0.0, 0.415, 0.670),
            Colors::Purple => Color::rgb(0.431, 0.215, 0.529),
            Colors::Pink => Color::rgb(0.823, 0.439, 0.619),
            Colors::Yellow => Color::rgb(0.890, 0.764, 0.003),
            Colors::Red => Color::rgb(0.819, 0.2, 0.223),
            Colors::Green => Color::rgb(0.0, 0.564, 0.337),
            Colors::Brown => Color::rgb(0.337, 0.149, 0.0),
        }
    }
}
