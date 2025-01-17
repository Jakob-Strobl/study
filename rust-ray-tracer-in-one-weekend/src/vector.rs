use std::ops::{
    Neg,
    Add,
    Sub,
    Mul,
    Div,
}; // Overload operators 

// Easy switching between f64 and f32
// WARN: Make sure the actual type aliased by fsize match between vector.rs and color.rs
#[allow(non_camel_case_types)]
pub type fsize = f64;

#[derive(PartialEq, Debug, Clone)]
pub struct Vec3(pub fsize, pub fsize, pub fsize);

impl Vec3 {
    // Associated Methods
    pub fn new() -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }

    pub fn dot(lhs: &Vec3, rhs: &Vec3) -> fsize {
        lhs.0 * rhs.0 + lhs.1 * rhs.1 + lhs.2 * rhs.2
    }
    
    pub fn cross(lhs: &Vec3, rhs: &Vec3) -> Vec3 {
        Vec3(
            lhs.1 * rhs.2 - lhs.2 * rhs.1,
            lhs.2 * rhs.0 - lhs.0 * rhs.2,
            lhs.0 * rhs.1 - lhs.1 * rhs.0,
        )
    }

    // Self-Methods 
    pub fn x(&self) -> fsize {
        self.0
    }

    pub fn y(&self) -> fsize {
        self.1
    }

    pub fn z(&self) -> fsize {
        self.2
    }

    pub fn length(&self) -> fsize {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> fsize {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn normalize(&self) -> Vec3 {
        // *self / self.length() // Causes copy, which is fine but i want to see pain points without deriving 
        let length = self.length();
        Vec3(
            self.0 / length,
            self.1 / length,
            self.2 / length
        )
    }
}

// Move Operators - Takes ownership of the operands 
impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec3(
            -self.0, 
            -self.1,
            -self.2,
        )
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
        )
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2,
        )
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3(
            self.0 * rhs.0,
            self.1 * rhs.1,
            self.2 * rhs.2,
        )
    }
}

impl Mul<Vec3> for fsize {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3(
            self * rhs.0,
            self * rhs.1,
            self * rhs.2,
        )
    }
}

impl Div<fsize> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: fsize) -> Vec3 {
        Vec3(
            self.0 / rhs,
            self.1 / rhs,
            self.2 / rhs,
        )
    }
}

// Immutable Borrow Operators 
// For now I think the Lifetimes are related? I could make them different, but I want to see if it causes an issue. 
impl Neg for &Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3(
            -self.0,
            -self.1,
            -self.2
        )
    }
}

impl Add<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: &Vec3) -> Vec3 {
        Vec3(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
        )
    }
}

impl Add<Vec3> for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
        )
    }
}

impl Sub<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: &Vec3) -> Vec3 {
        Vec3(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2,
        )
    }
}

impl Sub<Vec3> for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2,
        )
    }
}

impl Mul<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Vec3 {
        Vec3(
            self.0 * rhs.0,
            self.1 * rhs.1,
            self.2 * rhs.2,
        )
    }
}

impl Mul<&Vec3> for fsize {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Vec3 {
        Vec3(
            self * rhs.0,
            self * rhs.1,
            self * rhs.2,
        )
    }
}

impl Div<fsize> for &Vec3 {
    type Output = Vec3;
    fn div(self, rhs: fsize) -> Vec3 {
        Vec3(
            self.0 / rhs,
            self.1 / rhs,
            self.2 / rhs,
        )
    }
}

