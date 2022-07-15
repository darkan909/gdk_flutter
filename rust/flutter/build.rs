use std::env;

fn main(){
    println!("CIAOOOOOOOOOOOOOOOOOOOOOOOOOOOOO");
    let gdk_env_name = "GDK_DIR";
    println!("GDK_ENV_NAME{}", gdk_env_name);
    env::set_var(gdk_env_name, "/Users/nathan/gdk/gdk/build-clang/");
    let gdk_dir = env::var(gdk_env_name).unwrap();
    println!("GDK_DIR{}", gdk_dir);
    println!("cargo:rerun-if-env-changed={}", gdk_env_name);

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    match target_os.as_str() {
        "android" => {
            println!("cargo:rustc-link-lib=dylib=greenaddress");
            println!("cargo:rustc-link-search=native={}", gdk_dir,);
        }
        "linux" => {
            println!("cargo:rustc-link-lib=static=greenaddress_full");
            println!("cargo:rustc-link-lib=dylib=stdc++");
            println!("cargo:rustc-link-search=native={}", gdk_dir);
        }
        "windows" => {
            println!("cargo:rustc-link-lib=static=greenaddress_full");
            println!("cargo:rustc-link-lib=static=boost_filesystem");
            println!("cargo:rustc-link-lib=dylib=stdc++.dll");
            println!("cargo:rustc-link-lib=dylib=ssp.dll");
            println!("cargo:rustc-link-lib=dylib=crypt32");
            println!("cargo:rustc-link-lib=dylib=shell32");
            println!("cargo:rustc-link-lib=dylib=iphlpapi");
            println!("cargo:rustc-link-search=native={}", gdk_dir);
            println!("cargo:rustc-link-search=native={}/../boost/build/lib", gdk_dir);
            println!("cargo:rustc-link-search=native=/usr/lib/gcc/x86_64-w64-mingw32/10-posix");
        }
        "ios" => {
            println!("cargo:rustc-link-lib=static=greenaddress_full");
            println!("cargo:rustc-link-lib=dylib=stdc++");
            println!("cargo:rustc-link-search=native={}", gdk_dir);
        }
        "macos" => {
            println!("cargo:rustc-link-lib=dylib=greenaddress");
            println!("cargo:rustc-link-lib=dylib=c++");
            println!("cargo:rustc-link-search=native={}", gdk_dir);
        }
        _ => {
            unimplemented!("unsupported target_os: {}", target_os)
        }
    }
}