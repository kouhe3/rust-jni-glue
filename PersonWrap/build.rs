fn main() {
    let java_home = std::env::var("JAVA_HOME").expect("JAVA_HOME not set");
    let platform = if cfg!(target_os = "windows") {
        "win32"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else {
        panic!("Not support yet");
    };
    if platform == "linux" {
        println!("cargo:rustc-link-search=native={}/lib/server", java_home);
    } else {
        println!("cargo:rustc-link-search=native={}/lib", java_home);
    }
    println!("cargo:rustc-link-lib=jvm");
}
