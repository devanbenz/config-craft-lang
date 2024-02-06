use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char, multispace0};
use nom::sequence::{delimited, separated_pair};
use nom::IResult;

pub fn key_value_parse(input: &str) -> IResult<&str, &str> {
    let (rem, (_, value)) = delimited(
        multispace0,
        separated_pair(alpha1, tag(" = "), delimited(char('"'), alpha1, char('"'))),
        multispace0,
    )(input)?;

    Ok((rem, value))
}
