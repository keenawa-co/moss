use moss_addon::BUILTIN_ADDONS_DIR;
use moss_cache::{backend::moka::MokaBackend, Cache};
use moss_text::ReadOnlyStr;
use parking_lot::RwLock;
use std::sync::Arc;
use std::time::Duration;

use crate::command::CommandHandler;
use crate::contribution_registry::ContributionRegistry;
use crate::models::application::{LocaleDescriptor, ThemeDescriptor};

const STATE_CACHE_TTL: Duration = Duration::from_secs(60 * 3);
const STATE_MAX_CAPACITY: u64 = 100;

pub struct Preferences {
    pub theme: RwLock<Option<ThemeDescriptor>>,
    pub locale: RwLock<Option<LocaleDescriptor>>,
}

pub struct AppDefaults {
    pub theme: ThemeDescriptor,
    pub locale: LocaleDescriptor,
}

pub struct AppStateManager {
    pub contributions: ContributionRegistry,
    pub cache: Arc<Cache<MokaBackend>>,
    pub preferences: Preferences,
    pub defaults: AppDefaults,
}

impl AppStateManager {
    pub fn new() -> Self {
        let cache = Cache::new(MokaBackend::new(STATE_MAX_CAPACITY, STATE_CACHE_TTL));

        Self {
            cache: Arc::new(cache),
            preferences: Preferences {
                theme: RwLock::new(None),
                locale: RwLock::new(None),
            },
            defaults: AppDefaults {
                theme: ThemeDescriptor {
                    id: "theme-defaults.MossLightDefault".to_string(),
                    name: "Moss Light Default".to_string(),
                    source: BUILTIN_ADDONS_DIR
                        .join("theme-defaults")
                        .join("themes")
                        .join("light-default.json")
                        .to_string_lossy()
                        .to_string(),
                },
                locale: LocaleDescriptor {
                    code: "en".to_string(),
                    name: "English".to_string(),
                    direction: Some("ltr".to_string()),
                },
            },
            contributions: ContributionRegistry::new()
                .init(crate::contribution::CONTRIBUTIONS.iter().map(|c| &**c)),
        }
    }

    pub fn get_command(&self, id: &ReadOnlyStr) -> Option<CommandHandler> {
        self.contributions
            .commands
            .get(id)
            .map(|cmd| Arc::clone(&cmd))
    }

    pub fn change_language_pack(&self, locale_descriptor: LocaleDescriptor) {
        let mut locale_lock = self.preferences.locale.write();
        *locale_lock = Some(locale_descriptor);
    }

    pub fn change_color_theme(&self, theme_descriptor: ThemeDescriptor) {
        let mut theme_descriptor_lock = self.preferences.theme.write();
        *theme_descriptor_lock = Some(theme_descriptor);
    }
}
