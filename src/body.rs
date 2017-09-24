use super::*;
cpp! {{
    #include "chrono/physics/ChBody.h"
}}

#[repr(C)]
pub struct Body(ffi::chrono::ChBody, NotSendSync);

impl CppClass for Body {}

unsafe impl Inherits<Object> for Body {
    #[inline(always)]
    fn offset<'a, F>(_: F) -> isize where F: FnOnce() -> &'a Self, Self: 'a {
        unsafe fn make_offset() -> isize {
            cpp!([] -> isize as "ptrdiff_t" {
                chrono::ChBody* b = nullptr;
                chrono::ChObj* o = static_cast<chrono::ChObj*>(b);
                return ((uint8_t*)o) - ((uint8_t*)b);
            })
        }
        lazy_static! {
            static ref OFFSET: isize = {
                unsafe { make_offset() }
            };
        }
        *OFFSET
    }
}

unsafe impl Shareable for Body {
    type Inner = ffi::chrono::ChBody;
    type Args = ();

    fn make_shared_impl(_: ()) -> ffi::std::shared_ptr<Self::Inner> {
        unsafe {
            cpp!([] -> ffi::std::shared_ptr<ffi::chrono::ChBody> as "std::shared_ptr<chrono::ChBody>" {
                return std::make_shared<chrono::ChBody>();
            })
        }
    }
    fn clone_impl(ptr: &ffi::std::shared_ptr<Self::Inner>) -> ffi::std::shared_ptr<Self::Inner> {
        unsafe {
            cpp!([ptr as "const std::shared_ptr<chrono::ChBody>*"] -> ffi::std::shared_ptr<ffi::chrono::ChBody> as "std::shared_ptr<chrono::ChBody>" {
                return *ptr;
            })
        }
    }

    fn drop_impl(ptr: &mut ffi::std::shared_ptr<Self::Inner>) {
        unsafe {
            cpp!([ptr as "std::shared_ptr<chrono::ChBody>*"] {
                ptr->~shared_ptr();
            })
        }
    }
}

