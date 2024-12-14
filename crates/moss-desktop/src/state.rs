use anyhow::Result;
use dashmap::DashMap;
use hashbrown::HashMap;
use moss_text::ReadOnlyStr;
use parking_lot::RwLock;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::fmt::Debug;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;
use tauri::{AppHandle, Emitter, EventTarget, Manager, Window};

use crate::contributions::Contribution;
use crate::models::view::*;
use crate::models::window::LocaleDescriptor;
use crate::models::{actions::MenuItem, appearance::theming::ThemeDescriptor};

// NOTE: Temporary solution. Will be moved to crates/moss-desktop.

pub struct Appearance {
    pub theme: RwLock<ThemeDescriptor>,
}

pub struct CommandContext {
    pub app_handle: AppHandle,
    pub window: Window,
    args: HashMap<String, Value>,
}

impl CommandContext {
    pub fn new(app_handle: AppHandle, window: Window, args: HashMap<String, Value>) -> Self {
        Self {
            app_handle,
            window,
            args,
        }
    }

    pub fn get_arg<T>(&self, key: &str) -> Result<T, String>
    where
        T: DeserializeOwned,
    {
        let value = self
            .args
            .get(key)
            .ok_or_else(|| format!("Argument '{}' is not found", key))?;
        serde_json::from_value(value.clone())
            .map_err(|e| format!("Deserialization error for key'{}': {}", key, e))
    }
}

pub type CommandHandler =
    Arc<dyn Fn(CommandContext, &AppState) -> Result<Value, String> + Send + Sync>;

#[derive(Debug)]
pub struct ViewsRegistry {
    groups: HashMap<TreeViewGroupLocation, Vec<TreeViewGroup>>,
    views: HashMap<GroupId, Vec<TreeViewDescriptor>>,
}

impl ViewsRegistry {
    pub fn new() -> Self {
        ViewsRegistry {
            groups: HashMap::new(),
            views: HashMap::new(),
        }
    }

    pub(crate) fn append_view_group(
        &mut self,
        location: TreeViewGroupLocation,
        group: TreeViewGroup,
    ) {
        self.groups
            .entry(location)
            .or_insert_with(Vec::new)
            .push(group);
    }

    pub(crate) fn register_views(
        &mut self,
        id: ReadOnlyStr,
        batch: impl IntoIterator<Item = TreeViewDescriptor>,
    ) {
        self.views.entry(id).or_insert_with(Vec::new).extend(batch);
    }

    pub(crate) fn get_view_descriptors_by_group_id(
        &self,
        id: &ReadOnlyStr,
    ) -> Option<&Vec<TreeViewDescriptor>> {
        self.views.get(id)
    }

    pub fn get_view_model<T: Send + Sync + Debug + 'static>(
        &self,
        group_id: impl Into<ReadOnlyStr>,
        view_id: String,
    ) -> Option<Arc<T>> {
        self.views
            .get(&group_id.into())?
            .iter()
            .find(|item| item.id == view_id)
            .and_then(|item| Arc::downcast::<T>(Arc::clone(&item.model)).ok())
    }

    pub(crate) fn get_groups_by_location(
        &self,
        location: &TreeViewGroupLocation,
    ) -> Option<&Vec<TreeViewGroup>> {
        self.groups.get(location)
    }
}

pub struct MenuRegistry {
    menus: HashMap<ReadOnlyStr, Vec<MenuItem>>,
}

impl MenuRegistry {
    pub fn new() -> Self {
        Self {
            menus: HashMap::new(),
        }
    }

    pub fn append_menu_item(&mut self, menu_id: ReadOnlyStr, item: MenuItem) {
        self.menus
            .entry(menu_id.into())
            .or_insert_with(Vec::new)
            .push(item);
    }

    pub fn append_menu_items<I>(&mut self, items: I)
    where
        I: IntoIterator<Item = (ReadOnlyStr, MenuItem)>,
    {
        for (menu_id, item) in items {
            self.append_menu_item(menu_id, item);
        }
    }

