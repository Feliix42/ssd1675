/// Represents the state of a pixel in the display
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color {
    Black,
    White,
    Red,
}

#[cfg(feature = "graphics")]
extern crate embedded_graphics;
#[cfg(feature = "graphics")]
use self::embedded_graphics::pixelcolor::raw::RawU8;
#[cfg(feature = "graphics")]
use self::embedded_graphics::pixelcolor::{BinaryColor, Rgb888, RgbColor};
#[cfg(feature = "graphics")]
use self::embedded_graphics::prelude::*;
#[cfg(feature = "graphics")]
impl PixelColor for Color {
    type Raw = RawU8;
}

impl From<u8> for Color {
    fn from(value: u8) -> Self {
        match value {
            0 => Color::Black,
            1 => Color::White,
            2 => Color::Red,
            _ => panic!("invalid color value"),
        }
    }
}

impl From<RawU8> for Color {
    fn from(value: RawU8) -> Self {
        // NOTE(feliix42): This is a very ... liberal mapping of colors and may be refined at some
        // point
        match value.into_inner() {
            0 => Color::Black,
            _ => Color::White,
        }   
    }
}

#[cfg(feature = "graphics")]
impl From<BinaryColor> for Color {
    fn from(value: BinaryColor) -> Self {
        match value {
            BinaryColor::On => Color::Black,
            BinaryColor::Off => Color::White,
        }
    }
}

#[cfg(feature = "graphics")]
impl From<Rgb888> for Color {
    fn from(value: Rgb888) -> Self {
        // NOTE(feliix42): This is admittedly a very simplistic approximation, it'd be more
        // accurate to map the colors accordingly, but since that's an injective mapping anyways,
        // why not keep it simple for now.
        match value {
            Rgb888::WHITE => Color::White,
            Rgb888::BLACK => Color::Black,
            _ => Color::Red,
        }
    }
}

#[cfg(feature = "graphics")]
impl Into<Rgb888> for Color {
    fn into(self) -> Rgb888 {
        match self {
            Color::White => Rgb888::WHITE,
            Color::Black => Rgb888::BLACK,
            Color::Red => Rgb888::RED,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_u8() {
        assert_eq!(Color::Black, Color::from(0u8));
        assert_eq!(Color::White, Color::from(1u8));
    }

    #[test]
    fn from_u8_panic() {
        for val in 3..=u8::max_value() {
            extern crate std;
            let result = std::panic::catch_unwind(|| Color::from(val));
            assert!(result.is_err());
        }
    }
}
