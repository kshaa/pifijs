use glam::Vec2;
use approx::{RelativeEq, AbsDiffEq};
use derive_more::Deref;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, combinator::{map, map_res}, multi::many1, sequence::tuple, IResult
};
  
#[derive(Debug, Clone, Deref)]
pub struct WrappedVec2(pub Vec2);

impl WrappedVec2 {      
    fn parse_part(input: &str) -> IResult<&str, f32> {
        map_res(
            many1(alt((tag("-"), digit1, tag(".")))),
            |symbols: Vec<&str>| { symbols.join("").parse::<f32>() },
        )(input)
    }

    fn parse_parts(input: &str) -> IResult<&str, (f32, f32)> {
        map(
            tuple((
                WrappedVec2::parse_part,
                tag(","),
                WrappedVec2::parse_part
            )),
            |(x, _, y)| { (x, y) }
        )(input)
    }

    pub fn parse(input: &str) -> IResult<&str, WrappedVec2> {
        map(
            WrappedVec2::parse_parts,
            |(x, y)| { WrappedVec2(Vec2::new(x, y)) }
        )(input)
    }
}

impl Eq for WrappedVec2 {}

impl PartialEq for WrappedVec2 {
    fn eq(&self, other: &Self) -> bool {
        let epsilon = f32::default_max_relative();
        let x_eq = self.x.abs_diff_eq(&other.x, epsilon);
        let y_eq = self.y.abs_diff_eq(&other.y, epsilon);
        x_eq && y_eq
    }
}
