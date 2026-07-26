fn main() {
    println!("cargo:rerun-if-changed=../../assets/windows/uplot-rs.rc");
    println!("cargo:rerun-if-changed=../../assets/icons/uplot-rs.ico");

    if std::env::var("CARGO_CFG_TARGET_FAMILY").as_deref() == Ok("wasm") {
        return;
    }

    let sonuç = embed_resource::compile("../../assets/windows/uplot-rs.rc", embed_resource::NONE)
        .manifest_required();
    if let Err(hata) = sonuç {
        eprintln!("Windows uygulama ikonu EXE dosyasına eklenemedi: {hata}");
        std::process::exit(1);
    }
}
