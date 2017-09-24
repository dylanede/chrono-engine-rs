use super::*;

mod link;
pub use self::link::Link;

mod link_markers;
pub use self::link_markers::LinkMarkers;

mod link_lock;
pub use self::link_lock::LinkLock;

mod link_lock_lock {
    use super::*;

    #[repr(C)]
    pub struct LinkLockLock(ffi::chrono::ChLinkLockLock, NotSendSync);

    impl CppClass for LinkLockLock {}

    unsafe impl Inherits<LinkLock> for LinkLockLock {}

    impl Deref for LinkLockLock {
        type Target = LinkLock;

        fn deref(&self) -> &LinkLock {
            self.as_::<LinkLock>()
        }
    }

    unsafe impl Shareable for LinkLockLock {
        type Inner = ffi::chrono::ChLinkLockLock;
        type Args = ();

        fn make_shared_impl(_: Self::Args) -> ffi::std::shared_ptr<Self::Inner> {
            let mut inner = unsafe { mem::zeroed::<ffi::std::shared_ptr<ffi::chrono::ChLinkLockLock>>() };
            unsafe {
                cpp!([mut inner as "std::shared_ptr<chrono::ChLinkLockLock>"] {
                    new(&inner) std::shared_ptr<chrono::ChLinkLockLock>(std::move(std::make_shared<chrono::ChLinkLockLock>()));
                })
            }
            inner
        }

        fn clone_impl(ptr: &ffi::std::shared_ptr<Self::Inner>) -> ffi::std::shared_ptr<Self::Inner> {
            unsafe {
                cpp!([ptr as "const std::shared_ptr<chrono::ChLinkLockLock>*"] -> ffi::std::shared_ptr<ffi::chrono::ChLinkLockLock> as "std::shared_ptr<chrono::ChLinkLockLock>" {
                    return *ptr;
                })
            }
        }

        fn drop_impl(ptr: &mut ffi::std::shared_ptr<Self::Inner>) {
            unsafe {
                cpp!([ptr as "std::shared_ptr<chrono::ChLinkLockLock>*"] {
                    ptr->~shared_ptr();
                })
            }
        }
    }
}
pub use self::link_lock_lock::LinkLockLock;

mod link_lock_revolute;
pub use self::link_lock_revolute::LinkLockRevolute;

mod link_lock_point_line;
pub use self::link_lock_point_line::LinkLockPointLine;

mod link_engine;
pub use self::link_engine::{ LinkEngine, EngineMode };