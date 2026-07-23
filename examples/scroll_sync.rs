use uplot_rs::{Grafik, UplotHatası, scroll_sync_kartı};

fn main() -> Result<(), UplotHatası> {
    let (seçenekler, veri) = scroll_sync_kartı()?;
    let grafik = Grafik::yeni(seçenekler, veri)?;
    println!("{}", grafik.çiz().svg());
    Ok(())
}
