use super::*;

#[repr(C)]
pub struct LinkMarkers(ffi::chrono::ChLinkMarkers, NotSendSync);

impl CppClass for LinkMarkers {}

unsafe impl Inherits<Link> for LinkMarkers {}

unsafe impl<T> Inherits<Link> for T where T: Inherits<LinkMarkers> {
    #[inline(always)]
    fn offset<'a, F>(f: F) -> isize where F: FnOnce() -> &'a Self, Self: 'a {
        <T as Inherits<LinkMarkers>>::offset(f)
    }
}

impl Deref for LinkMarkers {
    type Target = Link;

    fn deref(&self) -> &Link {
        self.as_::<Link>()
    }
}

impl LinkMarkers {
    pub fn initialize_from_markers(&self, marker_1: Shared<Marker>, marker_2: Shared<Marker>) {
        let this_ = self as *const _;
        unsafe {
            cpp!([this_ as "chrono::ChLinkMarkers*", marker_1 as "std::shared_ptr<chrono::ChMarker>", marker_2 as "std::shared_ptr<chrono::ChMarker>"] {
                this_->Initialize(marker_1, marker_2);
            })
        }
    }
    pub fn initialize_from_bodies(&self, body_1: Shared<Body>, body_2: Shared<Body>, point: &CoordSys) {
        let this_ = self as *const _;
        unsafe {
            cpp!([this_ as "chrono::ChLinkMarkers*", body_1 as "std::shared_ptr<chrono::ChBody>", body_2 as "std::shared_ptr<chrono::ChBody>", point as "const chrono::ChCoordsys<double>*"] {
                this_->Initialize(body_1, body_2, *point);
            })
        }
    }
    pub fn marker_1(&self) -> Option<&Marker> {
        let this_ = self as *const _;
        let ptr = unsafe {
            cpp!([this_ as "chrono::ChLinkMarkers*"] -> *const Marker as "const chrono::ChMarker*" {
                return this_->GetMarker1();
            })
        };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { &*ptr })
        }
    }
    pub fn marker_2(&self) -> Option<&Marker> {
        let this_ = self as *const _;
        let ptr = unsafe {
            cpp!([this_ as "chrono::ChLinkMarkers*"] -> *const Marker as "const chrono::ChMarker*" {
                return this_->GetMarker2();
            })
        };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { &*ptr })
        }
    }
}
