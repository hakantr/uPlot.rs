use std::collections::BTreeSet;

use super::{ortak_kart_etkileşimleri, veri_uretici::KanıtRastgele};
use crate::{
    Aralık, BoşlukKipi, GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası,
    hizalı_verileri_birleştir,
};

pub const ALIGN_DATA_KANIT_TOHUMU: u32 = 0xA119_DA7A;

pub const ALIGN_DATA_KART_TANIM_ÖRNEĞİ: &str = r##"let (seçenekler, veri) = align_data_maliyet_kartı()?;
let mut grafik = Grafik::yeni(seçenekler, veri)?;
grafik.boşlukları_birleştir_ayarla(true);

let (karma_seçenekler, karma_veri) = align_data_çizgi_çubuk_kartı()?;
let karma_grafik = Grafik::yeni(karma_seçenekler, karma_veri)?;"##;

fn rastgele_tamsayı(rastgele: &mut KanıtRastgele, en_az: u32, en_çok: u32) -> u32 {
    let uzunluk = en_çok.saturating_sub(en_az).saturating_add(1);
    en_az.saturating_add((rastgele.sonraki() * f64::from(uzunluk)).floor() as u32)
}

fn kaynak_tabloları() -> Result<Vec<HizalıVeri>, UplotHatası> {
    let mut rastgele = KanıtRastgele::yeni(ALIGN_DATA_KANIT_TOHUMU);
    let mut tablolar = Vec::with_capacity(5);
    for _ in 0..5 {
        let mut x_kümesi = BTreeSet::new();
        while x_kümesi.len() < 1_000 {
            x_kümesi.insert(rastgele_tamsayı(&mut rastgele, 0, 100_000));
        }
        let x = x_kümesi.into_iter().map(f64::from).collect::<Vec<_>>();
        let seriler = (0..5)
            .map(|_| {
                x.iter()
                    .map(|_| {
                        let değer = rastgele_tamsayı(&mut rastgele, 0, 100);
                        (!değer.is_multiple_of(5)).then_some(f64::from(değer))
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        tablolar.push(HizalıVeri::yeni(x, seriler)?);
    }
    Ok(tablolar)
}

/// Kaynaktaki 5 tablo × 5 seri × 1000 nokta hizalama maliyeti grafiği.
pub fn align_data_maliyet_kartı() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let tablolar = kaynak_tabloları()?;
    let kipler = vec![vec![BoşlukKipi::Genişlet; 5]; 5];
    let veri = hizalı_verileri_birleştir(&tablolar, Some(&kipler))?;
    let renkler = [
        ("Red", "#ff0000", "#ff00001a"),
        ("Green", "#008000", "#00ff001a"),
        ("Blue", "#0000ff", "#0000ff1a"),
    ];
    let mut seçenekler = GrafikSeçenekleri::yeni(2_560, 600)?
        .x_zaman(false)
        .etkileşimler(ortak_kart_etkileşimleri());
    for seri_indeksi in 0..veri.seriler().len() {
        let seri = renkler.get(seri_indeksi).map_or_else(
            || SeriSeçenekleri::yeni(format!("Gizli {}", seri_indeksi + 1)).göster(false),
            |(etiket, renk, dolgu)| SeriSeçenekleri::yeni(*etiket).renk(*renk).dolgu(*dolgu),
        );
        seçenekler = seçenekler.seri(seri);
    }
    Ok((seçenekler, veri))
}

/// Kaynaktaki yoğun kırmızı çizgi + seyrek mavi bar `join()` örneği.
pub fn align_data_çizgi_çubuk_kartı() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let yoğun_y = [10.0, 20.0]
        .into_iter()
        .cycle()
        .take(38)
        .map(Some)
        .collect::<Vec<_>>();
    let seyrek_y = vec![Some(20.0), Some(10.0), Some(20.0), Some(10.0)];
    let yoğun_x = (0..yoğun_y.len())
        .map(|indeks| indeks as f64 / yoğun_y.len() as f64 * 100.0)
        .collect();
    let seyrek_x = (0..seyrek_y.len())
        .map(|indeks| indeks as f64 / seyrek_y.len() as f64 * 100.0)
        .collect();
    let tablolar = [
        HizalıVeri::yeni(yoğun_x, vec![yoğun_y])?,
        HizalıVeri::yeni(seyrek_x, vec![seyrek_y])?,
    ];
    let veri = hizalı_verileri_birleştir(&tablolar, None)?;
    let seçenekler = GrafikSeçenekleri::yeni(1_920, 600)?
        .başlık("Aligned Line + Bars")
        .x_zaman(false)
        .y_aralığı(Aralık::yeni(0.0, 20.0)?)
        .etkileşimler(ortak_kart_etkileşimleri())
        .seri(SeriSeçenekleri::yeni("Dense").renk("#ff0000"))
        .seri(
            SeriSeçenekleri::yeni("Sparse Bars")
                .renk("#0000ff")
                .dolgu("#0000ff1a")
                .çubuk(true),
        );
    Ok((seçenekler, veri))
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut};

    #[test]
    fn kaynak_hizalama_boyutu_ve_null_expand_korunur() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = align_data_maliyet_kartı()?;
        assert_eq!(veri.seriler().len(), 25);
        assert!((4_000..=5_000).contains(&veri.uzunluk()));
        assert!(veri.seriler().iter().flatten().any(Option::is_none));
        let mut grafik = Grafik::yeni(seçenekler, veri)?;
        let ayrı = grafik.çiz();
        assert!(grafik.boşlukları_birleştir_ayarla(true));
        assert_ne!(grafik.çiz(), ayrı);
        Ok(())
    }

    #[test]
    fn ikinci_kart_cizgi_ve_dort_cubuk_uretir() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = align_data_çizgi_çubuk_kartı()?;
        let sahne = Grafik::yeni(seçenekler, veri)?.çiz();
        assert!(
            sahne
                .komutlar()
                .iter()
                .any(|komut| matches!(komut, Komut::Yol { .. }))
        );
        assert_eq!(
            sahne
                .komutlar()
                .iter()
                .filter(
                    |komut| matches!(komut, Komut::Dikdörtgen { dolgu, .. } if dolgu == "#0000ff1a")
                )
                .count(),
            4
        );
        Ok(())
    }
}
