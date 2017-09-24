use super::*;

impl CppClass for Link {}

#[repr(C)]
pub struct Link(ffi::chrono::ChLink, NotSendSync);

unsafe impl Inherits<Object> for Link {}

unsafe impl<T> Inherits<Object> for T where T: Inherits<Link> {
    #[inline(always)]
    fn offset<'a, F>(f: F) -> isize where F: FnOnce() -> &'a Self, Self: 'a {
        <T as Inherits<Link>>::offset(f)
    }
}

unsafe impl Shareable for Link {
    type Inner = ffi::chrono::ChLink;
    type Args = Void;

    fn make_shared_impl(args: Void) -> ffi::std::shared_ptr<Self::Inner> {
        match args {}
    }

    fn clone_impl(ptr: &ffi::std::shared_ptr<Self::Inner>) -> ffi::std::shared_ptr<Self::Inner> {
        unsafe {
            cpp!([ptr as "const std::shared_ptr<chrono::ChLink>*"] -> ffi::std::shared_ptr<ffi::chrono::ChLink> as "std::shared_ptr<chrono::ChLink>" {
                return *ptr;
            })
        }
    }

    fn drop_impl(ptr: &mut ffi::std::shared_ptr<Self::Inner>) {
        unsafe {
            cpp!([ptr as "std::shared_ptr<chrono::ChLink>*"] {
                ptr->~shared_ptr();
            })
        }
    }
}
