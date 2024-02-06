use crate::parsers::package_parser::package_parse;
use crate::parsers::parser::parse_configuration;
use crate::types::config_enum::Configuration;

mod parsers;
mod types;

pub fn parse_ccl(input: &str) -> Configuration {
    package_parse(input).expect("could not get keys").1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_package_install() {
        let input = r#"package "nginx" {
            action = "install"
        }"#;

        let result = parse_ccl(input);

        let expected = Configuration::Package {
            name: "nginx".to_string(),
            value: "install".to_string(),
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn should_parse_package_remove() {
        let input = r#"package "nginx" {
            action = "remove"
        }"#;

        let result = parse_ccl(input);

        let expected = Configuration::Package {
            name: "nginx".to_string(),
            value: "remove".to_string(),
        };

        assert_eq!(expected, result);
    }
}
