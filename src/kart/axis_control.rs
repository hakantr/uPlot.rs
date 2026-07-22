use super::ortak_kart_etkileşimleri;
use super::veri_uretici::KanıtRastgele;
use crate::{Aralık, GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası};

pub const AXIS_CONTROL_KANIT_TOHUMU: u32 = 0xA815_C017;
const NOKTA_SAYISI: usize = 500_001;

pub const AXIS_CONTROL_KART_TANIM_ÖRNEĞİ: &str = r##"let (seçenekler, veri) = axis_control_kartı()?;
// 500.001 kaynak noktası korunur; sahne çekirdekte piksel kovalarına
// min/max korumalı olarak seyrekleştirilir.
let grafik = Grafik::yeni(seçenekler, veri)?;"##;

/// `demos/axis-control.html` içindeki 500.001 noktalı sinyal formülünü, sabit
/// Y aralığını, sağ Y eksenini ve iki eksen etiketini taşır.
pub fn axis_control_kartı() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let mut rastgele = KanıtRastgele::yeni(AXIS_CONTROL_KANIT_TOHUMU);
    let mut x = Vec::with_capacity(NOKTA_SAYISI);
    let mut y = Vec::with_capacity(NOKTA_SAYISI);
    for indeks in 0..NOKTA_SAYISI {
        let değer = indeks as f64;
        x.push(değer);
        y.push(Some(
            rastgele.sonraki() - 0.5
                + (değer * 0.00002).sin() * 40.0
                + (değer * 0.001).sin() * 5.0
                + (değer * 0.1).sin() * 2.0,
        ));
    }
    let seçenekler = GrafikSeçenekleri::yeni(1048, 600)?
        .başlık("Axis Control")
        .x_zaman(false)
        .y_aralığı(Aralık::yeni(-50.0, 50.0)?)
        .x_eksen_etiketi("X Axis Label")
        .y_eksen_etiketi("Y Axis Label")
        .birincil_y_sağda(true)
        .birincil_y_eksen_rengi("#ff0000")
        .etkileşimler(ortak_kart_etkileşimleri())
        .seri(SeriSeçenekleri::yeni("sin(x)").renk("#ff0000"));
    Ok((seçenekler, HizalıVeri::yeni(x, vec![y])?))
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut};

    #[test]
    fn yarım_milyon_nokta_ve_eksenler_korunup_sahne_seyrekleşir() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = axis_control_kartı()?;
        assert_eq!(veri.uzunluk(), NOKTA_SAYISI);
        let sahne = Grafik::yeni(seçenekler, veri)?.çiz();
        let çizilen = sahne.komutlar().iter().find_map(|komut| match komut {
            Komut::Yol { parçalar, .. } => Some(parçalar.iter().map(Vec::len).sum::<usize>()),
            _ => None,
        });
        assert!(çizilen.is_some_and(|sayı| sayı < 5_000));
        assert!(
            sahne.komutlar().iter().any(
                |komut| matches!(komut, Komut::Metin { içerik, .. } if içerik == "X Axis Label")
            )
        );
        assert!(
            sahne.komutlar().iter().any(
                |komut| matches!(komut, Komut::Metin { içerik, .. } if içerik == "Y Axis Label")
            )
        );
        Ok(())
    }
}
