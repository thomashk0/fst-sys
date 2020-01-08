#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::os::raw::{c_void, c_char, c_int};
use std::ffi::CString;

// The raw FFI are extracted for a call to:
// $ bindgen fstapi/fstapi.h  --whitelist-function "fstReader.*" --whitelist-type "fst.*" --ctypes-prefix "" -o src/bindings.rs

const FST_FT_VERILOG: i32 = 0;
const FST_FT_VHDL: i32 = 1;
const FST_FT_VERILOG_VHDL: i32 = 2;

extern "C" {
    pub fn fstReaderOpen(name: *const c_char) -> *mut c_void;
    pub fn fstReaderClose(ctx: *mut c_void);
    pub fn fstReaderGetFileType(ctx: *mut c_void) -> c_int;
    pub fn fstReaderGetEndTime(ctx: *mut c_void) -> u64;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FstError {
    InvalidFile,
    InvalidConversion
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FstFileType {
    Verilog,
    Vhdl,
    VerilogVhdl
}

#[derive(Debug)]
pub struct FstReader {
    handle: *mut c_void
}

impl FstReader {
    pub fn from_file(name: &str) -> Result<FstReader, FstError> {
        let p = unsafe { fstReaderOpen(CString::new(name).unwrap().as_ptr()) };
        if p.is_null() {
            return Err(FstError::InvalidFile);
        }
        Ok(FstReader { handle: p })
    }

    pub fn file_type(&self) -> Result<FstFileType, FstError> {
        let w = unsafe { fstReaderGetFileType(self.handle) };
        match w {
            FST_FT_VERILOG => Ok(FstFileType::Verilog),
            FST_FT_VHDL => Ok(FstFileType::Vhdl),
            FST_FT_VERILOG_VHDL => Ok(FstFileType::VerilogVhdl),
            _ => Err(FstError::InvalidConversion)
        }
    }

    pub fn end_time(&self) -> u64 {
        unsafe { fstReaderGetEndTime(self.handle) }
    }
}

impl Drop for FstReader {
    fn drop(&mut self) {
        if self.handle.is_null() {
            return;
        }
        unsafe {
            fstReaderClose(self.handle);
        }
    }
}
