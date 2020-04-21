use std::str;
use std::vec::Vec;

use lazy_static::lazy_static;
use wasmer_runtime::{func, imports, wasm, Backend, Ctx};
use wasmer_runtime_core::backend::Compiler;

#[cfg(not(any(feature = "cranelift", feature = "singlepass")))]
const BACKEND: Option<Backend> = None;

#[cfg(feature = "cranelift")]
const BACKEND: Option<Backend> = Some(Backend::Cranelift);

#[cfg(feature = "singlepass")]
const BACKEND: Option<Backend> = Some(Backend::Singlepass);

#[cfg(not(feature = "no_macro"))]
#[macro_use]
extern crate afl;

#[cfg(not(feature = "no_macro"))]
fn main() {
    fuzz!(|data: &[u8]| {
        fuzz_target(data);
    });
}

#[cfg(feature = "no_macro")]
fn main() {
    use std::io::Read;
    let mut input = vec![];
    std::io::stdin().read_to_end(&mut input).unwrap();
    fuzz_target(&input);
}

fn fuzz_target(_data: &[u8]) {
    println!("{:?}", RES.as_ref().unwrap());
}

static WASM: &'static [u8] =
    include_bytes!("../corpus/fuzz_wasmer/6b376b755baa414dd46d430c54a31dfe3cfac615");

lazy_static! {
    static ref RES: Option<Vec<wasm::Value>> = call();
}

fn call() -> Option<Vec<wasm::Value>> {
    let import_object = imports! {
        "env" => {
            "print_str" => func!(print_str),
        },
    };

    let default_compiler = wasmer_runtime::default_compiler();

    let compiler: &dyn Compiler = match BACKEND {
        None => &default_compiler,
        Some(backend) => Box::leak(wasmer_runtime::compiler_for_backend(backend).unwrap()),
    };

    let module = wasmer_runtime::compile_with(&WASM, compiler).expect("module");

    let instance = module.instantiate(&import_object).expect("instance");

    instance.call("hello_wasm", &[]).ok()
}

fn print_str(ctx: &mut Ctx, ptr: u32, len: u32) {
    let memory = ctx.memory(0);

    let str_vec: Vec<_> = memory.view()[ptr as usize..(ptr + len) as usize]
        .iter()
        .map(|cell| cell.get())
        .collect();

    let string = str::from_utf8(&str_vec).unwrap();

    println!("{}", string);
}
