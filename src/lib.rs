#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod raw;

use std::os::raw::{c_void, c_uchar, c_char};
use std::ffi::{CString, CStr};
use std::ptr::null_mut;


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FstError {
    InvalidFile,
    InvalidConversion,
    Utf8Error,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FstFileType {
    Verilog,
    Vhdl,
    VerilogVhdl,
}

#[derive(Debug)]
pub struct FstReader {
    handle: *mut c_void
}


extern "C" fn dump_command(_ctx: *mut c_void, time: u64, id: raw::fstHandle, value: *const c_uchar) {
    let value_str = unsafe { CStr::from_ptr(value as *const c_char).to_str().unwrap() };
    println!("BLOCK: #{} id={} ({})", time, id, value_str);
}

impl FstReader {
    pub fn from_file(name: &str, use_extensions: bool) -> Result<FstReader, FstError> {
        let p = unsafe { raw::fstReaderOpen(CString::new(name).unwrap().as_ptr()) };
        if p.is_null() {
            return Err(FstError::InvalidFile);
        }
        if use_extensions {
            unsafe {
                raw::fstReaderSetVcdExtensions(p, 1);
            }
        }
        Ok(FstReader { handle: p })
    }

    pub fn file_type(&self) -> Result<FstFileType, FstError> {
        let w = unsafe { raw::fstReaderGetFileType(self.handle) } as u32;
        match w {
            raw::fstFileType_FST_FT_VERILOG => Ok(FstFileType::Verilog),
            raw::fstFileType_FST_FT_VHDL => Ok(FstFileType::Vhdl),
            raw::fstFileType_FST_FT_VERILOG_VHDL => Ok(FstFileType::VerilogVhdl),
            _ => Err(FstError::InvalidConversion)
        }
    }

    pub fn iter_block(&self) -> i32 {
        unsafe {
            raw::fstReaderSetFacProcessMaskAll(self.handle);
            raw::fstReaderIterBlocks(self.handle, Some(dump_command), self.handle, null_mut())
        }
    }

    pub fn end_time(&self) -> u64 {
        unsafe { raw::fstReaderGetEndTime(self.handle) }
    }

    pub fn var_count(&self) -> u64 {
        unsafe { raw::fstReaderGetVarCount(self.handle) }
    }
}

impl Drop for FstReader {
    fn drop(&mut self) {
        if self.handle.is_null() {
            return;
        }
        unsafe {
            raw::fstReaderClose(self.handle);
        }
    }
}
