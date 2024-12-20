use dashmap::DashMap;
use moss_cache::{backend::moka::MokaBackend, Cache};
use moss_text::ReadOnlyStr;
use parking_lot::RwLock;
use std::time::Duration;
use std::{sync::atomic::AtomicUsize, sync::Arc};

use crate::command::CommandHandler;
use crate::contribution_collector::ContributionCollector;
use crate::models::{
    actions::MenuItem, appearance::theming::ThemeDescriptor, view::*, window::LocaleDescriptor,
};

const STATE_CACHE_TTL: Duration = Duration::from_secs(60 * 5);
const STATE_MAX_CAPACITY: u64 = 100;

pub struct Preferences {
    pub theme: RwLock<ThemeDescriptor>,
    pub locale: RwLock<LocaleDescriptor>,
}

pub struct AppState {
    next_window_id: AtomicUsize,
    pub cache: Arc<Cache<MokaBackend>>,
    pub preferences: Preferences,
    pub commands: DashMap<ReadOnlyStr, CommandHandler>,
    pub menus: DashMap<ReadOnlyStr, Vec<MenuItem>>,
    pub tree_view_groups: DashMap<TreeViewGroupLocation, Vec<TreeViewGroup>>,
    pub tree_views: DashMap<GroupId, Vec<TreeViewDescriptor>>,
}

impl AppState {
    pub fn new() -> Self {
        // FIXME: This should be abstracted in the future.
        let contribution_collector = ContributionCollector::new();
        let contribution_collection = contribution_collector.collect();
        let cache = Cache::new(MokaBackend::new(STATE_MAX_CAPACITY, STATE_CACHE_TTL));

        let state = Self {
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
            commands: contribution_collection.commands,
            menus: contribution_collection.menus,
            tree_view_groups: contribution_collection.tree_view_groups,
            tree_views: contribution_collection.tree_views,
        };

        state
    }

    pub fn inc_next_window_id(&self) -> usize {
        self.next_window_id
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    }

    pub fn get_command(&self, id: &ReadOnlyStr) -> Option<CommandHandler> {
        self.commands.get(id).map(|cmd| Arc::clone(&cmd))
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
