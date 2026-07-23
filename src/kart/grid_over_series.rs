use super::ortak_kart_etkileşimleri;
use super::veri_uretici::KanıtRastgele;
use crate::{
    Aralık, GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası, ÇizimSırası
};

pub const GRID_OVER_SERIES_KANIT_TOHUMU: u32 = 0x6A12_0E51;

pub const GRID_OVER_SERIES_KART_TANIM_ÖRNEĞİ: &str = r##"let (seçenekler, veri) = grid_over_series_kartı()?;
// `drawOrder: ["series", "axes"]` çekirdekte ÇizimSırası ile çözülür.
let grafik = Grafik::yeni(seçenekler, veri)?;"##;

/// Resmî `demos/grid-over-series.html` grafiğini taşır. Kaynaktaki
/// `Math.random()` akışı, aynı 21 tam sayı değerinden seçim yapan sabit kanıt
/// tohumu ile yeniden üretilebilir hale getirilmiştir.
pub fn grid_over_series_kartı() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let x = (1_u32..=30).map(f64::from).collect::<Vec<_>>();
    let mut rastgele = KanıtRastgele::yeni(GRID_OVER_SERIES_KANIT_TOHUMU);
    let seriler = (0..3)
        .map(|_| {
            x.iter()
                .map(|_| Some((rastgele.sonraki() * 21.0).floor() - 10.0))
                .collect()
        })
        .collect();
    let seçenekler = GrafikSeçenekleri::yeni(1920, 600)?
        .başlık("Grid Over Series")
        .x_zaman(false)
        .y_aralığı(Aralık::yeni(-12.0, 12.0)?)
        .çizim_sırası(ÇizimSırası::SerilerEksenler)
        .ızgara_rengi("#00000033")
        .etkileşimler(ortak_kart_etkileşimleri())
        .seri(SeriSeçenekleri::yeni("1").renk("#D32F2F").dolgu("#E57373"))
        .seri(SeriSeçenekleri::yeni("2").renk("#2E7D32").dolgu("#66BB6A"))
        .seri(SeriSeçenekleri::yeni("3").renk("#1565C0").dolgu("#42A5F5"));
    Ok((seçenekler, HizalıVeri::yeni(x, seriler)?))
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut};

    #[test]
    fn kaynak_veri_üreteci_ve_renkleri_korunur() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = grid_over_series_kartı()?;
        assert_eq!(veri.x().len(), 30);
        assert_eq!(veri.seriler().len(), 3);
        assert_eq!(
            seçenekler.seriler.first().map(|seri| seri.renk.as_str()),
            Some("#D32F2F")
        );
        assert_eq!(
            seçenekler
                .seriler
                .get(1)
                .and_then(|seri| seri.dolgu.as_deref()),
            Some("#66BB6A")
        );
        assert_eq!(seçenekler.y_aralığı, Some(Aralık::yeni(-12.0, 12.0)?));
        assert!(veri.seriler().iter().flatten().all(|değer| {
            değer.is_some_and(|değer| (-10.0..=10.0).contains(&değer) && değer.fract() == 0.0)
        }));
        Ok(())
    }

    #[test]
    fn eksen_ve_ızgara_komutları_serilerin_üstündedir() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = grid_over_series_kartı()?;
        let sahne = Grafik::yeni(seçenekler, veri)?.çiz();
        let son_seri = sahne
            .komutlar()
            .iter()
            .rposition(|komut| matches!(komut, Komut::Yol { .. } | Komut::Alan { .. }));
        let ilk_ızgara = sahne
            .komutlar()
            .iter()
            .position(|komut| matches!(komut, Komut::Çizgi { renk, .. } if renk == "#00000033"));
        assert!(matches!((son_seri, ilk_ızgara), (Some(seri), Some(ızgara)) if ızgara > seri));
        Ok(())
    }
}
