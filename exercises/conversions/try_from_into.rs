// TryFrom is a simple and safe type conversion that may fail in a controlled way under some circumstances.
// Basically, this is the same as From. The main difference is that this should return a Result type
// instead of the target type itself.
// You can read more about it at https://doc.rust-lang.org/std/convert/trait.TryFrom.html
use error::Error;
use std::convert::{TryFrom, TryInto};
use std::error;

#[derive(Debug, PartialEq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl TryFrom<(i16, i16, i16)> for Color {
    type Error = Box<dyn error::Error>;
    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        let (red, green, blue) = tuple;
        Ok(Color{
                red: try_convert_u16_to_u8(red)?,
                green: try_convert_u16_to_u8(green)?,
                blue: try_convert_u16_to_u8(blue)?
            })
    }
}

fn try_convert_u16_to_u8(integer: i16) -> Result<u8, Box<dyn error::Error>> {
    if is_valid(integer) {
        Ok(integer as u8)
    }
    else {
        Err(Box::<dyn Error>::from("Invalid color value!"))
    }
}

fn is_valid(value: i16) -> bool {
    value >= 0
            && value <= 255
}

// Array implementation
impl TryFrom<[i16; 3]> for Color {
    type Error = Box<dyn error::Error>;
    fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
        Color::try_from((arr[0], arr[1], arr[2]))
    }
}

// Slice implementation
impl TryFrom<&[i16]> for Color {
    type Error = Box<dyn error::Error>;
    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
        if slice.len() == 3 {
            Color::try_from((slice[0], slice[1], slice[2]))
        }
        else {
            Err(Box::<dyn Error>::from("Bad length!"))
        }
    }
}

fn main() {
    // Use the `from` function
    let c1 = Color::try_from((183, 65, 14));
    println!("{:?}", c1);

    // Since From is implemented for Color, we should be able to use Into
    let c2: Result<Color, _> = [183, 65, 14].try_into();
    println!("{:?}", c2);

    let v = vec![183, 65, 14];
    // With slice we should use `from` function
    let c3 = Color::try_from(&v[..]);
    println!("{:?}", c3);
    // or take slice within round brackets and use Into
    let c4: Result<Color, _> = (&v[..]).try_into();
    println!("{:?}", c4);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuple_out_of_range_positive() {
        assert!(Color::try_from((256, 1000, 10000)).is_err());
    }
    #[test]
    fn test_tuple_out_of_range_negative() {
        assert!(Color::try_from((-1, -10, -256)).is_err());
    }
    #[test]
    fn test_tuple_sum() {
        assert!(Color::try_from((-1, 255, 255)).is_err());
    }
    #[test]
    fn test_tuple_correct() {
        let c: Result<Color, _> = (183, 65, 14).try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }
    #[test]
    fn test_array_out_of_range_positive() {
        let c: Result<Color, _> = [1000, 10000, 256].try_into();
        assert!(c.is_err());
    }
    #[test]
    fn test_array_out_of_range_negative() {
        let c: Result<Color, _> = [-10, -256, -1].try_into();
        assert!(c.is_err());
    }
    #[test]
    fn test_array_sum() {
        let c: Result<Color, _> = [-1, 255, 255].try_into();
        assert!(c.is_err());
    }
    #[test]
    fn test_array_correct() {
        let c: Result<Color, _> = [183, 65, 14].try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }
    #[test]
    fn test_slice_out_of_range_positive() {
        let arr = [10000, 256, 1000];
        assert!(Color::try_from(&arr[..]).is_err());
    }
    #[test]
    fn test_slice_out_of_range_negative() {
        let arr = [-256, -1, -10];
        assert!(Color::try_from(&arr[..]).is_err());
    }
    #[test]
    fn test_slice_sum() {
        let arr = [-1, 255, 255];
        assert!(Color::try_from(&arr[..]).is_err());
    }
    #[test]
    fn test_slice_correct() {
        let v = vec![183, 65, 14];
        let c: Result<Color, _> = Color::try_from(&v[..]);
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }
    #[test]
    fn test_slice_excess_length() {
        let v = vec![0, 0, 0, 0];
        assert!(Color::try_from(&v[..]).is_err());
    }
    #[test]
    fn test_slice_insufficient_length() {
        let v = vec![0, 0];
        assert!(Color::try_from(&v[..]).is_err());
    }
}
