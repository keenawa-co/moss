pub mod contribution;
pub mod window;

use std::{
    borrow::BorrowMut,
    cell::{Cell, RefCell},
    path::PathBuf,
    rc::Rc,
    sync::Arc,
};

use anyhow::Result;
use contribution::WORKBENCH_TGUI_WINDOW;
use once_cell::unsync::OnceCell;
use platform_configuration::{
    attribute_name, configuration_policy::ConfigurationPolicyService,
    configuration_registry::ConfigurationRegistry, AbstractConfigurationService,
};
use platform_core::common::context::{
    entity::Model, model_context::ModelContext, subscription::Subscription, AnyContext,
    ContextInner,
};
use platform_formation::service_registry::ServiceRegistry;
use platform_fs::disk::file_system_service::{
    AbstractDiskFileSystemService, DiskFileSystemService,
};
use platform_user_profile::user_profile_service::UserProfileService as PlatformUserProfileService;
use platform_workspace::{Workspace, WorkspaceId};
use specta::Type;
use tauri::{AppHandle, Emitter, WebviewWindow};
use workbench_service_configuration_tgui::configuration_service::WorkspaceConfigurationService;
use workbench_service_environment_tgui::environment_service::NativeEnvironmentService;
use workbench_service_user_profile_tgui::user_profile_service::UserProfileService;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate anyhow;

#[macro_use]
extern crate lazy_static;

// TODO: this will be removed after testing is complete
struct MockFontSizeService {
    size: Cell<usize>,
}

impl MockFontSizeService {
    fn update_font_size(&self, value: usize) {
        self.size.replace(value);
    }
}

#[derive(Debug, Type, Serialize)]
pub enum WorkbenchState {
    Empty,
    Workspace,
}

pub struct Workbench<'a> {
    workspace_id: WorkspaceId,
    service_registry: Rc<RefCell<ServiceRegistry>>,
    configuration_registry: Model<ConfigurationRegistry<'a>>,
    // TODO: this will be removed after testing is complete
    font_size_service: Model<MockFontSizeService>,
    _observe_font_size_service: OnceCell<Subscription>,
    tao_handle: OnceCell<Rc<AppHandle>>,
}

unsafe impl<'a> Sync for Workbench<'a> {}
unsafe impl<'a> Send for Workbench<'a> {}

impl<'a> Workbench<'a> {
    pub fn new(
        ctx: &mut ContextInner,
        service_registry: ServiceRegistry,
        workspace_id: WorkspaceId,
    ) -> Result<Self> {
        let configuration_registry = ctx.new_model(|_| ConfigurationRegistry::new());

        let font_service_model = ctx.new_model(|_ctx| MockFontSizeService {
            size: Cell::new(10),
        });

        Ok(Self {
            workspace_id,
            service_registry: Rc::new(RefCell::new(service_registry)),
            configuration_registry,
            font_size_service: font_service_model,
            _observe_font_size_service: OnceCell::new(),
            tao_handle: OnceCell::new(),
        })
    }

    pub async fn initialize(&self, ctx: &mut ContextInner) -> Result<()> {
        ctx.update_model(&self.configuration_registry, |this, ctx| {
            // this.register_configuration(&WORKBENCH_TGUI_WINDOW);
        });

        self.initialize_services(ctx).await?;

        let service_registry = self.service_registry.as_ref().borrow();
        let config_service = service_registry.get_unchecked::<WorkspaceConfigurationService>();

        let value = config_service.get_value(attribute_name!(window.defaultWidth));
        println!("Value `window.defaultWidth` form None: {:?}", value);

        Ok(())
    }

    async fn initialize_services(&self, ctx: &mut ContextInner) -> Result<()> {
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

        let user_profile_service = UserProfileService::new(
            environment_service.user_home_dir().clone(),
            Arc::clone(&fs_service) as Arc<dyn AbstractDiskFileSystemService>,
        )
        .await?;

        let workspace_configuration_service = WorkspaceConfigurationService::new(
            workspace,
            &self.configuration_registry,
            configuration_policy_service,
            &user_profile_service.default_profile().settings_resource,
            Arc::clone(&fs_service) as Arc<dyn AbstractDiskFileSystemService>,
        )
        .await;

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

    pub fn set_tao_handle(&self, ctx: &mut ContextInner, handle: AppHandle) {
        let _ = self.tao_handle.set(Rc::new(handle));
        let tao_handle_clone = Rc::clone(self.tao_handle.get().unwrap());

        let _ = self._observe_font_size_service.set(ctx.observe(
            &self.font_size_service,
            move |this, cx| {
                let s = &this.read(cx).size;
                dbg!("AA");
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

impl<'a> Workbench<'a> {
    pub fn update_conf(&self, ctx: &mut ContextInner, value: usize) -> Result<()> {
        ctx.update_model(&self.font_size_service, |this, ctx| {
            this.update_font_size(value);
            dbg!("C");
            ctx.notify();
        });

        Ok(())
    }
}
