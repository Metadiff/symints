extern crate symbolic_polynomials;
use symbolic_polynomials::*;
use std::collections::HashMap;

#[allow(dead_code)]
type TestMonomial = Monomial<String, i64, u8>;
#[allow(dead_code)]
type TestPolynomial = Polynomial<String, i64, u8>;

#[test]
pub fn constructor() {
    let a = TestMonomial {
        coefficient: 1,
        powers: vec![(Composite::Variable("a".into()), 1)],
    };
    let minus_six = TestMonomial::from(-6);
    let thirteen = TestMonomial::from(13);

    assert!(minus_six.is_constant());
    assert_eq!(minus_six.coefficient, -6);
    assert_eq!(minus_six.powers.len(), 0);

    assert!(thirteen.is_constant());
    assert_eq!(thirteen.coefficient, 13);
    assert_eq!(thirteen.powers.len(), 0);

    assert!(!a.is_constant());
    assert_eq!(a.coefficient, 1);
    assert_eq!(a.powers, vec![(Composite::Variable("a".into()), 1)]);
}

#[test]
pub fn up_to_coefficient_test() {
    let a = TestMonomial {
        coefficient: 1,
        powers: vec![(Composite::Variable("a".into()), 1)],
    };
    let b = TestMonomial {
        coefficient: 1,
        powers: vec![(Composite::Variable("b".into()), 1)],
    };
    let three = TestMonomial::from(3);
    let five = TestMonomial::from(5);
    // 2a
    let a_times_2 = 2 * &a;
    // -5a
    let minus_5_a = -5 * &a;
    // a^2
    let a_square = &a * &a;
    // 3a^2
    let a_square_times_3 = 3 * &a_square;

    assert!(!a.up_to_coefficient(&three));
    assert!(!three.up_to_coefficient(&a));
    assert!(!a.up_to_coefficient(&five));
    assert!(!five.up_to_coefficient(&a));
    assert!(!b.up_to_coefficient(&three));
    assert!(!three.up_to_coefficient(&b));
    assert!(!b.up_to_coefficient(&five));
    assert!(!five.up_to_coefficient(&b));

    assert!(!a.up_to_coefficient(&b));
    assert!(!b.up_to_coefficient(&a));

    assert!(five.up_to_coefficient(&three));
    assert!(three.up_to_coefficient(&five));
    assert!(a.up_to_coefficient(&a_times_2));
    assert!(a_times_2.up_to_coefficient(&a));
    assert!(a.up_to_coefficient(&minus_5_a));
    assert!(minus_5_a.up_to_coefficient(&a));
    assert!(a_times_2.up_to_coefficient(&minus_5_a));
    assert!(minus_5_a.up_to_coefficient(&a_times_2));

    assert!(!a_times_2.up_to_coefficient(&three));
    assert!(!three.up_to_coefficient(&a_times_2));
    assert!(!a_times_2.up_to_coefficient(&five));
    assert!(!five.up_to_coefficient(&a_times_2));

    assert!(!a_square.up_to_coefficient(&a));
    assert!(!a.up_to_coefficient(&a_square));
    assert!(!a_square.up_to_coefficient(&a_times_2));
    assert!(!a_times_2.up_to_coefficient(&a_square));
    assert!(!a_square.up_to_coefficient(&minus_5_a));
    assert!(!minus_5_a.up_to_coefficient(&a_square));
    assert!(a_square.up_to_coefficient(&a_square_times_3));
    assert!(a_square_times_3.up_to_coefficient(&a_square));
}

#[test]
pub fn partial_eq_test() {
    let a = TestMonomial {
        coefficient: 1,
        powers: vec![(Composite::Variable("a".into()), 1)],
    };
    let b = TestMonomial {
        coefficient: 1,
        powers: vec![(Composite::Variable("b".into()), 1)],
    };
    let a_v2 = TestMonomial {
        coefficient: 1,
        powers: vec![(Composite::Variable("a".into()), 1)],
    };
    // a^2
    let a_square_v1 = TestMonomial {
        coefficient: 1,
        powers: vec![(Composite::Variable("a".into()), 2)],
    };
    let a_square_v2 = TestMonomial {
        coefficient: 1,
        powers: vec![(Composite::Variable("a".into()), 2)],
    };
    // 2a^2
    let two_a_square = TestMonomial {
        coefficient: 2,
        powers: vec![(Composite::Variable("a".into()), 2)],
    };
    // b^2
    let b_square = &b * &b;

    assert_ne!(a, 3);
    assert_ne!(3, a);
    assert_eq!(a, a_v2);
    assert_eq!(a_v2, a);

    assert_eq!(a_square_v1, a_square_v2);
    assert_eq!(a_square_v2, a_square_v1);
    assert_ne!(two_a_square, a_square_v1);
    assert_ne!(a_square_v1, two_a_square);
    assert_ne!(a_square_v1, b_square);
    assert_ne!(b_square, a_square_v1);
}

