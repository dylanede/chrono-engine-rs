#![allow(improper_ctypes)]

extern crate libc;
extern crate chrono_engine_sys;
extern crate nalgebra;
#[macro_use]
extern crate cpp;
#[macro_use]
extern crate lazy_static;
extern crate void;

use void::Void;
use chrono_engine_sys::root as ffi;

use std::{ mem, ptr };

use std::marker::PhantomData;
use std::rc::Rc;
use std::ops::Deref;

use nalgebra::{
    Vector3,
    Quaternion,
    Matrix3
};

#[derive(Copy, Clone)]
struct NotSendSync(PhantomData<*const ()>);

const NOT_SEND_SYNC: NotSendSync = NotSendSync(PhantomData);

pub trait CppClass {}

pub unsafe trait Inherits<T : CppClass> : CppClass {
    #[inline(always)]
    fn offset<'a, F>(_: F) -> isize where F : FnOnce() -> &'a Self, Self : 'a {
        0
    }
}

pub trait InheritsExt : CppClass {
    #[inline(always)]
    fn as_<T>(&self) -> &T where Self : Inherits<T>, T : CppClass {
        let ptr = unsafe { (self as *const _ as *const u8).offset(Self::offset(|| self)) as *const T };
        unsafe { &*ptr }
    }
}
impl<T : CppClass> InheritsExt for T {}

pub struct Shared<T : Shareable> {
    inner: ffi::std::shared_ptr<T::Inner>,
    marker_: PhantomData<Rc<T>>
}
impl<T : Shareable> Shared<T> {
    pub(crate) fn is_null(ptr: &Shared<T>) -> bool {
        (&ptr.inner as *const _ as *const *const T::Inner).is_null()
    }
    pub(crate) fn maybe_null(ptr: Shared<T>) -> Option<Shared<T>> {
        if Shared::is_null(&ptr) {
            mem::forget(ptr);
            None
        } else {
            Some(ptr)
        }
    }
}
impl<T1> Shared<T1> where T1 : Shareable {
    fn upcast<T2>(mut self) -> Shared<T2> where T1 : Inherits<T2>, T2 : Shareable + CppClass {
        let offset = T1::offset(|| &*self);
        // This relies on the pointer to T1/T2 being at the start of shared_ptr - in practice this is always the case
        {
            let mut start_ptr = unsafe { &mut *(&mut self.inner as *mut _ as *mut *mut T1::Inner) };
            *start_ptr = unsafe { start_ptr.offset(offset) };
        }
        unsafe { mem::transmute::<Shared<T1>, Shared<T2>>(self) }
    }
}

impl<T : Shareable> Clone for Shared<T> {
    fn clone(&self) -> Self {
        Shared {
            inner: T::clone_impl(&self.inner),
            marker_: PhantomData
        }
    }
}

impl<T : Shareable> Drop for Shared<T> {
    fn drop(&mut self) {
        T::drop_impl(&mut self.inner)
    }
}

impl<T> Deref for Shared<T> where T : Shareable {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &T {
        unsafe { mem::transmute_copy(&self.inner) }
    }
}

pub unsafe trait Shareable {
    type Inner;
    type Args;
    fn make_shared_impl(args: Self::Args) -> ffi::std::shared_ptr<Self::Inner>;
    fn clone_impl(ptr: &ffi::std::shared_ptr<Self::Inner>) -> ffi::std::shared_ptr<Self::Inner>;
    fn drop_impl(ptr: &mut ffi::std::shared_ptr<Self::Inner>);
}

pub fn make_shared<T : Shareable>(args: T::Args) -> Shared<T> {
    let raw = T::make_shared_impl(args);
    Shared {
        inner: raw,
        marker_: PhantomData
    }
}

pub struct Mnd<T>(Box<T>);

impl<T> Deref for Mnd<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &*self.0
    }
}

mod object;
pub use object::Object;

mod body;
pub use body::Body;

mod material_surface {
    use super::*;

    cpp!{{
        #include "chrono/physics/ChMaterialSurface.h"
    }}

    #[repr(C)]
    pub struct MaterialSurface(ffi::chrono::ChMaterialSurface, NotSendSync);

    impl CppClass for MaterialSurface {}

    unsafe impl Shareable for MaterialSurface {
        type Inner = ffi::chrono::ChMaterialSurface;
        type Args = Void;

        fn make_shared_impl(args: Void) -> ffi::std::shared_ptr<Self::Inner> {
            match args {}
        }

        fn clone_impl(ptr: &ffi::std::shared_ptr<Self::Inner>) -> ffi::std::shared_ptr<Self::Inner> {
            unsafe {
                cpp!([ptr as "const std::shared_ptr<chrono::ChMaterialSurface>*"] -> ffi::std::shared_ptr<ffi::chrono::ChMaterialSurface> as "std::shared_ptr<chrono::ChMaterialSurface>" {
                    return *ptr;
                })
            }
        }

