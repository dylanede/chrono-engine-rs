cpp!{{
    #include "chrono/physics/ChMarker.h"
}}

use super::*;

unsafe impl Inherits<Object> for Marker {
    #[inline(always)]
    fn offset<'a, F>(_: F) -> isize where F: FnOnce() -> &'a Self, Self: 'a {
        fn make_offset() -> isize {
            unsafe {
                cpp!([] -> isize as "ptrdiff_t" {
                    chrono::ChMarker* b = nullptr;
                    chrono::ChObj* o = static_cast<chrono::ChObj*>(b);
                    return ((uint8_t*)o) - ((uint8_t*)b);
                })
            }
        }
        lazy_static! {
            static ref OFFSET: isize = {
                make_offset()
            };
        }
        *OFFSET
    }
}

#[repr(C)]
pub struct Marker(ffi::chrono::ChMarker, NotSendSync);

impl CppClass for Marker {}

impl Marker {
    pub fn impose_abs_coord(&self, coord_sys: &CoordSys) {
        let this_ = &self.0 as *const _;
        let coord_sys = coord_sys as *const _;
        unsafe {
            cpp!([this_ as "chrono::ChMarker*", coord_sys as "const chrono::ChCoordsys<double>*"] {
                this_->Impose_Abs_Coord(*coord_sys);
            })
        }
    }
    pub fn abs_coord(&self) -> &CoordSys {
        let this_ = &self.0 as *const _;
        unsafe {
            cpp!([this_ as "chrono::ChMarker*"] -> &CoordSys as "const chrono::ChCoordsys<double>*" {
                return &this_->GetAbsCoord();
            })
        }
    }
}

unsafe impl Shareable for Marker {
    type Inner = ffi::chrono::ChMarker;
    type Args = ();

    fn make_shared_impl(_: ()) -> ffi::std::shared_ptr<Self::Inner> {
        let mut inner = unsafe { mem::zeroed::<ffi::std::shared_ptr<ffi::chrono::ChMarker>>() };
        unsafe {
            cpp!([mut inner as "std::shared_ptr<chrono::ChMarker>"] {
                new(&inner) std::shared_ptr<chrono::ChMarker>(std::move(std::make_shared<chrono::ChMarker>()));
            })
        }
        inner
    }
    fn clone_impl(ptr: &ffi::std::shared_ptr<Self::Inner>) -> ffi::std::shared_ptr<Self::Inner> {
        let ptr = ptr as *const _;
        unsafe {
            cpp!([ptr as "const std::shared_ptr<chrono::ChMarker>*"] -> ffi::std::shared_ptr<ffi::chrono::ChMarker> as "std::shared_ptr<chrono::ChMarker>" {
                return *ptr;
            })
        }
    }

    fn drop_impl(ptr: &mut ffi::std::shared_ptr<Self::Inner>) {
        unsafe {
            cpp!([ptr as "std::shared_ptr<chrono::ChMarker>*"] {
                ptr->~shared_ptr();
            })
        }
    }
}
