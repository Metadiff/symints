use std::collections::HashMap;
extern crate symbolic_polynomials;
use symbolic_polynomials::*;

#[allow(dead_code)]
type TestMonomial = Monomial<String, i64, u8>;
#[allow(dead_code)]
type TestPolynomial = Polynomial<String, i64, u8>;

#[test]
pub fn constructor() {
    let a: TestPolynomial = variable("a".into());
    let b_mon = TestMonomial {
        coefficient: 1,
        powers: vec![(Composite::Variable("b".into()), 1)],
    };
    let b = TestPolynomial::from(&(5 * &b_mon));
    let minus_six = TestPolynomial::from(-6);
    let thirteen = TestPolynomial::from(13);

    assert!(minus_six.is_constant());
    assert_eq!(minus_six.monomials.len(), 1);
    assert_eq!(minus_six.monomials[0].coefficient, -6);
    assert_eq!(minus_six.monomials[0].powers.len(), 0);

    assert!(thirteen.is_constant());
    assert_eq!(thirteen.monomials.len(), 1);
    assert_eq!(thirteen.monomials[0].coefficient, 13);
    assert_eq!(thirteen.monomials[0].powers.len(), 0);

    assert!(!a.is_constant());
    assert_eq!(a.monomials.len(), 1);
    assert_eq!(a.monomials[0].coefficient, 1);
    assert_eq!(a.monomials[0].powers, vec![(Composite::Variable("a".into()), 1)]);

    assert!(!b.is_constant());
    assert_eq!(b.monomials.len(), 1);
    assert_eq!(b.monomials[0].coefficient, 5);
    assert_eq!(b.monomials[0].powers, vec![(Composite::Variable("b".into()), 1)]);
}

#[test]
pub fn partial_eq_test() {
    let a: TestPolynomial = variable("a".into());
    let b: TestPolynomial = variable("b".into());
    let a_v2: TestPolynomial = variable("a".into());
    let b_v2: TestPolynomial = variable("b".into());
    // ab
    let ab = &a * &b;
    // a + b
    let a_plus_b = &a + &b;
    // a^2
    let a_square = &a * &a;
    // b^2
    let b_square = &b * &b;
    // a^2 + b^2 + 2ab = (a + b)^2
    let a_plus_b_square = &a_square + &b_square + 2 * &ab;

    assert_ne!(a, 1);
    assert_ne!(1, a);
    assert_ne!(b, 1);
    assert_ne!(1, b);

    assert_eq!(a, a_v2);
    assert_eq!(a_v2, a);
    assert_eq!(b, b_v2);
    assert_eq!(b_v2, b);


    assert_ne!(ab, a);
    assert_ne!(a, ab);
    assert_ne!(ab, b);
    assert_ne!(b, ab);

    assert_ne!(a_square, a);
    assert_ne!(a, a_square);
    assert_ne!(b_square, b);
    assert_ne!(b, b_square);
    assert_eq!(a_plus_b_square, (&a_plus_b * &a_plus_b));
}

#[test]
pub fn ord_test() {
    let a: TestPolynomial = variable("a".into());
    let b: TestPolynomial = variable("b".into());
    // a^2
    let a_square = &a * &a;
    // b^2
    let b_square = &b * &b;
    // a^2b
    let a_square_b = &a * &a * &b;
    // a^2b + a
    let a_square_b_plus_a = &a_square_b + &a;
    // a^2b + b
    let a_square_b_plus_b = &a_square_b + &b;
    // a + b
    let a_plus_b = &a + &b;
    // (a + b)^2
    let a_plus_b_square = &a_plus_b * &a_plus_b;

    assert!(a > 2);
    assert!(2 < a);
    assert!(b > 2);
    assert!(2 < b);

    assert!(a > b_square);
    assert!(b_square < a);
    assert!(b < b_square);
    assert!(b_square > b);
    assert!(a < a_square);
    assert!(a_square > a);
    assert!(b < a_square);
    assert!(a_square > b);
    assert!(b_square < a_square);
    assert!(a_square > b_square);

    assert!(a_square_b > a_square);
    assert!(a_square < a_square_b);
    assert!(a_square_b > b_square);
    assert!(b_square < a_square_b);
    assert!(a_square_b < a_square_b_plus_a);
    assert!(a_square_b_plus_a > a_square_b);
    assert!(a_square_b < a_square_b_plus_b);
    assert!(a_square_b_plus_b > a_square_b);
    assert!(a_square_b_plus_b < a_square_b_plus_a);
    assert!(a_square_b_plus_a > a_square_b_plus_b);

    assert!(a_plus_b > a);
    assert!(a < a_plus_b);
    assert!(a_plus_b > b);
    assert!(b < a_plus_b);
    assert!(a_plus_b < a_square);
    assert!(a_square > a_plus_b);
    assert!(a_plus_b > b_square);
    assert!(b_square < a_plus_b);

    assert!(a_plus_b_square > a_square);
    assert!(a_square < a_plus_b_square);
    assert!(a_plus_b_square < a_square_b_plus_a);
    assert!(a_square_b_plus_a > a_plus_b_square);
    assert!(a_plus_b_square < a_square_b_plus_b);
    assert!(a_square_b_plus_b > a_plus_b_square);
}

