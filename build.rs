fn main() {
    println!("cargo:rustc-link-search=native=vendor/SDL2-2.24.1/lib/x64");
    println!("cargo:rustc-env=PATH=vendor/SDL2-2.24.1/lib/x64");
    println!("cargo:rustc-link-lib=static=SDL2");
}
