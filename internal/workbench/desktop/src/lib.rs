pub mod contribution;
pub mod menu;
pub mod parts;
pub mod view;
pub mod window;

pub mod contributions;

use std::{
    any::Any,
    cell::{Cell, RefCell},
    path::PathBuf,
    rc::Rc,
    sync::Arc,
};

use anyhow::Result;
use contribution::WORKBENCH_TAO_WINDOW;
use contributions::{
    links::LinksContribution, resents::RecentsContribution,
    tree_view_groups::launchpad::LaunchpadGroupContribution,
};
use hashbrown::HashMap;
use hecs::Entity;
use menu::{MenuId, MenuItem, MenuRegistry};
use once_cell::unsync::OnceCell;
use parts::{
    primary_activitybar::PrimaryActivityBarPart, primary_sidebar::PrimarySideBarPart, AnyPart,
    PartId,
};
use platform_configuration::{
    attribute_name, configuration_policy::ConfigurationPolicyService,
    configuration_registry::ConfigurationRegistry, AbstractConfigurationService,
};
use platform_core::context_v2::{
    async_context::AsyncContext, atom::Atom, node::AnyNodeValue, subscription::Subscription,
    AnyContext, Context,
};
use platform_formation::service_registry::ServiceRegistry;
use platform_fs::disk::file_system_service::{
    AbstractDiskFileSystemService, DiskFileSystemService,
};
use platform_user_profile::user_profile_service::UserProfileService as PlatformUserProfileService;
use platform_workspace::{Workspace, WorkspaceId};
use tauri::{AppHandle, Emitter, WebviewWindow};
use view::{GroupId, ViewsRegistry};
use workbench_service_configuration_tao::configuration_service::WorkspaceConfigurationService;
use workbench_service_environment_tao::environment_service::NativeEnvironmentService;
use workbench_service_user_profile_tao::user_profile_service::UserProfileService;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate anyhow;

#[macro_use]
extern crate lazy_static;

// TODO: this will be removed after testing is
#[derive(Clone)]
struct MockFontSizeService {
    size: Cell<usize>,
}

impl AnyNodeValue for MockFontSizeService {
    fn as_any_ref(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl MockFontSizeService {
    fn update_font_size(&self, value: usize) {
        self.size.replace(value);
    }
}

#[derive(Debug, Serialize)]
pub enum WorkbenchState {
    Empty,
    Workspace,
}

pub trait Contribution {
    fn contribute(registry: &mut RegistryManager) -> Result<()>;
}

pub struct RegistryManager {
    pub views: ViewsRegistry,
    pub menus: MenuRegistry,
}

impl RegistryManager {
    pub fn new() -> Self {
        Self {
            views: ViewsRegistry::new(),
            menus: MenuRegistry::new(),
        }
    }
}

pub struct Workbench {
    workspace_id: WorkspaceId,
    registry: RegistryManager,
    service_registry: Rc<RefCell<ServiceRegistry>>,
    configuration_registry: Atom<ConfigurationRegistry>,
    // TODO: this will be removed after testing is complete
    font_size_service: Atom<MockFontSizeService>,
    _observe_font_size_service: OnceCell<Subscription>,
    tao_handle: OnceCell<Rc<AppHandle>>,

    parts: HashMap<PartId, Box<dyn Any>>,
}

unsafe impl<'a> Sync for Workbench {}
unsafe impl<'a> Send for Workbench {}

impl Workbench {
    pub fn new(
        ctx: &mut AsyncContext,
        service_registry: ServiceRegistry,
        workspace_id: WorkspaceId,
    ) -> Result<Self> {
        let configuration_registry = ctx.apply(move |tx_ctx| {
            let configuration_registry = tx_ctx.create_atom(|_| ConfigurationRegistry::new());

            tx_ctx.update_atom(&configuration_registry, |this, ctx| {
                this.register_configuration(&WORKBENCH_TAO_WINDOW);

                ctx.notify();
            });

            configuration_registry
        })?;

        let font_service_atom = ctx.apply(|tx_ctx| {
            let font_service_atom = tx_ctx.create_atom(|_ctx| MockFontSizeService {
                size: Cell::new(10),
            });

            font_service_atom
        })?;

        Ok(Self {
            workspace_id,
            registry: RegistryManager::new(),
            service_registry: Rc::new(RefCell::new(service_registry)),
            configuration_registry,
            font_size_service: font_service_atom,
            _observe_font_size_service: OnceCell::new(),
            tao_handle: OnceCell::new(),
            parts: HashMap::new(),
        })
    }

    pub fn registry(&self) -> &RegistryManager {
        &self.registry
    }

    pub fn add_part<T: AnyPart + 'static>(&mut self, part: T) {
        self.parts.insert(part.id(), Box::new(part));
    }

    pub fn add_contribution(
        &mut self,
        f: impl FnOnce(&mut crate::RegistryManager) -> anyhow::Result<()>,
    ) -> Result<()> {
        f(&mut self.registry)
    }

    pub fn get_part<T: AnyPart + 'static>(&self, part_id: PartId) -> Option<&T> {
        self.parts.get(part_id)?.downcast_ref::<T>()
    }

