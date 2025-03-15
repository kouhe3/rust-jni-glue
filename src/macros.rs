macro_rules! c {
    ($s:expr) => {
        CString::new($s).unwrap().into_raw()
    };
}
