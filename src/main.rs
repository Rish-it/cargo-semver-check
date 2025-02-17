use semver::{Version, VersionReq};

fn main() {
    let version = Version::parse("1.2.3").unwrap();
    let req = VersionReq::parse(">=1.0, <2.0").unwrap();

    if req.matches(&version) {
        println!("Version {} satisfies the requirement {}", version, req);
    } else {
        println!("Version {} does not satisfy the requirement {}", version, req);
    }
}
pub fn new_feature() {
    println!("This is a new feature!");
}
