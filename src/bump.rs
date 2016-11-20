pub struct Version {
    rel: ReleaseType,
    major: u8,
    minor: u8,
    patch: u8,
    ext: String,
    tag: String,
}

impl Version {
    pub fn init() -> Version {
        let tag = "fake-tag".to_string();
        Version {
            tag: tag,
            rel: ReleaseType::GIT,
        }
    }
    fn parse_tag() -> Version {}
    pub fn bump(&self, rel: ReleaseType) -> Version {
        match rel {
            ReleaseType::MAJOR => self.bump_major(),
            ReleaseType::MINOR => self.bump_minor(),
            ReleaseType::PATCH => self.bump_patch(),
        }
    }
    pub fn force(&self, tag: String) -> Version {
        return Version {
            tag: tag,
            rel: ReleaseType::FORCE,
        };
    }
    fn bump_major(&self) -> Version {}
    fn bump_minor(&self) -> Version {}
    fn bump_patch(&self) -> Version {}
}

enum ReleaseType {
    MAJOR,
    MINOR,
    PATCH,
    FORCE,
    COMMIT,
}

impl ReleaseType {
    fn string(&self) -> &str {
        match *self {
            ReleaseType::MAJOR => "major",
            ReleaseType::MINOR => "minor",
            ReleaseType::PATCH => "patch",
            ReleaseType::FORCE => "force", 
            ReleaseType::COMMIT => "commit", 
        }
    }
}
