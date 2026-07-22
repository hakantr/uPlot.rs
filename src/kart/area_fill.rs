use super::veri_uretici::KanıtRastgele;
use crate::{GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası};

pub const AREA_FILL_KANIT_TOHUMU: u32 = 0xC0DE_1234;

pub const AREA_FILL_KART_TANIM_ÖRNEĞİ: &str = r##"let (seçenekler, veri) = area_fill_kartı()?;
let grafik = Grafik::yeni(seçenekler, veri)?;"##;

/// `demos/area-fill.html` içindeki `xs`, `vals`, üç seri, stroke ve %10
/// dolguları aynı biçimde kurar. Kaynaktaki `Math.random` kanıt çekimlerinde
/// açık tohumlu eşdeğer akışla beslenir.
pub fn area_fill_kartı() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let x = (1_u32..=30).map(f64::from).collect::<Vec<_>>();
    let olası_değerler = (-10_i32..=10).map(f64::from).collect::<Vec<_>>();
    let mut rastgele = KanıtRastgele::yeni(AREA_FILL_KANIT_TOHUMU);
    let mut seriler = Vec::with_capacity(3);
    for _ in 0..3 {
        let değerler = x
            .iter()
            .map(|_| {
                let oran = rastgele.sonraki();
                let indeks = (oran * olası_değerler.len() as f64).floor() as usize;
                olası_değerler.get(indeks).copied()
            })
            .collect::<Vec<_>>();
        seriler.push(değerler);
    }

    let seçenekler = GrafikSeçenekleri::yeni(1920, 600)?
        .başlık("Area Fill")
        .x_zaman(false)
        .seri(
            SeriSeçenekleri::yeni("1")
                .renk("#ff0000")
                .dolgu("#ff00001a"),
        )
        .seri(
            SeriSeçenekleri::yeni("2")
                .renk("#008000")
                .dolgu("#00ff001a"),
        )
        .seri(
            SeriSeçenekleri::yeni("3")
                .renk("#0000ff")
                .dolgu("#0000ff1a"),
        );
    let veri = HizalıVeri::yeni(x, seriler)?;
    Ok((seçenekler, veri))
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut};

    #[test]
    fn kaynak_boyutu_verisi_ve_üç_dolgusu_korunur() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = area_fill_kartı()?;
        assert_eq!((seçenekler.genişlik, seçenekler.yükseklik), (1920, 600));
        assert_eq!(veri.x().first().copied(), Some(1.0));
        assert_eq!(veri.x().last().copied(), Some(30.0));
        assert_eq!(veri.seriler().len(), 3);
        assert!(veri.seriler().iter().all(|seri| seri.len() == 30));

        let sahne = Grafik::yeni(seçenekler, veri)?.çiz();
        let alan_sayısı = sahne
            .komutlar()
            .iter()
            .filter(|komut| matches!(komut, Komut::Alan { .. }))
            .count();
        assert_eq!(alan_sayısı, 3);
        Ok(())
    }
}
