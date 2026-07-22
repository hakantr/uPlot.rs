use uplot_rs::{Grafik, UplotHatası, cursor_tooltip_kartı};

fn main() -> Result<(), UplotHatası> {
    let (seçenekler, veri) = cursor_tooltip_kartı()?;
    println!("{}", Grafik::yeni(seçenekler, veri)?.çiz().svg());
    Ok(())
}