#[test]
pub fn ord_test() {
    let a = TestMonomial {
        coefficient: 1,
        powers: vec![(Composite::Variable("a".into()), 1)],
    };
    let b = TestMonomial {
        coefficient: 1,
        powers: vec![(Composite::Variable("b".into()), 1)],
    };
    // 2a^2b
    let a_square_b_times_2 = TestMonomial {
        coefficient: 2,
        powers: vec![
            (Composite::Variable("a".into()), 2),
            (Composite::Variable("b".into()), 1),
        ],
    };
    // 3a^2b
    let a_square_b_times_3 = TestMonomial {
        coefficient: 3,
        powers: vec![
            (Composite::Variable("a".into()), 2),
            (Composite::Variable("b".into()), 1),
        ],
    };
    // 3ab
    let ab_times_3 = TestMonomial {
        coefficient: 2,
        powers: vec![
            (Composite::Variable("a".into()), 1),
            (Composite::Variable("b".into()), 1),
        ],
    };

    assert!(a > 2);
    assert!(2 < a);
    assert!(b > 2);
    assert!(2 < b);

    assert!(a > b);
    assert!(b < a);
    assert!(a_square_b_times_3 > a_square_b_times_2);
    assert!(a_square_b_times_2 < a_square_b_times_3);
    assert!(a_square_b_times_3 > ab_times_3);
    assert!(ab_times_3 < a_square_b_times_3);
    assert!(a_square_b_times_2 > ab_times_3);
    assert!(ab_times_3 < a_square_b_times_2);
}

#[test]
pub fn mul_test() {
    let a = TestMonomial {
        coefficient: 1,
        powers: vec![(Composite::Variable("a".into()), 1)],
    };
    let b = TestMonomial {
        coefficient: 1,
        powers: vec![(Composite::Variable("b".into()), 1)],
    };
    let c = TestMonomial {
        coefficient: 1,
        powers: vec![(Composite::Variable("c".into()), 1)],
    };
    // 2abc
    let abc_times_2 = 2 * &a * &b * &c;
    // b^2
    let b_square = &b * &b;
    // 2ab^3c
    let ab_third_c_times_2 = &b_square * &abc_times_2;

    assert_eq!(abc_times_2.coefficient, 2);
    assert!(!abc_times_2.is_constant());
    assert_eq!(
        abc_times_2.powers,
        vec![
            (Composite::Variable("a".into()), 1),
            (Composite::Variable("b".into()), 1),
            (Composite::Variable("c".into()), 1),
        ]
    );

    assert_eq!(b_square.coefficient, 1);
    assert_eq!(b_square.powers, vec![(Composite::Variable("b".into()), 2)]);

    assert_eq!(ab_third_c_times_2.coefficient, 2);
    assert_eq!(
        ab_third_c_times_2.powers,
        vec![
            (Composite::Variable("a".into()), 1),
            (Composite::Variable("b".into()), 3),
            (Composite::Variable("c".into()), 1),
        ]
    );
}

#[test]
pub fn div_test() {
    let a = TestMonomial {
        coefficient: 1,
        powers: vec![(Composite::Variable("a".into()), 1)],
    };
    let b = TestMonomial {
        coefficient: 1,
        powers: vec![(Composite::Variable("b".into()), 1)],
    };
    let c = TestMonomial {
        coefficient: 1,
        powers: vec![(Composite::Variable("c".into()), 1)],
    };
    let d = TestMonomial {
        coefficient: 1,
        powers: vec![(Composite::Variable("d".into()), 1)],
    };
    // a^2
    let a_square = &a * &a;
    // b^2
    let b_square = &b * &b;
    // c^2
    let c_square = &c * &c;
    // 2abc
    let abc_times_2 = 2 * &a * &b * &c;
    // ab
    let ab = &a * &b;
    // ac
    let ac = &a * &c;
    // bc
    let bc = &b * &c;

    // abc
    let abc = &abc_times_2 / 2;
    assert_eq!(abc.coefficient, 1);
    assert_eq!(
        abc.powers,
        vec![
            (Composite::Variable("a".into()), 1),
            (Composite::Variable("b".into()), 1),
            (Composite::Variable("c".into()), 1),
        ]
    );

    // 2bc
    let bc2 = &abc_times_2 / &a;
    assert_eq!(bc2.coefficient, 2);
    assert_eq!(
        bc2.powers,
        vec![
            (Composite::Variable("b".into()), 1),
            (Composite::Variable("c".into()), 1),
        ]
    );

    // 2ac
    let ac2 = &abc_times_2 / &b;
    assert_eq!(ac2.coefficient, 2);
    assert_eq!(
        ac2.powers,
        vec![
            (Composite::Variable("a".into()), 1),
            (Composite::Variable("c".into()), 1),
        ]
    );

    // 2ab
    let ab2 = &abc_times_2 / &c;
    assert_eq!(ab2.coefficient, 2);
    assert_eq!(
        ab2.powers,
        vec![
            (Composite::Variable("a".into()), 1),
            (Composite::Variable("b".into()), 1),
        ]
    );

    // 2
    let two = &(&(&abc_times_2 / &b) / &c) / &a;
    assert_eq!(two.coefficient, 2);
    assert_eq!(two.powers, vec![]);

    // 2c
    let c2 = &abc_times_2 / &ab;
    assert_eq!(c2.coefficient, 2);
    assert_eq!(c2.powers, vec![(Composite::Variable("c".into()), 1)]);

    // 2b
    let b2 = &abc_times_2 / &ac;
    assert_eq!(b2.coefficient, 2);
    assert_eq!(b2.powers, vec![(Composite::Variable("b".into()), 1)]);

    // 2a
    let a2 = &abc_times_2 / &bc;
    assert_eq!(a2.coefficient, 2);
    assert_eq!(a2.powers, vec![(Composite::Variable("a".into()), 1)]);

    assert!(abc_times_2.checked_div(&4.into()).is_none());
    assert!(abc_times_2.checked_div(&a_square).is_none());
    assert!(abc_times_2.checked_div(&b_square).is_none());
    assert!(abc_times_2.checked_div(&c_square).is_none());
    assert!(abc_times_2.checked_div(&d).is_none());
}

