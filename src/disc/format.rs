use std::convert::TryFrom;
use std::fmt;

/// The format of the disc. In the future, the filesystem will support
/// ZSO and BIN/CUE.
#[derive(AsExpression, Clone, Copy, Debug, PartialEq, SqlStringEnum)]
#[sql_type = "Text"]
pub enum Format {
    /// ISO9660 image
    ISO
}

/// Convert a format into a string
///
/// # Examples
///
/// ```
/// use oplfs::Format;
/// assert_eq!(Format::ISO.to_string(), "ISO");
/// ```
impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Format::ISO => write!(f, "ISO")
        }
    }
}

/// Attempt to convert a string into an image format
///
/// # Examples
///
/// ```
/// use oplfs::Format;
/// let f = Format::try_from(String::from("ISO"))?;
/// assert_eq!(f, Format::ISO);
/// # Ok::<(), Box<(dyn std::error::Error + Send + Sync + 'static)>>(())
/// ```
impl TryFrom<String> for Format {
    type Error = Box<(dyn std::error::Error + Send + Sync + 'static)>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Format::try_from(value.as_str())
    }
}

/// Attempt to convert a string into an image format
///
/// # Examples
///
/// ```
/// use oplfs::Format;
/// let f = Format::try_from("ISO")?;
/// assert_eq!(f, Format::ISO);
/// # Ok::<(), Box<(dyn std::error::Error + Send + Sync + 'static)>>(())
/// ```
impl TryFrom<&str> for Format {
    type Error = Box<(dyn std::error::Error + Send + Sync + 'static)>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "ISO"    => Ok(Format::ISO),
            x  => Err(format!("unrecognized format {}", x).into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_handles_unknown_values() {
        assert_eq!(Format::try_from("FLP").unwrap_err().to_string(), "unrecognized format FLP");
    }
}
