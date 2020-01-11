use std::ffi::CString;
use std::path::PathBuf;

use fst_sys::{
    fstReaderClose, fstReaderGetEndTime, fstReaderGetMaxHandle, fstReaderGetStartTime,
    fstReaderGetVarCount, fstReaderOpen,
};

fn fst_asset(rel_path: &str) -> PathBuf {
    let mut path = PathBuf::from(file!());
    path.pop();
    path.pop();
    path.push("assets/fst");
    path.push(rel_path);
    path
}

#[test]
pub fn basic_test() {
    let fst_path = fst_asset("des.fst");
    unsafe {
        let handle = fstReaderOpen(CString::new(fst_path.to_str().unwrap()).unwrap().as_ptr());
        assert_eq!(fstReaderGetMaxHandle(handle), 1287);
        assert_eq!(fstReaderGetVarCount(handle), 1432);
        assert_eq!(fstReaderGetStartTime(handle), 0);
        assert_eq!(fstReaderGetEndTime(handle), 704);
        fstReaderClose(handle);
    }
}
