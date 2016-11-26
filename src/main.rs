#[macro_use]
extern crate clap;
extern crate git2;
#[macro_use]
extern crate lazy_static;
extern crate regex;
mod bump;

use bump::{Version, ReleaseType, push_tags};
use clap::{App, ArgGroup};


const ZERO_TAG: &'static str = "v0.0.0";


fn cli() -> Result<String, String> {
    // Parse args
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml)
        .group(ArgGroup::with_name("vers").args(&["init", "major", "minor", "patch"]))
        .version(crate_version!())
        .get_matches();
    // Init Version
    let path = matches.value_of("path").unwrap_or(".");
    let mut vers = try!(Version::from_tag(path, ZERO_TAG.to_string()));
    if matches.is_present("init") {
        println!("Initializing first bump");
    } else {
        vers = try!(Version::from_repo(path));
    }
    // Select release type
    let mut rel = ReleaseType::PATCH;
    if matches.is_present("major") {
        rel = ReleaseType::MAJOR;
    } else if matches.is_present("minor") {
        rel = ReleaseType::MINOR;
    }
    // Bump
    let bumped = vers.bump(rel);
    if matches.is_present("dry") {
        println!("DRY RUN: No changes will be made.");
    } else {
        try!(bumped.set_tag());
        //if matches.is_present("push") {
        //    let remote = matches.value_of("push").unwrap_or("origin");
        //    try!(push_tags(path, remote));
        //}
    }
    return Ok(format!("Bumped {} to {}",
                      bumped.release_type().string(),
                      bumped.tag()));
}

fn main() {
    match cli() {
        Ok(m) => println!("{}", m),
        Err(err) => println!("ERROR: {}", err),
    };
}
