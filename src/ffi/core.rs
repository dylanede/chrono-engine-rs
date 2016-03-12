#[repr(C)]
pub struct InternalPointer<T>(*mut ::libc::c_void, ::std::marker::PhantomData<T>);
impl<T> Copy for InternalPointer<T> {}
impl<T> Clone for InternalPointer<T> {
    fn clone(&self) -> InternalPointer<T> {
        *self
    }
}
impl<T> PartialEq for InternalPointer<T> {
    fn eq(&self, other: &InternalPointer<T>) -> bool {
        self.0 == other.0
    }
}
impl<T> Eq for InternalPointer<T> {}
