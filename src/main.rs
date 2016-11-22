#[macro_use]
extern crate clap;
extern crate git2;
#[macro_use]
extern crate lazy_static;
extern crate regex;
mod bump;

use clap::{App, ArgGroup};
use bump::{Version, ReleaseType};

const ZERO_TAG: &'static str = "v0.0.0";

fn cli() -> Result<String, String> {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml)
        .group(ArgGroup::with_name("vers").args(&["init", "major", "minor", "patch"]))
        .get_matches();
    let mut ver = try!(Version::from_tag(ZERO_TAG.to_string()));
    if matches.is_present("init") {
        println!("Initializing first bump");
    } else {
        ver = try!(Version::init("."));
    }
    let mut rel = ReleaseType::PATCH;
    if matches.is_present("major") {
        rel = ReleaseType::MAJOR;
    } else if matches.is_present("minor") {
        rel = ReleaseType::MINOR;
    }
    let bumped = ver.bump(rel);
    if matches.is_present("dry") {
        println!("DRY RUN: No changes will be made.");
    } else {
        try!(bumped.set_tag());
    }
    return Ok(format!("Bumped {} to {}", rel.string(), bumped.tag()));
}

fn main() {
    match cli() {
        Ok(m) => println!("{}", m),
        Err(err) => println!("ERROR: {}", err),
    };
}
