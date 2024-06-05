use glam::Vec2;
use pifijs_domain_lib::{linestrip::Linestrip, message::{PifijsMessage, PlotMessage}};

#[test]
fn it_can_be_parsed() {
    let message = String::from("!plot 0,1>0,-1 -1,0>1,0");
    let parsed = PifijsMessage::parse(&message);
    let expected = Some(Ok(PifijsMessage::Plot(PlotMessage(String::from("0,1>0,-1 -1,0>1,0"), vec!(
        Linestrip::from_vec(vec!(
            Vec2::new(0., 1.),
            Vec2::new(0., -1.)
        )),
        Linestrip::from_vec(vec!(
            Vec2::new(-1., 0.),
            Vec2::new(1., 0.)
        ))
    )))));
    assert_eq!(parsed, expected)
}