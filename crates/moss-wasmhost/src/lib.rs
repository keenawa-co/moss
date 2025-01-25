use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::fs::create_dir;
use wasmtime::component::{bindgen, Component, Instance, Linker};
use wasmtime::{AsContextMut, Config, Engine, Store, StoreContextMut};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

// TODO: Better pathing
const ADDON_PATH: &str = "addons";

enum ArtifactError {
    NoArtifact,
    NoLockfile,
    MismatchingHash,
}

type AddonHash = String;
pub struct AddonContext {
    addon_hash: AddonHash,
    wasi_ctx: WasiCtx,
    wasi_table: ResourceTable,
}

impl WasiView for AddonContext {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.wasi_table
    }
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx
    }
}

pub struct AddonInstance {
    store: Store<AddonContext>,
    component: Component,
    instance: Instance,
}

pub struct WasmHost {
    hasher: Sha256,
    engine: Engine,
    linker: Linker<AddonContext>,
    addon_registry: HashMap<String, AddonInstance>,
}

impl WasmHost {
    pub fn new() -> Self {
        let mut config = Config::new();
        let engine = Engine::new(&config).unwrap();
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker_sync(&mut linker).unwrap();
        let mut host_functions = linker.instance("addon:demo/host-functions").unwrap();

        host_functions
            .func_wrap(
                "get-hash",
                move |store: StoreContextMut<'_, AddonContext>, _params: ()| {
                    Ok((store.data().addon_hash.clone(),))
                },
            )
            .unwrap();
        host_functions
            .func_wrap(
                "create-folder",
                move |store: StoreContextMut<'_, AddonContext>, params: (String,)| {
                    Ok(create_dir(format!("{}/{}", ADDON_PATH, params.0)).unwrap())
                },
            )
            .unwrap();
        Self {
            hasher: Sha256::new(),
            engine,
            linker,
            addon_registry: HashMap::new(),
        }
    }

    fn instantiate_component(&mut self, component: Component) -> AddonInstance {
        let hash = self.calculate_hash(&component);
        let mut store = Store::new(
            &self.engine,
            AddonContext {
                addon_hash: hash,
                wasi_ctx: WasiCtxBuilder::new().build(),
                wasi_table: ResourceTable::new(),
            },
        );
        let mut instance = self
            .linker
            .instantiate(store.as_context_mut(), &component)
            .unwrap();
        AddonInstance {
            store,
            component,
            instance,
        }
    }

    pub fn register_addon(&mut self, addon_name: &str, _permissions: Vec<String>) {
        let component = self.compile_addon(addon_name, _permissions);
        self.save_artifact(addon_name, &component);
        let addon_instance = self.instantiate_component(component);
        self.addon_registry
            .insert(addon_name.to_string(), addon_instance);
    }

    pub fn load_addon(&mut self, addon_name: &str) {
        // TODO: making it transparent to the user
        let mut component;
        match self.load_artifact(addon_name) {
            Ok(artifact_bytes) => unsafe {
                component = Component::deserialize(&self.engine, &artifact_bytes).unwrap();
            },

            Err(_) => {
                component = self.compile_addon(addon_name, vec![]);
                self.save_artifact(addon_name, &component);
            }
        }
        let addon_instance = self.instantiate_component(component);
        self.addon_registry
            .insert(addon_name.to_string(), addon_instance);
    }

    fn calculate_hash(&mut self, component: &Component) -> AddonHash {
        self.hasher.update(&component.serialize().unwrap());
        format!("{:X}", self.hasher.finalize_reset())
    }

    fn compile_addon(&mut self, addon_name: &str, _permissions: Vec<String>) -> Component {
        let wasm_bytes = fs::read(format!("{}/{}.wasm", ADDON_PATH, addon_name)).unwrap();
        Component::new(&self.engine, wasm_bytes).unwrap()
    }

    fn load_artifact(&mut self, addon_name: &str) -> Result<Vec<u8>, ArtifactError> {
        let artifact_bytes = fs::read(format!("{}/{}.component", ADDON_PATH, addon_name))
            .map_err(|_| ArtifactError::NoArtifact)?;
        self.hasher.update(&artifact_bytes);
        let artifact_hash = format!("{:X}", self.hasher.finalize_reset());
        if let Ok(stored_hash) = fs::read_to_string(format!("{}/{}.lock", ADDON_PATH, addon_name)) {
            if artifact_hash == stored_hash {
                Ok(artifact_bytes)
            } else {
                Err(ArtifactError::MismatchingHash)
            }
        } else {
            Err(ArtifactError::NoLockfile)
        }
    }

    fn save_artifact(&mut self, addon_name: &str, component: &Component) {
        let component_bytes = component.serialize().unwrap();
        fs::write(
            format!("{}/{}.component", ADDON_PATH, addon_name),
            &component_bytes,
        )
        .unwrap();
        self.hasher.update(&component_bytes);
        let addon_hash = format!("{:X}", self.hasher.finalize_reset());
        fs::write(format!("{}/{}.lock", ADDON_PATH, addon_name), addon_hash).unwrap();
    }

    fn execute_addon(&mut self, addon_name: &str, _params: Vec<String>) {
        let addon_instance = self.addon_registry.get_mut(addon_name).unwrap();
        let mut store = &mut addon_instance.store;
        let func = addon_instance
            .instance
            .get_typed_func::<(), ()>(&mut store, "execute")
            .unwrap();
        func.call(&mut store, ()).unwrap();
    }
}

mod test {

    use super::*;

    #[test]
    fn test_rust() {
        let mut host = WasmHost::new();
        host.register_addon("rust_demo", vec![]);
        host.execute_addon("rust_demo", vec![]);
    }

    #[test]
    fn test_js() {
        let mut host = WasmHost::new();
        host.register_addon("js_demo", vec![]);
        host.execute_addon("js_demo", vec![]);
    }
}
// policy
// TODO: Data passing (working example)
// addon access to certain app_handler api

// TODO: display plugin compilation status
// plugin page: description
// logging: failed to start, checksum error etc
// List of all plugins
// Addon/Addon descriptor registry
// Combine addons and configuration files
