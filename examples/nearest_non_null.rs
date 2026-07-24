use uplot_rs::{Grafik, NearestNonNullÖrneği, UplotHatası, nearest_non_null_kartı};

fn main() -> Result<(), UplotHatası> {
    for örnek in NearestNonNullÖrneği::TÜMÜ {
        let (seçenekler, veri) = nearest_non_null_kartı(örnek)?;
        let grafik = Grafik::yeni(seçenekler, veri)?;
        let çözüm = grafik.imleç_çözümü(0.6, 960.0);
        println!("{}: {çözüm:?}", örnek.başlık());
    }
    Ok(())
}
