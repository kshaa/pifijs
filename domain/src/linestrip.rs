use std::ops::Deref;
use glam::Vec2;
use nom::{bytes::complete::tag, character::complete::space1, combinator::{map, opt}, multi::many1, sequence::tuple, IResult};
use crate::{placement::Placement, vec::WrappedVec2};
use derive_more::Deref;

#[derive(Eq, PartialEq, Clone, Debug, Deref)]
pub struct Linestrip(Vec<WrappedVec2>);

impl Linestrip {
    pub fn from_vec(points: Vec<Vec2>) -> Linestrip {
        Linestrip(points.into_iter().map(WrappedVec2).collect())
    }

    pub fn deref(&self) -> Vec<Vec2> {
        self.iter().map(|p| { p.deref().clone() }).collect::<Vec<Vec2>>()
    }

    pub fn sum(&self) -> f32 {
        self.iter().map(|p| { p.x + p.y }).sum::<f32>()
    }

    pub fn parts(&self, extract: fn(&WrappedVec2) -> f32) -> Vec<f32> {
        self.iter().map(extract).collect()
    }

    pub fn xs(&self) -> Vec<f32> {
        self.parts(|v| { v.x })
    }

    pub fn ys(&self) -> Vec<f32> {
        self.parts(|v| { v.y })
    }

    pub fn placement(linestrips: Vec<Linestrip>, identity_scale: f32) -> Placement {
        let xs = linestrips.iter().flat_map(Linestrip::xs);
        let ys = linestrips.iter().flat_map(Linestrip::ys);
        let min_x = xs.clone().min_by(f32::total_cmp).unwrap();
        let max_x = xs.max_by(f32::total_cmp).unwrap();
        let min_y = ys.clone().min_by(f32::total_cmp).unwrap();
        let max_y = ys.max_by(f32::total_cmp).unwrap();

        let position = Vec2 {
            x: (min_x + max_x) / 2.,
            y: (min_y + max_y) / 2.,
        };

        let width = max_x - min_x;
        let height = max_y - min_y;
        let max_size = width.max(height);
        let scale = identity_scale * max_size;

        Placement::new(position, scale)
    }
    
    pub fn parse_part(input: &str) -> IResult<&str, WrappedVec2> {
        map(
            tuple((
                WrappedVec2::parse,
                opt(tag(">"))
            )),
            |(v, _)| { v },
        )(input)
    }

    pub fn parse(input: &str) -> IResult<&str, Linestrip> {
        map(
            many1(Linestrip::parse_part),
            Linestrip,
        )(input)
    }

    pub fn parse_multiple(input: &str) -> IResult<&str, Vec<Linestrip>> {
        map(many1(map(
            tuple((
                Linestrip::parse,
                opt(space1),
            )),
            |(strip, _)| { strip }
        )), |strips| { strips })(input)
    }
}
