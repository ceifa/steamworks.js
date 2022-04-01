extern crate napi_build;

fn main() {
    napi_build::setup();

    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");
}