#[test]
pub fn mul_test() {
    let a: TestPolynomial = variable("a".into());
    let b: TestPolynomial = variable("b".into());
    // ab + a^2 + 1
    let ab_plus_a_square_plus_one = &a * &b + &a * &a + 1;
    // ab + b^2 + 1
    let ab_plus_b_square_plus_two = &a * &b + &b * &b + 2;
    // a^3b + 2a^2b^2 + 2a^2 + ab^3 + 3ab + b^2 + 2
    let product = &ab_plus_a_square_plus_one * &ab_plus_b_square_plus_two;

    assert_eq!(product.monomials.len(), 7);
    assert_eq!(product.monomials[0].coefficient, 1);
    assert_eq!(
        product.monomials[0].powers,
        vec![
            (Composite::Variable("a".into()), 3),
            (Composite::Variable("b".into()), 1),
        ]
    );
    assert_eq!(product.monomials[1].coefficient, 2);
    assert_eq!(
        product.monomials[1].powers,
        vec![
            (Composite::Variable("a".into()), 2),
            (Composite::Variable("b".into()), 2),
        ]
    );
    assert_eq!(product.monomials[2].coefficient, 2);
    assert_eq!(product.monomials[2].powers, vec![(Composite::Variable("a".into()), 2)]);
    assert_eq!(product.monomials[3].coefficient, 1);
    assert_eq!(
        product.monomials[3].powers,
        vec![
            (Composite::Variable("a".into()), 1),
            (Composite::Variable("b".into()), 3),
        ]
    );
    assert_eq!(product.monomials[4].coefficient, 3);
    assert_eq!(
        product.monomials[4].powers,
        vec![
            (Composite::Variable("a".into()), 1),
            (Composite::Variable("b".into()), 1),
        ]
    );
    assert_eq!(product.monomials[5].coefficient, 1);
    assert_eq!(product.monomials[5].powers, vec![(Composite::Variable("b".into()), 2)]);
    assert_eq!(product.monomials[6].coefficient, 2);
    assert_eq!(product.monomials[6].powers.len(), 0);
}

#[test]
pub fn div_test() {
    let a: TestPolynomial = variable("a".into());
    let b: TestPolynomial = variable("b".into());
    // ab + a^2 + 1
    let ab_plus_a_square_plus_one = &a * &b + &a * &a + 1;
    // ab + b^2 + 1
    let ab_plus_b_square_plus_two = &a * &b + &b * &b + 2;
    // a^3b + 2a^2b^2 + 2a^2 + ab^3 + 3ab + b^2 + 2
    let product = &ab_plus_a_square_plus_one * &ab_plus_b_square_plus_two;
    // (ab + a^2 + 1) = a * (a + b) + 1
    let (a_plus_b, one) = ab_plus_a_square_plus_one.div_rem(&a);

    assert_eq!(&product / &ab_plus_a_square_plus_one, ab_plus_b_square_plus_two);
    assert_eq!(&product / &ab_plus_b_square_plus_two, ab_plus_a_square_plus_one);

    assert_eq!(a_plus_b, &a + &b);
    assert_eq!(one, 1);
    assert!(product.checked_div(&(&a * &a)).is_none());
    assert!(product.checked_div(&(&b * &b)).is_none());
    assert!(product.checked_div(&2.into()).is_none());
    assert_eq!(product.checked_div(&1.into()).unwrap(), product);
}

#[test]
pub fn add_test() {
    let a_mon = TestMonomial {
        coefficient: 1,
        powers: vec![(Composite::Variable("a".into()), 1)],
    };
    let a: TestPolynomial = variable("a".into());
    let b: TestPolynomial = variable("b".into());
    // a + b + 1
    let a_plus_b_plus_1_v1 = &a + &b + 1;
    let a_plus_b_plus_1_v2 = &a_mon + &b + 1;
    // 2a + 2b + 2
    let a_plus_b_plus_1_times_2 = &a_plus_b_plus_1_v1 + &a_plus_b_plus_1_v2;

    assert_eq!(a_plus_b_plus_1_v1.monomials.len(), 3);
    assert_eq!(a_plus_b_plus_1_v1.monomials[0].coefficient, 1);
    assert_eq!(
        a_plus_b_plus_1_v1.monomials[0].powers,
        vec![(Composite::Variable("a".into()), 1)]
    );
    assert_eq!(a_plus_b_plus_1_v1.monomials[1].coefficient, 1);
    assert_eq!(
        a_plus_b_plus_1_v1.monomials[1].powers,
        vec![(Composite::Variable("b".into()), 1)]
    );
    assert_eq!(a_plus_b_plus_1_v1.monomials[2].coefficient, 1);
    assert_eq!(a_plus_b_plus_1_v1.monomials[2].powers.len(), 0);
    assert_eq!(a_plus_b_plus_1_v1, a_plus_b_plus_1_v2);

    assert_eq!(a_plus_b_plus_1_times_2.monomials.len(), 3);
    assert_eq!(a_plus_b_plus_1_times_2.monomials[0].coefficient, 2);
    assert_eq!(
        a_plus_b_plus_1_times_2.monomials[0].powers,
        vec![(Composite::Variable("a".into()), 1)]
    );
    assert_eq!(a_plus_b_plus_1_times_2.monomials[1].coefficient, 2);
    assert_eq!(
        a_plus_b_plus_1_times_2.monomials[1].powers,
        vec![(Composite::Variable("b".into()), 1)]
    );
    assert_eq!(a_plus_b_plus_1_times_2.monomials[2].coefficient, 2);
    assert_eq!(a_plus_b_plus_1_times_2.monomials[2].powers.len(), 0);
}

