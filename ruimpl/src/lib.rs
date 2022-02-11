//! # ruimpl
//!
//! A library for rust utility implements.

/// Remove EOL from the primitive [`str`].
///
/// # Examples
/// ```rust
/// use ruimpl::rmeol;
///
/// assert_eq!(rmeol("a"), "a");
/// assert_eq!(rmeol("a\r\n"), "a");
/// assert_eq!(rmeol("a\n"), "a");
/// ```
pub fn rmeol(r#str: &str) -> String {
    r#str[..r#str.rfind("\r").unwrap_or(r#str.rfind("\n").unwrap_or(r#str.len()))].to_owned()
}

#[cfg(test)]
mod tests {}
