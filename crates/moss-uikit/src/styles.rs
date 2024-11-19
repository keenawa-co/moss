/// Represents the alignment of an element within a container.
///
/// This enum is used to define how an element is positioned horizontally
/// or vertically relative to its container or surrounding elements.
///
/// Variants:
/// - `Start`: Aligns the element to the start of its container (e.g., `align-items: flex-start`).
/// - `Center`: Aligns the element to the center of its container.
/// - `End`: Aligns the element to the end of its container (e.g., `align-items: flex-end`).
#[derive(Serialize, Debug, Clone, Eq, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "styles.ts")]
pub enum Alignment {
    Start,
    Center,
    End,
}

/// Represents the orientation of an element or layout.
///
/// This enum is commonly used to define whether an element or layout
/// is arranged vertically or horizontally.
///
/// Variants:
/// - `Vertical`: The element or layout is oriented vertically.
/// - `Horizontal`: The element or layout is oriented horizontally.
#[derive(Serialize, Debug, Clone, Eq, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "styles.ts")]
pub enum Orientation {
    Vertical,
    Horizontal,
}
