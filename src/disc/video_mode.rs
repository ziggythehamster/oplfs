use std::convert::TryFrom;
use std::fmt;

/// The video mode of a disc
#[derive(AsExpression, Clone, Copy, Debug, PartialEq, SqlStringEnum)]
#[sql_type = "Text"]
pub enum VideoMode {
    /// Supports multiple video modes
    Multi,

    /// Supports NTSC
    NTSC,

    /// Supports PAL
    PAL
}

/// Convert a video mode into a string
///
/// # Examples
///
/// ```
/// use oplfs::disc::VideoMode;
/// assert_eq!(VideoMode::NTSC.to_string(), "NTSC");
/// ```
impl fmt::Display for VideoMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VideoMode::Multi => write!(f, "multi"),
            VideoMode::NTSC  => write!(f, "NTSC"),
            VideoMode::PAL   => write!(f, "PAL")
        }
    }
}

/// Attempt to convert a string into a video mode
///
/// # Examples
///
/// ```
/// use oplfs::disc::VideoMode;
/// let v = VideoMode::try_from(String::from("NTSC"))?;
/// assert_eq!(v, VideoMode::NTSC);
/// # Ok::<(), Box<(dyn std::error::Error + Send + Sync + 'static)>>(())
/// ```
impl TryFrom<String> for VideoMode {
    type Error = Box<(dyn std::error::Error + Send + Sync + 'static)>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        VideoMode::try_from(value.as_str())
    }
}

/// Attempt to convert a string into a video mode
///
/// # Examples
///
/// ```
/// use oplfs::disc::VideoMode;
/// let v = VideoMode::try_from("NTSC")?;
/// assert_eq!(v, VideoMode::NTSC);
/// # Ok::<(), Box<(dyn std::error::Error + Send + Sync + 'static)>>(())
/// ```
impl TryFrom<&str> for VideoMode {
    type Error = Box<(dyn std::error::Error + Send + Sync + 'static)>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "multi" => Ok(VideoMode::Multi),
            "NTSC"  => Ok(VideoMode::NTSC),
            "PAL"   => Ok(VideoMode::PAL),
            x => Err(format!("unrecognized video mode {}", x).into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_handles_unknown_values() {
        assert_eq!(VideoMode::try_from("spectravision").unwrap_err().to_string(), "unrecognized video mode spectravision");
    }
}
