fn main() {
    // Replace with the absolute or relative path to your .lib folder
    println!("cargo:rustc-link-search=native=lib");
    
    // Tell Cargo to link against sdl3.lib
    println!("cargo:rustc-link-lib=sdl3");
}