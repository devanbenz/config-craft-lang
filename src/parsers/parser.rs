use crate::parsers::package_parser::package_parse;
use crate::types::config_enum::Configuration;
use nom::branch::alt;
use nom::multi::many0;
use nom::IResult;

pub fn parse_configuration(input: &str) -> IResult<&str, Vec<Configuration>> {
    todo!()
}

// pub fn network_parse(input: &str) -> IResult<&str, PackageConfiguration> {
//     let (_, (_, name, values)) = tuple((
//         tag("package"),
//         delimited(
//             multispace1,
//             delimited(char('"'), alpha1, char('"')),
//             multispace0,
//         ),
//         delimited(
//             char('{'),
//             separated_list1(multispace1, key_value_parse),
//             char('}'),
//         ),
//     ))(input)?;
//
//     Ok((
//         "",
//         PackageConfiguration {
//             value: name.to_string(),
//             action: action.to_string(),
//         },
//     ))
// }
