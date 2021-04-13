#[test]
fn test_readme() {
    doc_comment::doctest!("../README.md");
}

#[test]
fn test_random() {
    for _ in 0..1000 {
        let r = easyrand::random();

        // Check valid range
        assert!(r >= 0.0 && r <= 1.0);

        // Check it doesn't produce same result (arguably it could produce a sequences of same numbers, but, seems.. unlikely)
        for _ in 1..1000 {
            assert!(r != easyrand::random());
        }
    }
}

#[test]
fn test_randint() {
    for _ in 0..1000 {
        let r = easyrand::randint(12, 40);
        assert!(r >= 12 && r <= 40);
    }
}

#[test]
fn test_randint_seeded() {
    let mut s = easyrand::with_seed(42);

    for _ in 0..1000 {
        let r = s.randint(12, 40);
        assert!(r >= 12 && r <= 40);
    }

    assert_eq!(s.randint(10, 20), 12);
    assert_eq!(s.randint(10, 20), 15);
    assert_eq!(s.randint(10, 20), 13);
}

#[test]
fn test_bool() {
    let mut found_true = false;
    let mut found_false = false;

    for _ in 0..1000 {
        if easyrand::randbool() {
            found_true = true;
        } else {
            found_false = true;
        }
    }
    assert!(found_true);
    assert!(found_false);
}

#[test]
fn test_bool_seeded() {
    let mut s = easyrand::with_seed(42);
    assert_eq!(s.randbool(), false);
    assert_eq!(s.randbool(), true);
    assert_eq!(s.randbool(), false);
    assert_eq!(s.randbool(), true);
    assert_eq!(s.randbool(), true);
    assert_eq!(s.randbool(), true);
    assert_eq!(s.randbool(), true);
    assert_eq!(s.randbool(), false);
}

#[test]
fn test_range() {
    for _ in 0..1000 {
        let r = easyrand::randrange(1.0, 2.0);
        assert!(r >= 1.0 && r <= 2.0);

        for _ in 1..1000 {
            assert!(r != easyrand::randrange(1.0, 2.0));
        }
    }
}

#[test]
fn test_range_seeded() {
    let mut s = easyrand::with_seed(42);

    for _ in 0..1000 {
        let r = s.randrange(1.0, 2.0);
        assert!(r >= 1.0 && r <= 2.0);

        for _ in 1..1000 {
            assert!(r != s.randrange(1.0, 2.0));
        }
    }

    let r1 = s.randrange(1.0, 4.0);
    let r2 = s.randrange(1.0, 4.0);
    assert!(r1.abs() - 2.6298566353376964 < 1e-8);
    assert!(r2.abs() - 1.7994980528472595 < 1e-8);
}

#[test]
fn test_choose() {
    let things = vec!["abc", "def"];
    let r = easyrand::choose(&things).unwrap();
    assert!(*r == "abc" || *r == "def");
}

#[test]
fn test_choose_seeded() {
    let mut s = easyrand::with_seed(42);
    let things = vec!["abc", "def"];
    assert_eq!(*s.choose(&things).unwrap(), "abc");
    assert_eq!(*s.choose(&things).unwrap(), "def");
    assert_eq!(*s.choose(&things).unwrap(), "abc");
    assert_eq!(*s.choose(&things).unwrap(), "def");
    assert_eq!(*s.choose(&things).unwrap(), "def");
}

#[test]
fn test_seeded() {
    for _ in 0..100 {
        let mut s = easyrand::with_seed(42);
        let r1 = dbg!(s.random());
        let r2 = dbg!(s.random());
        let r3 = dbg!(s.random());

        // Check values are consistent
        assert!(r1.abs() - 0.9728880888443334 < 1e-8);
        assert!(r2.abs() - 0.5427252099031439 < 1e-8);
        assert!(r3.abs() - 0.6364650991438949 < 1e-8);

        // Check same results with new instance
        let mut s2 = easyrand::with_seed(42);
        assert!(s2.random() == r1);
        assert!(s2.random() == r2);
        assert!(s2.random() == r3);

        // Check different seed gives different result
        let mut s3 = easyrand::with_seed(43);
        assert!(s3.random() != r1);
        assert!(s3.random() != r2);
        assert!(s3.random() != r3);
    }
}

#[test]
fn test_shuffle() {
    for _ in 0..100 {
        let src = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut dst = src.clone();
        easyrand::shuffle(&mut dst);
        assert_ne!(src, dst);
    }
}

#[test]
fn test_shuffle_seeded() {
    for _ in 0..100 {
        let mut s = easyrand::with_seed(42);
        let mut src = vec![1, 2, 3];
        s.shuffle(&mut src);
        assert_eq!(src, vec![3, 2, 1]);
    }
}
