use std::path::PathBuf;

use uplot_rs::{BoyutSenkronAkışı, Grafik, update_cursor_select_resize_kartı};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let çıktı = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/update-cursor-select-resize.svg"));
    let akış = BoyutSenkronAkışı::yeni();
    let (seçenekler, veri) = update_cursor_select_resize_kartı(akış.boyut())?;
    let grafik = Grafik::yeni(seçenekler, veri)?;
    std::fs::write(çıktı, grafik.çiz().svg())?;
    Ok(())
}
