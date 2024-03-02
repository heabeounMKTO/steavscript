use nom::{branch::alt,bytes::complete::tag ,character::complete::char, IResult};

fn parse_bscript(input: &str) -> IResult<&str, &str> {
    tag("jongYeyTha")(input)
}