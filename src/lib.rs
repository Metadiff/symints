//! The crate provides manipulation, calculation and evaluation of integer polynomials.
//!
//! The central struct of the crate is `Polynomial<I, C, P>`. The three generic arguments
//! specify the types of three internal parts:
//!
//! `I: Id` - the type which uniquely identifies a single variable variable, e.g. if we have a^x ,
//! then `a: Id`. Note that this does not mean that it is an `Integer`, but rather it is a type
//! which can uniquely define variables. For instance one can use a `String` by naming each
//! variable. Importantly, the ordering of `I` defines the total ordering of the monomials
//! which is used to define the ordering of the polynomials as well -
//! [Wikipedia](https://en.wikipedia.org/wiki/Gr%C3%B6bner_basis#Monomial_ordering).
//!
//! `C: Coefficient` - the variable `Integer` type of the internal coefficients for each monomial.
//! The `Polynomial<C, I, P>` have implemented standard operators for interacting with type `C`.
//! Whenever you evaluate a polynomial, the output would be of this type.
//!
//! `P: Power` - the `Integer` type of the power values of each monomial, e.g. if we have a^x ,
//! then `x: Power`. Note that it is required that `P` is an `Unsigned` type.
//!
//! The choice of `C` and `P` should depend on the problem you are using the library for. If you
//! do not expect too high power values setting `P` to `u8` or `u16` should suffice.
//! In all of the tests `Polynomial<String, i64, u8>` is used.
//!
//! # Examples
//! ```
//! use std::collections::HashMap;
//!
//! extern crate symbolic_polynomials;
//! use symbolic_polynomials::*;
//!
//! type SymInt = Polynomial<String, i64, u8>;
//!
//! pub fn main() {
//!     // Create symbolic variables
//!     let a: &SymInt = &variable("a".into());
//!     let b: &SymInt = &variable("b".into());
//!     let c: &SymInt = &variable("c".into());
//!
//!     // Build polynomials
//!     // 5b + 2
//!     let poly1 = 5 * b + 2;
//!     // ab
//!     let poly2 = a * b;
//!     // ab + ac + b + c
//!     let poly3 = a * b + a * c + b + c;
//!     // a^2 - ab + 12
//!     let poly4 = a * a - a * b + 12;
//!     // ac^2 + 3a + bc^2 + 3b + c^2 + 3
//!     let poly5 = a * c * c + 3 * a + b * c * c + 3 * b + c * c + 3;
//!     // floor(a^2, b^2)
//!     let poly6 = floor(a * a, b * b);
//!     // ceil(a^2, b^2)
//!     let poly7 = ceil(a * a, b * b);
//!     // min(ab + 12, ab + a)
//!     let poly8 = min(a * b + 12, a * b + a);
//!     // max (ab + 12, ab + a)
//!     let poly9 = max(a * b + 12, a * b + a);
//!     // max(floor(a^2, b) - 4, ceil(c, b) + 1)
//!     let poly10 = max(floor(a * a, b) - 2, ceil(c, b) + 1);
//!
//!     // Polynomial printing
//!     let print_function = &|x: String| x;
//!     println!("{}", (0..50).map(|_| "=").collect::<String>());
//!     println!("Displaying polynomials (string representation = code representation):");
//!     println!("{} = {}", poly1, poly1.to_code(print_function));
//!     println!("{} = {}", poly2, poly2.to_code(print_function));
//!     println!("{} = {}", poly3, poly3.to_code(print_function));
//!     println!("{} = {}", poly4, poly4.to_code(print_function));
//!     println!("{} = {}", poly5, poly5.to_code(print_function));
//!     println!("{} = {}", poly6, poly6.to_code(print_function));
//!     println!("{} = {}", poly7, poly7.to_code(print_function));
//!     println!("{} = {}", poly8, poly8.to_code(print_function));
//!     println!("{} = {}", poly9, poly9.to_code(print_function));
//!     println!("{} = {}", poly10, poly10.to_code(print_function));
//!     println!("{}", (0..50).map(|_| "=").collect::<String>());
//!
//!     // Polynomial evaluation
//!     let values = &mut HashMap::<String, i64>::new();
//!     values.insert("a".into(), 3);
//!     values.insert("b".into(), 2);
//!     values.insert("c".into(), 5);
//!     println!("Evaluating for a = 3, b = 2, c = 5.");
//!     println!("{} = {} [Expected 12]", poly1, poly1.eval(values).unwrap());
//!     println!("{} = {} [Expected 6]", poly2, poly2.eval(values).unwrap());
//!     println!("{} = {} [Expected 28]", poly3, poly3.eval(values).unwrap());
//!     println!("{} = {} [Expected 15]", poly4, poly4.eval(values).unwrap());
//!     println!("{} = {} [Expected 168]", poly5, poly5.eval(values).unwrap());
//!     println!("{} = {} [Expected 2]", poly6, poly6.eval(values).unwrap());
//!     println!("{} = {} [Expected 3]", poly7, poly7.eval(values).unwrap());
//!     println!("{} = {} [Expected 9]", poly8, poly8.eval(values).unwrap());
//!     println!("{} = {} [Expected 18]", poly9, poly9.eval(values).unwrap());
//!     println!("{} = {} [Expected 4]", poly10, poly10.eval(values).unwrap());
//!     println!("{}", (0..50).map(|_| "=").collect::<String>());
//!
//!     // Variable deduction
//!     values.insert("a".into(), 5);
//!     values.insert("b".into(), 3);
//!     values.insert("c".into(), 8);
//!     let implicit_values = &vec![
//!         (&poly1, poly1.eval(values).unwrap()),
//!         (&poly2, poly2.eval(values).unwrap()),
//!         (&poly3, poly3.eval(values).unwrap()),
//!     ];
//!     let deduced_values = deduce_values(implicit_values).unwrap();
//!     println!("Deduced values:");
//!     println!("a = {} [Expected 5]", deduced_values["a"]);
//!     println!("b = {} [Expected 3]", deduced_values["b"]);
//!     println!("c = {} [Expected 8]", deduced_values["c"]);
//!     println!("{}", (0..50).map(|_| "=").collect::<String>());
//! }
//! ```
//! The main goal of the library is to provide integer polynomials which to be used
//! for automatic shape inference when modelling tensor shapes in symbolic mathematical engines
//! (such as Theano, Tensorflow, MXNet and others).
//! A small example of how this can be achieved is shown
//! [here](https://gist.github.com/botev/bac770e32f7df341ce18562f5333e5e5).

#[cfg(feature = "serialize")]
#[macro_use]
extern crate serde_derive;

extern crate num;

mod traits;
mod functions;
mod monomial;
mod polynomial;
mod composite;
mod integer_impl;

pub use traits::*;
pub use monomial::*;
pub use polynomial::*;
pub use composite::*;
pub use functions::*;
pub use integer_impl::*;
