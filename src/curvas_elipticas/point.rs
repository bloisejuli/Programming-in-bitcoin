use std::{
    fmt, 
    ops::{Add, Mul}};

use rug::{Assign, Integer};

use crate::{campos_finitos::finite_fields::FieldElement};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Point {
    a: FieldElement,
    b: FieldElement,
    x: Option<FieldElement>,
    y: Option<FieldElement>,
}

#[derive(Debug)]
pub enum CreationError {
    PointNotInCurve,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) ->  Self::Output {
        assert_ne!(self.a, other.a);
        assert_ne!(self.b, other.b);

        if self.x == None {
            return other;
        } else if self.y == None { 
            return self
        }
        
        match (
            self.x.as_ref(),
            self.y.as_ref(),
            other.x.as_ref(),
            other.y.as_ref(),
        ) {
            (Some(x1), Some(y1), Some(x2), Some(y2)) => {
                let x1 = x1.clone();
                let y1 = y1.clone();
                let x2 = x2.clone();
                let y2 = y2.clone();

                if self.a != other.a || self.b != other.b {
                    panic!();
                }

                let x_sum;
                let y_sum;
                if self == other {
                    if y1 == FieldElement::new(Integer::from(0i32), y1.clone().prime()).unwrap() {
                        return Point {
                            x: None,
                            y: None,
                            a: self.a,
                            b: self.b,
                        };
                    }
                    let slope = (FieldElement::new(Integer::from(3i32), self.a.clone().prime()).unwrap()
                        * x1.pow(&Integer::from(2i32))
                        + self.a.clone())
                        / (FieldElement::new(Integer::from(2i32), self.a.clone().prime()).unwrap()
                            * y1.clone());
                    
                    x_sum = slope.pow(&Integer::from(2i32)) - x1.clone() - x2;
                    y_sum = slope * (x1 - x_sum.clone()) - y1;
                    
                } else {
                    let slope = (y2 - y1.clone()) / (x2.clone() - x1.clone());
                    x_sum = slope.pow(&Integer::from(2i32)) - x1.clone() - x2;
                    y_sum = slope * (x1 - x_sum.clone()) - y1;
                }

                Point {
                    a: self.a,
                    b: self.b,
                    x: Some(x_sum),
                    y: Some(y_sum),
                }
            }

            _ => Point::infinity(self.a, self.b),
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Point({}, {})_{}_{}",
            self.x.clone().unwrap(),
            self.y.clone().unwrap(),
            self.a,
            self.b
        )
    }
}

impl Mul<Point> for Integer {
    type Output = Point;

    fn mul(self, point: Point) -> Self::Output {
        let mut coef = self;
        let mut current = point.clone();
        let mut result = Point::infinity(point.a, point.b);

        while coef > 0 {
            if coef.is_odd() {
                result = result + current.clone(); 
            }
            current = current.clone() + current;
            coef >>= 1;
        }
        result
    }
}


impl Point {
    pub fn new(x: FieldElement, y: FieldElement, a: FieldElement, b: FieldElement) -> Result<Point, CreationError>{
        if y.pow(&Integer::from(2i32)) != x.pow(&Integer::from(3i32)) + a.clone() * x.clone() + b.clone() {
            return Err(CreationError::PointNotInCurve);
        } 

        return Ok(Point {
            a,
            b,
            x: Some(x),
            y: Some(y)
        })
    }

    pub fn infinity(a:FieldElement, b:FieldElement) -> Point {
        Point { a, b, x: None, y: None,}
    }

    pub fn equal(&self, other: &Point) -> bool {
        return self.x == other.x && self.y == other.y && self.a == other.a && self.b == other.b;
    }

    pub fn not_equal(&self, other: &Point) -> bool {
        return self.x != other.x || self.y != other.y || self.a != other.a || self.b != other.b;
    }



}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_on_curve() {
        let prime = 223;
        let a = 0;
        let b = 7;

        assert!(point(192, 105, a, b, prime).is_ok());
        assert!(point(17, 56, a, b, prime).is_ok());
        assert!(point(200, 119, a, b, prime).is_err());
        assert!(point(1, 193, a, b, prime).is_ok());
        assert!(point(42, 99, a, b, prime).is_err());
    }

    #[test]
    fn test_add() {
        let prime = 223;
        let a = 0;
        let b = 7;

        let p1 = point(192, 105, a, b, prime).unwrap();
        let p2 = point(17, 56, a, b, prime).unwrap();
        let p3 = point(170, 142, a, b, prime).unwrap();

        assert_eq!(p1 + p2, p3);

        let p1 = point(47, 71, a, b, prime).unwrap();
        let p2 = point(117, 141, a, b, prime).unwrap();
        let p3 = point(60, 139, a, b, prime).unwrap();

        assert_eq!(p1 + p2, p3);

        let p1 = point(143, 98, a, b, prime).unwrap();
        let p2 = point(76, 66, a, b, prime).unwrap();
        let p3 = point(47, 71, a, b, prime).unwrap();

        assert_eq!(p1 + p2, p3);
    }

    #[test]
    fn test_mul() {
        let prime = 223;
        let a = 0;
        let b = 7;

        let p1 = point(170, 142, a, b, prime).unwrap();
        assert_eq!(p1.clone() + p1.clone(), Integer::from(2) * p1.clone());
        assert_eq!(
            Integer::from(2) * p1.clone() + p1.clone(),
            Integer::from(3) * p1
        );
    }

    fn point(x: i128, y: i128, a: i128, b: i128, prime: i128) -> Result<Point, PointError> {
        Point::new(
            FieldElement::new(Integer::from(x), Integer::from(prime)).unwrap(),
            FieldElement::new(Integer::from(y), Integer::from(prime)).unwrap(),
            FieldElement::new(Integer::from(a), Integer::from(prime)).unwrap(),
            FieldElement::new(Integer::from(b), Integer::from(prime)).unwrap(),
        )
    }
}