    pub fn get_view<T: 'static>(&self, group_id: GroupId, view_id: String) -> Option<&T> {
        self.registry.views.get_view_model(group_id, view_id)
    }

    pub fn get_menu_items(&self, menu_id: &MenuId) -> Option<&Vec<MenuItem>> {
        self.registry.menus.get_menu_items(menu_id)
    }

    pub fn initialize<'a>(&'a mut self, ctx: &mut AsyncContext) -> Result<()> {
        self.add_contribution(LaunchpadGroupContribution::contribute)?;

        self.add_contribution(RecentsContribution::contribute)?;
        self.add_contribution(LinksContribution::contribute)?;

        self.add_part(PrimaryActivityBarPart::new());
        self.add_part(PrimarySideBarPart::new());

        ctx.apply(|cx| self.initialize_services(cx))??;

        let service_registry = self.service_registry.as_ref().borrow();
        let config_service = service_registry.get_unchecked::<WorkspaceConfigurationService>();

        let value = config_service.get_value(attribute_name!(window.defaultWidth));
        println!("Value `window.defaultWidth` form None: {:?}", value);

        Ok(())
    }

    fn initialize_services(&self, ctx: &mut Context) -> Result<()> {
        let workspace = self.restore_workspace();

        let configuration_policy_service = ConfigurationPolicyService {
            definitions: {
                use platform_configuration::policy::PolicyDefinitionType;

                let mut this = hashbrown::HashMap::new();

                this.insert(
                    "editorLineHeightPolicy".to_string(),
                    PolicyDefinitionType::Number,
                );

                this
            },
            policies: {
                let mut this = hashbrown::HashMap::new();
                this.insert(
                    "editorLineHeightPolicy".to_string(),
                    serde_json::Value::Number(serde_json::Number::from(45)),
                );

                this
            },
        };

        let mut service_registry = self.service_registry.as_ref().borrow_mut();

        let fs_service = service_registry.get_unchecked::<Arc<DiskFileSystemService>>();
        let environment_service = service_registry.get_unchecked::<NativeEnvironmentService>();

        let user_profile_service = ctx.block_on_with({
            UserProfileService::new(
                environment_service.user_home_dir().clone(),
                Arc::clone(&fs_service) as Arc<dyn AbstractDiskFileSystemService>,
            )
        })?;

        // let user_profile_service = UserProfileService::new(
        //     environment_service.user_home_dir().clone(),
        //     Arc::clone(&fs_service) as Arc<dyn AbstractDiskFileSystemService>,
        // )
        // .await?;

        let workspace_configuration_service = WorkspaceConfigurationService::new(
            ctx,
            workspace,
            self.configuration_registry.clone(),
            configuration_policy_service,
            user_profile_service
                .default_profile()
                .settings_resource
                .clone(),
            Arc::clone(&fs_service) as Arc<dyn AbstractDiskFileSystemService>,
        );

        service_registry.insert(workspace_configuration_service);

        Ok(())
    }

    fn restore_workspace(&self) -> Workspace {
        match &self.workspace_id {
            WorkspaceId::Empty => Workspace {
                id: WorkspaceId::Empty,
                folders: vec![],
                configuration_uri: None,
            },
            WorkspaceId::Some(_id) => {
                struct SimpleWorkspaceData {
                    path: PathBuf,
                }

                // TODO: This data should be obtained from the storage service
                // and represent the project from the previous session.
                let mock_workspace_data = SimpleWorkspaceData {
                    path: PathBuf::from(format!(".moss/settings.json")),
                };

                Workspace {
                    id: self.workspace_id.clone(),
                    folders: vec![],
                    configuration_uri: Some(mock_workspace_data.path),
                }
            }
        }
    }

    pub fn set_tao_handle(&self, ctx: &mut Context, handle: AppHandle) {
        let _ = self.tao_handle.set(Rc::new(handle));
        let tao_handle_clone = Rc::clone(self.tao_handle.get().unwrap());

        let _ = self._observe_font_size_service.set(ctx.observe(
            &self.font_size_service,
            move |this, cx| {
                let s = &this.read(cx).size;
                tao_handle_clone
                    .emit("font-size-update-event", s.get())
                    .unwrap();
            },
        ));
    }

    pub fn set_configuration_window_size(&self, window: WebviewWindow) -> Result<()> {
        use tauri::{LogicalSize, Size::Logical};

        let service_registry = self.service_registry.as_ref().borrow();
        let config_service = service_registry.get_unchecked::<WorkspaceConfigurationService>();

        let width_value = config_service
            .get_value(attribute_name!(window.defaultWidth))
            .expect(
                "The default window width size must be set in the workbench configuration schema",
            )
            .as_i64()
            .expect("The default window width size must be a number");

        let height_value = config_service
            .get_value(attribute_name!(window.defaultHeight))
            .expect(
                "The default window height size must be set in the workbench configuration schema",
            )
            .as_i64()
            .expect("The default window height size must be a number");

        window
            .set_size(Logical(LogicalSize {
                width: width_value as f64,
                height: height_value as f64,
            }))
            .unwrap();
        Ok(())
    }

    pub fn get_state(&self) -> WorkbenchState {
        WorkbenchState::Empty
    }
}

impl<'a> Workbench {
    pub fn update_conf(&self, ctx: &AsyncContext, value: usize) -> Result<()> {
        ctx.apply(|tx_ctx| {
            tx_ctx.update_atom(&self.font_size_service, |this, ctx| {
                this.update_font_size(value);
                ctx.notify();
            })
        })
    }
}
