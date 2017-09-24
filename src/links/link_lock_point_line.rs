use super::*;

#[repr(C)]
pub struct LinkLockPointLine(ffi::chrono::ChLinkLockPointLine, NotSendSync);

impl CppClass for LinkLockPointLine {}

unsafe impl Inherits<LinkLock> for LinkLockPointLine {}

impl Deref for LinkLockPointLine {
    type Target = LinkLock;

    fn deref(&self) -> &LinkLock {
        self.as_::<LinkLock>()
    }
}

unsafe impl Shareable for LinkLockPointLine {
    type Inner = ffi::chrono::ChLinkLockPointLine;
    type Args = ();

    fn make_shared_impl(_: Self::Args) -> ffi::std::shared_ptr<Self::Inner> {
        let mut inner = unsafe { mem::zeroed::<ffi::std::shared_ptr<ffi::chrono::ChLinkLockPointLine>>() };
        unsafe {
            cpp!([mut inner as "std::shared_ptr<chrono::ChLinkLockPointLine>"] {
                new(&inner) std::shared_ptr<chrono::ChLinkLockPointLine>(std::move(std::make_shared<chrono::ChLinkLockPointLine>()));
            })
        }
        inner
    }

    fn clone_impl(ptr: &ffi::std::shared_ptr<Self::Inner>) -> ffi::std::shared_ptr<Self::Inner> {
        unsafe {
            cpp!([ptr as "const std::shared_ptr<chrono::ChLinkLockPointLine>*"] -> ffi::std::shared_ptr<ffi::chrono::ChLinkLockPointLine> as "std::shared_ptr<chrono::ChLinkLockPointLine>" {
                return *ptr;
            })
        }
    }

    fn drop_impl(ptr: &mut ffi::std::shared_ptr<Self::Inner>) {
        unsafe {
            cpp!([ptr as "std::shared_ptr<chrono::ChLinkLockPointLine>*"] {
                ptr->~shared_ptr();
            })
        }
    }
}
