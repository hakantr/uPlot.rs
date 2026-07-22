use super::ortak_kart_etkileşimleri;
use super::veri_uretici::KanıtRastgele;
use crate::{GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası};

pub const CURSOR_SNAP_KANIT_TOHUMU: u32 = 0x534E_4150;

pub const CURSOR_SNAP_KART_TANIM_ÖRNEĞİ: &str = r##"let (seçenekler, veri) = cursor_snap_kartı()?;
// 10×10 piksel imleç ızgarası çekirdekte tanımlıdır; GPUI ve WASM
// yüzeylerinin ayrıca yuvarlama kodu yazması gerekmez.
let grafik = Grafik::yeni(seçenekler, veri)?;"##;

/// Resmî `demos/cursor-snap.html` verisini, üç serisini, dolgu renklerini ve
/// `cursor.move` içindeki 10×10 piksel yuvarlama davranışını taşır.
pub fn cursor_snap_kartı() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let x = (1_u32..=30).map(f64::from).collect::<Vec<_>>();
    let mut rastgele = KanıtRastgele::yeni(CURSOR_SNAP_KANIT_TOHUMU);
    let seriler = (0..3)
        .map(|_| {
            x.iter()
                .map(|_| Some((rastgele.sonraki() * 21.0).floor() - 10.0))
                .collect()
        })
        .collect();
    let seçenekler = GrafikSeçenekleri::yeni(1920, 600)?
        .başlık("Cursor Snap (to 10x10 grid)")
        .x_zaman(false)
        .imleç_ızgara_adımı(10.0)
        .etkileşimler(ortak_kart_etkileşimleri())
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
    Ok((seçenekler, HizalıVeri::yeni(x, seriler)?))
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::Grafik;

    #[test]
    fn kaynak_serileri_ve_on_piksel_ızgarası_korunur() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = cursor_snap_kartı()?;
        assert_eq!(veri.seriler().len(), 3);
        assert_eq!(seçenekler.imleç_ızgara_adımı, Some(10.0));
        let grafik = Grafik::yeni(seçenekler, veri)?;
        assert_eq!(
            grafik.imleç_oranlarını_uyarla(0.14, 0.16, 100.0, 100.0),
            Some((0.1, 0.2))
        );
        Ok(())
    }
}
