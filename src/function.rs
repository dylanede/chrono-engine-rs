use super::*;

#[repr(C)]
pub struct Function(ffi::chrono::ChFunction, NotSendSync);

impl CppClass for Function {}

unsafe impl Shareable for Function {
    type Inner = ffi::chrono::ChFunction;
    type Args = Void;

    fn make_shared_impl(args: Void) -> ffi::std::shared_ptr<Self::Inner> {
        match args {}
    }

    fn clone_impl(ptr: &ffi::std::shared_ptr<Self::Inner>) -> ffi::std::shared_ptr<Self::Inner> {
        unsafe {
            cpp!([ptr as "const std::shared_ptr<chrono::ChFunction>*"] -> ffi::std::shared_ptr<ffi::chrono::ChFunction> as "std::shared_ptr<chrono::ChFunction>" {
                return *ptr;
            })
        }
    }

    fn drop_impl(ptr: &mut ffi::std::shared_ptr<Self::Inner>) {
        unsafe {
            cpp!([ptr as "std::shared_ptr<chrono::ChFunction>*"] {
                ptr->~shared_ptr();
            })
        }
    }
}

/*fn from_closures<F,F1,F2>(f: F, df_dx: F1, ddf_dx2: F2) -> Shared<Function> {
    #[repr(C)]
    struct Header {
        ch_function: ffi::chrono::ChFunction,
        f: fn(&Header, x: f64) -> f64,
        f1: fn(&Header, x: f64) -> f64,
        f2: fn(&Header, x: f64) -> f64,
        drop: fn(&mut Header)
    }
    #[no_mangle]
    pub extern "C" fn custom_func_f(header: &Header, x: f64) -> f64 {
        (header.f)(header, x)
    }
    #[no_mangle]
    pub extern "C" fn custom_func_f1(header: &Header, x: f64) -> f64 {
        (header.f1)(header, x)
    }
    #[no_mangle]
    pub extern "C" fn custom_func_f2(header: &Header, x: f64) -> f64 {
        (header.f2)(header, x)
    }
    #[no_mangle]
    pub extern "C" fn custom_func_drop(header: &mut Header) {
        let drop = header.drop;
        drop(header)
    }
    cpp!{{
        extern "C" {
            double custom_func_f(const void*, double);
            double custom_func_f1(const void*, double);
            double custom_func_f2(const void*, double);
            void custom_func_drop(void*);
        }
        class CustomFuncWrapper : chrono::ChFunction {
            override virtual double Get_y(double x) const {
                return custom_func_f((const void*)this, x);
            }
            override virtual double Get_y_dx(double x) const {
                return custom_func_f1((const void*)this, x);
            }
            override virtual double Get_y_dxdx(double x) const {
                return custom_func_f2((const void*)this, x);
            }
            virtual ~CustomFuncWrapper() {
                custom_func_drop((void*)this);
            }
        };
    }}
    #[repr(C)]
    struct Wrapper<F, F1, F2> {
        header: Header,
        f: F,
        f1: F1,
        f2: F2
    }
}*/

#[repr(C)]
pub struct Const(ffi::chrono::ChFunction_Const, NotSendSync);

impl CppClass for Const {}

unsafe impl Inherits<Function> for Const {}

impl Deref for Const {
    type Target = Function;

    fn deref(&self) -> &Function {
        self.as_::<Function>()
    }
}

unsafe impl Shareable for Const {
    type Inner = ffi::chrono::ChFunction_Const;
    type Args = f64;

    fn make_shared_impl(value: f64) -> ffi::std::shared_ptr<Self::Inner> {
        let mut inner = unsafe { mem::zeroed::<ffi::std::shared_ptr<ffi::chrono::ChFunction_Const>>() };
        unsafe {
            cpp!([mut inner as "std::shared_ptr<chrono::ChFunction_Const>", value as "double"] {
                new(&inner) std::shared_ptr<chrono::ChFunction_Const>(std::move(std::make_shared<chrono::ChFunction_Const>(value)));
            })
        }
        inner
    }

    fn clone_impl(ptr: &ffi::std::shared_ptr<Self::Inner>) -> ffi::std::shared_ptr<Self::Inner> {
        unsafe {
            cpp!([ptr as "const std::shared_ptr<chrono::ChFunction_Const>*"] -> ffi::std::shared_ptr<ffi::chrono::ChFunction_Const> as "std::shared_ptr<chrono::ChFunction_Const>" {
                return *ptr;
            })
        }
    }

    fn drop_impl(ptr: &mut ffi::std::shared_ptr<Self::Inner>) {
        unsafe {
            cpp!([ptr as "std::shared_ptr<chrono::ChFunction_Const>*"] {
                ptr->~shared_ptr();
            })
        }
    }
}
