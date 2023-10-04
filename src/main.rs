use std::{fmt::Display, path::Path};

use anyhow::{bail, Context};
use wasm_wave::{
    completion::params_completions,
    fmt::{DisplayFuncArgs, DisplayFuncResults},
    func::{CallExpr, WasmFunc},
    value::resolve_wit_func_type,
};
use wasmtime::{
    component::{Component, Func, Instance, Linker, Val},
    Config, Engine, Store,
};
use wasmtime_wasi::preview2::{self, Table, WasiCtx, WasiCtxBuilder, WasiView};
use wit_component::DecodedWasm;
use wit_parser::{Resolve, WorldId, WorldItem, WorldKey};

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

    if call_expr_str == "--complete" {
        let expr = args
            .next()
            .context("--complete <expr>")?
            .to_str()
            .context("invalid --complete <expr>")?
            .to_string();
        let call_expr_str = expr.trim_matches(|c| c == '\'' || c == '"');
        let completer = WasmCompleter::new(wasm_path)?;
        for comp in completer.complete(call_expr_str)? {
            println!(
                "{}",
                expr.replace(call_expr_str, &format!("{call_expr_str}{comp}"))
            );
        }
        std::process::exit(0);
    }

    let call_expr = CallExpr::parse(&call_expr_str)?;

    let mut instance = WasmInstance::new(wasm_path)?;
    let mut prepared_call = instance.prepare_call(&call_expr)?;

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
        let mut table = Default::default();
        let ctx = WasiCtxBuilder::new().build(&mut table)?;
        let data = Data { ctx, table };
        let mut store = Store::new(&engine, data);
        let instance = linker.instantiate(&mut store, &component)?;
        Ok(Self { store, instance })
    }

    fn prepare_call(&mut self, call_expr: &CallExpr) -> anyhow::Result<PreparedCall> {
        let name = call_expr.func_name.to_string();
        let func = self
            .instance
            .get_func(&mut self.store, &name)
            .with_context(|| format!("instance has no func export {name:?}"))?;
        let func_type = wasm_wave::wasmtime::get_func_type(&func, &self.store);
        let params = wasm_wave::parser::Parser::new(call_expr.args)
            .parse_params::<Val>(func_type.params.as_ref())?;
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
    table: Table,
}

impl WasiView for Data {
    fn table(&self) -> &Table {
        &self.table
    }

    fn table_mut(&mut self) -> &mut Table {
        &mut self.table
    }

    fn ctx(&self) -> &WasiCtx {
        &self.ctx
    }

    fn ctx_mut(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}

struct WasmCompleter {
    resolve: Resolve,
    world_id: WorldId,
}

impl WasmCompleter {
    fn new(wasm_path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let wasm = std::fs::read(wasm_path)?;
        let decoded = wit_component::decode(&wasm)?;
        let DecodedWasm::Component(resolve, world_id) = decoded else {
            bail!("expected a component binary; got a WIT package");
        };
        Ok(Self { resolve, world_id })
    }

    fn complete(&self, call_expr_str: &str) -> anyhow::Result<Vec<String>> {
        if call_expr_str.contains('(') {
            self.complete_args(call_expr_str)
        } else {
            self.complete_func_name(call_expr_str)
        }
    }

    fn complete_func_name(&self, partial_name: &str) -> anyhow::Result<Vec<String>> {
        let world = &self.resolve.worlds[self.world_id];
        let mut candidates: Vec<String> = world
            .exports
            .values()
            .filter_map(move |item| {
                if let WorldItem::Function(func) = item {
                    if let Some(comp) = func.name.strip_prefix(partial_name) {
                        if func.name == partial_name {
                            return Some(format!("{comp}("));
                        } else {
                            return Some(comp.to_string());
                        }
                    }
                }
                None
            })
            .collect();
        candidates.sort();
        Ok(candidates)
    }

    fn complete_args(&self, partial_call: &str) -> anyhow::Result<Vec<String>> {
        let paren_idx = partial_call.find('(').unwrap();
        let (func_name, partial_args) = partial_call.split_at(paren_idx);
        let name = func_name.to_string();
        let export = self.resolve.worlds[self.world_id]
            .exports
            .get(&WorldKey::Name(name))
            .with_context(|| format!("no export {func_name:?}"))?;
        let WorldItem::Function(func) = export else {
            bail!("{func_name:?} is not a func");
        };
        let func_type = resolve_wit_func_type(&self.resolve, func)?;
        let completions =
            params_completions(func_type.params(), partial_args).context("no completions")?;
        Ok(completions.candidates().map(|c| c.to_string()).collect())
    }
}
