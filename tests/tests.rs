use easyrand::{choose, random, randrange, with_seed};

#[test]
fn test_readme() {
    doc_comment::doctest!("../README.md");
}

#[test]
fn test_random() {
    for _ in 0..1000 {
        let r = random();

        // Check valid range
        assert!(r >= 0.0 && r <= 1.0);

        // Check it doesn't produce same result (arguably it could produce a sequences of same numbers, but, seems.. unlikely)
        for _ in 1..1000 {
            assert!(r != random());
        }
    }
}

#[test]
fn test_range() {
    for _ in 0..1000 {
        let r = randrange(1.0, 2.0);
        assert!(r >= 1.0 && r <= 2.0);

        for _ in 1..1000 {
            assert!(r != randrange(1.0, 2.0));
        }
    }
}

#[test]
fn test_choose() {
    let things = vec!["abc", "def"];
    let r = choose(&things).unwrap();
    assert!(*r == "abc" || *r == "def");
}

#[test]
fn test_seeded() {
    let mut s = with_seed(42);
    let r1 = dbg!(s.random());
    let r2 = dbg!(s.random());
    let r3 = dbg!(s.random());

    // Check values are consistent
    assert!(r1.abs() - 0.9728880888443334 < 1e-8);
    assert!(r2.abs() - 0.5427252099031439 < 1e-8);
    assert!(r3.abs() - 0.6364650991438949 < 1e-8);

    // Check same results with new instance
    let mut s2 = with_seed(42);
    assert!(s2.random() == r1);
    assert!(s2.random() == r2);
    assert!(s2.random() == r3);

    // Check different seed gives different result
    let mut s3 = with_seed(43);
    assert!(s3.random() != r1);
    assert!(s3.random() != r2);
    assert!(s3.random() != r3);
}
