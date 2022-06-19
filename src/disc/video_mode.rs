use diesel::backend::Backend;
use diesel::deserialize;
use diesel::serialize;
use diesel::sql_types::Text;
use diesel::types::{ToSql, FromSql};
use std::convert::TryFrom;
use std::io::Write;

/// The video mode of a disc
#[derive(AsExpression, Clone, Copy, Debug)]
#[sql_type = "Text"]
pub enum VideoMode {
    /// Supports multiple video modes
    Multi,

    /// Supports NTSC
    NTSC,

    /// Supports PAL
    PAL
}

/// Attempt to convert a string into a video mode
impl TryFrom<String> for VideoMode {
    type Error = Box<(dyn std::error::Error + Send + Sync + 'static)>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        VideoMode::try_from(value.as_str())
    }
}

/// Attempt to convert a string into a video mode
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

/// Deserialize the video mode from a database text field
impl<DB> FromSql<Text, DB> for VideoMode
where DB: Backend,
      String: FromSql<Text, DB>
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        VideoMode::try_from(String::from_sql(bytes)?)
    }
}

/// Serialize the video mode into a database text field
impl<DB> ToSql<Text, DB> for VideoMode
where DB: Backend,
      str: ToSql<Text, DB>
{
    fn to_sql<W: Write>(&self, out: &mut serialize::Output<W, DB>) -> serialize::Result {
        match self {
            VideoMode::Multi => "multi",
            VideoMode::NTSC  => "NTSC",
            VideoMode::PAL   => "PAL"
        }.to_sql(out)
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
