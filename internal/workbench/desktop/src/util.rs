use std::{borrow::Borrow, fmt, sync::Arc};

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct ReadOnlyId(Arc<str>);

impl ReadOnlyId {
    pub fn new<'a>(value: impl AsRef<str>) -> Self {
        Self(Arc::<str>::from(value.as_ref()))
    }
}

impl From<&str> for ReadOnlyId {
    fn from(s: &str) -> Self {
        ReadOnlyId::new(s)
    }
}

impl From<String> for ReadOnlyId {
    fn from(s: String) -> Self {
        ReadOnlyId::new(s)
    }
}

impl AsRef<str> for ReadOnlyId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for ReadOnlyId {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ReadOnlyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
