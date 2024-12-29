macro_rules! gen_jni_method {
    ($method_name:ident, $return_type:ty) => {
        fn $method_name(&mut self) -> $return_type {
            unsafe {
                self.functions
                    .as_ref()
                    .unwrap()
                    .$method_name
                    .unwrap()(self)
            }
        }
    };
    ($method_name:ident, $return_type:ty, $($param_name:ident: $param_type:ty),*) => {
        fn $method_name(&mut self, $($param_name: $param_type),*) -> $return_type {
            unsafe {
                self.functions
                    .as_ref()
                    .unwrap()
                    .$method_name
                    .unwrap()(self, $($param_name),*)
            }
        }
    };
}

macro_rules! c {
    ($s:expr) => {
        CString::new($s).unwrap().into_raw()
    };
}
