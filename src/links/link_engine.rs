use super::*;

#[repr(i32)]
pub enum EngineMode {
    Rotation = 0,
    Speed,
    Torque,
    KeyRotation,
    KeyPolar,
    ToPowertrainShaft
}

#[repr(C)]
pub struct LinkEngine(ffi::chrono::ChLinkEngine, NotSendSync);

impl CppClass for LinkEngine {}

unsafe impl Inherits<LinkMarkers> for LinkEngine {}

impl Deref for LinkEngine {
    type Target = LinkMarkers;

    fn deref(&self) -> &LinkMarkers {
        self.as_::<LinkMarkers>()
    }
}

unsafe impl Shareable for LinkEngine {
    type Inner = ffi::chrono::ChLinkEngine;
    type Args = ();

    fn make_shared_impl(_: Self::Args) -> ffi::std::shared_ptr<Self::Inner> {
        let mut inner = unsafe { mem::zeroed::<ffi::std::shared_ptr<ffi::chrono::ChLinkEngine>>() };
        unsafe {
            cpp!([mut inner as "std::shared_ptr<chrono::ChLinkEngine>"] {
                new(&inner) std::shared_ptr<chrono::ChLinkEngine>(std::move(std::make_shared<chrono::ChLinkEngine>()));
            })
        }
        inner
    }

    fn clone_impl(ptr: &ffi::std::shared_ptr<Self::Inner>) -> ffi::std::shared_ptr<Self::Inner> {
        unsafe {
            cpp!([ptr as "const std::shared_ptr<chrono::ChLinkEngine>*"] -> ffi::std::shared_ptr<ffi::chrono::ChLinkEngine> as "std::shared_ptr<chrono::ChLinkEngine>" {
                return *ptr;
            })
        }
    }

    fn drop_impl(ptr: &mut ffi::std::shared_ptr<Self::Inner>) {
        unsafe {
            cpp!([ptr as "std::shared_ptr<chrono::ChLinkEngine>*"] {
                ptr->~shared_ptr();
            })
        }
    }
}

impl LinkEngine {
    pub fn set_eng_mode(&self, mode: EngineMode) {
        let this_ = self as *const _;
        unsafe {
            cpp!([this_ as "chrono::ChLinkEngine*", mode as "int32_t"] {
                this_->Set_eng_mode(static_cast<chrono::ChLinkEngine::eCh_eng_mode>(mode));
            })
        }
    }
    pub fn set_speed_fn<T>(&self, function: Shared<T>) where T: Inherits<function::Function> + Shareable {
        let function = function.upcast::<function::Function>();
        let this_ = self as *const _;
        unsafe {
            cpp!([this_ as "chrono::ChLinkEngine*", function as "std::shared_ptr<chrono::ChFunction>"] {
                this_->Set_spe_funct(function);
            })
        }
    }
    pub fn mot_retorque(&self) -> f64 {
        let this_ = self as *const _;
        unsafe {
            cpp!([this_ as "chrono::ChLinkEngine*"] -> f64 as "double" {
                return this_->Get_mot_retorque();
            })
        }
    }
}
