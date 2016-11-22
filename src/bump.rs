use git2;
use regex;

// const SEMVER_FORMAT: &'static str = "v{MAJOR}.{MINOR}.{PATCH}";
const SEMVER_REGEX: &'static str = r"v(?P<MAJOR>\d+).(?P<MINOR>\d+).(?P<PATCH>\d+)";
lazy_static! {
    static ref RE: regex::Regex = regex::Regex::new(SEMVER_REGEX).unwrap();
}

#[derive(Debug)]
pub struct Version {
    rel: ReleaseType,
    major: u16,
    minor: u16,
    patch: u16,
}

impl Version {
    // init from a git directory path
    pub fn init(dir: &str) -> Result<Version, String> {
        let repo = try!(git2::Repository::open(dir).map_err(|e| e.to_string()));
        // TODO hint --init
        let desc = try!(repo.describe(&git2::DescribeOptions::new().describe_tags())
            .map_err(|e| String::from(e.message())));
        let tag = try!(desc.format(Some(&git2::DescribeFormatOptions::new()))
            .map_err(|e| e.to_string()));
        let v = try!(Version::from_tag(tag));
        return Ok(v);
    }

    // from_tag builds from a tagged string
    pub fn from_tag(tag: String) -> Result<Version, String> {
        let caps = match RE.captures(&tag) {
            // TODO show tag
            None => return Err(String::from("Tag did not match semver format")),
            Some(caps) => caps, 
        };
        let major = caps.name("MAJOR").unwrap().parse::<u16>().unwrap();
        let minor = caps.name("MINOR").unwrap().parse::<u16>().unwrap();
        let patch = caps.name("PATCH").unwrap().parse::<u16>().unwrap();
        let vers = Version {
            major: major,
            minor: minor,
            patch: patch,
            rel: ReleaseType::COMMIT,
        };
        return Ok(vers);
    }

    pub fn tag(&self) -> String {
        return format!("v{MAJOR}.{MINOR}.{PATCH}",
                       MAJOR = self.major,
                       MINOR = self.minor,
                       PATCH = self.patch);
    }

    pub fn bump(&self, rel: ReleaseType) -> Version {
        match rel {
            ReleaseType::MAJOR => self.bump_major(),
            ReleaseType::MINOR => self.bump_minor(),
            _ => self.bump_patch(),
        }
    }

    pub fn set_tag(&self) -> Result<(), String> {
        let spec = "HEAD";
        let repo = try!(git2::Repository::open(".").map_err(|e| e.to_string()));
        match repo.describe(&git2::DescribeOptions::new()
            .describe_tags()
            .max_candidates_tags(0)) {
            Err(_) => (),
            Ok(_) => return Err(String::from("Spec already has tag")),
        }
        let rev = try!(repo.revparse_single(spec).map_err(|e| e.to_string()));
        try!(repo.tag_lightweight(self.tag().as_str(), &rev, false).map_err(|e| e.to_string()));
        return Ok(());
    }

    fn bump_major(&self) -> Version {
        return Version {
            rel: ReleaseType::MAJOR,
            major: self.major + 1,
            minor: 0,
            patch: 0,
        };
    }

    fn bump_minor(&self) -> Version {
        return Version {
            rel: ReleaseType::MINOR,
            minor: self.minor + 1,
            patch: 0,
            ..*self
        };
    }
    fn bump_patch(&self) -> Version {
        return Version {
            rel: ReleaseType::PATCH,
            patch: self.patch + 1,
            ..*self
        };
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ReleaseType {
    MAJOR,
    MINOR,
    PATCH,
    COMMIT,
}

impl ReleaseType {
    pub fn string(&self) -> &str {
        match *self {
            ReleaseType::MAJOR => "major",
            ReleaseType::MINOR => "minor",
            ReleaseType::PATCH => "patch",
            ReleaseType::COMMIT => "commit", 
        }
    }
}
