use core::ffi::{c_char, c_int};

use crate::base::{sqlite3, sqlite3_api_routines};

#[cfg(feature = "ext-spellfix")]
extern "C" {
    pub fn sqlite3_spellfix_init(
        db: *mut sqlite3,
        pzErrMsg: *mut *mut c_char, 
        pApi: *const sqlite3_api_routines,
    ) -> c_int;
}
