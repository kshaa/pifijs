use glam::Vec2;
use pifijs_domain_lib::vec::WrappedVec2;

#[test]
fn vec_with_whole_numbers_can_be_parsed() {
    let input = String::from("0,1");
    let parsed = WrappedVec2::parse(&input);
    let expected = Ok(("", WrappedVec2(Vec2::new(0.0, 1.0))));
    assert_eq!(parsed, expected)
}

#[test]
fn vec_with_negative_fractions_can_be_parsed() {
    let input = String::from("-0,1.2");
    let parsed = WrappedVec2::parse(&input);
    let expected = Ok(("", WrappedVec2(Vec2::new(-0., 1.2))));
    assert_eq!(parsed, expected)
}