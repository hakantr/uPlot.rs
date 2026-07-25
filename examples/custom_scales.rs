use uplot_rs::{Grafik, UplotHatası, custom_scales_kartları};

fn main() -> Result<(), UplotHatası> {
    for (örnek, seçenekler, veri) in custom_scales_kartları()? {
        eprintln!("{}", örnek.başlık());
        println!("{}", Grafik::yeni(seçenekler, veri)?.çiz().svg());
    }
    Ok(())
}