#[test]
pub fn add_test() {
    let a = TestMonomial {
        coefficient: 1,
        powers: vec![(Composite::Variable("a".into()), 1)],
    };
    let b = TestMonomial {
        coefficient: 1,
        powers: vec![(Composite::Variable("b".into()), 1)],
    };

    // a + b
    let a_plus_b = &a + &b;
    assert_eq!(a_plus_b.monomials.len(), 2);
    assert_eq!(a_plus_b.monomials[0], a);
    assert_eq!(a_plus_b.monomials[1], b);

    // a + 2b
    let a_plus_two_b = &a_plus_b + &b;
    println!("{}", a_plus_two_b);
    assert_eq!(a_plus_b.monomials.len(), 2);
    assert_eq!(a_plus_two_b.monomials[0], a);
    assert_eq!(a_plus_two_b.monomials[1], 2 * &b);

    // 2a + 2b
    let a_plus_b_twice = &a_plus_two_b + &a;
    assert_eq!(a_plus_b.monomials.len(), 2);
    assert_eq!(a_plus_b_twice.monomials[0], 2 * &a);
    assert_eq!(a_plus_b_twice.monomials[1], 2 * &b);

    // 0
    let zero = &a_plus_b - &a_plus_b;
    assert_eq!(zero.monomials.len(), 0);
}

#[test]
pub fn sub_test() {
    let a = TestMonomial {
        coefficient: 1,
        powers: vec![(Composite::Variable("a".into()), 1)],
    };
    let b = TestMonomial {
        coefficient: 1,
        powers: vec![(Composite::Variable("b".into()), 1)],
    };

    // a - b
    let a_minus_b = &a - &b;
    assert_eq!(a_minus_b.monomials.len(), 2);
    assert_eq!(a_minus_b.monomials[0], a);
    assert_eq!(a_minus_b.monomials[1], -&b);

    // a - 2b
    let a_minus_two_b = &a_minus_b - &b;
    assert_eq!(a_minus_two_b.monomials.len(), 2);
    assert_eq!(a_minus_two_b.monomials[0], a);
    assert_eq!(a_minus_two_b.monomials[1], -2 * &b);

    // a
    let a_v2 = &a_minus_two_b + 2 * &b;
    assert_eq!(a_v2.monomials.len(), 1);
    assert_eq!(a_v2.monomials[0], a);

    // 0
    let zero = &a_v2 + &(-&a);
    assert_eq!(zero.monomials.len(), 0);
}

#[test]
pub fn eval_test() {
    let a = TestMonomial {
        coefficient: 1,
        powers: vec![(Composite::Variable("a".into()), 1)],
    };
    let b = TestMonomial {
        coefficient: 1,
        powers: vec![(Composite::Variable("b".into()), 1)],
    };
    let c = TestMonomial {
        coefficient: 1,
        powers: vec![(Composite::Variable("c".into()), 1)],
    };
    let d = TestMonomial {
        coefficient: 1,
        powers: vec![(Composite::Variable("d".into()), 1)],
    };

    let mut values = HashMap::<String, i64>::new();
    values.insert("a".into(), 3);
    values.insert("b".into(), 7);
    values.insert("c".into(), 5);

    assert_eq!(a.eval(&values), Ok(3));
    assert_eq!(b.eval(&values), Ok(7));
    assert_eq!(c.eval(&values), Ok(5));
    assert_eq!(d.eval(&values), Err(("d".into(), "Value not provided for d.".into())));

    assert_eq!((&a * 2 * &a).eval(&values), Ok(18));
    assert_eq!((&a * 2 * &b).eval(&values), Ok(42));
    assert_eq!((&c * &a * &b).eval(&values), Ok(105));
    assert_eq!(
        (&d * &c * &a * &b).eval(&values),
        Err(("d".into(), "Value not provided for d.".into()))
    );

    assert_eq!((&a + &b + &c + 2).eval(&values), Ok(17));
    assert_eq!(
        (&a + &b + &c + &d).eval(&values),
        Err(("d".into(), "Value not provided for d.".into()))
    );
}
