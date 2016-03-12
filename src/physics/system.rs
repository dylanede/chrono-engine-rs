use ::ffi;

pub struct System {
    ptr: ffi::core::InternalPointer<System>
}

impl System {
    pub fn new(max_objects: u32, scene_size: f64, init_sys: bool) -> System {
        System {
            ptr: unsafe { ffi::physics::new_system(max_objects, scene_size, init_sys as u32) }
        }
    }
    pub fn set_step(&self, step: f64) {
        unsafe { ffi::physics::system_set_step(self.ptr, step) }
    }
    pub fn get_step(&self) -> f64 {
        unsafe { ffi::physics::system_get_step(self.ptr) }
    }
}

impl Drop for System {
    fn drop(&mut self) {
        unsafe {
            ffi::physics::delete_system(self.ptr);
        }
    }
}

#[test]
fn system_lifetime() {
    let _ = System::new(1000, 1000.0, true);
}
