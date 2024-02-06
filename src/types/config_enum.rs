#[derive(Debug, PartialEq)]
pub enum Configuration {
    Package {
        name: String,
        value: String,
    },
    Network {
        name: String,
        value: String,
        dns: String,
        netmask: String,
        ip: String,
    },
}
