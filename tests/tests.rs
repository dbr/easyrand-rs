use easyrand::{choose, random, randrange};

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
