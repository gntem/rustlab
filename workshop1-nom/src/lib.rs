use nom::{
    bytes::{complete::take_until, tag},
    character::complete::{alphanumeric1, char, space0},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

fn parse_command(input: &str) -> IResult<&str, (&str, Option<(&str, &str)>)> {
    let (input, command) = alphanumeric1(input)?;
    let (input, _) = space0(input)?;
    let (input, args) = match command {
        "INFO" => {
            let (input, _) = tag("\r\n").parse(input)?;
            (input, None)
        }
        "PUB" => {
            let mut p = delimited(char(' '), take_until(" "), take_until("\r\n"));
            let parsed = p.parse(input)?;
            (input, Some(parsed))
        }
        "SUB" => {
            let (input, topic) = take_until("\r\n")(input)?;
            (input, Some((topic, "")))
        }
        _ => (input, None),
    };
    Ok((input, (command, args)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub_cmd() {
        assert_eq!(
            parse_command("SUB foo\r\n"),
            Ok(("\r\n", ("SUB", Some(("foo", "")))))
        );
    }

    #[test]
    fn test_pub_cmd() {
        assert_eq!(
            parse_command("PUB foo bar\r\n"),
            Ok(("\r\n", ("PUB", Some(("foo", "bar")))))
        );
    }

    #[test]
    fn test_info_cmd() {
        assert_eq!(parse_command("INFO\r\n"), Ok(("", ("INFO", None))));
    }
}
 