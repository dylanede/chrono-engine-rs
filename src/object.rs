use super::*;
cpp! {{
    #include "chrono/physics/ChObject.h"
}}

#[repr(C)]
pub struct Object(ffi::chrono::ChObj, NotSendSync);

impl CppClass for Object {}

impl Object {
    pub fn set_name<T>(&self, name: T) where T: Into<String> {
        let this_ = &self.0 as *const _;
        let name_cstr = std::ffi::CString::new(name.into()).unwrap();
        let name_ptr = name_cstr.as_ptr();
        unsafe {
            cpp!([this_ as "chrono::ChObj*", name_ptr as "const char*"] {
            this_->SetName(name_ptr);
        })
        }
    }
}
