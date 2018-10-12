extern crate pkg_config;

fn main() {
    let lib = pkg_config::Config::new().statik(false).atleast_version("1.7").probe("lept").expect("Could not find leptonica 1.7+");

    // Link path
    for path in &lib.link_paths {
        println!("cargo:rustc-link-search={}", path.to_str().expect("Could not convert path to str"));
    }

    for lib in &lib.libs {
        println!("cargo:rustc-link-lib={}", lib);
    }
}