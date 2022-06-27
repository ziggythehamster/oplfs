#[macro_use]
extern crate delegate;

extern crate derive_more;

#[macro_use]
extern crate lazy_static;

use ascii::*;
use derive_more::{From, Index, Into, IntoIterator};
use regex::Regex;

use std::collections::HashMap;

/// A struct holding a parsed SYSTEM.CNF file
#[derive(Clone, Debug, Default, Eq, From, Index, Into, IntoIterator, PartialEq)]
pub struct SystemCnf(HashMap<AsciiString, AsciiString>);

lazy_static! {
    /// The ASCII string "BOOT2"
    static ref BOOT2_KEY: &'static AsciiStr = unsafe { "BOOT2".as_ascii_str_unchecked() };

    /// The title ID regular expression
    static ref TITLE_ID_RE: Regex = Regex::new(r"^(?:cdrom(?:\d)?:\\)?(?P<title_id>.+?)(?:;\d+)?$").unwrap();
}

impl SystemCnf {
    delegate! {
        to self.0 {
            pub fn get(&self, k: &AsciiString) -> Option<&AsciiString>;
        }
    }

    /// Returns the game's title ID from the boot path, if at all possible.
    ///
    /// # Examples
    ///
    /// With a normal ISO9660-compatible boot path:
    ///
    /// ```
    /// use ascii::*;
    /// use system_cnf_parser::SystemCnf;
    ///
    /// let system_cnf = "BOOT2 = cdrom0:\\SCUS_987.65;1\n".into_ascii_string()?;
    ///
    /// assert_eq!(SystemCnf::from(&system_cnf).title_id().unwrap(), "SCUS_987.65");
    ///
    /// # Ok::<(), Box<(dyn std::error::Error + Send + Sync + 'static)>>(())
    /// ```
    ///
    /// With a boot path missing the ISO9660 version ID
    ///
    /// ```
    /// use ascii::*;
    /// use system_cnf_parser::SystemCnf;
    ///
    /// let system_cnf = "BOOT2 = cdrom0:\\SCUS_987.65\n".into_ascii_string()?;
    ///
    /// assert_eq!(SystemCnf::from(&system_cnf).title_id().unwrap(), "SCUS_987.65");
    ///
    /// # Ok::<(), Box<(dyn std::error::Error + Send + Sync + 'static)>>(())
    /// ```
    ///
    /// With the bare minimum mandatory things to parse:
    ///
    /// ```
    /// use ascii::*;
    /// use system_cnf_parser::SystemCnf;
    ///
    /// let system_cnf = "BOOT2 = cdrom:\\SLES_001.01\n".into_ascii_string()?;
    ///
    /// assert_eq!(SystemCnf::from(&system_cnf).title_id().unwrap(), "SLES_001.01");
    ///
    /// # Ok::<(), Box<(dyn std::error::Error + Send + Sync + 'static)>>(())
    /// ```
    ///
    /// With the BOOT2 line from Gauntlet - Seven Sorrows:
    ///
    /// ```
    /// use ascii::*;
    /// use system_cnf_parser::SystemCnf;
    ///
    /// let system_cnf = "BOOT2 = cdrom0:\\SLUS_210.77;1    \n".into_ascii_string()?;
    ///
    /// assert_eq!(SystemCnf::from(&system_cnf).title_id().unwrap(), "SLUS_210.77");
    ///
    /// # Ok::<(), Box<(dyn std::error::Error + Send + Sync + 'static)>>(())
    pub fn title_id(&self) -> Option<&str> {
        self.0.get(*BOOT2_KEY).and_then(|boot2| {
            if let Some(captures) = TITLE_ID_RE.captures(boot2.trim().as_str()) {
                captures.name("title_id").map(|c| c.as_str())
            } else {
                None
            }
        })
    }
}

/// Conversion from a SYSTEM.CNF file
impl From<&AsciiStr> for SystemCnf {
    /// Parses a SYSTEM.CNF
    ///
    /// # Examples
    ///
    /// ```
    /// use ascii::*;
    /// use system_cnf_parser::SystemCnf;
    ///
    /// // You would load this from disc
    /// let system_cnf = concat!(
    ///    "BOOT2 = cdrom0:\\SCUS_123.45;1\n",
    ///    "VER = 1.00\n",
    ///    "VMODE = NTSC\n"
    /// ).into_ascii_string()?;
    ///
    /// assert_eq!(
    ///     SystemCnf::from(&system_cnf).get(&"BOOT2".into_ascii_string()?).unwrap(),
    ///     &"cdrom0:\\SCUS_123.45;1".into_ascii_string()?
    /// );
    ///
    /// # Ok::<(), Box<(dyn std::error::Error + Send + Sync + 'static)>>(())
    /// ```
    fn from(data: &AsciiStr) -> Self {
        let mut system_cnf = HashMap::new();

        for line in data.lines() {
            let mut kv = None;

            for (index, char) in line.chars().enumerate() {
                if char != AsciiChar::Equal { continue; }

                kv = Some((line[..index].trim_end(), line[index+1..line.len()].trim_start()));

                break;
            }

            if let Some((key, value)) = kv {
                system_cnf.insert(key.to_ascii_string(), value.to_ascii_string());
            }
        }

        Self(system_cnf)
    }
}

/// Conversion from a SYSTEM.CNF file
impl From<&AsciiString> for SystemCnf {
    fn from(data: &AsciiString) -> Self {
        // NB: this is unchecked because AsciiString is already checked and checking it twice is pointless
        Self::from(unsafe { data.as_ascii_str_unchecked() })
    }
}

/// Conversion from a SYSTEM.CNF file
impl From<AsciiString> for SystemCnf {
    fn from(data: AsciiString) -> Self {
        // NB: this is unchecked because AsciiString is already checked and checking it twice is pointless
        Self::from(unsafe { data.as_ascii_str_unchecked() })
    }
}
