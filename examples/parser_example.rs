use ccl_parser::parse_ccl;

fn main() {
    let test_value = r#"package "nginx" { 
        value = "install" 
    }
    
    package "postgres" {
        value = "install"
    }"#;
    let conf = parse_ccl(test_value);
    println!("{:?}", conf);
}
