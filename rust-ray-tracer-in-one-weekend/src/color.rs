use std::fmt;
use std::ops::{
    Add,
    Sub,
    Mul,
    Div,
};


// Easy switching between f64 and f32
// WARN: Make sure the actual type aliased by fsize match between vector.rs and color.rs
#[allow(non_camel_case_types)]
pub type fsize = f64;

#[derive(PartialEq, Debug)]
pub struct Color3(pub fsize, pub fsize, pub fsize); // Newtype Pattern, Not sure if I like it or not 

impl Color3 {
    // Self-Methods 
    pub fn r(&self) -> fsize {
        self.0
    }

    pub fn g(&self) -> fsize {
        self.1
    }

    pub fn b(&self) -> fsize {
        self.2
    }

    pub fn to_u8(&self) -> (u8, u8, u8) {
        (
            (self.0 * 255.999) as u8,
            (self.1 * 255.999) as u8,
            (self.2 * 255.999) as u8,
        )
    }
}

// This is the to_string trait implementation. I didn't realize this implicitly adds a to_string() method! 
impl fmt::Display for Color3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rgb = self.to_u8();
        write!(f, "{} {} {}", rgb.0, rgb.1, rgb.2)
    }
}

impl Add for Color3 {
    type Output = Color3;
    fn add(self, rhs: Color3) -> Color3 {
        Color3(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
        )
    }
}

impl Sub for Color3 {
    type Output = Color3;
    fn sub(self, rhs: Color3) -> Color3 {
        Color3(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2,
        )
    }
}

impl Mul for Color3 {
    type Output = Color3;
    fn mul(self, rhs: Color3) -> Color3 {
        Color3(
            self.0 * rhs.0,
            self.1 * rhs.1,
            self.2 * rhs.2,
        )
    }
}

impl Mul<Color3> for fsize {
    type Output = Color3;
    fn mul(self, rhs: Color3) -> Color3 {
        Color3(
            self * rhs.0,
            self * rhs.1,
            self * rhs.2,
        )
    }
}

impl Div<fsize> for Color3 {
    type Output = Color3;
    fn div(self, rhs: fsize) -> Color3 {
        Color3(
            self.0 / rhs,
            self.1 / rhs,
            self.2 / rhs,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Color3;
    #[test]
    fn create_color() {
        let color = Color3(0.0, 1.0, 2.0);
        
        // I don't know if I like this, but its pretty cool 
        assert_eq!(color.0, 0.0);
        assert_eq!(color.1, 1.0);
        assert_eq!(color.2, 2.0);
    }

    #[test]
    fn color_add() {
        let a = Color3(1.0, 2.0, 3.0);
        let b = Color3(1.5, 2.0, -5.0);

        let expected = Color3(2.5, 4.0, -2.0);

        assert_eq!(a + b, expected);
    }

    #[test]
    fn color_sub() {
        let a = Color3(1.0, 2.0, 3.0);
        let b = Color3(1.5, 2.0, -5.0);

        let expected = Color3(-0.5, 0.0, 8.0);

        assert_eq!(a - b, expected);
    }

    #[test]
    fn color_multiply() {
        let a = Color3(1.0, 2.0, 3.0);
        let b = Color3(1.5, 2.0, -5.0);

        let expected = Color3(1.5, 4.0, -15.0);

        assert_eq!(a * b, expected);
    }

    #[test]
    fn color_scalar_multiply() {
        let a = Color3(1.0, 2.0, 3.0);
        let scalar = 2.5;

        let expected = Color3(2.5, 5.0, 7.5);

        assert_eq!(scalar * a, expected);
    }

    #[test]
    fn color_scalar_division() {
        let a = Color3(1.0, 2.0, 3.0);
        let scalar = 2.0;

        let expected = Color3(0.5, 1.0, 1.5);

        assert_eq!(a / scalar, expected);
    }

    #[test]
    fn color_to_u8() {
        let color = Color3(1.0, 0.5, 0.2);
        
        let expected = (255, 127, 51);

        assert_eq!(color.to_u8(), expected);
    }

    #[test]
    fn color_to_string() {
        let color = Color3(1.0, 0.5, 0.2);

        let expected = "255 127 51";

        assert_eq!(color.to_string(), expected);
    }
}