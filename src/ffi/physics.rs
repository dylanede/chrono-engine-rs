use libc;

use ::physics::System;
use ::physics::Body;
use ::core::SharedData;
use ::ffi::core::InternalPointer;

extern {
    pub fn new_system(max_objects: libc::c_uint,
                      scene_size: libc::c_double,
                      init_sys: libc::c_uint) -> InternalPointer<System>;
    pub fn delete_system(system: InternalPointer<System>);
    pub fn system_set_step(this: InternalPointer<System>, step: libc::c_double);
    pub fn system_get_step(this: InternalPointer<System>) -> libc::c_double;
}

#[test]
fn system_lifetime() {
    unsafe {
        delete_system(new_system(0, 0.0, 1))
    }
}

extern {
    pub fn delete_body(this: InternalPointer<Body>);
    pub fn make_shared_body() -> SharedData<Body>;
    pub fn body_into_shared(this: InternalPointer<Body>) -> SharedData<Body>;
    pub fn shared_body_clone(this: *const SharedData<Body>) -> SharedData<Body>;
    pub fn drop_shared_body(this: *mut SharedData<Body>);
}
#[cfg(test)]
extern {
    pub fn shared_body_deref(this: *const SharedData<Body>) -> InternalPointer<Body>;
}
