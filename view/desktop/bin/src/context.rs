use desktop_models::appearance::theming::ThemeDescriptor;

use crate::cmd_dummy::Locale;

pub struct Appearance {
    pub theme: ThemeDescriptor,
    pub primary_color: String,   // TODO: change to Color type
    pub statusbar_color: String, // TODO: change to Color type
}

struct MockType;

/// The main storage and control object for a application.
pub struct Context {
    appearance: Appearance,
    locale: Locale,
    cache: MockType,
    menus: MockType,
    views: MockType,
    data: MockType,
    signals: MockType,
    event_queue: MockType, // ?
    listeners: MockType,
}
