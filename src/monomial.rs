use std::ops::{MulAssign, DivAssign, Add, Neg, Sub, Mul, Div};
use std::collections::{HashMap, HashSet};
use std::cmp::{Ord, Ordering};
use std::convert::AsRef;

use traits::*;
use polynomial::Polynomial;
use composite::Composite;

#[derive(Clone, Default, Debug, Eq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "repr_c", repr(C))]
/// A symbolic monomial represented as  `C * a_1^p_1 * a_2^p_2 * ... * a_n^p_n`.
pub struct Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    /// The constant coefficient C
    pub coefficient: C,
    /// A vector of the pairs `(a_i, p_i)`, where a_i is a `Composite` expression.
    pub powers: Vec<(Composite<I, C, P>, P)>,
}

impl<I, C, P> AsRef<Monomial<I, C, P>> for Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<I, C, P> Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    /// Returns `true` if the the two monomials are equal, ignoring their coefficients.
    pub fn up_to_coefficient<T: AsRef<Monomial<I, C, P>>>(&self, other: T) -> bool {
        let other = other.as_ref();
        if self.powers.len() == other.powers.len() {
            for (&(ref c, ref power), &(ref o_c, ref o_power)) in
                self.powers.iter().zip(other.powers.iter()) {
                if c != o_c || power != o_power {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }

    /// Returns `true` only if the monomial does not depend on any symbolic variables.
    pub fn is_constant(&self) -> bool {
        self.powers.is_empty()
    }

    /// Evaluates the `Monomial` given the provided mapping of identifiers to value assignments.
    pub fn eval(&self, values: &HashMap<I, C>) -> Result<C, (I, String)> {
        let mut value = self.coefficient.clone();
        for &(ref c, ref pow) in &self.powers {
            value *= ::num::pow(c.eval(values)?, pow.to_usize().unwrap());
        }
        Ok(value)
    }

    /// Returns a code equivalent string representation of the `Monomial`.
    /// The `format` specifies a function how to render the identifiers.
    pub fn to_code<F>(&self, format: &F) -> String
        where F: ::std::ops::Fn(I) -> String {
        let mut str: String = "".into();
        if self.coefficient == C::zero() {
            return "0".into();
        } else if self.coefficient == C::one() && self.powers.is_empty() {
            return "1".into();
        } else if self.coefficient == -C::one() && self.powers.is_empty() {
            return "- 1".into();
        } else if self.coefficient == -C::one() {
            str = "- ".into();
        } else if self.coefficient < C::zero() {
            str = "- ".into();
            str.push_str(&(-self.coefficient.clone()).to_string());
        } else if self.coefficient != C::one() {
            str = self.coefficient.clone().to_string();
        }
        let mut first = true;
        for &(ref c, ref pow) in &self.powers {
            if !first || (self.coefficient != C::one() && self.coefficient != -C::one()) {
                str.push_str(" * ");
            }
            if pow == &P::one() {
                str.push_str(&c.to_code(format));
            } else if pow != &P::zero() {
                str.push_str(&c.to_code(format));
                for _ in 0..(pow.clone() - P::one()).to_usize().unwrap() {
                    str.push_str(" * ");
                    str.push_str(&c.to_code(format));
                }
            }
            first = false;
        }
        str
    }

    /// Fills into the `HashSet` all of the identifiers used in this `Monomial`.
    pub fn unique_identifiers(&self, unique: &mut HashSet<I>) {
        for &(ref c, _) in &self.powers {
            c.unique_identifiers(unique);
        }
    }
}

impl<I, C, P> ::std::fmt::Display for Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        if self.coefficient == C::zero() {
            return write!(f, "0");
        } else if self.coefficient == C::one() && self.powers.is_empty() {
            return write!(f, "1");
        } else if self.coefficient == -C::one() && self.powers.is_empty() {
            return write!(f, "- 1");
        } else if self.coefficient == -C::one() {
            write!(f, "- ")?;
        } else if self.coefficient < C::zero() {
            write!(f, "- {}", -self.coefficient.clone())?;
        } else if self.coefficient != C::one() {
            write!(f, "{}", self.coefficient.clone())?;
        }
        for &(ref c, ref pow) in &self.powers {
            if pow == &P::one() {
                write!(f, "{}", c)?;
            } else if pow != &P::zero() {
                write!(f, "{}^{}", c, pow)?;
            }
        }
        Ok(())
    }
}

