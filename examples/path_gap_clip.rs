use std::{fs, path::PathBuf};
use uplot_rs::{Grafik, PathGapClipÖrneği, UplotHatası, path_gap_clip_kartı};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hedef = PathBuf::from("target/path-gap-clip");
    fs::create_dir_all(&hedef)?;
    for örnek in PathGapClipÖrneği::TÜMÜ {
        let (seçenekler, veri) = path_gap_clip_kartı(örnek)?;
        let grafik = Grafik::yeni(seçenekler, veri)?;
        let dosya = hedef.join(format!("{}.svg", örnek.kimlik()));
        fs::write(dosya, grafik.çiz().svg())?;
    }
    Ok(())
}

#[allow(dead_code)]
fn tipli_kullanım() -> Result<(), UplotHatası> {
    let (seçenekler, veri) = path_gap_clip_kartı(PathGapClipÖrneği::VeriDışınaTaşanÖlçek)?;
    let _grafik = Grafik::yeni(seçenekler, veri)?;
    Ok(())
}
