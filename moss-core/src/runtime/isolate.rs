use deno_runtime::deno_core::{Extension, JsRuntime};
use deno_runtime::worker::MainWorker;
use deno_runtime::BootstrapOptions;
use deno_runtime::{
    deno_core::ModuleSpecifier,
    deno_core::RuntimeOptions,
    deno_web::BlobStore,
    permissions::{Permissions, PermissionsContainer},
    worker::WorkerOptions,
};
use std::fs::File;
use std::io::Read;
use std::path::Path;

// use swc::config::{JscConfig, Options};
// use swc::ecmascript::ast::EsVersion;
// use swc::Compiler;

pub struct Isolate;

impl Isolate {
    pub fn new() -> Self {
        Isolate
    }

    pub async fn run(&self, path_to_main_module: &Path) -> anyhow::Result<()> {
        let main_module = ModuleSpecifier::from_file_path(path_to_main_module).map_err(|_| {
            anyhow::anyhow!(
                "Failed to create module specifier from path: {:?}",
                path_to_main_module
            )
        })?;

        let worker_options = WorkerOptions {
            ..Default::default()
        };

        let mut main_worker = deno_runtime::worker::MainWorker::bootstrap_from_options(
            main_module.clone(),
            PermissionsContainer::new(Permissions::allow_all()),
            worker_options,
        );

        main_worker.execute_main_module(&main_module).await?;
        Ok(())
    }
}

// fn compile_typescript(input: &str) -> Result<String, Box<dyn std::error::Error>> {
//     let cm = swc::common::source_map::SourceMap::default();
//     let c = Compiler::new(&cm);

//     let fm = cm.new_source_file(
//         swc::common::FileName::Real("script.ts".into()),
//         input.to_string(),
//     );

//     let options = Options {
//         config: swc::config::Config {
//             jsc: JscConfig {
//                 target: Some(EsVersion::Es2020),
//                 syntax: Some(swc_ecma_parser::Syntax::Typescript(Default::default())),
//                 ..Default::default()
//             },
//             ..Default::default()
//         },
//         ..Default::default()
//     };

//     let result = c.process_js_file(fm, &options)?;
//     Ok(result.code)
// }

pub async fn create_worker(ts_script_path: &Path) -> anyhow::Result<()> {
    let worker_options = WorkerOptions {
        ..Default::default()
    };

    let r = deno_typescript::compile_bundle(ts_script_path, vec![], None).unwrap();
    println!("{r}");

    let main_module = ModuleSpecifier::from_file_path(ts_script_path).map_err(|_| {
        anyhow::anyhow!(
            "Failed to create module specifier from path: {:?}",
            ts_script_path
        )
    })?;

    let mut worker = MainWorker::bootstrap_from_options(
        main_module.clone(),
        PermissionsContainer::new(Permissions::allow_all()),
        worker_options,
    );

    worker.execute_main_module(&main_module).await?;
    worker.run_event_loop(false).await?;

    Ok(())
}

use std::str::from_utf8;
use wasmtime::*;

pub fn wasm() -> anyhow::Result<()> {
    let engine = Engine::default();
    let module = Module::from_file(&engine, "./testdata/hello-wasm/pkg/hello_wasm_bg.wasm")?;

    let mut linker = Linker::new(&engine);
    linker.func_wrap(
        "wasm_module_bg",
        "add",
        |caller: Caller<'_, u32>, param: i32| {
            println!("Got {} from WebAssembly", param);
            println!("my host state is: {}", caller.data());
        },
    )?;

    let mut store = Store::new(&engine, 4);
    let instance = linker.instantiate(&mut store, &module)?;

    let get_string = instance.get_typed_func::<(), i32>(&mut store, "get_string")?;
    let get_string_length = instance.get_typed_func::<(), i32>(&mut store, "get_string_length")?;

    let ptr = get_string.call(&mut store, ())?;
    let len = get_string_length.call(&mut store, ())? as usize;

    let memory = instance.get_memory(&mut store, "memory").unwrap();
    let data = {
        let data = memory.data(&store);
        from_utf8(&data[ptr as usize..ptr as usize + 10])?
    };

    println!("String from WASM: {}", data);

    Ok(())
}