impl<I, C, P> From<C> for Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    fn from(other: C) -> Self {
        Monomial {
            coefficient: other,
            powers: Vec::new(),
        }
    }
}

impl<I, C, P> PartialEq<C> for Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    fn eq(&self, other: &C) -> bool {
        if self.coefficient == *other {
            self.is_constant()
        } else {
            false
        }
    }
}

impl<I, C, P> PartialEq for Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    fn eq(&self, other: &Monomial<I, C, P>) -> bool {
        if self.coefficient == other.coefficient {
            self.up_to_coefficient(other)
        } else {
            false
        }
    }
}

impl<I, C, P> PartialEq<Polynomial<I, C, P>> for Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    fn eq(&self, other: &Polynomial<I, C, P>) -> bool {
        other.eq(self)
    }
}

impl<I, C, P> PartialOrd<C> for Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    fn partial_cmp(&self, other: &C) -> Option<Ordering> {
        if self.is_constant() {
            self.coefficient.partial_cmp(&other.clone().into())
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl<I, C, P> PartialOrd for Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    fn partial_cmp(&self, other: &Monomial<I, C, P>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<I, C, P> PartialOrd<Polynomial<I, C, P>> for Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    fn partial_cmp(&self, other: &Polynomial<I, C, P>) -> Option<Ordering> {
        match other.partial_cmp(self) {
            Some(Ordering::Less) => Some(Ordering::Greater),
            Some(Ordering::Equal) => Some(Ordering::Equal),
            Some(Ordering::Greater) => Some(Ordering::Less),
            None => None,
        }
    }
}

impl<I, C, P> Ord for Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    fn cmp(&self, other: &Monomial<I, C, P>) -> Ordering {
        let min = ::std::cmp::min(self.powers.len(), other.powers.len());
        for i in 0..min {
            match Ord::cmp(&self.powers[i].0, &other.powers[i].0) {
                Ordering::Equal => {
                    match Ord::cmp(&self.powers[i].1, &other.powers[i].1) {
                        Ordering::Equal => {}
                        v => return v,
                    }
                }
                v => return v,
            }
        }
        match Ord::cmp(&self.powers.len(), &other.powers.len()) {
            Ordering::Equal => Ord::cmp(&self.coefficient, &other.coefficient),
            v => v,
        }
    }
}

impl<I, C, P> MulAssign<C> for Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    fn mul_assign(&mut self, rhs: C) {
        self.coefficient *= rhs;
    }
}

impl<'a, I, C, P> Mul<C> for &'a Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    type Output = Monomial<I, C, P>;
    fn mul(self, rhs: C) -> Self::Output {
        let mut result = self.clone();
        result *= rhs;
        result
    }
}

impl<'a, I, C, P> Mul<C> for Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    type Output = Monomial<I, C, P>;
    fn mul(self, rhs: C) -> Self::Output {
        (&self).mul(rhs)
    }
}

