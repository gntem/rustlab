use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{line_ending, space0, space1},
    combinator::{rest},
    IResult, Parser,
};

#[derive(Debug, PartialEq)]
pub enum Command {
    Publish { topic: String, message: String },
    Subscribe { topic: String, mode: String },
    Unsubscribe { topic: String },
    Info,
    Ping,
    Pong,
    Connect,
    Disconnect,
}

pub fn parse_command(input: &str) -> IResult<&str, Command, nom::error::Error<&str>> {
    let (input, cmd) = take_until("\r\n")(input)?;
    let (input, _) = line_ending(input)?;

    match cmd {
        "CONNECT" => IResult::Ok((input, Command::Connect)),
        "INFO" => IResult::Ok((input, Command::Info)),
        "PING" => IResult::Ok((input, Command::Ping)),
        "PONG" => IResult::Ok((input, Command::Pong)),
        "DISCONNECT" => IResult::Ok((input, Command::Disconnect)),
        _ => {
            let known = alt([
                tag::<&str, &str, nom::error::Error<&str>>("PUBLISH"),
                tag("SUBSCRIBE"),
                tag("UNSUBSCRIBE"),
            ])
            .parse(cmd);

            if known.is_ok() {
                let w = match known.unwrap().1 {
                    "PUBLISH" => parse_publish(cmd),
                    "SUBSCRIBE" => parse_subscribe(cmd),
                    "UNSUBSCRIBE" => parse_unsubscribe(cmd),
                    _ => unreachable!(),
                };
                if w.is_ok() {
                    return w;
                }
            }
            IResult::Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Tag,
            )))
        }
    }
}

fn parse_publish(input: &str) -> IResult<&str, Command, nom::error::Error<&str>> {
    let (input, _) = tag("PUBLISH")(input)?;
    let (input, _) = space0(input)?;
    let (input, topic) = take_until(" ")(input)?;
    let (input, _) = space1(input)?;
    let (leftover, input) = rest(input)?;

    Ok((
        leftover,
        Command::Publish {
            topic: topic.to_string(),
            message: input.to_string(),
        },
    ))
}

fn parse_subscribe(input: &str) -> IResult<&str, Command, nom::error::Error<&str>> {
    let (input, _) = tag("SUBSCRIBE")(input)?;
    let (input, _) = space0(input)?;
    let (input, topic) = take_until(" ")(input)?;
    let (input, _) = space1(input)?;
    let (leftover, input) = rest(input)?;

    Ok((
        leftover,
        Command::Subscribe {
            topic: topic.to_string(),
            mode: input.to_string(),
        },
    ))
}

fn parse_unsubscribe(input: &str) -> IResult<&str, Command, nom::error::Error<&str>> {
    let (input, _) = tag("UNSUBSCRIBE")(input)?;
    let (input, _) = space0(input)?;
    let (leftover, input) = rest(input)?;

    Ok((
        leftover,
        Command::Unsubscribe {
            topic: input.to_string(),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_no_args_cmds() {
        assert_eq!(parse_command("CONNECT\r\n"), Ok(("", Command::Connect)));
        assert_eq!(parse_command("INFO\r\n"), Ok(("", Command::Info)));
        assert_eq!(parse_command("PING\r\n"), Ok(("", Command::Ping)));
        assert_eq!(parse_command("PONG\r\n"), Ok(("", Command::Pong)));
        assert_eq!(
            parse_command("DISCONNECT\r\n"),
            Ok(("", Command::Disconnect))
        );
    }

    #[test]
    fn test_parse_publish() {
        assert_eq!(
            parse_command("PUBLISH topic message\r\n"),
            Ok((
                "",
                Command::Publish {
                    topic: "topic".to_string(),
                    message: "message".to_string(),
                }
            ))
        );
    }

    #[test]
    fn test_parse_subscribe() {
        assert_eq!(
            parse_command("SUBSCRIBE topic mode\r\n"),
            Ok((
                "",
                Command::Subscribe {
                    topic: "topic".to_string(),
                    mode: "mode".to_string(),
                }
            ))
        );
    }

    #[test]
    fn test_parse_unsubscribe() {
        assert_eq!(
            parse_command("UNSUBSCRIBE topic\r\n"),
            Ok((
                "",
                Command::Unsubscribe {
                    topic: "topic".to_string()
                }
            ))
        );
    }
}
