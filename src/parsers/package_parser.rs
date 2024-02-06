use crate::parsers::key_value::key_value_parse;
use crate::types::config_enum::Configuration;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char, multispace0, multispace1};
use nom::sequence::{delimited, tuple};
use nom::IResult;

pub fn package_parse(input: &str) -> IResult<&str, Configuration> {
    let (_, (_, package, action)) = tuple((
        tag("package"),
        delimited(
            multispace1,
            delimited(char('"'), alpha1, char('"')),
            multispace0,
        ),
        delimited(char('{'), key_value_parse, char('}')),
    ))(input)?;

    Ok((
        "",
        Configuration::Package {
            name: package.to_string(),
            value: action.to_string(),
        },
    ))
}
