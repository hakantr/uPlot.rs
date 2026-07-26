use super::{ortak_kart_etkileşimleri, veri_uretici::KanıtRastgele};
use crate::{
    GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası, İmleçBağSeçenekleri
};

pub const CURSOR_BIND_KANIT_TOHUMU: u32 = 0xC0B1_1D00;

pub const CURSOR_BIND_KART_TANIM_ÖRNEĞİ: &str = r##"let (seçenekler, veri) = cursor_bind_kartı()?;
// cursor.bind: birincil tuş filtresi, click iletimi ve Ctrl seçim politikası.
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
    let etkileşimler =
        ortak_kart_etkileşimleri().imleç_bağları(İmleçBağSeçenekleri::cursor_bind());
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
        assert_eq!(
            grafik.etkileşim_seçenekleri().imleç_bağları,
            İmleçBağSeçenekleri::cursor_bind()
        );
        assert_eq!(
            grafik
                .seri_seçenekleri()
                .iter()
                .map(|seri| (seri.renk.as_str(), seri.dolgu.as_deref()))
                .collect::<Vec<_>>(),
            vec![
                ("#ff0000", Some("#ff00001a")),
                ("#008000", Some("#00ff001a")),
                ("#0000ff", Some("#0000ff1a")),
            ]
        );
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

        let (seçenekler, veri) = cursor_bind_kartı()?;
        let mut sıfır_eşik = Grafik::yeni(seçenekler, veri)?;
        assert_eq!(
            sıfır_eşik.seçimi_bitir(0.5, 0.500_001, false)?,
            SeçimEylemi::Yakınlaştırıldı
        );
        Ok(())
    }
}
