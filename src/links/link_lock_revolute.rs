use super::*;

#[repr(C)]
pub struct LinkLockRevolute(ffi::chrono::ChLinkLockRevolute, NotSendSync);

impl CppClass for LinkLockRevolute {}

unsafe impl Inherits<LinkLock> for LinkLockRevolute {}

impl Deref for LinkLockRevolute {
    type Target = LinkLock;

    fn deref(&self) -> &LinkLock {
        self.as_::<LinkLock>()
    }
}

unsafe impl Shareable for LinkLockRevolute {
    type Inner = ffi::chrono::ChLinkLockRevolute;
    type Args = ();

    fn make_shared_impl(_: Self::Args) -> ffi::std::shared_ptr<Self::Inner> {
        let mut inner = unsafe { mem::zeroed::<ffi::std::shared_ptr<ffi::chrono::ChLinkLockRevolute>>() };
        unsafe {
            cpp!([mut inner as "std::shared_ptr<chrono::ChLinkLockRevolute>"] {
                new(&inner) std::shared_ptr<chrono::ChLinkLockRevolute>(std::move(std::make_shared<chrono::ChLinkLockRevolute>()));
            })
        }
        inner
    }

    fn clone_impl(ptr: &ffi::std::shared_ptr<Self::Inner>) -> ffi::std::shared_ptr<Self::Inner> {
        unsafe {
            cpp!([ptr as "const std::shared_ptr<chrono::ChLinkLockRevolute>*"] -> ffi::std::shared_ptr<ffi::chrono::ChLinkLockRevolute> as "std::shared_ptr<chrono::ChLinkLockRevolute>" {
                return *ptr;
            })
        }
    }

    fn drop_impl(ptr: &mut ffi::std::shared_ptr<Self::Inner>) {
        unsafe {
            cpp!([ptr as "std::shared_ptr<chrono::ChLinkLockRevolute>*"] {
                ptr->~shared_ptr();
            })
        }
    }
}
