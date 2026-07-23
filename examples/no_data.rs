use std::{fs, path::PathBuf};
use uplot_rs::{Grafik, NoDataÖrneği, UplotHatası, no_data_kartı};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hedef = PathBuf::from("target/no-data");
    fs::create_dir_all(&hedef)?;
    for örnek in NoDataÖrneği::TÜMÜ {
        let (seçenekler, veri) = no_data_kartı(örnek)?;
        let grafik = Grafik::yeni(seçenekler, veri)?;
        let dosya = hedef.join(format!("{}.svg", örnek.kimlik()));
        fs::write(dosya, grafik.çiz().svg())?;
    }
    Ok(())
}

#[allow(dead_code)]
fn tipli_kullanım() -> Result<(), UplotHatası> {
    let (seçenekler, veri) = no_data_kartı(NoDataÖrneği::BOŞ_ÖZEL_ARALIK)?;
    let _grafik = Grafik::yeni(seçenekler, veri)?;
    Ok(())
}
