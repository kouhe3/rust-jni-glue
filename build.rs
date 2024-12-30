use std::env;

fn main() {
    let java_home = env::var("JAVA_HOME").expect("JAVA_HOME not set");
    println!("cargo:rustc-link-search={}lib", java_home);
    println!("cargo:rustc-link-lib=jvm");

    let bindgens = bindgen::Builder::default()
        .header("wrap.h")
        .clang_arg(format!("-I{java_home}include"))
        .clang_arg(format!("-I{java_home}include/win32"))
        .clang_arg("-x")
        .clang_arg("c++")
        .generate()
        .expect("Err bindgen");
    bindgens.write_to_file("JNI.rs").unwrap();
}
