pub fn main() {
    cc::Build::new()
        .file("fstapi/fastlz.c")
        .file("fstapi/lz4.c")
        .file("fstapi/fstapi.c")
        .compile("fstapi");
}
