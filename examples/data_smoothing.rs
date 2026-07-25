use uplot_rs::{Grafik, UplotHatası, data_smoothing_kartları};

fn main() -> Result<(), UplotHatası> {
    for (_örnek, seçenekler, veri) in data_smoothing_kartları()? {
        println!("{}", Grafik::yeni(seçenekler, veri)?.çiz().svg());
    }
    Ok(())
}
