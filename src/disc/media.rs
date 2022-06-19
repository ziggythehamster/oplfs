use std::convert::TryFrom;
use std::fmt;

/// The physical media of the disc, either CD or DVD.
#[derive(AsExpression, Clone, Copy, Debug, PartialEq, SqlStringEnum)]
#[sql_type = "Text"]
pub enum Media {
    /// A PS2 CD or a PS1 disc
    CD,
    /// A PS2 single or dual layer DVD
    DVD
}

/// Convert a media type into a string
///
/// # Examples
///
/// ```
/// use oplfs::disc::Media;
/// assert_eq!(Media::CD.to_string(), "CD");
/// ```
impl fmt::Display for Media {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Media::CD  => write!(f, "CD"),
            Media::DVD => write!(f, "DVD")
        }
    }
}

/// Attempt to convert a string into a media type
///
/// # Examples
///
/// ```
/// use oplfs::disc::Media;
/// let m = Media::try_from(String::from("CD"))?;
/// assert_eq!(m, Media::CD);
/// # Ok::<(), Box<(dyn std::error::Error + Send + Sync + 'static)>>(())
/// ```
impl TryFrom<String> for Media {
    type Error = Box<(dyn std::error::Error + Send + Sync + 'static)>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Media::try_from(value.as_str())
    }
}

/// Attempt to convert a string into a media type
///
/// # Examples
///
/// ```
/// use oplfs::disc::Media;
/// let m = Media::try_from("CD")?;
/// assert_eq!(m, Media::CD);
/// # Ok::<(), Box<(dyn std::error::Error + Send + Sync + 'static)>>(())
/// ```
impl TryFrom<&str> for Media {
    type Error = Box<(dyn std::error::Error + Send + Sync + 'static)>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "CD"     => Ok(Media::CD),
            "DVD"    => Ok(Media::DVD),
            x  => Err(format!("unrecognized media {}", x).into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_handles_unknown_values() {
        assert_eq!(Media::try_from("VHS").unwrap_err().to_string(), "unrecognized media VHS");
    }
}
