use uplot_rs::{Grafik, UplotHatası, align_data_kartları};

fn main() -> Result<(), UplotHatası> {
    let mut paneller = align_data_kartları()?
        .into_iter()
        .map(|(örnek, seçenekler, veri)| Ok((örnek, Grafik::yeni(seçenekler, veri)?)))
        .collect::<Result<Vec<_>, UplotHatası>>()?;
    let panel_sayısı = paneller.len();
    let Some((_, maliyet)) = paneller.first_mut() else {
        return Err(UplotHatası::YetersizVeri {
            uzunluk: panel_sayısı,
        });
    };
    maliyet.boşlukları_birleştir_ayarla(true);
    for (örnek, grafik) in paneller {
        println!("<!-- {} -->\n{}", örnek.başlık(), grafik.çiz().svg());
    }
    Ok(())
}
