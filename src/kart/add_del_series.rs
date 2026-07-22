use super::{ortak_kart_etkileşimleri, veri_uretici::KanıtRastgele};
use crate::{GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası};

pub const ADD_DEL_SERIES_KANIT_TOHUMU: u32 = 0xADDE_1500;

pub const ADD_DEL_SERIES_KART_TANIM_ÖRNEĞİ: &str = r##"let (seçenekler, veri) = add_del_series_kartı()?;
let mut grafik = Grafik::yeni(seçenekler, veri)?;
grafik.seri_ekle(
    1,
    SeriSeçenekleri::yeni("Orange").renk("#ffa500").dolgu("#ffa5001a"),
    add_del_series_ek_verisi(0),
)?;
grafik.seri_sil(1)?;"##;

fn seri_üret(rastgele: &mut KanıtRastgele) -> Vec<Option<f64>> {
    let değerler = (-10..=10).map(f64::from).collect::<Vec<_>>();
    (0..30)
        .map(|_| {
            let indeks = (rastgele.sonraki() * değerler.len() as f64).floor() as usize;
            değerler.get(indeks).copied()
        })
        .collect()
}

/// Resmî demonun her `Add Series` tıklamasında ürettiği turuncu seri için
/// belirlenimci kanıt akışı sağlar.
pub fn add_del_series_ek_verisi(ekleme_sırası: u32) -> Vec<Option<f64>> {
    let tohum = ADD_DEL_SERIES_KANIT_TOHUMU
        .wrapping_add(0x9E37_79B9_u32.wrapping_mul(ekleme_sırası.wrapping_add(1)));
    seri_üret(&mut KanıtRastgele::yeni(tohum))
}

/// `demos/add-del-series.html` başlangıç grafiğini üretir.
pub fn add_del_series_kartı() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let x = (1..=30).map(f64::from).collect::<Vec<_>>();
    let mut rastgele = KanıtRastgele::yeni(ADD_DEL_SERIES_KANIT_TOHUMU);
    let veri = HizalıVeri::yeni(x, (0..3).map(|_| seri_üret(&mut rastgele)).collect())?;
    let seçenekler = GrafikSeçenekleri::yeni(1_920, 600)?
        .başlık("Area Fill")
        .x_zaman(false)
        .etkileşimler(ortak_kart_etkileşimleri())
        .seri(
            SeriSeçenekleri::yeni("Red")
                .renk("#ff0000")
                .dolgu("#ff00001a"),
        )
        .seri(
            SeriSeçenekleri::yeni("Green")
                .renk("#008000")
                .dolgu("#00ff001a"),
        )
        .seri(
            SeriSeçenekleri::yeni("Blue")
                .renk("#0000ff")
                .dolgu("#0000ff1a"),
        );
    Ok((seçenekler, veri))
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::Grafik;

    #[test]
    fn kaynak_serileri_atomik_eklenip_silinir() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = add_del_series_kartı()?;
        let mut grafik = Grafik::yeni(seçenekler, veri)?;
        assert_eq!(grafik.seri_seçenekleri().len(), 3);
        assert!(grafik.seçim_yakınlaştır(0.2, 0.8)?);
        assert!(grafik.yakınlaştırılmış());
        let (_, yenilenen_veri) = add_del_series_kartı()?;
        grafik.veriyi_ayarla(yenilenen_veri)?;
        assert!(!grafik.yakınlaştırılmış());
        grafik.seri_ekle(
            1,
            SeriSeçenekleri::yeni("Orange")
                .renk("#ffa500")
                .dolgu("#ffa5001a"),
            add_del_series_ek_verisi(0),
        )?;
        assert_eq!(grafik.seri_seçenekleri().len(), 4);
        assert_eq!(
            grafik
                .seri_seçenekleri()
                .get(1)
                .map(|seri| seri.etiket.as_str()),
            Some("Orange")
        );
        assert!(grafik.çiz().svg().contains("#ffa500"));
        grafik.seri_sil(1)?;
        assert_eq!(grafik.seri_seçenekleri().len(), 3);
        assert!(matches!(
            grafik.seri_sil(9),
            Err(UplotHatası::GeçersizSeriİndeksi { .. })
        ));
        assert!(matches!(
            grafik.seri_ekle(
                9,
                SeriSeçenekleri::yeni("Geçersiz"),
                add_del_series_ek_verisi(1),
            ),
            Err(UplotHatası::GeçersizSeriİndeksi { .. })
        ));
        Ok(())
    }
}
