fn main() {
    // Set a custom cfg flag named 'my_custom_flag'
    println!("cargo::rustc-cfg=alt_test");
}
