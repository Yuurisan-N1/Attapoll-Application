use std::env;

fn main() {
    println!("cargo:rerun-if-changed=assets/icon.ico");

    if let Ok(target_os) = env::var("CARGO_CFG_TARGET_OS") {
        if target_os == "windows" {
            let mut res = winres::WindowsResource::new();
            res.set_icon("assets/icon.ico");
            if let Err(err) = res.compile() {
                println!("cargo:warning=ikon windows gagal dipasang: {}", err);
            }
        }
    }
}
