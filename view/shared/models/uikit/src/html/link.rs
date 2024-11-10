/// Represents an HTML link (`<a>`) with attributes commonly used in web development.
#[derive(Debug, Serialize, TS)]
pub struct HtmlLink {
    href: String,           // The URL the link points to
    target: Option<Target>, // Optional target attribute for specifying where to open the link
    rel: Option<Rel>, // Optional rel attribute for relationship between linked document and current document
    text: String,     // The text displayed for the link
}

impl HtmlLink {
    /// Creates a new `HtmlLink` with a specified `href` and `text`.
    ///
    /// # Arguments
    /// * `href` - The URL to which the link should point.
    /// * `text` - The display text for the link.
    ///
    /// # Example
    /// ```
    /// let link = HtmlLink::new("https://example.com", "Visit Example");
    /// ```
    pub fn new(href: &str, text: &str) -> Self {
        Self {
            href: href.to_string(),
            target: None,
            rel: None,
            text: text.to_string(),
        }
    }

    /// Sets the `target` attribute for the link, specifying where to open the link.
    pub fn target(mut self, target: Target) -> Self {
        self.target = Some(target);
        self
    }

    /// Sets the `rel` attribute for the link, defining the relationship between
    /// the current document and the linked resource.
    pub fn rel(mut self, rel: Rel) -> Self {
        self.rel = Some(rel);
        self
    }
}

/// Represents possible values for the `target` attribute in an HTML link.
///
/// The `target` attribute specifies where to open the linked document.
#[derive(Debug, Serialize, TS)]
pub enum Target {
    /// Opens the link in the same frame as it was clicked (default behavior).
    #[serde(rename = "_self")]
    SelfTarget,
    /// Opens the link in a new tab or window.
    #[serde(rename = "_blank")]
    Blank,
    /// Opens the link in the parent frame.
    #[serde(rename = "_parent")]
    Parent,
    /// Opens the link in the top-most frame.
    #[serde(rename = "_top")]
    Top,
}

impl ToString for Target {
    fn to_string(&self) -> String {
        match self {
            Target::SelfTarget => "_self".to_string(),
            Target::Blank => "_blank".to_string(),
            Target::Parent => "_parent".to_string(),
            Target::Top => "_top".to_string(),
        }
    }
}

/// Represents possible values for the `rel` attribute in an HTML link.
///
/// The `rel` attribute specifies the relationship between the current document
/// and the linked document.
#[derive(Debug, Serialize, TS)]
pub enum Rel {
    /// Specifies that the target should not have any control over the current page.
    #[serde(rename = "noopener")]
    Noopener,
    /// Prevents the browser from sending a referrer header when following the link.
    #[serde(rename = "noreferrer")]
    Noreferrer,
    /// Specifies that the linked document is not endorsed by the current document.
    #[serde(rename = "nofollow")]
    NoFollow,
}

impl ToString for Rel {
    fn to_string(&self) -> String {
        match self {
            Rel::Noopener => "noopener".to_string(),
            Rel::Noreferrer => "noreferrer".to_string(),
            Rel::NoFollow => "nofollow".to_string(),
        }
    }
}