// Unit tests 
#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    fn create_vector() {
        let vector = Vec3(0.0, 1.0, 2.0);
        
        assert_eq!(vector.0, 0.0);
        assert_eq!(vector.1, 1.0);
        assert_eq!(vector.2, 2.0);
    }

    #[test]
    fn vector_length() {
        let vector = Vec3(4.0, 0.0, 3.0);

        assert_eq!(vector.length(), 5.0);
    }
    
    #[test]
    fn vector_negate() {
        let vector = Vec3(1.0, 2.0, 3.0);
        let expected = Vec3(-1.0, -2.0, -3.0);

        assert_eq!(-vector, expected);
    }
    
    #[test]
    fn vector_add() {
        let a = Vec3(1.0, 2.0, 3.0);
        let b = Vec3(1.5, 2.0, -5.0);

        let expected = Vec3(2.5, 4.0, -2.0);

        assert_eq!(a + b, expected);
    }

    #[test]
    fn vector_sub() {
        let a = Vec3(1.0, 2.0, 3.0);
        let b = Vec3(1.5, 2.0, -5.0);

        let expected = Vec3(-0.5, 0.0, 8.0);

        assert_eq!(a - b, expected);
    }

    #[test]
    fn vector_multiply() {
        let a = Vec3(1.0, 2.0, 3.0);
        let b = Vec3(1.5, 2.0, -5.0);

        let expected = Vec3(1.5, 4.0, -15.0);

        assert_eq!(a * b, expected);
    }

    #[test]
    fn vector_scalar_multiply() {
        let a = Vec3(1.0, 2.0, 3.0);
        let scalar = 2.5;

        let expected = Vec3(2.5, 5.0, 7.5);

        assert_eq!(scalar * a, expected);
    }

    #[test]
    fn vector_scalar_division() {
        let a = Vec3(1.0, 2.0, 3.0);
        let scalar = 2.0;

        let expected = Vec3(0.5, 1.0, 1.5);

        assert_eq!(a / scalar, expected);
    }

    #[test]
    fn vector_normalize() {
        let vector = Vec3(1.0, 2.0, 3.0);
        
        let expected = Vec3(0.2672612419124244, 0.5345224838248488, 0.8017837257372732);

        assert_eq!(vector.normalize(), expected);
    }

    #[test]
    fn vector_reference_negate() {
        let vector = Vec3(1.0, 2.0, 3.0);
        let expected = Vec3(-1.0, -2.0, -3.0);

        assert_eq!(-&vector, expected);
        // Check we can still use vector after borrow
        assert_eq!(vector, Vec3(1.0, 2.0, 3.0));
    }

    #[test]
    fn vector_reference_add() {
        let a = Vec3(1.0, 2.0, 3.0);
        let b = Vec3(1.5, 2.0, -5.0);

        let expected = Vec3(2.5, 4.0, -2.0);

        assert_eq!(&a + &b, expected);
        // Check we can still use vector after borrow
        assert_eq!(a, Vec3(1.0, 2.0, 3.0));
        assert_eq!(b, Vec3(1.5, 2.0, -5.0));
    }

    #[test]
    fn vector_reference_sub() {
        let a = Vec3(1.0, 2.0, 3.0);
        let b = Vec3(1.5, 2.0, -5.0);

        let expected = Vec3(-0.5, 0.0, 8.0);

        assert_eq!(&a - &b, expected);
        // Check we can still use vector after borrow
        assert_eq!(a, Vec3(1.0, 2.0, 3.0));
        assert_eq!(b, Vec3(1.5, 2.0, -5.0));
    }

    #[test]
    fn vector_reference_multiply() {
        let a = Vec3(1.0, 2.0, 3.0);
        let b = Vec3(1.5, 2.0, -5.0);

        let expected = Vec3(1.5, 4.0, -15.0);

        assert_eq!(&a * &b, expected);
        // Check we can still use vector after borrow
        assert_eq!(a, Vec3(1.0, 2.0, 3.0));
        assert_eq!(b, Vec3(1.5, 2.0, -5.0));
    }

    #[test]
    fn vector_reference_scalar_multiply() {
        let a = Vec3(1.0, 2.0, 3.0);
        let scalar = 2.5;

        let expected = Vec3(2.5, 5.0, 7.5);

        assert_eq!(scalar * &a, expected);
        // Check we can still use vector after borrow
        assert_eq!(a, Vec3(1.0, 2.0, 3.0));
    }

    #[test]
    fn vector_reference_scalar_division() {
        let a = Vec3(1.0, 2.0, 3.0);
        let scalar = 2.0;

        let expected = Vec3(0.5, 1.0, 1.5);

        assert_eq!(&a / scalar, expected);
        // Check we can still use vector after borrow
        assert_eq!(a, Vec3(1.0, 2.0, 3.0));
    }

}