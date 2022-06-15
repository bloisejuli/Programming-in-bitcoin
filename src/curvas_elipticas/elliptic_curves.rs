use std::ops::{Add};

pub struct Point {
    a: i32,
    b: i32,
    x: i32,
    y: i32,
}

#[derive(PartialEq, Debug)]
enum CreationError {
    PointNotInCurve,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        assert_ne!(self.a, other.a);
        assert_ne!(self.b, other.b);

        if self.x.is_none() {
            return other
        }

        if other.x.is_none() {
            return self
        }
    }
}

impl Point {
    fn new(x: i32, y: i32, a: i32, b: i32) -> Result<Point, CreationError>{
        if y**2 != x**3 + a * x + b {
            return Err(CreationError::PointNotInCurve);
        } 

        return Ok(Point {
            a,
            b,
            x,
            y,
        })
    }

    fn equal(&self, other: &Point) -> bool {
        return (self.x == other.x && self.y == other.y && self.a == other.a && self.b == other.b);
    }

    fn not_equal(&self, other: &Point) {
        return (self.x != other.x || self.y != other.y || self.a != other.a || self.b != other.b);
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_not_equal() {
        let a: Point = Point{x=3, y=-7, a=5, b=7};
        let b: Point = Point{x=18, y=77, a=5, b=7};

        assert_eq!(a, a);
        assert_ne!(a, b);
    }

    #[test]
    fn test_add0() {
        let a: Point = Point{x=None, y=None, a=5, b=7};
        let b: Point = Point{x=2, y=5, a=5, b=7};
        let c: Point = Point{x=2, y=-5, a=5, b=7};

        assert_eq!(a + b, b);
        assert_eq!(b + a, b);
        assert_eq!(b + c, a);
    }

    #[test]
    fn test_add1() {
        let a: Point = Point {x=3, y=7, a=5, b=7};
        let b: Point = Point {x=-1, y=-1, a=5, b=7};

        assert_eq!(a + b, Point {x=2, y=-5, a=5, b=7};)
    }

    
}