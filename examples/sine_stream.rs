use uplot_rs::{Grafik, SineAkışı, UplotHatası};

fn main() -> Result<(), UplotHatası> {
    let akış = SineAkışı::yeni()?;
    let (seçenekler, veri) = akış.kartı()?;
    let grafik = Grafik::yeni(seçenekler, veri)?;
    println!("{}", grafik.çiz().svg());
    Ok(())
}