impl<'a, I, C, P> MulAssign<&'a Monomial<I, C, P>> for Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    fn mul_assign(&mut self, rhs: &'a Monomial<I, C, P>) {
        self.coefficient *= rhs.coefficient.clone();
        let mut i1 = 0;
        let mut i2 = 0;
        while i1 < self.powers.len() && i2 < rhs.powers.len() {
            match Ord::cmp(&self.powers[i1].0, &rhs.powers[i2].0) {
                Ordering::Greater => {}
                Ordering::Less => {
                    self.powers.insert(i1, rhs.powers[i2].clone());
                    i2 += 1;
                }
                Ordering::Equal => {
                    self.powers[i1] = (
                        self.powers[i1].0.clone(),
                        self.powers[i1].1.clone() + rhs.powers[i2].1.clone(),
                    );
                    i2 += 1;
                }
            }
            i1 += 1;
        }
        while i2 < rhs.powers.len() {
            self.powers.push(rhs.powers[i2].clone());
            i2 += 1;
        }
    }
}

impl<I, C, P> MulAssign<Monomial<I, C, P>> for Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    fn mul_assign(&mut self, rhs: Monomial<I, C, P>) {
        self.mul_assign(&rhs)
    }
}

impl<'a, 'b, I, C, P> Mul<&'b Monomial<I, C, P>> for &'a Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    type Output = Monomial<I, C, P>;
    fn mul(self, rhs: &'b Monomial<I, C, P>) -> Self::Output {
        let mut result = self.clone();
        result *= rhs;
        result
    }
}

impl<'b, I, C, P> Mul<&'b Monomial<I, C, P>> for Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    type Output = Monomial<I, C, P>;
    fn mul(self, rhs: &'b Monomial<I, C, P>) -> Self::Output {
        (&self).mul(rhs)
    }
}

impl<'a, I, C, P> Mul<Monomial<I, C, P>> for &'a Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    type Output = Monomial<I, C, P>;
    fn mul(self, rhs: Monomial<I, C, P>) -> Self::Output {
        self.mul(&rhs)
    }
}

impl<I, C, P> Mul<Monomial<I, C, P>> for Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    type Output = Monomial<I, C, P>;
    fn mul(self, rhs: Monomial<I, C, P>) -> Self::Output {
        (&self).mul(&rhs)
    }
}


impl<'a, 'b, I, C, P> Mul<&'a Polynomial<I, C, P>> for &'b Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    type Output = Polynomial<I, C, P>;
    fn mul(self, rhs: &'a Polynomial<I, C, P>) -> Self::Output {
        let mut result = rhs.clone();
        result *= self;
        result
    }
}

impl<'b, I, C, P> Mul<&'b Polynomial<I, C, P>> for Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    type Output = Polynomial<I, C, P>;
    fn mul(self, rhs: &'b Polynomial<I, C, P>) -> Self::Output {
        (&self).mul(rhs)
    }
}

impl<'a, I, C, P> Mul<Polynomial<I, C, P>> for &'a Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    type Output = Polynomial<I, C, P>;
    fn mul(self, rhs: Polynomial<I, C, P>) -> Self::Output {
        self.mul(&rhs)
    }
}

impl<I, C, P> Mul<Polynomial<I, C, P>> for Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    type Output = Polynomial<I, C, P>;
    fn mul(self, rhs: Polynomial<I, C, P>) -> Self::Output {
        (&self).mul(&rhs)
    }
}


// impl<I, C, P> CheckedDiv<C> for Monomial<I, C, P>
//    where I: Id, C: Coefficient, P: Power {
//    type Output = Monomial<I, C, P>;
//    fn checked_div(&self, other: C) -> Option<Self::Output> {
//        let (d, rem) = self.coefficient.div_rem(&other);
//        if rem == C::zero() {
//            Some(Monomial {
//                coefficient: d,
//                powers: self.powers.clone(),
//            })
//        } else {
//            None
//        }
//    }
//

impl<'a, I, C, P> Div<C> for &'a Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    type Output = Monomial<I, C, P>;
    fn div(self, other: C) -> Self::Output {
        self.checked_div(&other.into()).unwrap()
    }
}

impl<I, C, P> Div<C> for Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    type Output = Monomial<I, C, P>;
    fn div(self, other: C) -> Self::Output {
        self.checked_div(&other.into()).unwrap()
    }
}

