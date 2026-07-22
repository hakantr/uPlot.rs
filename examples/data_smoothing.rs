use uplot_rs::{Grafik, SmoothingÖrneği, UplotHatası, data_smoothing_kartı};

fn main() -> Result<(), UplotHatası> {
    for örnek in SmoothingÖrneği::TÜMÜ {
        let (seçenekler, veri) = data_smoothing_kartı(örnek)?;
        println!("{}", Grafik::yeni(seçenekler, veri)?.çiz().svg());
    }
    Ok(())
}
