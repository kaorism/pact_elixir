#[macro_use] extern crate rustler;
#[macro_use] extern crate rustler_codegen;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate pact_mock_server;
extern crate libc;

use rustler::{NifEnv, NifTerm, NifResult, NifEncoder};
use pact_mock_server::create_mock_server;
use pact_mock_server::mock_server_mismatches_ffi;
use pact_mock_server::mock_server_matched_ffi;
use pact_mock_server::write_pact_file_ffi;
use pact_mock_server::cleanup_mock_server_ffi;
use std::ffi::CString;
use std::ffi::CStr;
mod atoms {
    rustler_atoms! {
        atom ok;
        atom error;
        //atom __true__ = "true";
        //atom __false__ = "false";
    }
}

rustler_export_nifs! {
    "Elixir.PactElixir.RustPactMockServerFacade",
    [
        ("create_mock_server", 2, create_mock_server_call),
        ("mock_server_mismatches", 1, mock_server_mismatches_call),
        ("mock_server_matched", 1, mock_server_matched_call),
        ("write_pact_file", 2, write_pact_file_call),
        ("cleanup_mock_server", 1, cleanup_mock_server_call)
    ],
    None
}

fn create_mock_server_call<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    let pact_json: &str = try!(args[0].decode());
    let port_arg: i32 = try!(args[1].decode());

    match create_mock_server(pact_json, port_arg) {
        Ok(port) => Ok((atoms::ok(), port).encode(env)),
        Err(err) => Ok((atoms::error(), 0).encode(env))
    }
}

fn mock_server_mismatches_call<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    let port: i32 = try!(args[0].decode());

    let c_buf: *mut i8 = mock_server_mismatches_ffi(port);
    let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
    let str_slice: &str = c_str.to_str().unwrap();
    let str_buf: String = str_slice.to_owned();  // if necessary

    Ok((atoms::ok(), str_buf).encode(env))
}

fn mock_server_matched_call<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    let port: i32 = try!(args[0].decode());

    let matched: bool = mock_server_matched_ffi(port);

    Ok((atoms::ok(), matched).encode(env))
}

fn write_pact_file_call<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    let port: i32 = try!(args[0].decode());
    let dir_path: String = try!(args[1].decode());

    let s = CString::new(dir_path).unwrap();

    let result = write_pact_file_ffi(port, s.as_ptr());

    Ok((atoms::ok(), result).encode(env))
}

fn cleanup_mock_server_call<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    let port: i32 = try!(args[0].decode());

    let success: bool = cleanup_mock_server_ffi(port);

    Ok((atoms::ok(), success).encode(env))
}

