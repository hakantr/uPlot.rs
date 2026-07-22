use super::{ortak_kart_etkileşimleri, veri_uretici::KanıtRastgele};
use crate::{GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası};

pub const CURSOR_BIND_KANIT_TOHUMU: u32 = 0xC0B1_1D00;

pub const CURSOR_BIND_KART_TANIM_ÖRNEĞİ: &str = r##"let (seçenekler, veri) = cursor_bind_kartı()?;
// Ctrl + sürükleme açıklama seçimi kart etkileşim seçeneğinden yüzeylere aktarılır.
let grafik = Grafik::yeni(seçenekler, veri)?;"##;

/// `demos/cursor-bind.html` grafiğini ve Ctrl + sürükleme bağını üretir.
pub fn cursor_bind_kartı() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let x = (1..=30).map(f64::from).collect::<Vec<_>>();
    let değerler = (-10..=10).map(f64::from).collect::<Vec<_>>();
    let mut rastgele = KanıtRastgele::yeni(CURSOR_BIND_KANIT_TOHUMU);
    let seriler = (0..3)
        .map(|_| {
            (0..x.len())
                .map(|_| {
                    let indeks = (rastgele.sonraki() * değerler.len() as f64).floor() as usize;
                    değerler.get(indeks).copied()
                })
                .collect::<Vec<_>>()
        })
        .collect();
    let veri = HizalıVeri::yeni(x, seriler)?;
    let etkileşimler = ortak_kart_etkileşimleri().ctrl_açıklama(true);
    let seçenekler = GrafikSeçenekleri::yeni(1_920, 600)?
        .başlık("Cursor Bind (try Ctrl + drag)")
        .x_zaman(false)
        .etkileşimler(etkileşimler)
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
    use crate::{Grafik, SeçimEylemi};

    #[test]
    fn kaynak_aralığı_ve_ctrl_bağı_korunur() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = cursor_bind_kartı()?;
        assert_eq!(veri.uzunluk(), 30);
        assert_eq!(veri.seriler().len(), 3);
        assert!(
            veri.seriler()
                .iter()
                .all(|seri| seri.iter().all(Option::is_some))
        );
        assert!(
            veri.seriler()
                .iter()
                .flatten()
                .flatten()
                .all(|değer| (-10.0..=10.0).contains(değer))
        );
        let mut grafik = Grafik::yeni(seçenekler, veri)?;
        assert!(grafik.etkileşim_seçenekleri().ctrl_açıklama);
        assert_eq!(
            grafik.seçimi_bitir(0.2, 0.6, true)?,
            SeçimEylemi::Açıklamaİstendi
        );
        assert!(!grafik.yakınlaştırılmış());
        assert_eq!(
            grafik.seçimi_bitir(0.2, 0.6, false)?,
            SeçimEylemi::Yakınlaştırıldı
        );
        assert!(grafik.yakınlaştırılmış());
        Ok(())
    }
}
