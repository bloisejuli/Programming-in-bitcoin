//use std::intrinsics::unreachable;
use std::ops::{Add};
use std::f32::INFINITY;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Point {
    a: f32,
    b: f32,
    x: f32,
    y: f32,
}

#[derive(PartialEq, Debug)]
enum CreationError {
    PointNotInCurve,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) ->  Self::Output {
        assert_ne!(self.a, other.a);
        assert_ne!(self.b, other.b);

        if self.x == INFINITY {
            return other;
        }  

        return self
    }
}

impl Point {
    fn new(x: f32, y: f32, a: f32, b: f32) -> Result<Point, CreationError>{
        if y.powf(2.0) != x.powf(3.0) + a * x + b {
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
        return self.x == other.x && self.y == other.y && self.a == other.a && self.b == other.b;
    }

    fn not_equal(&self, other: &Point) -> bool {
        return self.x != other.x || self.y != other.y || self.a != other.a || self.b != other.b;
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_not_equal() {
        let a: Point = Point{x: 3.0, y: -7.0, a: 5.0, b: 7.0};
        let b: Point = Point{x: 18.0, y: 77.0, a: 5.0, b: 7.0};

        assert!(a.equal(&a));
        assert!(a.not_equal(&b));
    }
 /* 
    #[test]
    fn test_add0() {
        let a: Point = Point{x: INFINITY, y: INFINITY, a: 5.0, b: 7.0};
        let b: Point = Point{x: 2.0, y: 5.0, a: 5.0, b: 7.0};
        let c: Point = Point{x: 2.0, y: -5.0, a: 5.0, b: 7.0};

        assert_eq!(a + b, b);
        assert_eq!(b + a, b);
        assert_eq!(b + c, a);
    }*/

    #[test]
    fn test_add1() {
        let a: Point = Point {x: 3.0, y: 7.0, a: 5.0, b: 7.0};
        let b: Point = Point {x:-1.0, y: -1.0, a: 5.0, b: 7.0};
        let c: Point = a + b;
        println!("HOLA !uhJKAHSDKAJSDHKJASHDASD");

        println!("C: {:?}",c);
        assert_eq!(a + b, Point {x: 2.0, y:-5.0, a:5.0, b: 7.0});
    }
}