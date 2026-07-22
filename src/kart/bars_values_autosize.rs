use super::{ortak_kart_etkileşimleri, veri_uretici::KanıtRastgele};
use crate::{
    GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası, ÇubukDüzeni, ÇubukYönü
};

pub const BARS_VALUES_AUTOSIZE_KANIT_TOHUMU: u32 = 8;

pub const BARS_VALUES_AUTOSIZE_KART_TANIM_ÖRNEĞİ: &str = r##"let (seçenekler, veri) = bars_values_autosize_kartı(ÇubukYönü::Dikey)?;
// Değer üretimi, kompakt etiket ve kullanılabilir alana göre yazı boyutu çekirdektedir.
let grafik = Grafik::yeni(seçenekler, veri)?;"##;

/// `demos/bars-values-autosize.html` içindeki dikey veya yatay grafiği üretir.
pub fn bars_values_autosize_kartı(
    yön: ÇubukYönü,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let mut rastgele = KanıtRastgele::yeni(BARS_VALUES_AUTOSIZE_KANIT_TOHUMU);
    let adet = rastgele_tamsayı(&mut rastgele, 5, 50) as usize;
    let değerler = (0..adet)
        .map(|_| Some(rastgele_tamsayı(&mut rastgele, -100_000, 100_000) as f64))
        .collect::<Vec<_>>();
    let x = (0..adet).map(|indeks| indeks as f64).collect::<Vec<_>>();
    let veri = HizalıVeri::yeni(x, vec![değerler])?;
    let yatay = yön == ÇubukYönü::Yatay;
    let kimlik = if yatay {
        "bars-values-autosize-horizontal"
    } else {
        "bars-values-autosize-vertical"
    };
    let düzen = ÇubukDüzeni::yeni(yön)
        .ters(yatay)
        .değer_etiketi_otomatik(true);
    let seçenekler = GrafikSeçenekleri::yeni(1_275, 600)?
        .başlık(kimlik)
        .x_zaman(false)
        .çubuk_düzeni(düzen)
        .etkileşimler(ortak_kart_etkileşimleri())
        .seri(
            SeriSeçenekleri::yeni("Value")
                .renk("#00000033")
                .dolgu("#00000033")
                .çizgi_kalınlığı(0.0),
        );
    Ok((seçenekler, veri))
}

fn rastgele_tamsayı(rastgele: &mut KanıtRastgele, en_az: i32, en_çok: i32) -> i32 {
    let açıklık = f64::from(en_çok - en_az + 1);
    en_az + (rastgele.sonraki() * açıklık).floor() as i32
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut};

    #[test]
    fn iki_yön_aynı_kaynak_verisini_kullanır() -> Result<(), UplotHatası> {
        let (dikey_seçenekler, dikey_veri) = bars_values_autosize_kartı(ÇubukYönü::Dikey)?;
        let (yatay_seçenekler, yatay_veri) = bars_values_autosize_kartı(ÇubukYönü::Yatay)?;
        assert_eq!(dikey_veri, yatay_veri);
        assert_eq!(dikey_veri.uzunluk(), 12);
        let seri = dikey_veri.seriler().first();
        assert!(seri.is_some());
        let Some(seri) = seri else {
            return Ok(());
        };
        assert_eq!(
            seri.as_slice(),
            [
                25_039.0, -39_867.0, 39_191.0, 17_002.0, -10_120.0, 63_752.0, -52_237.0, 1_324.0,
                -87_973.0, 2_837.0, -85_329.0, 13_891.0,
            ]
            .map(Some)
            .as_slice()
        );
        assert!(
            seri.iter()
                .flatten()
                .all(|değer| (-100_000.0..=100_000.0).contains(değer))
        );

        for (seçenekler, veri) in [
            (dikey_seçenekler, dikey_veri.clone()),
            (yatay_seçenekler, yatay_veri),
        ] {
            let sahne = Grafik::yeni(seçenekler, veri)?.çiz();
            assert!(sahne.komutlar().iter().any(|komut| {
                matches!(komut, Komut::Dikdörtgen { dolgu, .. } if dolgu == "#00ff0022")
            }));
        }
        Ok(())
    }

    #[test]
    fn ortak_yakınlaştırma_çubukları_çizim_alanında_kırpar() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = bars_values_autosize_kartı(ÇubukYönü::Dikey)?;
        let mut grafik = Grafik::yeni(seçenekler, veri)?;
        assert!(grafik.tekerlek(0.5, 0.5, 1.0, false)?);
        let (sol, sağ, üst, alt) = grafik.çizim_alanı_boyutta(1_275, 600);
        let sahne = grafik.çiz();
        assert!(sahne.komutlar().iter().all(|komut| match komut {
            Komut::Dikdörtgen {
                konum,
                genişlik,
                yükseklik,
                dolgu,
                ..
            } if dolgu == "#00000033" => {
                konum.x >= sol
                    && konum.x + genişlik <= sağ
                    && konum.y >= üst
                    && konum.y + yükseklik <= alt
            }
            _ => true,
        }));
        Ok(())
    }
}
