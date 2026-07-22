use uplot_rs::{Grafik, UplotHatası, align_data_maliyet_kartı, align_data_çizgi_çubuk_kartı};

fn main() -> Result<(), UplotHatası> {
    let (seçenekler, veri) = align_data_maliyet_kartı()?;
    let mut maliyet = Grafik::yeni(seçenekler, veri)?;
    maliyet.boşlukları_birleştir_ayarla(true);
    println!("{}", maliyet.çiz().svg());

    let (seçenekler, veri) = align_data_çizgi_çubuk_kartı()?;
    println!("{}", Grafik::yeni(seçenekler, veri)?.çiz().svg());
    Ok(())
}
