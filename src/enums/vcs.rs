/// Supported VCS
#[derive(Debug, PartialEq)]
pub enum VersionControlSystem {
    Git,
    Mercurial,
    Subversion,
    Bazaar,
    Unknown,
}
