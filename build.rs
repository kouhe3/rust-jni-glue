use std::{env, path::PathBuf};

fn main() {
    println!(r"cargo:rustc-link-search=C:\Program Files\Zulu\zulu-21\lib");
    println!("cargo:rustc-link-lib=jvm");

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
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindgens
        .write_to_file(out_path.join("bindgens.rs"))
        .unwrap();
}
