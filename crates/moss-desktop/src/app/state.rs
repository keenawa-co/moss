use moss_cache::{backend::moka::MokaBackend, Cache};
use moss_text::ReadOnlyStr;
use parking_lot::RwLock;
use std::time::Duration;
use std::{sync::atomic::AtomicUsize, sync::Arc};

use crate::command::CommandHandler;
use crate::contribution_registry::ContributionRegistry;
use crate::models::application::{LocaleDescriptor, ThemeDescriptor};

const STATE_CACHE_TTL: Duration = Duration::from_secs(60 * 3);
const STATE_MAX_CAPACITY: u64 = 100;

pub struct Preferences {
    pub theme: RwLock<ThemeDescriptor>,
    pub locale: RwLock<LocaleDescriptor>,
}

pub struct AppState {
    next_window_id: AtomicUsize,
    pub contributions: ContributionRegistry,
    pub cache: Arc<Cache<MokaBackend>>,
    pub preferences: Preferences,
}

impl AppState {
    pub fn new() -> Self {
        let cache = Cache::new(MokaBackend::new(STATE_MAX_CAPACITY, STATE_CACHE_TTL));

        Self {
            next_window_id: AtomicUsize::new(0),
            cache: Arc::new(cache),
            preferences: Preferences {
                theme: RwLock::new(ThemeDescriptor {
                    id: "theme-light".to_string(),
                    name: "Theme Light".to_string(),
                    source: "moss-light.css".to_string(),
                }),
                locale: RwLock::new(LocaleDescriptor {
                    code: "en".to_string(),
                    name: "English".to_string(),
                    direction: Some("ltr".to_string()),
                }),
            },
            contributions: ContributionRegistry::new()
                .init(crate::contribution::CONTRIBUTIONS.iter().map(|c| &**c)),
        }
    }

    pub fn inc_next_window_id(&self) -> usize {
        self.next_window_id
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    }

    pub fn get_command(&self, id: &ReadOnlyStr) -> Option<CommandHandler> {
        self.contributions
            .commands
            .get(id)
            .map(|cmd| Arc::clone(&cmd))
    }

    pub fn change_language_pack(&self, locale_descriptor: LocaleDescriptor) {
        let mut locale_lock = self.preferences.locale.write();
        *locale_lock = locale_descriptor;
    }

    pub fn change_color_theme(&self, theme_descriptor: ThemeDescriptor) {
        let mut theme_descriptor_lock = self.preferences.theme.write();
        *theme_descriptor_lock = theme_descriptor;
    }
}