#[test]
pub fn sub_test() {
    let a: TestPolynomial = variable("a".into());
    let b: TestPolynomial = variable("b".into());
    // a + b + 1
    let a_plus_b_plus_1 = &a + &b + 1;
    // 2a + 2b + 2
    let a_plus_b_plus_1_times_2 = 2 * &a_plus_b_plus_1;

    assert_eq!(&a_plus_b_plus_1_times_2 - &a_plus_b_plus_1, a_plus_b_plus_1);
    assert_eq!(&a_plus_b_plus_1 - 1, &a + &b);
    assert_eq!(&a_plus_b_plus_1 - &a, &b + 1);
    assert_eq!(&a_plus_b_plus_1 - &b, &a + 1);
    assert_eq!(&a_plus_b_plus_1 - &(&a + &b), 1);
}

#[test]
pub fn eval_test() {
    let a: TestPolynomial = variable("a".into());
    let b: TestPolynomial = variable("b".into());
    let c: TestPolynomial = variable("c".into());
    let d: TestPolynomial = variable("d".into());

    let mut values = HashMap::<String, i64>::new();
    values.insert("a".into(), 3);
    values.insert("b".into(), 7);
    values.insert("c".into(), 5);

    // a + b + 1
    let a_plus_b_plus_1 = &a + &b + 1;
    assert_eq!(a_plus_b_plus_1.eval(&values), Ok(11));

    // ab + a^2 + 1
    let ab_plus_a_square_plus_one = &a * &b + &a * &a + 1;
    assert_eq!(ab_plus_a_square_plus_one.eval(&values), Ok(31));

    // a + b + c + 1
    let a_plus_b_plus_c_plus_1 = &a + &b + &c + 1;
    assert_eq!(a_plus_b_plus_c_plus_1.eval(&values), Ok(16));

    // ab + bc + cd
    let ab_plus_bc_plus_cd = &a * &b * &b + &c + &c + &d;
    assert_eq!(
        ab_plus_bc_plus_cd.eval(&values),
        Err(("d".into(), "Value not provided for d.".into()))
    );

    // a^3 + 2a^2b + a^2c + a^2 + a b^2 + abc + ab + a + b + c + 1 =
    let product = &ab_plus_a_square_plus_one * &a_plus_b_plus_c_plus_1;
    assert_eq!(product.eval(&values), Ok(496));

    assert_eq!(floor(&product, TestPolynomial::from(3)).eval(&values), Ok(165));
    assert_eq!(ceil(&product, TestPolynomial::from(3)).eval(&values), Ok(166));

    assert_eq!(floor(&product, TestPolynomial::from(16)).eval(&values), Ok(31));
    assert_eq!(ceil(&product, TestPolynomial::from(16)).eval(&values), Ok(31));

    assert_eq!(floor(&product, &a).eval(&values), Ok(165));
    assert_eq!(ceil(&product, &a).eval(&values), Ok(166));

    assert_eq!(floor(&product, &b).eval(&values), Ok(70));
    assert_eq!(ceil(&product, &b).eval(&values), Ok(71));

    assert_eq!(floor(&product, &c).eval(&values), Ok(99));
    assert_eq!(ceil(&product, &c).eval(&values), Ok(100));

    assert_eq!(max(&product, &ab_plus_a_square_plus_one).eval(&values), Ok(496));
    assert_eq!(min(&product, &ab_plus_a_square_plus_one).eval(&values), Ok(31));

    assert_eq!(max(&-&product, &ab_plus_a_square_plus_one).eval(&values), Ok(31));
    assert_eq!(min(&-&product, &ab_plus_a_square_plus_one).eval(&values), Ok(-496));

    // Making a + b + 1 = 0
    values.clear();
    values.insert("a".into(), 3);
    values.insert("b".into(), -4);
    values.insert("c".into(), 5);
    assert_eq!(
        floor(&product, &a_plus_b_plus_1).eval(&values),
        Err(("".into(), "Attempting division by zero.".into()))
    );
    assert_eq!(
        ceil(&product, &a_plus_b_plus_1).eval(&values),
        Err(("".into(), "Attempting division by zero.".into()))
    );
}
