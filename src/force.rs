use super::*;

cpp!{{
    #include "chrono/physics/ChForce.h"
}}

unsafe impl Inherits<Object> for Force {}

#[repr(C)]
pub struct Force(ffi::chrono::ChForce, NotSendSync);

impl CppClass for Force {}

unsafe impl Shareable for Force {
    type Inner = ffi::chrono::ChForce;
    type Args = ();

    fn make_shared_impl(_: ()) -> ffi::std::shared_ptr<Self::Inner> {
        let mut inner = unsafe { mem::zeroed::<ffi::std::shared_ptr<ffi::chrono::ChForce>>() };
        unsafe {
            cpp!([mut inner as "std::shared_ptr<chrono::ChForce>"] {
                new(&inner) std::shared_ptr<chrono::ChForce>(std::move(std::make_shared<chrono::ChForce>()));
            })
        }
        inner
    }
    fn clone_impl(ptr: &ffi::std::shared_ptr<Self::Inner>) -> ffi::std::shared_ptr<Self::Inner> {
        let ptr = ptr as *const _;
        unsafe {
            cpp!([ptr as "const std::shared_ptr<chrono::ChForce>*"] -> ffi::std::shared_ptr<ffi::chrono::ChForce> as "std::shared_ptr<chrono::ChForce>" {
                return *ptr;
            })
        }
    }

    fn drop_impl(ptr: &mut ffi::std::shared_ptr<Self::Inner>) {
        unsafe {
            cpp!([ptr as "std::shared_ptr<chrono::ChForce>*"] {
                ptr->~shared_ptr();
            })
        }
    }
}
