use git2;
use regex;

// const SEMVER_FORMAT: &'static str = "v{MAJOR}.{MINOR}.{PATCH}";
//
const REFSPECS: &'static str = "refs/tags/*:refs/tags/*";
const SEMVER_REGEX: &'static str = r"v(?P<MAJOR>\d+).(?P<MINOR>\d+).(?P<PATCH>\d+)";
lazy_static! {
    static ref RE: regex::Regex = regex::Regex::new(SEMVER_REGEX).unwrap();
}

pub struct Version<'a> {
    major: u16,
    minor: u16,
    patch: u16,
    path: &'a str,
    rel: ReleaseType,
}

impl<'a> Version<'a> {
    // init from a git directory path
    pub fn from_repo(path: &str) -> Result<Version, String> {
        let repo = try!(git2::Repository::open(path).map_err(|e| e.to_string()));
        let desc = try!(repo.describe(&git2::DescribeOptions::new().describe_tags())
            .map_err(|e| format!("{} (try --init)", e.message())));
        let tag = try!(desc.format(Some(&git2::DescribeFormatOptions::new()))
            .map_err(|e| e.to_string()));
        let vers = try!(Version::from_tag(path, tag));
        return Ok(vers);
    }

    // from_tag builds from a tagged string
    pub fn from_tag(path: &str, tag: String) -> Result<Version, String> {
        let caps = match RE.captures(&tag) {
            None => {
                return Err(format!("Latest tag '{}' did not match semver format 'v0.0.0'", tag))
            }
            Some(caps) => caps, 
        };
        let major = caps.name("MAJOR").unwrap().parse::<u16>().unwrap();
        let minor = caps.name("MINOR").unwrap().parse::<u16>().unwrap();
        let patch = caps.name("PATCH").unwrap().parse::<u16>().unwrap();
        let vers = Version {
            path: path,
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

    pub fn release_type(&self) -> ReleaseType {
        return self.rel;
    }

    pub fn set_tag(&self) -> Result<(), String> {
        let spec = "HEAD";
        let repo = try!(git2::Repository::open(self.path).map_err(|e| e.to_string()));
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
            ..*self
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

// TODO complete auth
pub fn push_tags(path: &str, remote: &str) -> Result<(), String> {
    let repo = try!(git2::Repository::open(path).map_err(|e| e.to_string()));
    let mut remote = try!(repo.find_remote(remote).map_err(|e| e.to_string()));
    try!(remote.push(&[REFSPECS],
              Some(&mut git2::PushOptions::new().remote_callbacks(git2::RemoteCallbacks::new())))
        .map_err(|e| e.to_string()));
    return Ok(());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        let ver = Version::from_tag(".", "v5.5.5".to_owned()).unwrap();
        assert_eq!("v5.5.6", ver.bump(ReleaseType::PATCH).tag());
        assert_eq!("v5.6.0", ver.bump(ReleaseType::MINOR).tag());
        assert_eq!("v6.0.0", ver.bump(ReleaseType::MAJOR).tag());
        // TODO assert bad tag
    }
}
