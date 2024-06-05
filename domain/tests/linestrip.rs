use glam::Vec2;
use pifijs_domain_lib::linestrip::Linestrip;

#[test]
fn linestrip_can_be_parsed() {
    let input = String::from("0,1>-1,2.0");
    let parsed = Linestrip::parse(&input);
    let expected = Ok(("", Linestrip::from_vec(vec!(
        Vec2::new(0., 1.),
        Vec2::new(-1., 2.),
    ))));
    assert_eq!(parsed, expected)
}

#[test]
fn linestrips_can_be_parsed() {
    let input = String::from("0,1>-1,2.0 0,1>-1,2.0 0,0");
    let parsed = Linestrip::parse_multiple(&input);
    let expected = Ok(("", vec!(
        Linestrip::from_vec(vec!(
            Vec2::new(0., 1.),
            Vec2::new(-1., 2.),
        )),
        Linestrip::from_vec(vec!(
            Vec2::new(0., 1.),
            Vec2::new(-1., 2.),
        )),
        Linestrip::from_vec(vec!(
            Vec2::new(0., 0.),
        )),
    )));
    assert_eq!(parsed, expected)
}