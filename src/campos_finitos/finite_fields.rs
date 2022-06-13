use std::ops::{Add, Sub, Mul, Div};

#[derive(PartialEq, Debug)]
pub struct FieldElement {
    num: i32,
    prime: i32,
}

fn modular_pow(base: i32, exponent: i32, modulus: i32) -> i32{
    if modulus == 1 {
        return 0;
    }
    
    let mut c = 1;
    
    for _ in 0 .. exponent {
        c = (c * base).rem_euclid(modulus);
    }

    return c;
}
enum CreationError {
    Negative,
    NumberGreaterPrime,
    NumbersInDiferentFields
}

impl Add for FieldElement {
    type Output = FieldElement;

    fn add(self, other: FieldElement) -> FieldElement {
        assert_eq!(self.prime, other.prime);
        FieldElement {num: (self.num + other.num).rem_euclid(self.prime), prime: self.prime}
    }
}

impl Sub for FieldElement {
    type Output = FieldElement;

    fn sub(self, other: FieldElement) -> FieldElement {
        assert_eq!(self.prime, other.prime);
        FieldElement { num: (self.num - other.num).rem_euclid(self.prime), prime: self.prime }
    }
}

impl Mul for FieldElement {
    type Output = FieldElement;

    fn mul(self, other: FieldElement) -> Self::Output {
        assert_eq!(self.prime, other.prime);
        
        FieldElement {num: (self.num * other.num).rem_euclid(self.prime),  prime: self.prime}
    }
}

impl Div for FieldElement {
    type Output = FieldElement;

    fn div(self, other: FieldElement) -> Self::Output {
        assert_eq!(self.prime, other.prime);
        assert_eq!(other.prime, 0);

        FieldElement {
            num: (self.num * modular_pow(other.num, self.prime -2, self.prime)).rem_euclid(self.prime),
            prime: self.prime,
        }
    }
}
 

    
impl FieldElement {
    fn new(num:i32, prime:i32) -> Result<FieldElement, CreationError> {
        if num >= prime {
            return Err(CreationError::NumberGreaterPrime);
        } else if num < 0 {
            return Err(CreationError::Negative);
        } else {
            Ok(FieldElement {
                num,
                prime,
            })
        }      
    }

    fn repr() -> String{
        return "FieldElement_{}({})".to_string()
    } 

    fn equal(&self, other: &FieldElement) -> bool {
        if self.num == other.num && self.prime == other.prime {
            return true
        }
        else {
            return false
        } 
    }
 
    fn not_equal(&self, other: &FieldElement) -> bool {
        return self.num != other.num || self.prime != other.prime
    }

    fn pow(&self, exponent: i32) ->  FieldElement {
        let n: i32 = exponent.rem_euclid((self.prime - 1) as i32);
        let number = modular_pow(self.num, n, self.prime);
        return FieldElement { num: number, prime: self.prime }
    }
}





#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_not_equal() {
        let a = FieldElement{num:2, prime:31};
        let b = FieldElement{num:2, prime:31};
        let c = FieldElement{num:15, prime:31};
        assert_eq!(a.equal(&b), true);
        assert_eq!(a.equal(&c), false);
        assert_eq!(a.not_equal(&b), false);

    }

     
    #[test]
    fn test_add() {
        let a = FieldElement{num:2, prime:31};
        let b = FieldElement{num:15, prime:31};
        assert_eq!(a + b, FieldElement{num: 17, prime:31});

        let a = FieldElement{num:17, prime:31};
        let b = FieldElement{num:21, prime:31};
        assert_eq!(a + b, FieldElement{num: 7, prime:31});
        
    }

    #[test]
    fn test_sub() {
        let a = FieldElement{num:29, prime:31};
        let b = FieldElement{num:4, prime:31};
        assert_eq!(a - b, FieldElement{num: 25, prime:31});

        let a = FieldElement{num:15, prime:31};
        let b = FieldElement{num:30, prime:31};
        assert_eq!(a - b, FieldElement{num: 16, prime:31});
        
    }

    #[test]
    fn test_mul() {
        let a = FieldElement{num:24, prime:31};
        let b = FieldElement{num:19, prime:31};
        assert_eq!(a * b, FieldElement{num: 22, prime:31});
        
    }

    #[test]
    fn test_pow() {
        let a: FieldElement = FieldElement{num:17, prime:31};
        assert_eq!(a.pow(3), FieldElement{num:15, prime:31});

        let a: FieldElement = FieldElement { num: 5, prime: 31 };
        let b: FieldElement = FieldElement { num: 18, prime: 31 };
        assert_eq!(a.pow(5)*b, FieldElement{num: 16, prime:31});
        
    }

    #[test]
    fn test_div() {
        let a: FieldElement = FieldElement { num: 3, prime: 31 };
        let b: FieldElement = FieldElement { num: 24, prime: 31 };
        assert_eq!(a/b, FieldElement{num:4, prime:31});

        let a: FieldElement = FieldElement{num:17, prime:31};
        assert_eq!(a.pow(-3), FieldElement{num:29, prime:31});

        let a: FieldElement = FieldElement { num: 4, prime: 31 };
        let b: FieldElement = FieldElement { num: 11, prime: 31 };
        assert_eq!(a.pow(-4) * b, FieldElement{num:13, prime:31});

    }

}