impl<I, C, P> DivAssign<C> for Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    fn div_assign(&mut self, rhs: C) {
        let (d, rem) = self.coefficient.div_rem(&rhs);
        if rem == C::zero() {
            self.coefficient = d;
        } else {
            panic!("Non integer division via DivAssign")
        }
    }
}

// impl<'a, I, C, P> CheckedDiv<&'a Monomial<I, C, P>> for Monomial<I, C, P>
//    where I: Id, C: Coefficient, P: Power {
//    type Output = Monomial<I, C, P>;
impl<I, C, P> Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    /// If the the monomial is divisible by `rhs` than returns the result
    /// of that division, otherwise None.
    pub fn checked_div(&self, rhs: &Monomial<I, C, P>) -> Option<Self> {
        let (d, rem) = self.coefficient.div_rem(&rhs.coefficient);
        if rem == C::zero() {
            let mut result = Monomial {
                coefficient: d,
                powers: self.powers.clone(),
            };
            let mut i1 = 0;
            let mut i2 = 0;
            while i1 < result.powers.len() && i2 < rhs.powers.len() {
                match Ord::cmp(&result.powers[i1].0, &rhs.powers[i2].0) {
                    Ordering::Less => return None,
                    Ordering::Greater => {
                        i1 += 1;
                    }
                    Ordering::Equal => {
                        match Ord::cmp(&result.powers[i1].1, &rhs.powers[i2].1) {
                            Ordering::Less => return None,
                            Ordering::Equal => {
                                result.powers.remove(i1);
                                i2 += 1;
                            }
                            Ordering::Greater => {
                                result.powers[i1] = (
                                    result.powers[i1].0.clone(),
                                    result.powers[i1].1.clone() - rhs.powers[i2].1.clone(),
                                );
                                i1 += 1;
                                i2 += 1;
                            }
                        }
                    }
                }
            }
            if i2 < rhs.powers.len() {
                None
            } else {
                Some(result)
            }
        } else {
            None
        }
    }
}

impl<'a, 'b, I, C, P> Div<&'b Monomial<I, C, P>> for &'a Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    type Output = Monomial<I, C, P>;
    fn div(self, rhs: &'b Monomial<I, C, P>) -> Self::Output {
        self.checked_div(rhs).unwrap()
    }
}

impl<'b, I, C, P> Div<&'b Monomial<I, C, P>> for Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    type Output = Monomial<I, C, P>;
    fn div(self, rhs: &'b Monomial<I, C, P>) -> Self::Output {
        self.checked_div(rhs).unwrap()
    }
}

impl<'a, I, C, P> Div<Monomial<I, C, P>> for &'a Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    type Output = Monomial<I, C, P>;
    fn div(self, rhs: Monomial<I, C, P>) -> Self::Output {
        self.checked_div(&rhs).unwrap()
    }
}
impl<I, C, P> Div<Monomial<I, C, P>> for Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    type Output = Monomial<I, C, P>;
    fn div(self, rhs: Monomial<I, C, P>) -> Self::Output {
        self.checked_div(&rhs).unwrap()
    }
}

impl<'a, I, C, P> DivAssign<&'a Monomial<I, C, P>> for Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    fn div_assign(&mut self, rhs: &'a Monomial<I, C, P>) {
        let result = (self as &Monomial<I, C, P>).checked_div(rhs).unwrap();
        self.coefficient = result.coefficient;
        self.powers = result.powers.clone();
    }
}

impl<I, C, P> DivAssign<Monomial<I, C, P>> for Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    fn div_assign(&mut self, rhs: Monomial<I, C, P>) {
        self.div_assign(&rhs)
    }
}

impl<'a, I, C, P> Neg for &'a Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    type Output = Monomial<I, C, P>;
    fn neg(self) -> Self::Output {
        Monomial {
            coefficient: -self.coefficient.clone(),
            powers: self.powers.clone(),
        }
    }
}

