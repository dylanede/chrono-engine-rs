use ::ffi;
use ::core::{Shared, SharedData, Shareable, AccessSharedData};
pub struct Body {
    ptr: ffi::core::InternalPointer<Body>
}
impl Drop for Body {
    fn drop(&mut self) {
        unsafe { ffi::physics::delete_body(self.ptr) }
    }
}
impl Shareable for Body {
    fn into_shared(self) -> Shared<Body> {
        let result = Shared::<Body>::from_data(unsafe {
            ffi::physics::body_into_shared(self.ptr)
        });
        ::std::mem::forget(self);
        result
    }
    fn clone_shared(ptr_data: &SharedData<Body>) -> SharedData<Body> {
        unsafe { ffi::physics::shared_body_clone(ptr_data as *const _) }
    }
    fn drop_shared(ptr_data: &mut SharedData<Body>) {
        unsafe { ffi::physics::drop_shared_body(ptr_data as *mut _) }
    }
    fn deref_shared(ptr_data: &SharedData<Body>) -> &Body {
        unsafe {
            // This relies on the check_shared_body_layout test passing
            &*::std::mem::transmute::<*const SharedData<Body>, *const Body>(ptr_data as *const _)
        }
    }
}

#[test]
fn check_shared_body_layout() {
    use ::std::mem;
    unsafe {
        // Check location in std::shared_ptr
        let dummy: SharedData<Body> = mem::uninitialized();
        let ptr = ffi::physics::shared_body_deref(&dummy as *const _);
        let ptr_val = mem::transmute::<_, usize>(ptr);
        let dummy_val = mem::transmute::<_, [usize; 2]>(dummy);
        assert_eq!(dummy_val[0], ptr_val);
        // Check location in Body
        let dummy: Body = mem::uninitialized();
        let dummy_ptr_val = &dummy as *const _ as usize;
        let dummy_internal_ptr_val = &dummy.ptr as *const _ as usize;
        assert_eq!(dummy_ptr_val, dummy_internal_ptr_val);
        mem::forget(dummy);
    }
}

impl Body {
    pub fn new_shared() -> Shared<Body> {
        Shared::<Body>::from_data(unsafe {
            ffi::physics::make_shared_body()
        })
    }
}
