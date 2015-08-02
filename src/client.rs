extern crate libc;

use std::ptr;
use std::ffi::CString;
use ffi::{
    memcached,
    memcached_flush,
    memcached_last_error,
    memcached_return_t,
    memcached_st,
};
use error::{
    MemcacheError,
    MemcacheResult,
};

#[derive(Debug)]
pub struct Client {
    c_client: *const memcached_st,
}

impl Client {
    pub fn connect(host: &str, port: u16) -> MemcacheResult<Client> {
        let mut s = "--SERVER=".to_string();
        s.push_str(host);
        s.push(':');
        s.push_str(&port.to_string());
        let cstring = CString::new(s).unwrap();
        let s_len = cstring.to_bytes().len();
        unsafe {
            let c_client = memcached(cstring.as_ptr(), s_len as u64);
            if c_client.is_null() {
                let error_code = memcached_last_error(c_client);
                return Err(MemcacheError::new(error_code));
            }
            return Ok(Client{ c_client: c_client });
        }
    }

    pub fn flush(&self, expire: libc::time_t) -> MemcacheResult<()> {
        unsafe {
            let r = memcached_flush(self.c_client, expire);
            match r {
                memcached_return_t::MEMCACHED_SUCCESS => {
                    return Ok(());
                }
                _ => {
                    return Err(MemcacheError::new(r));
                }
            }
        }
    }
}
