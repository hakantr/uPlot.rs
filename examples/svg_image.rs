use std::error::Error;
use std::path::PathBuf;
use uplot_rs::svg_image_belgesi;

fn main() -> Result<(), Box<dyn Error>> {
    let çıktı = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/svg-image.svg"));
    if let Some(üst) = çıktı.parent() {
        std::fs::create_dir_all(üst)?;
    }
    std::fs::write(&çıktı, svg_image_belgesi()?)?;
    println!(
        "Bağımsız SVG görüntüsü üretildi: {}",
        çıktı.canonicalize()?.display()
    );
    Ok(())
}
