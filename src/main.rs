#[macro_use]
extern crate clap;
use clap::{App, ArgGroup};
mod bump;

fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml)
        .group(ArgGroup::with_name("vers").args(&["major", "minor", "patch", "force"]))
        .get_matches();

    let v = bump::Version::init();
    if matches.is_present("force") {
        let ver = matches.value_of("force").unwrap();
        println!("Bumped VERSION to {}", ver);
    } else if matches.is_present("major") {
        println!("Bumped MAJOR")
    } else if matches.is_present("minor") {
        println!("Bumped MINOR")
    } else {
        println!("Bumped PATCH")
    }
}