        fn drop_impl(ptr: &mut ffi::std::shared_ptr<Self::Inner>) {
            unsafe {
                cpp!([ptr as "std::shared_ptr<chrono::ChMaterialSurface>*"] {
                    ptr->~shared_ptr();
                })
            }
        }
    }

    cpp!{{
        #include "chrono/physics/ChMaterialSurfaceNSC.h"
    }}

    #[repr(C)]
    pub struct MaterialSurfaceNSC(ffi::chrono::ChMaterialSurfaceNSC, NotSendSync);

    impl CppClass for MaterialSurfaceNSC {}

    unsafe impl Inherits<MaterialSurface> for MaterialSurfaceNSC {}

    impl Deref for MaterialSurfaceNSC {
        type Target = MaterialSurface;

        fn deref(&self) -> &MaterialSurface {
            self.as_::<MaterialSurface>()
        }
    }

    unsafe impl Shareable for MaterialSurfaceNSC {
        type Inner = ffi::chrono::ChMaterialSurfaceNSC;
        type Args = ();

        fn make_shared_impl(_: Self::Args) -> ffi::std::shared_ptr<Self::Inner> {
            let mut inner = unsafe { mem::zeroed::<ffi::std::shared_ptr<ffi::chrono::ChMaterialSurfaceNSC>>() };
            unsafe {
                cpp!([mut inner as "std::shared_ptr<chrono::ChMaterialSurfaceNSC>"] {
                    new(&inner) std::shared_ptr<chrono::ChMaterialSurfaceNSC>(std::move(std::make_shared<chrono::ChMaterialSurfaceNSC>()));
                })
            }
            inner
        }

        fn clone_impl(ptr: &ffi::std::shared_ptr<Self::Inner>) -> ffi::std::shared_ptr<Self::Inner> {
            unsafe {
                cpp!([ptr as "const std::shared_ptr<chrono::ChMaterialSurfaceNSC>*"] -> ffi::std::shared_ptr<ffi::chrono::ChMaterialSurfaceNSC> as "std::shared_ptr<chrono::ChMaterialSurfaceNSC>" {
                    return *ptr;
                })
            }
        }

        fn drop_impl(ptr: &mut ffi::std::shared_ptr<Self::Inner>) {
            unsafe {
                cpp!([ptr as "std::shared_ptr<chrono::ChMaterialSurfaceNSC>*"] {
                    ptr->~shared_ptr();
                })
            }
        }
    }

    impl MaterialSurfaceNSC {
        pub fn set_restitution(&self, restitution: f32) {
            let this_ = self as *const _;
            unsafe {
                cpp!([this_ as "chrono::ChMaterialSurfaceNSC*", restitution as "float"] {
                    this_->SetRestitution(restitution);
                })
            }
        }
        pub fn set_rolling_friction(&self, rolling_friction: f32) {
            let this_ = self as *const _;
            unsafe {
                cpp!([this_ as "chrono::ChMaterialSurfaceNSC*", rolling_friction as "float"] {
                    this_->SetRollingFriction(rolling_friction);
                })
            }
        }
        pub fn set_cohesion(&self, cohesion: f32) {
            let this_ = self as *const _;
            unsafe {
                cpp!([this_ as "chrono::ChMaterialSurfaceNSC*", cohesion as "float"] {
                    this_->SetCohesion(cohesion);
                })
            }
        }
    }
}
pub use material_surface::{
    MaterialSurface,
    MaterialSurfaceNSC
};

mod collision_model;
pub use collision_model::CollisionModel;

mod marker;
pub use marker::Marker;

mod force;
pub use force::Force;

mod system_nsc;
pub use system_nsc::SystemNSC;

mod logging;
pub use logging::{
    StdoutLogger,
    StreamOutAscii,
    stdout_logger,
    Log,
    stdout_logger_output
};

#[repr(C)]
pub struct CoordSys {
    pub position: Vector3<f64>,
    pub orientation: Quaternion<f64>
}

impl CoordSys {
    pub fn new(position: Vector3<f64>, orientation: Quaternion<f64>) -> CoordSys {
        CoordSys {
            position: position,
            orientation: orientation,
        }
    }
    pub fn from_pos(position: Vector3<f64>) -> CoordSys {
        CoordSys {
            position: position,
            orientation: Quaternion::identity(),
        }
    }
    pub fn from_rot(orientation: Quaternion<f64>) -> CoordSys {
        CoordSys {
            position: Vector3::new(0.0, 0.0, 0.0),
            orientation: orientation
        }
    }
}

mod links;
pub use links::{
    Link,
    LinkMarkers,
    LinkLockRevolute,
    LinkLockPointLine,
    LinkEngine,
    EngineMode
};

pub mod function;