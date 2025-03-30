use crate::colour_print;
use serde::Deserialize;
use std::fs;
use toml::de::from_str;

#[derive(Debug, Deserialize)]
struct Package {
    name: String,
    version: String,
    edition: String,
}

#[derive(Debug, Deserialize)]
struct CargoToml {
    package: Package,
}

fn parse_cargo_toml(file_path: &str) {
    // Read the content of the Cargo.toml file
    let content = fs::read_to_string(file_path).expect("Failed to read Cargo.toml file");

    // Parse the TOML content into the CargoToml struct
    let cargo_toml: CargoToml = from_str(&content).expect("Failed to parse Cargo.toml");

    // Print the extracted package information
    // println!("\t Package Name: {}", cargo_toml.package.name);
    // println!("\t Package Version: {}", cargo_toml.package.version);
    // println!("\t Package Edition: {}", cargo_toml.package.edition);

    let out_message_0 = format!("\t Package name: '{}'.", cargo_toml.package.name);
    colour_print(&out_message_0, "purple");

    let out_message_1 = format!("\t Package version: '{}'.", cargo_toml.package.version);
    colour_print(&out_message_1, "purple");

    let out_message_2 = format!("\t Package edition: '{}'.\n", cargo_toml.package.edition);
    colour_print(&out_message_2, "purple");
}

pub fn main() {
    let file_path = "Cargo.toml"; // Path to your Cargo.toml file
    parse_cargo_toml(file_path);
}
