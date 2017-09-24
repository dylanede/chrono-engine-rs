use super::*;

#[repr(C)]
pub struct LinkLock(ffi::chrono::ChLinkLock, NotSendSync);

impl CppClass for LinkLock {}

unsafe impl Inherits<LinkMarkers> for LinkLock {}

impl Deref for LinkLock {
    type Target = LinkMarkers;

    fn deref(&self) -> &LinkMarkers {
        self.as_::<LinkMarkers>()
    }
}

unsafe impl Shareable for LinkLock {
    type Inner = ffi::chrono::ChLinkLock;
    type Args = Void;

    fn make_shared_impl(args: Void) -> ffi::std::shared_ptr<Self::Inner> {
        match args {}
    }

    fn clone_impl(ptr: &ffi::std::shared_ptr<Self::Inner>) -> ffi::std::shared_ptr<Self::Inner> {
        let mut inner = unsafe { mem::zeroed::<ffi::std::shared_ptr<ffi::chrono::ChLinkLock>>() };
        unsafe {
            cpp!([ptr as "const std::shared_ptr<chrono::ChLinkLock>*", mut inner as "std::shared_ptr<chrono::ChLinkLock>"] {
                new(&inner) std::shared_ptr<chrono::ChLinkLock>(*ptr);
            })
        }
        inner
    }

    fn drop_impl(ptr: &mut ffi::std::shared_ptr<Self::Inner>) {
        unsafe {
            cpp!([ptr as "std::shared_ptr<chrono::ChLinkLock>*"] {
                ptr->~shared_ptr();
            })
        }
    }
}

#[repr(i32)]
pub enum AngleSet {
    AngleAxis=0,
    Euler,
    Cardan,
    HPB,
    RXYZ,
    Rodriguez,
    Quaternion
}

impl LinkLock {
    pub fn set_motion_x<T>(&self, function: Shared<T>) where T: Inherits<function::Function> + Shareable {
        let function = function.upcast::<function::Function>();
        let this_ = self as *const _;
        unsafe {
            cpp!([this_ as "chrono::ChLinkLock*", function as "std::shared_ptr<chrono::ChFunction>"] {
                this_->SetMotion_X(function);
            })
        }
    }
    pub fn set_motion_y<T>(&self, function: Shared<T>) where T: Inherits<function::Function> + Shareable {
        let function = function.upcast::<function::Function>();
        let this_ = self as *const _;
        unsafe {
            cpp!([this_ as "chrono::ChLinkLock*", function as "std::shared_ptr<chrono::ChFunction>"] {
                this_->SetMotion_Y(function);
            })
        }
    }
    pub fn set_motion_z<T>(&self, function: Shared<T>) where T: Inherits<function::Function> + Shareable {
        let function = function.upcast::<function::Function>();
        let this_ = self as *const _;
        unsafe {
            cpp!([this_ as "chrono::ChLinkLock*", function as "std::shared_ptr<chrono::ChFunction>"] {
                this_->SetMotion_Z(function);
            })
        }
    }
    pub fn set_angle_set(&self, angle_set: AngleSet) {
        let this_ = self as *const _;
        unsafe {
            cpp!([this_ as "chrono::ChLinkLock*", angle_set as "chrono::AngleSet"] {
                this_->Set_angleset(angle_set);
            })
        }
    }
}