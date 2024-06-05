use std::str::FromStr;

use crate::linestrip::Linestrip;
use nom::{bytes::complete::tag, character::complete::{alpha0, space0}, combinator::{map, map_res}, sequence::tuple, IResult};
use strum_macros::{Display, EnumString};

#[derive(Display, EnumString)]
// If we don't care about inner capitals, we don't need to set `serialize_all` 
// and can leave parenthesis empty.
#[strum(serialize_all = "snake_case")]
pub enum PifijsMessageLabel {
    Ping,
    Plot,
}
impl PifijsMessageLabel {
    pub fn parse(input: &str) -> IResult<&str, PifijsMessageLabel> {
        map_res(
            alpha0,
            PifijsMessageLabel::from_str
        )(input)
    }
    
    pub fn parse_as_command(input: &str) -> IResult<&str, PifijsMessageLabel> {
        map(tuple((
            tag("!"),
            PifijsMessageLabel::parse,
            space0,
        )), |(_, label, _)| { label })(input)
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct PingMessage();

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct PlotMessage(pub String, pub Vec<Linestrip>);


#[derive(Eq, PartialEq, Clone, Debug)]
pub enum PifijsMessage {
    Ping(PingMessage),
    Plot(PlotMessage),
}

impl PifijsMessage {
    pub fn parse_ping(input: &str) -> IResult<&str, PingMessage> {
        Ok((input, PingMessage()))
    }

    pub fn parse_plot(input: &str) -> IResult<&str, PlotMessage> {
        map(Linestrip::parse_multiple, |parsed| { PlotMessage(input.to_string(), parsed) })(input)
    }

    pub fn parse(input: &str) -> Option<Result<PifijsMessage, nom::Err<nom::error::Error<&str>>>> {
        let parsed_command = PifijsMessageLabel::parse_as_command(input);
        match parsed_command {
            Err(_) => None,
            Ok((command_input, label)) => Some(
                (match label {
                    PifijsMessageLabel::Ping => PifijsMessage::parse_ping(command_input).map(|(a, b)| { (a, PifijsMessage::Ping(b)) }),
                    PifijsMessageLabel::Plot => PifijsMessage::parse_plot(command_input).map(|(a, b)| { (a, PifijsMessage::Plot(b)) }),
                }).map(|(_, message)| { message })
            )
        }
    }
}
