use uplot_rs::{CustomScaleÖrneği, Grafik, UplotHatası, custom_scales_kartı};

fn main() -> Result<(), UplotHatası> {
    for örnek in CustomScaleÖrneği::TÜMÜ {
        let (seçenekler, veri) = custom_scales_kartı(örnek)?;
        println!("{}", Grafik::yeni(seçenekler, veri)?.çiz().svg());
    }
    Ok(())
}
