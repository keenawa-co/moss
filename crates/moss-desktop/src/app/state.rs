use moss_cache::{backend::moka::MokaBackend, Cache};
use moss_text::ReadOnlyStr;
use parking_lot::RwLock;
use std::time::Duration;
use std::{sync::atomic::AtomicUsize, sync::Arc};

use crate::command::CommandHandler;
use crate::contribution_collector::ContributionRegistry;
use crate::models::application::{LocaleDescriptor, ThemeDescriptor};

use super::service::ServiceManager;

const STATE_CACHE_TTL: Duration = Duration::from_secs(60 * 3);
const STATE_MAX_CAPACITY: u64 = 100;

pub struct Preferences {
    pub theme: RwLock<ThemeDescriptor>,
    pub locale: RwLock<LocaleDescriptor>,
}

#[repr(i8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LifecyclePhase {
    Starting = 0,
}

pub struct AppState {
    next_window_id: AtomicUsize,
    pub contributions: ContributionRegistry,
    pub cache: Arc<Cache<MokaBackend>>,
    pub preferences: Preferences,
    pub services: ServiceManager,
    // pub themes: RwLock<Vec<ThemeDescriptor>>,
    // pub commands: DashMap<ReadOnlyStr, CommandHandler>,
    // pub menus: DashMap<ReadOnlyStr, Vec<MenuItem>>,
    // pub tree_view_groups: DashMap<TreeViewGroupLocation, Vec<TreeViewGroup>>,
    // pub tree_views: DashMap<GroupId, Vec<TreeViewDescriptor>>,
}

impl AppState {
    pub fn new(services: ServiceManager) -> Self {
        // FIXME: This should be abstracted in the future.
        // let contribution_collector = ContributionCollector::new();
        // let contribution_collection = contribution_collector.collect();
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
            services,
            contributions: ContributionRegistry::new().init(),
            // themes: RwLock::new(Vec::new()),
            // commands: contribution_collection.commands,
            // menus: contribution_collection.menus,
            // tree_view_groups: contribution_collection.tree_view_groups,
            // tree_views: contribution_collection.tree_views,
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
