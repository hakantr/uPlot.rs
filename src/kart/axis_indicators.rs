use super::ortak_kart_etkileşimleri;
use super::veri_uretici::KanıtRastgele;
use crate::{
    GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası, YÖlçekSeçenekleri
};

pub const AXIS_INDICATORS_KANIT_TOHUMU: u32 = 0xA815_1D1C;

pub const AXIS_INDICATORS_KART_TANIM_ÖRNEĞİ: &str = r##"let (seçenekler, veri) = axis_indicators_kartı()?;
// Üç bağımsız renkli Y ekseni ve imlece bağlı eksen rozetleri çekirdekte etkin.
let grafik = Grafik::yeni(seçenekler, veri)?;"##;

/// `demos/axis-indicators.html` verisini, üç bağımsız Y ölçeğini, ilk serinin
/// null boşluğunu ve `axisIndicsPlugin` gösterge seçeneğini taşır.
pub fn axis_indicators_kartı() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let x = (1_u32..=30).map(f64::from).collect::<Vec<_>>();
    let mut rastgele = KanıtRastgele::yeni(AXIS_INDICATORS_KANIT_TOHUMU);
    let mut seriler = (0..3)
        .map(|_| {
            x.iter()
                .map(|_| Some((rastgele.sonraki() * 21.0).floor() - 10.0))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    if let Some(ilk) = seriler.first_mut() {
        for indeks in 13..=15 {
            if let Some(değer) = ilk.get_mut(indeks) {
                *değer = None;
            }
        }
    }
    let seçenekler = GrafikSeçenekleri::yeni(1920, 600)?
        .başlık("Axis indicators")
        .x_zaman(false)
        .birincil_y_eksen_rengi("#ff0000")
        .eksen_göstergeleri(true)
        .etkileşimler(ortak_kart_etkileşimleri())
        .y_ölçeği(
            YÖlçekSeçenekleri::yeni("y2")
                .ızgara(false)
                .eksen(true)
                .eksen_rengi("#008000"),
        )
        .y_ölçeği(
            YÖlçekSeçenekleri::yeni("y3")
                .ızgara(false)
                .eksen(true)
                .eksen_rengi("#0000ff"),
        )
        .seri(
            SeriSeçenekleri::yeni("1")
                .renk("#ff0000")
                .dolgu("#ff00001a"),
        )
        .seri(
            SeriSeçenekleri::yeni("2")
                .renk("#008000")
                .dolgu("#00ff001a")
                .ölçek("y2"),
        )
        .seri(
            SeriSeçenekleri::yeni("3")
                .renk("#0000ff")
                .dolgu("#0000ff1a")
                .ölçek("y3"),
        );
    Ok((seçenekler, HizalıVeri::yeni(x, seriler)?))
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::Grafik;

    #[test]
    fn üç_eksen_null_boşluğu_ve_gösterge_seçeneği_korunur() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = axis_indicators_kartı()?;
        assert_eq!(veri.seriler().len(), 3);
        assert!(
            veri.seriler()
                .first()
                .and_then(|seri| seri.get(13..=15))
                .is_some_and(|boşluk| boşluk.iter().all(Option::is_none))
        );
        let grafik = Grafik::yeni(seçenekler, veri)?;
        assert!(grafik.eksen_göstergeleri_etkin());
        assert_eq!(
            grafik
                .seri_seçenekleri()
                .get(1)
                .map(|seri| seri.ölçek.as_str()),
            Some("y2")
        );
        assert_eq!(
            grafik
                .seri_seçenekleri()
                .get(2)
                .map(|seri| seri.ölçek.as_str()),
            Some("y3")
        );
        assert!(grafik.çizim_alanı_boyutta(1920, 600).0 > 150.0);
        Ok(())
    }
}
