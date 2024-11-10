use bstr::{BStr, BString, ByteSlice};
use serde::{Serialize, Serializer};
use std::ops::{Deref, DerefMut};

/// A variant of `BString` for structures serialized as strings for frontend display.
///
/// ### Note
///
/// While `BString` has its own serialization implementation, it serializes as a byte array,
/// which can break the UI. When `BString` is involved, either custom serialization or this type
/// is needed to ensure correct frontend display.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub struct BStringForFrontend(BString);

impl Serialize for BStringForFrontend {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.to_str_lossy().serialize(s)
    }
}

impl Deref for BStringForFrontend {
    type Target = BString;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BStringForFrontend {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<[u8]> for BStringForFrontend {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl AsRef<BStr> for BStringForFrontend {
    fn as_ref(&self) -> &BStr {
        self.0.as_ref()
    }
}

impl From<String> for BStringForFrontend {
    fn from(value: String) -> Self {
        BStringForFrontend(value.into())
    }
}

impl From<BString> for BStringForFrontend {
    fn from(value: BString) -> Self {
        BStringForFrontend(value)
    }
}

impl From<&BStr> for BStringForFrontend {
    fn from(value: &BStr) -> Self {
        BStringForFrontend(value.into())
    }
}

impl From<&str> for BStringForFrontend {
    fn from(value: &str) -> Self {
        BStringForFrontend(value.into())
    }
}

impl PartialEq<&str> for BStringForFrontend {
    fn eq(&self, other: &&str) -> bool {
        self.0.eq(other)
    }
}
