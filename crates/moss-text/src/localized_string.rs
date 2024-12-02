use serde::ser::{Serialize, SerializeSeq, Serializer};

use crate::{bstring::BStringForFrontend, ReadOnlyStr};

/// Macro for creating a `LocalizedString` with a given key, origin, and an optional description.
///
/// # Example
///
/// ```rust
/// use moss_text::{localized_string::LocalizedString, localize};
///
/// // Create a LocalizedString with only key and origin
/// let greeting = localize!("greeting.hello", "Hello, World!");
///
/// // Create a LocalizedString with key, origin, and description
/// let farewell = localize!("farewell.goodbye", "Goodbye!", "A friendly farewell");
/// ```
#[macro_export]
macro_rules! localize {
    // Pattern for two arguments: key and origin
    ($key:expr, $origin:expr) => {
        $crate::localized_string::LocalizedString::new(
            $key,
            $origin,
            Option::<moss_text::bstring::BStringForFrontend>::None,
        )
    };
    // Pattern for three arguments: key, origin, and description
    ($key:expr, $origin:expr, $description:expr) => {
        LocalizedString::new($key, $origin, Some($description))
    };
}

/// A structure representing a localized string with a key, original text, and optional description.
///
/// This struct is serialized as an array `[key, origin, description]` for frontend compatibility.
/// The `key` field serves as a unique identifier for the localization entry, while `origin`
/// is the fallback or original text, and `description` is optional metadata or context.
///
/// # Example
///
/// ```rust
/// use moss_text::{localized_string::LocalizedString, localize};
///
/// // Create a LocalizedString with key and origin only
/// let greeting = LocalizedString::new("greeting.hello", "Hello, World!", None);
///
/// // Create a LocalizedString with key, origin, and description
/// let farewell = LocalizedString::new("farewell.goodbye", "Goodbye!", Some("A friendly farewell"));
///
/// // Using the `localize!` macro for concise creation
/// let welcome = localize!("welcome.message", "Welcome!");
/// let exit = localize!("exit.message", "Goodbye!", "Displayed on exit");
/// ```
#[derive(Debug, Clone)]
pub struct LocalizedString {
    /// The unique key identifying the localized string.
    key: ReadOnlyStr,

    /// The original text or fallback string associated with the key.
    origin: BStringForFrontend,

    /// An optional description providing context or additional information.
    description: Option<BStringForFrontend>,
}

impl LocalizedString {
    /// Creates a new `LocalizedString` with the specified key, origin text, and an optional description.
    ///
    /// # Parameters
    ///
    /// - `key`: The localization key, typically a unique string identifier.
    /// - `origin`: The original or fallback text displayed if a localized version is unavailable.
    /// - `description`: An optional description providing additional context.
    ///
    /// # Returns
    ///
    /// Returns an instance of `LocalizedString`.
    pub fn new(
        key: impl Into<ReadOnlyStr>,
        origin: impl Into<BStringForFrontend>,
        description: Option<impl Into<BStringForFrontend>>,
    ) -> Self {
        Self {
            key: key.into(),
            origin: origin.into(),
            description: description.map(Into::into),
        }
    }
}

impl Serialize for LocalizedString {
    /// Custom serialization for `LocalizedString`.
    ///
    /// This implementation serializes `LocalizedString` as a JSON array `[key, origin, description]`.
    ///
    /// - `key`: Serialized as the first element, representing the unique localization key.
    /// - `origin`: Serialized as the second element, containing the original or fallback text.
    /// - `description`: Serialized as the third element, containing optional context or metadata.
    ///   If `description` is `None`, it will be represented as `null` in the array.
    ///
    /// # Example
    ///
    /// ```json
    /// ["greeting.hello", "Hello, World!", "A friendly greeting"]
    /// ```
    ///
    /// If `description` is absent:
    ///
    /// ```json
    /// ["greeting.hello", "Hello, World!", null]
    /// ```
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(3))?;
        seq.serialize_element(&self.key)?;
        seq.serialize_element(&self.origin)?;
        seq.serialize_element(&self.description)?;
        seq.end()
    }
}
