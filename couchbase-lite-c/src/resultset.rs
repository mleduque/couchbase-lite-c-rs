use ffi;
use std::ffi::CStr;
use core::slice;
use std::str;

use crate::to_ptr;


pub struct ResultSet{
    pub rs: *mut ffi::CBLResultSet,
}

impl ResultSet{

    pub fn has_next(&self) -> bool{
        let next = unsafe{ ffi::CBLResultSet_Next(self.rs) };
        next
    }

    // return Doc
    pub fn value(&self, key: String) -> String{
        let slice = unsafe {
            //let value = ffi::CBLResultSet_ValueAtIndex(self.rs, 1);
            let value = ffi::CBLResultSet_ValueForKey(self.rs, to_ptr(key));
            let fl_type = ffi::FLValue_GetType(value);
            match fl_type  {
                6 => {
                    let fl_slice = ffi::FLValue_ToJSON(value); // TODO regarder FLValue_ToJSON5
                    CStr::from_bytes_with_nul_unchecked(slice::from_raw_parts(fl_slice.buf as *const u8, fl_slice.size+1)).to_bytes()
                },
                _ => {
                    let fl_slice = ffi::FLValue_AsString(value);
                    CStr::from_bytes_with_nul_unchecked(slice::from_raw_parts(fl_slice.buf as *const u8, fl_slice.size+1)).to_bytes()
                },
            }
        };
        str::from_utf8(slice).unwrap().to_string()
    }

}