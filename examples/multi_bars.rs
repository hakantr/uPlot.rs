use uplot_rs::{Grafik, MultiBarsÖrneği, UplotHatası, multi_bars_kartı};

fn main() -> Result<(), UplotHatası> {
    for örnek in MultiBarsÖrneği::TÜMÜ {
        let (seçenekler, veri) = multi_bars_kartı(örnek)?;
        let sahne = Grafik::yeni(seçenekler, veri)?.çiz();
        println!(
            "{}: {} çizim komutu",
            örnek.kimlik(),
            sahne.komutlar().len()
        );
    }
    Ok(())
}