impl Body {
    pub fn sphere(radius: f64, density: f64) -> Shared<Body> {
        use std::f64::consts::PI;
        let mass = density * (4.0 / 3.0) * PI * radius.powi(3);
        let inertia = (2.0 / 5.0) * mass * radius.powi(2);
        let sphere = make_shared::<Body>(());
        sphere.set_density(density as f32);
        sphere.set_mass(mass);
        sphere.set_inertia_xx(&Vector3::new(inertia, inertia, inertia));
        sphere.set_collide(true);
        {
            let model = sphere.get_collision_model().unwrap();
            model.clear_model();
            model.add_sphere(radius, &Vector3::new(0.0, 0.0, 0.0));
            model.build_model();
        }
        sphere
    }
    pub fn add_marker(&self, marker: Shared<Marker>) {
        let ptr = unsafe { ptr::read(&marker.inner as *const _) };
        mem::forget(marker);
        let this_ = &self.0 as *const _;
        unsafe {
            cpp!([this_ as "chrono::ChBody*", ptr as "std::shared_ptr<chrono::ChMarker>"] {
                this_->AddMarker(ptr);
            })
        }
    }
    pub fn add_force(&self, force: Shared<Force>) {
        let ptr = unsafe { ptr::read(&force.inner as *const _) };
        mem::forget(force);
        let this_ = &self.0 as *const _;
        unsafe {
            cpp!([this_ as "chrono::ChBody*", ptr as "std::shared_ptr<chrono::ChForce>"] {
                this_->AddForce(ptr);
            })
        }
    }
    pub fn remove_all_markers(&self) {
        let this_ = &self.0 as *const _;
        unsafe {
            cpp!([this_ as "chrono::ChBody*"] {
                this_->RemoveAllMarkers();
            })
        }
    }
    pub fn remove_all_forces(&self) {
        let this_ = &self.0 as *const _;
        unsafe {
            cpp!([this_ as "chrono::ChBody*"] {
                this_->RemoveAllForces();
            })
        }
    }
    pub fn set_name<T>(&self, name: T) where T: Into<String> {
        let this_ = &self.0 as *const _;
        let name_cstr = std::ffi::CString::new(name.into()).unwrap();
        let name_ptr = name_cstr.as_ptr();
        unsafe {
            cpp!([this_ as "chrono::ChBody*", name_ptr as "const char*"] {
                this_->SetName(name_ptr);
            })
        }
    }
    pub fn set_body_fixed(&self, fixed: bool) {
        let this_ = &self.0 as *const _;
        let fixed = fixed as u8;
        unsafe {
            cpp!([this_ as "chrono::ChBody*", fixed as "uint8_t"] {
                this_->SetBodyFixed(fixed != 0);
            })
        }
    }
    pub fn set_collide(&self, collide: bool) {
        let this_ = &self.0 as *const _;
        let collide = collide as u8;
        unsafe {
            cpp!([this_ as "chrono::ChBody*", collide as "uint8_t"] {
                this_->SetCollide(collide != 0);
            })
        }
    }
    pub fn get_collision_model(&self) -> Option<Shared<CollisionModel>> {
        let this_ = &self.0 as *const _;
        Shared::maybe_null(unsafe {
            cpp!([this_ as "chrono::ChBody*"] -> Shared<CollisionModel> as "std::shared_ptr<chrono::collision::ChCollisionModel>" {
                return this_->GetCollisionModel();
            })
        })
    }
    pub fn set_pos(&self, pos: &Vector3<f64>) {
        let this_ = &self.0 as *const _;
        unsafe {
            cpp!([this_ as "chrono::ChBody*", pos as "const chrono::ChVector<double>*"] {
                this_->SetPos(*pos);
            })
        }
    }
    pub fn pos(&self) -> Vector3<f64> {
        let this_ = &self.0 as *const _;
        unsafe {
            cpp!([this_ as "chrono::ChBody*"] -> Vector3<f64> as "chrono::ChVector<double>" {
                return this_->GetPos();
            })
        }
    }
    pub fn set_material_surface<T>(&self, material: Shared<T>) where T: Shareable + Inherits<MaterialSurface> {
        let material = material.upcast::<MaterialSurface>();
        let ptr = unsafe { ptr::read(&material.inner as *const _) };
        mem::forget(material);
        let this_ = &self.0 as *const _;
        unsafe {
            cpp!([this_ as "chrono::ChBody*", ptr as "std::shared_ptr<chrono::ChMaterialSurface>"] {
                this_->SetMaterialSurface(ptr);
            })
        }
    }
    pub fn set_density(&self, density: f32) {
        let this_ = &self.0 as *const _;
        unsafe {
            cpp!([this_ as "chrono::ChBody*", density as "float"] {
                this_->SetDensity(density);
            })
        }
    }
    pub fn set_mass(&self, mass: f64) {
        let this_ = &self.0 as *const _;
        unsafe {
            cpp!([this_ as "chrono::ChBody*", mass as "double"] {
                this_->SetMass(mass);
            })
        }
    }
    pub fn set_inertia(&self, inertia_tensor: &Matrix3<f64>) {
        let this_ = &self.0 as *const _;
        let i = inertia_tensor;
        unsafe {
            cpp!([this_ as "chrono::ChBody*", i as "const double*"] {
                chrono::ChMatrix33<double> tensor(
                    i[0 + 3*0], i[0 + 3*1], i[0 + 3*2],
                    i[1 + 3*0], i[1 + 3*1], i[1 + 3*2],
                    i[2 + 3*0], i[2 + 3*1], i[2 + 3*2]
                );
                this_->SetInertia(tensor);
            })
        }
    }
    pub fn set_inertia_xx(&self, inertia: &Vector3<f64>) {
        let this_ = &self.0 as *const _;
        unsafe {
            cpp!([this_ as "chrono::ChBody*", inertia as "const chrono::ChVector<double>*"] {
                this_->SetInertiaXX(*inertia);
            })
        }
    }
}