impl<I, C, P> Neg for Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    type Output = Monomial<I, C, P>;
    fn neg(mut self) -> Self::Output {
        self.coefficient = -self.coefficient;
        self
    }
}

impl<'a, I, C, P> Add<C> for &'a Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    type Output = Polynomial<I, C, P>;
    fn add(self, rhs: C) -> Self::Output {
        if rhs == C::zero() {
            Polynomial { monomials: vec![self.clone()] }
        } else if self.is_constant() {
            if rhs == -self.coefficient.clone() {
                Polynomial { monomials: Vec::new() }
            } else {
                let mut result = Polynomial::from(self);
                result.monomials[0].coefficient += rhs;
                result
            }
        } else {
            Polynomial { monomials: vec![self.clone(), Monomial::from(rhs)] }
        }
    }
}

impl<I, C, P> Add<C> for Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    type Output = Polynomial<I, C, P>;
    fn add(self, rhs: C) -> Self::Output {
        (&self).add(rhs)
    }
}

impl<'a, 'b, I, C, P> Add<&'b Monomial<I, C, P>> for &'a Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    type Output = Polynomial<I, C, P>;
    fn add(self, rhs: &'b Monomial<I, C, P>) -> Self::Output {
        if rhs.coefficient == C::zero() && self.coefficient == C::zero() {
            Polynomial { monomials: Vec::new() }
        } else if rhs.coefficient == C::zero() {
            Polynomial { monomials: vec![self.clone()] }
        } else if self.coefficient == C::zero() {
            Polynomial { monomials: vec![rhs.clone()] }
        } else if self.up_to_coefficient(rhs) {
            if self.coefficient == -rhs.coefficient.clone() {
                Polynomial { monomials: Vec::new() }
            } else {
                let mut result = Polynomial { monomials: vec![self.clone()] };
                result.monomials[0].coefficient += rhs.coefficient.clone();
                result
            }
        } else if self > rhs {
            Polynomial { monomials: vec![self.clone(), rhs.clone()] }
        } else {
            Polynomial { monomials: vec![rhs.clone(), self.clone()] }
        }
    }
}

impl<'b, I, C, P> Add<&'b Monomial<I, C, P>> for Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    type Output = Polynomial<I, C, P>;
    fn add(self, rhs: &'b Monomial<I, C, P>) -> Self::Output {
        (&self).add(rhs)
    }
}

impl<'a, I, C, P> Add<Monomial<I, C, P>> for &'a Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    type Output = Polynomial<I, C, P>;
    fn add(self, rhs: Monomial<I, C, P>) -> Self::Output {
        self.add(&rhs)
    }
}

impl<I, C, P> Add<Monomial<I, C, P>> for Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    type Output = Polynomial<I, C, P>;
    fn add(self, rhs: Monomial<I, C, P>) -> Self::Output {
        (&self).add(&rhs)
    }
}


impl<'a, 'b, I, C, P> Add<&'b Polynomial<I, C, P>> for &'a Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    type Output = Polynomial<I, C, P>;
    fn add(self, rhs: &'b Polynomial<I, C, P>) -> Self::Output {
        let mut result = rhs.clone();
        result += self;
        result
    }
}

impl<'b, I, C, P> Add<&'b Polynomial<I, C, P>> for Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    type Output = Polynomial<I, C, P>;
    fn add(self, rhs: &'b Polynomial<I, C, P>) -> Self::Output {
        (&self).add(rhs)
    }
}

impl<'a, I, C, P> Add<Polynomial<I, C, P>> for &'a Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    type Output = Polynomial<I, C, P>;
    fn add(self, rhs: Polynomial<I, C, P>) -> Self::Output {
        self.add(&rhs)
    }
}

impl<I, C, P> Add<Polynomial<I, C, P>> for Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    type Output = Polynomial<I, C, P>;
    fn add(self, rhs: Polynomial<I, C, P>) -> Self::Output {
        (&self).add(&rhs)
    }
}

