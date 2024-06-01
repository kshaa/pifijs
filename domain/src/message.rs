use crate::linestrip::Linestrip;

#[derive(PartialEq, Clone, Debug)]
pub enum PifijsMessage {
    Ping(),
    Plot((String, Vec<Linestrip>)),
}

impl PifijsMessage {
    pub fn parse_plot(leftovers: String) -> Result<PifijsMessage, String> {
        let parsed_strips = Linestrip::parse_strips(leftovers.clone());
        match parsed_strips {
            Some(parsed) => Ok(PifijsMessage::Plot((leftovers, parsed))),
            None => Err(String::from(format!("Failed to parse your plot {}. Here's an example: '0,1>0,-1 -1,0>1,0'", leftovers)))
        }
    }

    pub fn parse(content: String) -> Option<Result<PifijsMessage, String>> {
        let split_message = content.split_whitespace().collect::<Vec<&str>>();
        let call = split_message.get(0).map(|s| { s.to_owned() });
        let leftovers = split_message[1..].to_vec().join(" ");
        if let Some("!plot") = call {
            Some(PifijsMessage::parse_plot(leftovers))
        } else if let Some("!ping") = call {
            Some(Ok(PifijsMessage::Ping()))
        } else {
            None
        }
    }
}
