use super::ortak_kart_etkileşimleri;
use super::veri_uretici::KanıtRastgele;
use crate::{GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası};

pub const AXIS_AUTOSIZE_KANIT_TOHUMU: u32 = 0xA170_512E;
const NOKTA_SAYISI: usize = 501;

pub const AXIS_AUTOSIZE_KART_TANIM_ÖRNEĞİ: &str = r##"let çarpan = 1_000.0;
let (seçenekler, veri) = axis_autosize_kartı(çarpan)?;
// Otomatik eksen ölçümü uzun Y değerlerine ve son X etiketine yeniden yer açar.
let grafik = Grafik::yeni(seçenekler, veri)?;"##;

/// `demos/axis-autosize.html` içindeki veri formülünü ve 1…10⁹ dinamik
/// çarpan davranışını, belirlenimci görsel kanıt üretimiyle taşır.
pub fn axis_autosize_kartı(
    çarpan: f64,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    if !çarpan.is_finite() || çarpan <= 0.0 {
        return Err(UplotHatası::GeçersizÇarpan { değer: çarpan });
    }
    // Kaynak her `setData(getData(...))` çağrısında Math.random akışını
    // ilerletir. Çarpan bitlerini temel tohuma katarak her aşamada farklı,
    // fakat yeniden üretilebilir bir kanıt geometrisi elde edilir.
    let çarpan_bitleri = çarpan.to_bits();
    let aşama_tohumu =
        AXIS_AUTOSIZE_KANIT_TOHUMU ^ çarpan_bitleri as u32 ^ (çarpan_bitleri >> 32) as u32;
    let mut rastgele = KanıtRastgele::yeni(aşama_tohumu);
    let mut x = Vec::with_capacity(NOKTA_SAYISI);
    let mut y = Vec::with_capacity(NOKTA_SAYISI);
    for indeks in 0..NOKTA_SAYISI {
        let değer = indeks as f64;
        x.push(değer);
        y.push(Some(
            (rastgele.sonraki() - 0.5
                + (değer * 0.00002).sin() * 40.0
                + (değer * 0.001).sin() * 5.0
                + (değer * 0.1).sin() * 2.0)
                * çarpan,
        ));
    }
    let seçenekler = GrafikSeçenekleri::yeni(1048, 600)?
        .başlık("Axis AutoSize")
        .x_zaman(false)
        .x_eksen_etiketi("X Axis Label")
        .y_eksen_etiketi("Y Axis Label")
        .birincil_y_eksen_rengi("#ff0000")
        .x_eksen_değer_çarpanı(çarpan)
        .otomatik_x_sağ_pay(true)
        .otomatik_y_eksen_genişliği(true)
        .etkileşimler(ortak_kart_etkileşimleri())
        .seri(SeriSeçenekleri::yeni("sin(x)").renk("#ff0000"));
    Ok((seçenekler, HizalıVeri::yeni(x, vec![y])?))
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut};

    #[test]
    fn çarpan_veriyi_etiketleri_ve_eksen_paylarını_büyütür() -> Result<(), UplotHatası> {
        let (küçük_seçenekler, küçük_veri) = axis_autosize_kartı(1.0)?;
        let küçük_normalize = ilk_y(&küçük_veri);
        let küçük = Grafik::yeni(küçük_seçenekler, küçük_veri)?;
        let (büyük_seçenekler, büyük_veri) = axis_autosize_kartı(1e9)?;
        let büyük_normalize = ilk_y(&büyük_veri).map(|değer| değer / 1e9);
        let büyük = Grafik::yeni(büyük_seçenekler, büyük_veri)?;
        assert!(
            küçük_normalize
                .zip(büyük_normalize)
                .is_some_and(|(a, b)| a != b)
        );
        assert_eq!(küçük_veri_uzunluğu(&küçük), NOKTA_SAYISI);
        let (küçük_sol, küçük_sağ, _, _) = küçük.çizim_alanı_boyutta(1048, 600);
        let (büyük_sol, büyük_sağ, _, _) = büyük.çizim_alanı_boyutta(1048, 600);
        assert!(büyük_sol > küçük_sol);
        assert!(büyük_sağ < küçük_sağ);
        assert!(büyük.çiz().komutlar().iter().any(
            |komut| matches!(komut, Komut::Metin { içerik, .. } if içerik == "500000000000.00")
        ));
        Ok(())
    }

    fn ilk_y(veri: &HizalıVeri) -> Option<f64> {
        veri.seriler().first()?.first().copied().flatten()
    }

    fn küçük_veri_uzunluğu(grafik: &Grafik) -> usize {
        grafik
            .çiz()
            .komutlar()
            .iter()
            .find_map(|komut| match komut {
                Komut::Yol { parçalar, .. } => Some(parçalar.iter().map(Vec::len).sum()),
                _ => None,
            })
            .unwrap_or(0)
    }
}
