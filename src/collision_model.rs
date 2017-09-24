use super::*;
cpp! {{
    #include "chrono/collision/ChCCollisionModel.h"
}}
#[repr(C)]
pub struct CollisionModel(ffi::chrono::collision::ChCollisionModel, NotSendSync);

impl CppClass for CollisionModel {}

unsafe impl Shareable for CollisionModel {
    type Inner = ffi::chrono::collision::ChCollisionModel;
    type Args = Void;

    fn make_shared_impl(args: Void) -> ffi::std::shared_ptr<Self::Inner> {
        match args {}
    }

    fn clone_impl(ptr: &ffi::std::shared_ptr<Self::Inner>) -> ffi::std::shared_ptr<Self::Inner> {
        unsafe {
            cpp!([ptr as "const std::shared_ptr<chrono::collision::ChCollisionModel>*"] -> ffi::std::shared_ptr<ffi::chrono::collision::ChCollisionModel> as "std::shared_ptr<chrono::collision::ChCollisionModel>" {
                return *ptr;
            })
        }
    }

    fn drop_impl(ptr: &mut ffi::std::shared_ptr<Self::Inner>) {
        unsafe {
            cpp!([ptr as "std::shared_ptr<chrono::collision::ChCollisionModel>*"] {
                ptr->~shared_ptr();
            })
        }
    }
}

impl CollisionModel {
    pub fn clear_model(&self) -> i32 {
        let this_ = self as *const _;
        unsafe {
            cpp!([this_ as "chrono::collision::ChCollisionModel*"] -> i32 as "int32_t" {
                return this_->ClearModel();
            })
        }
    }
    pub fn build_model(&self) -> i32 {
        let this_ = self as *const _;
        unsafe {
            cpp!([this_ as "chrono::collision::ChCollisionModel*"] -> i32 as "int32_t" {
                return this_->BuildModel();
            })
        }
    }
    pub fn add_sphere(&self, radius: f64, position: &Vector3<f64>) -> bool {
        let this_ = self as *const _;
        unsafe {
            cpp!([this_ as "chrono::collision::ChCollisionModel*", radius as "double", position as "const chrono::ChVector<double>*"] -> u8 as "uint8_t" {
                return this_->AddSphere(radius, *position) ? 1 : 0;
            }) != 0
        }
    }
    pub fn add_box(&self, hx: f64, hy: f64, hz: f64, position: &Vector3<f64>) -> bool {
        let this_ = self as *const _;
        unsafe {
            cpp!([this_ as "chrono::collision::ChCollisionModel*", hx as "double", hy as "double", hz as "double", position as "const chrono::ChVector<double>*"] -> u8 as "uint8_t" {
                return this_->AddBox(hx, hy, hz, *position) ? 1 : 0;
            }) != 0
        }
    }
}
