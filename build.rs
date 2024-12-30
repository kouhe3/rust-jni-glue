use std::env;

fn main() {
    let java_home = env::var("JAVA_HOME").expect("JAVA_HOME not set");
    println!("cargo:rustc-link-search={}/lib",java_home);
    println!("cargo:rustc-link-lib=jvm");
    /*
    let libclang_path = r"C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\Llvm\x64\bin\libclang.dll";
    unsafe {
        env::set_var("LIBCLANG_PATH", libclang_path);
    }
    let bindgens = bindgen::Builder::default()
    .header("wrap.h")
    .clang_arg(r"-IC:\Program Files\Zulu\zulu-21\include")
    .clang_arg(r"-IC:\Program Files\Zulu\zulu-21\include\win32")
    .clang_arg("-x")
    .clang_arg("c++")
    .generate()
    .expect("Err bindgen");
    bindgens
    .write_to_file("JNI.rs")
    .unwrap();
        */
}
