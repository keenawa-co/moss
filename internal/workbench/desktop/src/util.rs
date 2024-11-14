// use std::{borrow::Borrow, fmt, sync::Arc};

// #[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash)]
// pub struct ReadOnlyStr(Arc<str>);

// impl ReadOnlyStr {
//     pub fn new<'a>(value: impl AsRef<str>) -> Self {
//         Self(Arc::<str>::from(value.as_ref()))
//     }
// }

// impl From<&str> for ReadOnlyStr {
//     fn from(s: &str) -> Self {
//         ReadOnlyStr::new(s)
//     }
// }

// impl From<String> for ReadOnlyStr {
//     fn from(s: String) -> Self {
//         ReadOnlyStr::new(s)
//     }
// }

// impl AsRef<str> for ReadOnlyStr {
//     fn as_ref(&self) -> &str {
//         &self.0
//     }
// }

// impl Borrow<str> for ReadOnlyStr {
//     fn borrow(&self) -> &str {
//         &self.0
//     }
// }

// impl fmt::Display for ReadOnlyStr {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.write_str(&self.0)
//     }
// }
