use super::ortak_kart_etkileşimleri;
use crate::{
    Aralık, GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası, ÇizimKancasıDüzeni,
};

pub const DRAW_HOOKS_KART_TANIM_ÖRNEĞİ: &str = r##"let (seçenekler, veri) = draw_hooks_kartı()?;
let grafik = Grafik::yeni(seçenekler, veri)?;"##;

pub fn draw_hooks_kartı() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let veri = HizalıVeri::yeni(
        vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 9.0, 10.0],
        vec![
            vec![40.0, 43.0, 60.0, 65.0, 71.0, 73.0, 40.0, 22.0, 20.0],
            vec![30.0, 23.0, 35.0, 27.0, 11.0, 13.0, 30.0, 32.0, 30.0],
            vec![15.0, 13.0, 39.0, 17.0, 21.0, 53.0, 10.0, 11.0, 13.0],
        ]
        .into_iter()
        .map(|seri| seri.into_iter().map(Some).collect())
        .collect(),
    )?;
    let kancalar = ÇizimKancasıDüzeni::default()
        .gradyan("#666666", "#000000")
        .seri_medyanları(50.0, 6.0)
        .yıldız_noktalar(6, 8.0, 4.0)
        .çizim_süresi_metni(true);
    let seçenekler = GrafikSeçenekleri::yeni(600, 400)?
        .başlık("Draw Hooks")
        .x_zaman(false)
        .y_aralığı(Aralık::yeni(0.0, 80.0)?)
        .ızgara_rengi("#000000")
        .çizim_kancaları(kancalar)
        .etkileşimler(ortak_kart_etkileşimleri())
        .seri(SeriSeçenekleri::yeni("blah").renk("#ff3333"))
        .seri(SeriSeçenekleri::yeni("yerp").renk("#33ccff"))
        .seri(SeriSeçenekleri::yeni("zort").renk("#ffcc33"));
    Ok((seçenekler, veri))
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut};

    #[test]
    fn kaynak_veri_ve_dört_çizim_kancası_korunur() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = draw_hooks_kartı()?;
        assert_eq!(veri.x(), &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 9.0, 10.0]);
        let sahne = Grafik::yeni(seçenekler, veri)?.çiz();
        let komutlar = sahne.komutlar();
        assert!(komutlar.iter().any(|komut| matches!(
            komut,
            Komut::Metin { içerik, renk, .. }
                if içerik == "Time to Draw: 0ms" && renk == "#ffffff"
        )));
        assert_eq!(
            komutlar
                .iter()
                .filter(|komut| matches!(komut, Komut::Alan { .. }))
                .count(),
            27
        );
        for renk in ["#ff333333", "#33ccff33", "#ffcc3333"] {
            assert!(komutlar.iter().any(|komut| matches!(
                komut,
                Komut::Çizgi { renk: çizgi, kalınlık, .. }
                    if çizgi == renk && (*kalınlık - 50.0).abs() <= f32::EPSILON
            )));
        }
        Ok(())
    }
}