impl<'a, I, C, P> Sub<C> for &'a Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    type Output = Polynomial<I, C, P>;
    fn sub(self, rhs: C) -> Self::Output {
        if rhs == C::zero() {
            Polynomial { monomials: vec![self.clone()] }
        } else if self.is_constant() {
            if rhs == self.coefficient {
                Polynomial { monomials: Vec::new() }
            } else {
                let mut result = Polynomial::from(self);
                result.monomials[0].coefficient -= rhs;
                result
            }
        } else {
            Polynomial { monomials: vec![self.clone(), Monomial::from(-rhs)] }
        }
    }
}

impl<I, C, P> Sub<C> for Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    type Output = Polynomial<I, C, P>;
    fn sub(self, rhs: C) -> Self::Output {
        (&self).sub(rhs)
    }
}

impl<'a, 'b, I, C, P> Sub<&'b Monomial<I, C, P>> for &'a Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    type Output = Polynomial<I, C, P>;
    fn sub(self, rhs: &'b Monomial<I, C, P>) -> Self::Output {
        if self.coefficient == C::zero() && rhs.coefficient == C::zero() {
            Polynomial { monomials: Vec::new() }
        } else if rhs.coefficient == C::zero() {
            Polynomial { monomials: vec![self.clone()] }
        } else if self.coefficient == C::zero() {
            Polynomial { monomials: vec![-rhs] }
        } else if self.up_to_coefficient(rhs) {
            if self.coefficient == rhs.coefficient {
                Polynomial { monomials: Vec::new() }
            } else {
                let mut result = Polynomial { monomials: vec![self.clone()] };
                result.monomials[0].coefficient -= rhs.coefficient.clone();
                result
            }
        } else if self > rhs {
            Polynomial { monomials: vec![self.clone(), -rhs] }
        } else {
            Polynomial { monomials: vec![-rhs, self.clone()] }
        }
    }
}

impl<'b, I, C, P> Sub<&'b Monomial<I, C, P>> for Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    type Output = Polynomial<I, C, P>;
    fn sub(self, rhs: &'b Monomial<I, C, P>) -> Self::Output {
        (&self).sub(rhs)
    }
}

impl<'a, I, C, P> Sub<Monomial<I, C, P>> for &'a Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    type Output = Polynomial<I, C, P>;
    fn sub(self, rhs: Monomial<I, C, P>) -> Self::Output {
        self.sub(&rhs)
    }
}

impl<I, C, P> Sub<Monomial<I, C, P>> for Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    type Output = Polynomial<I, C, P>;
    fn sub(self, rhs: Monomial<I, C, P>) -> Self::Output {
        (&self).sub(&rhs)
    }
}

impl<'a, 'b, I, C, P> Sub<&'b Polynomial<I, C, P>> for &'a Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    type Output = Polynomial<I, C, P>;
    fn sub(self, rhs: &'b Polynomial<I, C, P>) -> Self::Output {
        let mut result = -rhs;
        result += self;
        result
    }
}

impl<'b, I, C, P> Sub<&'b Polynomial<I, C, P>> for Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    type Output = Polynomial<I, C, P>;
    fn sub(self, rhs: &'b Polynomial<I, C, P>) -> Self::Output {
        (&self).sub(rhs)
    }
}

impl<'a, I, C, P> Sub<Polynomial<I, C, P>> for &'a Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    type Output = Polynomial<I, C, P>;
    fn sub(self, rhs: Polynomial<I, C, P>) -> Self::Output {
        self.sub(&rhs)
    }
}

impl<I, C, P> Sub<Polynomial<I, C, P>> for Monomial<I, C, P>
    where I: Id,
          C: Coefficient,
          P: Power {
    type Output = Polynomial<I, C, P>;
    fn sub(self, rhs: Polynomial<I, C, P>) -> Self::Output {
        (&self).sub(&rhs)
    }
}
