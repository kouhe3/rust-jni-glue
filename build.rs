use std::env;

fn main() {
    let java_home = env::var("JAVA_HOME").expect("JAVA_HOME not set");
    let platform = if cfg!(target_os = "windows"){
        "win32"
    }else if cfg!(target_os = "linux") {
        "linux"
    }else{
        panic!("Not support yet");
    };
    if platform == "linux"{
        println!("cargo:rustc-link-search=native={}/lib/server", java_home);
    }else{
        println!("cargo:rustc-link-search=native={}/lib", java_home);
    }
    println!("cargo:rustc-link-lib=jvm");

    let bindgens = bindgen::Builder::default()
        .header("wrap.h")
        .clang_arg(format!("-I{java_home}/include"))
        .clang_arg(format!("-I{java_home}/include/{platform}"))
        .clang_arg("-x")
        .clang_arg("c++")
        .generate()
        .expect("Err bindgen");
    bindgens
        .write_to_file("src/JNI.rs")
        .unwrap();
}