    pub fn get_menu_items_by_namespace(&self, namespace: &ReadOnlyStr) -> Option<&Vec<MenuItem>> {
        self.menus.get(namespace)
    }
}

pub struct AppState {
    pub next_window_id: AtomicUsize,
    pub appearance: Appearance,
    pub locale: RwLock<LocaleDescriptor>,
    pub commands: DashMap<ReadOnlyStr, CommandHandler>,
    pub views: Arc<RwLock<ViewsRegistry>>,
    pub menus: Arc<RwLock<MenuRegistry>>,
}

impl AppState {
    pub fn new() -> Self {
        let commands = DashMap::new();
        commands.insert(
            "workbench.changeColorTheme".into(),
            Arc::new(change_color_theme) as CommandHandler,
        );

        commands.insert(
            "workbench.changeLanguagePack".into(),
            Arc::new(change_language_pack) as CommandHandler,
        );

        let mut state = Self {
            next_window_id: AtomicUsize::new(0),
            appearance: Appearance {
                theme: RwLock::new(ThemeDescriptor {
                    id: "theme-light".to_string(),
                    name: "Theme Light".to_string(),
                    source: "moss-light.css".to_string(),
                }),
            },
            locale: RwLock::new(LocaleDescriptor {
                code: "en".to_string(),
                name: "English".to_string(),
                direction: Some("ltr".to_string()),
            }),
            commands,
            views: Arc::new(RwLock::new(ViewsRegistry::new())),
            menus: Arc::new(RwLock::new(MenuRegistry::new())),
        };

        crate::contributions::workbench::WorkbenchContribution::contribute(&mut state).unwrap();
        crate::contributions::resents::RecentsContribution::contribute(&mut state).unwrap();
        crate::contributions::links::LinksContribution::contribute(&mut state).unwrap();
        crate::contributions::layout_controls::LayoutControlsContribution::contribute(&mut state)
            .unwrap();

        state
    }

    pub fn get_command(&self, id: &ReadOnlyStr) -> Option<CommandHandler> {
        self.commands.get(id).map(|cmd| Arc::clone(&cmd))
    }

    pub fn change_language_pack(&self, locale_descriptor: LocaleDescriptor) {
        let mut locale_lock = self.locale.write();
        *locale_lock = locale_descriptor;
    }

    pub fn change_color_theme(&self, new_theme_descriptor: ThemeDescriptor) {
        let mut theme_descriptor_lock = self.appearance.theme.write();
        *theme_descriptor_lock = new_theme_descriptor;
    }
}

// FIXME: Temporary placement of this function here. It will be moved later.
pub fn change_color_theme(ctx: CommandContext, app_state: &AppState) -> Result<Value, String> {
    let theme_descriptor_arg = ctx.get_arg::<ThemeDescriptor>("themeDescriptor")?;

    app_state.change_color_theme(theme_descriptor_arg.clone());

    for (label, _) in ctx.app_handle.webview_windows() {
        if ctx.window.label() == &label {
            continue;
        }

        ctx.app_handle
            .emit_to(
                EventTarget::webview_window(label),
                "core://color-theme-changed",
                theme_descriptor_arg.clone(),
            )
            .unwrap();
    }

    Ok(Value::Null)
}

// FIXME: Temporary placement of this function here. It will be moved later.
pub fn change_language_pack(ctx: CommandContext, app_state: &AppState) -> Result<Value, String> {
    let locale_descriptor_arg = ctx.get_arg::<LocaleDescriptor>("localeDescriptor")?;

    app_state.change_language_pack(locale_descriptor_arg.clone());

    for (label, _) in ctx.app_handle.webview_windows() {
        if ctx.window.label() == &label {
            continue;
        }

        ctx.app_handle
            .emit_to(
                EventTarget::webview_window(label),
                "core://language-pack-changed",
                locale_descriptor_arg.clone(),
            )
            .unwrap();
    }

    Ok(Value::Null)
}
