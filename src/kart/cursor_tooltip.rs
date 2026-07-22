use super::ortak_kart_etkileşimleri;
use crate::{GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası};

pub const CURSOR_TOOLTIP_KART_TANIM_ÖRNEĞİ: &str = r##"let (seçenekler, veri) = cursor_tooltip_kartı()?;
let grafik = Grafik::yeni(seçenekler, veri)?;
// Bilgi kutusu GpuiGrafik/WASM yüzeyi tarafından otomatik yerleştirilir."##;

pub fn cursor_tooltip_kartı() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let veri = HizalıVeri::yeni(
        vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0],
        vec![
            vec![40.0, 43.0, 60.0, 65.0, 71.0, 73.0, 80.0]
                .into_iter()
                .map(Some)
                .collect(),
        ],
    )?;
    let seçenekler = GrafikSeçenekleri::yeni(600, 400)?
        .başlık("placement.js tooltip")
        .x_zaman(false)
        .etkileşimler(ortak_kart_etkileşimleri().imleç_bilgi_kutusu(true))
        .seri(SeriSeçenekleri::yeni("blah").renk("#008000"));
    Ok((seçenekler, veri))
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::Grafik;

    #[test]
    fn kaynak_verisi_ve_bilgi_kutusu_seçeneği_korunur() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = cursor_tooltip_kartı()?;
        assert_eq!(veri.x(), &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0]);
        assert_eq!(
            veri.seriler().first().and_then(|seri| seri.get(3)).copied(),
            Some(Some(65.0))
        );
        let grafik = Grafik::yeni(seçenekler, veri)?;
        assert!(grafik.etkileşim_seçenekleri().imleç_bilgi_kutusu);
        Ok(())
    }
}
