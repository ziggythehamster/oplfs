use std::convert::TryFrom;
use std::fmt;

/// The platform of the disc. PS1 will be supported in the future.
#[derive(AsExpression, Clone, Copy, Debug, SqlStringEnum, PartialEq)]
#[sql_type = "Text"]
pub enum Platform {
    /// A PlayStation 2 disc
    PS2
}

/// Convert a platform into a string
///
/// # Examples
///
/// ```
/// use oplfs::Platform;
/// assert_eq!(Platform::PS2.to_string(), "PS2");
/// ```
impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Platform::PS2 => write!(f, "PS2")
        }
    }
}

/// Attempt to convert a string into a platform
///
/// # Examples
///
/// ```
/// use oplfs::Platform;
/// let p = Platform::try_from(String::from("PS2"))?;
/// assert_eq!(p, Platform::PS2);
/// # Ok::<(), Box<(dyn std::error::Error + Send + Sync + 'static)>>(())
/// ```
impl TryFrom<String> for Platform {
    type Error = Box<(dyn std::error::Error + Send + Sync + 'static)>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Platform::try_from(value.as_str())
    }
}

/// Attempt to convert a string into a platform
///
/// # Examples
///
/// ```
/// use oplfs::Platform;
/// let p = Platform::try_from("PS2")?;
/// assert_eq!(p, Platform::PS2);
/// # Ok::<(), Box<(dyn std::error::Error + Send + Sync + 'static)>>(())
/// ```
impl TryFrom<&str> for Platform {
    type Error = Box<(dyn std::error::Error + Send + Sync + 'static)>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "PS2"   => Ok(Platform::PS2),
            x => Err(format!("unrecognized platform {}", x).into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_handles_unknown_values() {
        assert_eq!(Platform::try_from("PS9000").unwrap_err().to_string(), "unrecognized platform PS9000");
    }
}
