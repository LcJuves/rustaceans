#![allow(dead_code)]

use std::sync::Once;

pub(crate) mod data;
pub(crate) use data::MIME_MAPPING;

pub struct MediaType;

impl MediaType {
    /// Get the media type from the file suffix
    ///
    /// # Examples
    /// ```rust
    /// assert_eq!("image/png", MediaType::from_file_extension("png").unwrap());
    /// assert_eq!(
    ///     "image/jpeg",
    ///     MediaType::from_file_extension("jpeg").unwrap()
    /// );
    /// assert_eq!(
    ///     "image/svg+xml",
    ///     MediaType::from_file_extension("svg").unwrap()
    /// );
    /// assert_eq!(
    ///     "application/json",
    ///     MediaType::from_file_extension("json").unwrap()
    /// );
    /// ```
    pub fn from_file_extension(ext: &str) -> Option<String> {
        for (fext, media_ty) in MIME_MAPPING.iter() {
            if *fext == ext {
                return Some((*media_ty).to_string());
            }
        }
        None
    }
}

pub struct FileExtension;

impl FileExtension {
    const ONCE_INIT: Once = Once::new();

    /// Get its suffix by file media type
    ///
    /// Return one or more results
    ///
    /// # Examples
    /// ```rust
    /// assert_eq!(
    ///     vec!["png"],
    ///     FileExtension::from_media_type("image/png").unwrap()
    /// );
    /// assert_eq!(
    ///     vec!["jpe", "jpeg", "jpg"],
    ///     FileExtension::from_media_type("image/jpeg").unwrap()
    /// );
    /// assert_eq!(
    ///     vec!["svg", "svgz"],
    ///     FileExtension::from_media_type("image/svg+xml").unwrap()
    /// );
    /// assert_eq!(
    ///     vec!["json"],
    ///     FileExtension::from_media_type("application/json").unwrap()
    /// );
    /// ```
    pub fn from_media_type(media_type: &str) -> Option<Vec<String>> {
        let mut found = false;
        let mut ret = Vec::<String>::new();
        for (fext, media_ty) in MIME_MAPPING.iter() {
            if *media_ty == media_type {
                Self::ONCE_INIT.call_once(|| found = true);
                ret.push((*fext).to_string());
            }
        }
        match found {
            true => Some(ret),
            _ => None,
        }
    }
}
