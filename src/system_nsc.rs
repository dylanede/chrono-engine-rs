use super::*;

cpp! {{
    #include "chrono/physics/ChSystemNSC.h"
}}

#[repr(C)]
pub struct SystemNSC(ffi::chrono::ChSystemNSC, NotSendSync);

impl SystemNSC {
    pub fn new(max_objects: u32, scene_size: f64, init_sys: bool) -> Mnd<SystemNSC> {
        let mut boxed = Box::new(SystemNSC(unsafe { mem::zeroed::<ffi::chrono::ChSystemNSC>() }, NOT_SEND_SYNC));
        {
            let inner: &mut ffi::chrono::ChSystemNSC = &mut boxed.0;
            let init_sys = init_sys as u8;
            unsafe {
                cpp!([inner as "chrono::ChSystemNSC*", max_objects as "uint32_t", scene_size as "double", init_sys as "uint8_t"] {
                new(inner) chrono::ChSystemNSC(max_objects, scene_size, init_sys != 0);
            })
            }
        }
        Mnd(boxed)
    }
    pub fn add_body(&self, body: Shared<Body>) {
        let ptr = unsafe { ptr::read(&body.inner as *const _) };
        mem::forget(body);
        let this_ = &self.0 as *const _;
        unsafe {
            cpp!([this_ as "chrono::ChSystemNSC*", ptr as "std::shared_ptr<chrono::ChBody>"] {
                this_->AddBody(ptr);
            })
        }
    }
    pub fn remove_body(&self, body: Shared<Body>) {
        let ptr = unsafe { ptr::read(&body.inner as *const _) };
        mem::forget(body);
        let this_ = &self.0 as *const _;
        unsafe {
            cpp!([this_ as "chrono::ChSystemNSC*", ptr as "std::shared_ptr<chrono::ChBody>"] {
                this_->RemoveBody(ptr);
            })
        }
    }
    pub fn add_link<T>(&self, link: Shared<T>) where T: Shareable + Inherits<Link> {
        let link = link.upcast::<Link>();
        let ptr = unsafe { ptr::read(&link.inner as *const _) };
        mem::forget(link);
        let this_ = &self.0 as *const _;
        unsafe {
            cpp!([this_ as "chrono::ChSystemNSC*", ptr as "std::shared_ptr<chrono::ChLink>"] {
                this_->AddLink(ptr);
            })
        }
    }
    pub fn show_hierarchy(&self, logger: &StreamOutAscii) {
        let ptr = &logger.0 as *const _;
        let this_ = &self.0 as *const _;
        unsafe {
            cpp!([this_ as "chrono::ChSystemNSC*", ptr as "chrono::ChStreamOutAscii*"] {
                this_->ShowHierarchy(*ptr);
            })
        }
    }
    pub fn do_frame_dynamics(&self, end_time: f64) -> bool {
        let this_ = &self.0 as *const _;
        unsafe {
            cpp!([this_ as "chrono::ChSystemNSC*", end_time as "double"] -> u8 as "uint8_t" {
                try {
                    return this_->DoFrameDynamics(end_time) ? 1 : 0;
                } catch(...) {
                    std::cout << "Shit" << std::endl;
                    return 1;
                }
            }) != 0
        }
    }
    pub fn set_g_acc(&self, acc: &Vector3<f64>) {
        let this_ = self as *const _;
        unsafe {
            cpp!([this_ as "chrono::ChSystemNSC*", acc as "const chrono::ChVector<double>*"] {
                this_->Set_G_acc(*acc);
            })
        }
    }
}

impl Clone for SystemNSC {
    fn clone(&self) -> Self {
        let mut inner = unsafe { mem::zeroed::<ffi::chrono::ChSystemNSC>() };
        let other = &self.0 as *const _;
        unsafe {
            cpp!([mut inner as "chrono::ChSystemNSC", other as "const chrono::ChSystemNSC*"] {
                new(&inner) chrono::ChSystemNSC(*other);
            })
        }
        SystemNSC(inner, NOT_SEND_SYNC)
    }
}

impl Drop for SystemNSC {
    fn drop(&mut self) {
        let inner = &mut self.0 as *mut _;
        unsafe {
            cpp!([inner as "chrono::ChSystemNSC*"] {
                inner->~ChSystemNSC();
            })
        }
    }
}
