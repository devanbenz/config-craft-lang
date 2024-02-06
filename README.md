# ConfigCraft Language Parser

Parser for ConfigCraft DSL language. This is a configuration language to be
intepreted by ConfigCraft which is a configuration management system for Linux based
servers.

```
package "nginx" {
    value = "install"
}

network_config "net01" {
    value = "configure"
    ip = "192.168.1.20"
    netmask = "255.255.255.0"
    dns = "1.1.1.1"
}
```


