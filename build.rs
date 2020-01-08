pub fn main() {
    println!("cargo:rustc-link-lib=z");
    cc::Build::new()
        .files(&["fstapi/fastlz.c", "fstapi/lz4.c", "fstapi/fstapi.c"])
        .compile("fstapi");
}
