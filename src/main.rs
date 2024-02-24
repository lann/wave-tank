use std::{fmt::Display, path::Path};

use anyhow::Context;
use wasm_wave::{
    untyped::UntypedFuncCall,
    wasm::{DisplayFuncArgs, DisplayFuncResults},
};
use wasmtime::{
    component::{Component, Func, Instance, Linker, ResourceTable, Val},
    Config, Engine, Store,
};
use wasmtime_wasi::preview2::{self, WasiCtx, WasiCtxBuilder, WasiView};

const USAGE: &str = "<COMPONENT> '<FUNC_NAME>([ARG]...)'";

fn main() -> anyhow::Result<()> {
    // Prepare args
    let mut args = std::env::args_os();
    let arg0 = args.next().unwrap().to_string_lossy().to_string();
    let usage = || format!("{arg0} {USAGE}");

    let wasm_path = args.next().with_context(usage)?;
    let call_expr_str = args
        .next()
        .with_context(usage)?
        .to_str()
        .context("func expr must be valid unicode")?
        .to_string();
    let func_call = UntypedFuncCall::parse(&call_expr_str)?;

    let mut instance = WasmInstance::new(wasm_path)?;
    let mut prepared_call = instance.prepare_call(&func_call)?;

    let results = prepared_call.call()?;
    println!("{prepared_call} -> {results}");

    Ok(())
}

struct WasmInstance {
    store: Store<Data>,
    instance: Instance,
}

impl WasmInstance {
    fn new(wasm_path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let engine = Engine::new(
            Config::new()
                .cache_config_load_default()?
                .wasm_component_model(true),
        )?;
        let component = Component::from_file(&engine, wasm_path)?;
        let mut linker: Linker<Data> = Linker::new(&engine);
        preview2::command::sync::add_to_linker(&mut linker)?;
        let table = Default::default();
        let ctx = WasiCtxBuilder::new().build();
        let data = Data { ctx, table };
        let mut store = Store::new(&engine, data);
        let instance = linker.instantiate(&mut store, &component)?;
        Ok(Self { store, instance })
    }

    fn prepare_call(&mut self, func_call: &UntypedFuncCall) -> anyhow::Result<PreparedCall> {
        let name = func_call.name().to_string();
        let func = self
            .instance
            .get_func(&mut self.store, &name)
            .with_context(|| format!("instance has no func export {name:?}"))?;
        let func_type = wasm_wave::wasmtime::get_func_type(&func, &self.store);
        let params = func_call.to_wasm_params(func_type.params.iter())?;
        Ok(PreparedCall {
            store: &mut self.store,
            func,
            name,
            params,
            results_len: func_type.results.len(),
        })
    }
}

struct PreparedCall<'a> {
    store: &'a mut Store<Data>,
    func: Func,
    name: String,
    params: Vec<Val>,
    results_len: usize,
}

impl<'a> PreparedCall<'a> {
    fn call(&mut self) -> anyhow::Result<WasmResults> {
        let mut results = vec![Val::U32(0xfefefefe); self.results_len];
        self.func
            .call(&mut self.store, &self.params, &mut results)?;
        Ok(WasmResults(results))
    }
}

impl<'a> Display for PreparedCall<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)?;
        DisplayFuncArgs(&self.params).fmt(f)
    }
}

struct WasmResults(Vec<Val>);

impl Display for WasmResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        DisplayFuncResults(&self.0).fmt(f)
    }
}

struct Data {
    ctx: WasiCtx,
    table: ResourceTable,
}

impl WasiView for Data {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}
