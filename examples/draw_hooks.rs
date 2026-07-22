use uplot_rs::{Grafik, UplotHatası, draw_hooks_kartı};

fn main() -> Result<(), UplotHatası> {
    let (seçenekler, veri) = draw_hooks_kartı()?;
    println!("{}", Grafik::yeni(seçenekler, veri)?.çiz().svg());
    Ok(())
}
