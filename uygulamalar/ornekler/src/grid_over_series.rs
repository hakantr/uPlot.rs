use super::ortak_kart_etkileşimleri;
use super::veri_uretici::KanıtRastgele;
use crate::{GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası, ÇizimKatmanı};

pub const GRID_OVER_SERIES_KANIT_TOHUMU: u32 = 0x6A12_0E51;

pub const GRID_OVER_SERIES_KART_TANIM_ÖRNEĞİ: &str = r##"let (seçenekler, veri) = grid_over_series_kartı()?;
// Özel bir kart API'si yoktur; aynı genel katman sırası tüm grafiklerde kullanılabilir.
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
        .katman_sırası([
            ÇizimKatmanı::ArkaPlan,
            ÇizimKatmanı::Veri,
            ÇizimKatmanı::IzgaraEksen,
            ÇizimKatmanı::Bilgi,
        ])
        .ızgara_rengi("#00000033")
        .eksen_göstergeleri(true)
        .x_eksen_rengi("#000000")
        .x_eksen_çentik_rengi("#00000033")
        .birincil_y_eksen_rengi("#000000")
        .birincil_y_eksen_çentik_rengi("#00000033")
        .etkileşimler(ortak_kart_etkileşimleri())
        .seri(SeriSeçenekleri::yeni("1").renk("#D32F2F").dolgu("#E57373"))
        .seri(SeriSeçenekleri::yeni("2").renk("#2E7D32").dolgu("#66BB6A"))
        .seri(SeriSeçenekleri::yeni("3").renk("#1565C0").dolgu("#42A5F5"));
    Ok((seçenekler, HizalıVeri::yeni(x, seriler)?))
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Aralık, Grafik, Komut};

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
        assert_eq!(seçenekler.y_aralığı, None);
        assert!(seçenekler.eksen_göstergeleri);
        assert_eq!(seçenekler.x_eksen_rengi, "#000000");
        assert_eq!(
            seçenekler.x_eksen_çentik_rengi.as_deref(),
            Some("#00000033")
        );
        assert_eq!(
            seçenekler.birincil_y_eksen_çentik_rengi.as_deref(),
            Some("#00000033")
        );
        assert!(veri.seriler().iter().flatten().all(|değer| {
            değer.is_some_and(|değer| (-10.0..=10.0).contains(&değer) && değer.fract() == 0.0)
        }));
        let (_, ikinci_veri) = grid_over_series_kartı()?;
        assert_eq!(veri, ikinci_veri);
        assert_eq!(
            veri.seriler().first().and_then(|seri| seri.get(..10)),
            Some(
                [
                    Some(-2.0),
                    Some(-8.0),
                    Some(-8.0),
                    Some(0.0),
                    Some(-8.0),
                    Some(-2.0),
                    Some(-8.0),
                    Some(10.0),
                    Some(10.0),
                    Some(-6.0),
                ]
                .as_slice()
            )
        );
        Ok(())
    }

    #[test]
    fn eksen_ızgara_çentik_ve_etiketleri_serilerin_üstündedir() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = grid_over_series_kartı()?;
        let mut grafik = Grafik::yeni(seçenekler, veri)?;
        assert_eq!(grafik.görünür_y_aralığı(), Aralık::yeni(-12.0, 12.0)?);
        assert!(grafik.görünür_x_aralığını_ayarla(Aralık::yeni(10.0, 13.0)?, true));
        assert_eq!(grafik.görünür_y_aralığı(), Aralık::yeni(-7.5, 10.5)?);
        let sahne = grafik.çiz();
        let Some(son_seri) = sahne.komutlar().iter().rposition(|komut| {
            matches!(
                komut,
                Komut::Yol { .. }
                    | Komut::Alan { .. }
                    | Komut::Daire { .. }
                    | Komut::Daireler { .. }
            )
        }) else {
            return Err(UplotHatası::YetersizVeri { uzunluk: 0 });
        };
        let eksen_çizgileri = sahne
            .komutlar()
            .iter()
            .enumerate()
            .filter_map(|(indeks, komut)| {
                matches!(
                    komut,
                    Komut::Çizgi { renk, .. } if renk == "#00000033" || renk == "#000000"
                )
                .then_some(indeks)
            })
            .collect::<Vec<_>>();
        assert!(!eksen_çizgileri.is_empty());
        assert!(eksen_çizgileri.iter().all(|indeks| *indeks > son_seri));
        let sayısal_etiketler = sahne
            .komutlar()
            .iter()
            .enumerate()
            .filter_map(|(indeks, komut)| match komut {
                Komut::Metin { içerik, .. } if içerik.parse::<f64>().is_ok() => Some(indeks),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert!(!sayısal_etiketler.is_empty());
        assert!(sayısal_etiketler.iter().all(|indeks| *indeks > son_seri));
        Ok(())
    }
}
