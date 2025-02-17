use semver::{Version, VersionReq};

pub fn check_version(version: &str, req: &str) -> bool {
    let version = Version::parse(version).expect("Invalid version format");
    let req = VersionReq::parse(req).expect("Invalid version requirement");
    req.matches(&version)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_check() {
        assert!(check_version("1.2.3", ">=1.0, <2.0"));
        assert!(!check_version("2.0.0", ">=1.0, <2.0"));
    }
}
