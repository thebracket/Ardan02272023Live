#[cfg(feature = "normal")]
pub const MODE: &str = "NORMAL";

#[cfg(all(not(feature = "normal"), feature = "other"))]
pub const MODE: &str = "OTHER";