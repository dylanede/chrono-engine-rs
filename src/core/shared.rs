use std::marker::PhantomData;

#[repr(C)]
pub struct SharedData<T: Shareable> {
    data: [usize; 2],
    marker: PhantomData<*const T>
}

/// A shared pointer to a Chrono Engine type. This type is not `Send` or `Sync`.
///
/// Internally this is a C++ `std::shared_ptr`.
pub struct Shared<T: Shareable> {
    data: SharedData<T>
}

impl<T: Shareable> Clone for Shared<T> {
    fn clone(&self) -> Shared<T> {
        Shared {
            data: T::clone_shared(&self.data)
        }
    }
}

impl<T: Shareable> Drop for Shared<T> {
    fn drop(&mut self) {
        T::drop_shared(&mut self.data)
    }
}

/// This is implemented for Chrono Engine types that can be put into a shared pointer
/// (specifically a C++ `std::shared_ptr`). You cannot implement this yourself.
pub trait Shareable: Sized {
    #[doc(hidden)]
    fn into_shared(self: Self) -> Shared<Self>;
    #[doc(hidden)]
    fn clone_shared(ptr_data: &SharedData<Self>) -> SharedData<Self>;
    #[doc(hidden)]
    fn drop_shared(ptr_data: &mut SharedData<Self>);
    #[doc(hidden)]
    fn deref_shared(ptr_data: &SharedData<Self>) -> &Self;
}

impl<T: Shareable> ::std::ops::Deref for Shared<T> {
    type Target = T;
    fn deref(&self) -> &T {
        T::deref_shared(&self.data)
    }
}

pub trait AccessSharedData {
    type Target: Shareable;
    fn from_data(data: SharedData<Self::Target>) -> Self;
    fn data(&self) -> &SharedData<Self::Target>;
    fn data_mut(&mut self) -> &mut SharedData<Self::Target>;
}

impl<T: Shareable> AccessSharedData for Shared<T> {
    type Target = T;
    fn from_data(data: SharedData<T>) -> Shared<T> {
        Shared {
            data: data
        }
    }
    fn data(&self) -> &SharedData<T> {
        &self.data
    }
    fn data_mut(&mut self) -> &mut SharedData<T> {
        &mut self.data
    }
}
