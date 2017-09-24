use super::*;

#[repr(C)]
pub struct StreamOutAscii(pub(crate) ffi::chrono::ChStreamOutAscii);

#[repr(C)]
pub struct Log(ffi::chrono::ChLog, NotSendSync);

impl Deref for Log {
    type Target = StreamOutAscii;

    fn deref(&self) -> &StreamOutAscii {
        let this_ = &self.0 as *const _;
        unsafe {
            cpp!([this_ as "chrono::ChLog*"] -> &StreamOutAscii as "chrono::ChStreamOutAscii*" {
                return this_;
            })
        }
    }
}

#[repr(C)]
pub struct StdoutLogger {
    base: Log
}

impl Deref for StdoutLogger {
    type Target = Log;

    fn deref(&self) -> &Log {
        &self.base
    }
}

cpp! {{
    #include "chrono/core/ChLog.h"

    class StdoutLogger;

    extern "C" {
        void stdout_logger_output(const char *data, size_t n);
    }

    class StdoutLogger : public chrono::ChLog {
    public:
        StdoutLogger() : ChLog() {}
    protected:
        virtual void Output(const char *data, size_t n) {
            stdout_logger_output(data, n);
        }
    };
}}


impl Drop for StdoutLogger {
    fn drop(&mut self) {
        let this_ = self as *mut _;
        unsafe {
            cpp!([this_ as "StdoutLogger*"] {
                this_->~StdoutLogger();
            })
        }
    }
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern fn stdout_logger_output(data: *const u8, n: usize) {
    let bytes = ::std::slice::from_raw_parts(data, n);
    if let Ok(s) = ::std::str::from_utf8(bytes) {
        print!("{}", s);
    } else {
        println!("PRINTING ERROR: INVALID UTF8");
    }
}

pub fn stdout_logger() -> StdoutLogger {
    StdoutLogger {
        base: unsafe {
            cpp! {[] -> Log as "StdoutLogger" {
                return StdoutLogger();
            }}
        }
    }
}
