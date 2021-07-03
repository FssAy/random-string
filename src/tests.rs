use super::*;

#[test]
fn ascii_generation() {
    let charset = "123456";
    assert_eq!(generate(6, charset).len(), 6_usize);
}

#[test]
fn unicode_generation() {
    let charset = "ó❤⚙";
    let result = generate(6, charset).chars().count();
    assert_eq!(result, 6_usize);
}
