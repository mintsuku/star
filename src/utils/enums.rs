#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArchiveType {
    Gz,
    Xz,
    Bz2,
    Unknown,
}
