use super::{ortak_kart_etkileşimleri, veri_uretici::KanıtRastgele};
use crate::{
    GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası, ÇubukDüzeni, ÇubukYönü
};

pub const BARS_VALUES_AUTOSIZE_KANIT_TOHUMU: u32 = 8;

pub const BARS_VALUES_AUTOSIZE_KART_TANIM_ÖRNEĞİ: &str = r##"let (seçenekler, veri) = bars_values_autosize_kartı(ÇubukYönü::Dikey)?;
// Değer üretimi, kompakt etiket ve kullanılabilir alana göre yazı boyutu çekirdektedir.
let grafik = Grafik::yeni(seçenekler, veri)?;

// Kaynak sayfadaki dikey ve yatay bağımsız yüzeyleri birlikte kurar.
let yüzeyler = bars_values_autosize_kartları()?;"##;

/// Resmî sayfadaki dikey ve yatay yüzeyi kaynak sırasıyla döndürür.
pub fn bars_values_autosize_kartları()
-> Result<Vec<(ÇubukYönü, GrafikSeçenekleri, HizalıVeri)>, UplotHatası> {
    [ÇubukYönü::Dikey, ÇubukYönü::Yatay]
        .into_iter()
        .map(|yön| {
            bars_values_autosize_kartı(yön).map(|(seçenekler, veri)| (yön, seçenekler, veri))
        })
        .collect()
}

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
    let düzen = ÇubukDüzeni::yeni(yön)
        .ters(yatay)
        .değer_etiketi_otomatik(true);
    let seçenekler = GrafikSeçenekleri::yeni(1_275, 600)?
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
        let kartlar = bars_values_autosize_kartları()?;
        assert_eq!(kartlar.len(), 2);
        let dikey = kartlar
            .first()
            .ok_or_else(|| UplotHatası::GeçersizKaynakVeri {
                varlık: "bars-values-autosize",
                açıklama: "kaynak dikey yüzeyi eksik".to_string(),
            })?;
        let yatay = kartlar
            .get(1)
            .ok_or_else(|| UplotHatası::GeçersizKaynakVeri {
                varlık: "bars-values-autosize",
                açıklama: "kaynak yatay yüzeyi eksik".to_string(),
            })?;
        assert_eq!(dikey.0, ÇubukYönü::Dikey);
        assert_eq!(yatay.0, ÇubukYönü::Yatay);
        assert!(
            kartlar
                .iter()
                .all(|(_, seçenekler, _)| seçenekler.başlık.is_empty())
        );
        let (_, dikey_seçenekler, dikey_veri) = dikey.clone();
        let (_, yatay_seçenekler, yatay_veri) = yatay.clone();
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

    #[test]
    fn kaynak_set_data_önbelleği_ve_ortak_font_kararı_korunur() -> Result<(), UplotHatası> {
        for yön in [ÇubukYönü::Dikey, ÇubukYönü::Yatay] {
            let (seçenekler, veri) = bars_values_autosize_kartı(yön)?;
            let mut grafik = Grafik::yeni(seçenekler, veri)?;
            let sahne = grafik.çiz_boyutta(1_275, 600, None);
            let yazı_boyutları = sahne
                .komutlar()
                .iter()
                .filter_map(|komut| match komut {
                    Komut::Metin {
                        içerik,
                        renk,
                        boyut,
                        ..
                    } if renk == "#111111"
                        && (içerik.contains('K')
                            || içerik.contains('M')
                            || içerik.parse::<f64>().is_ok()) =>
                    {
                        Some(*boyut)
                    }
                    _ => None,
                })
                .collect::<Vec<_>>();
            // Kaynak eklenti tek bir ortak font kararı verir; uç değerlerin
            // etiketleri çizim sınırında kalırsa ilgili metin ayrıca kırpılabilir.
            assert!(!yazı_boyutları.is_empty());
            let ortak_boyut = yazı_boyutları.first().copied().unwrap_or_default();
            assert!(yazı_boyutları.iter().all(|boyut| {
                (10.0..=25.0).contains(boyut) && (*boyut - ortak_boyut).abs() <= f32::EPSILON
            }));

            let yeni_veri = HizalıVeri::yeni(
                vec![0.0, 1.0, 2.0],
                vec![vec![Some(99_000.0), Some(-1_250.0), Some(42.0)]],
            )?;
            grafik.canlı_veriyi_ayarla(yeni_veri)?;
            let güncel = grafik.çiz();
            assert!(
                güncel
                    .komutlar()
                    .iter()
                    .any(|komut| matches!(komut, Komut::Metin { içerik, renk, .. }
                    if renk == "#111111" && içerik == "42"))
            );
            assert!(
                !güncel
                    .komutlar()
                    .iter()
                    .any(|komut| matches!(komut, Komut::Metin { içerik, renk, .. }
                    if renk == "#111111" && içerik == "25K"))
            );
        }
        Ok(())
    }
}
