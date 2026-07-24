use super::{ortak_kart_etkileşimleri, veri_uretici::KanıtRastgele};
use crate::{
    AçıklamaDüzeni, AçıklamaHizası, AçıklamaStili, Açıklamaİşareti, GrafikSeçenekleri, HizalıVeri,
    SeriSeçenekleri, UplotHatası,
};

pub const ANNOTATIONS_KANIT_TOHUMU: u32 = 0x41_4E_4E_4F;

pub const ANNOTATIONS_KART_TANIM_ÖRNEĞİ: &str = r##"let (seçenekler, veri) = annotations_kartı()?;
let grafik = Grafik::yeni(seçenekler, veri)?;"##;

pub fn annotations_kartı() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let mut rastgele = KanıtRastgele::yeni(ANNOTATIONS_KANIT_TOHUMU);
    let seriler = (0..2)
        .map(|_| {
            (0..30)
                .map(|_| Some((rastgele.sonraki() * 21.0).floor() - 10.0))
                .collect()
        })
        .collect();
    let veri = HizalıVeri::yeni((1..=30).map(f64::from).collect(), seriler)?;

    let açıklamalar = AçıklamaDüzeni::default()
        .stil(AçıklamaStili::yeni(
            "tor",
            "rgb(255 193 7)",
            "rgb(255 193 7 / 20%)",
            AçıklamaHizası::Üst,
        ))
        .stil(AçıklamaStili::yeni(
            "eqk",
            "rgb(76 175 80)",
            "rgb(76 175 80 / 20%)",
            AçıklamaHizası::Alt,
        ))
        .işaret(
            Açıklamaİşareti::yeni("eqk", 4.0, 4.0, "eqk_01")
                .açıklama("Earthquake 01!")
                .bağlantı("https://google.com/"),
        )
        .işaret(
            Açıklamaİşareti::yeni("tor", 9.3, 18.5, "tor_20")
                .açıklama("Tornado 20!")
                .bağlantı("https://google.com/"),
        );

    let seçenekler = GrafikSeçenekleri::yeni(1_920, 600)?
        .başlık("Annotations")
        .x_zaman(false)
        .açıklamalar(açıklamalar)
        .etkileşimler(ortak_kart_etkileşimleri())
        .seri(
            SeriSeçenekleri::yeni("Series 1")
                .renk("red")
                .dolgu("rgba(255,0,0,0.1)"),
        )
        .seri(
            SeriSeçenekleri::yeni("Series 2")
                .renk("blue")
                .dolgu("rgba(0,0,255,0.1)"),
        );

    Ok((seçenekler, veri))
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut};

    fn test_hatası(açıklama: &str) -> UplotHatası {
        UplotHatası::GeçersizKaynakVeri {
            varlık: "annotations testi",
            açıklama: açıklama.to_string(),
        }
    }

    #[test]
    fn kaynak_veri_ve_annotation_modeli_korunur() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = annotations_kartı()?;
        assert_eq!(veri.x(), (1..=30).map(f64::from).collect::<Vec<_>>());
        assert_eq!(veri.seriler().len(), 2);
        assert!(veri.seriler().iter().flatten().all(|değer| {
            değer.is_some_and(|değer| (-10.0..=10.0).contains(&değer) && değer.fract() == 0.0)
        }));
        let düzen = seçenekler
            .açıklama_düzeni
            .as_ref()
            .ok_or_else(|| test_hatası("açıklama düzeni bulunamadı"))?;
        assert_eq!(düzen.stiller.len(), 2);
        assert_eq!(düzen.işaretler.len(), 2);
        let [deprem, tornado] = düzen.işaretler.as_slice() else {
            return Err(test_hatası("iki kaynak işareti bekleniyordu"));
        };
        assert_eq!(deprem.başlangıç, deprem.bitiş);
        assert_eq!(tornado.başlangıç, 9.3);
        assert_eq!(tornado.bitiş, 18.5);
        assert_eq!(tornado.açıklama, "Tornado 20!");
        Ok(())
    }

    #[test]
    fn tam_görünüm_tekil_ve_aralık_işaretlerini_çizer() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = annotations_kartı()?;
        let grafik = Grafik::yeni(seçenekler, veri)?;
        let sahne = grafik.çiz();
        let komutlar = sahne.komutlar();
        assert!(
            komutlar
                .iter()
                .any(|komut| matches!(komut, Komut::Metin { içerik, .. } if içerik == "eqk_01"))
        );
        assert!(
            komutlar
                .iter()
                .any(|komut| matches!(komut, Komut::Metin { içerik, .. } if içerik == "tor_20"))
        );
        assert!(komutlar.iter().any(|komut| matches!(
            komut,
            Komut::Dikdörtgen { dolgu, genişlik, .. }
                if dolgu == "rgb(255 193 7 / 20%)" && *genişlik > 0.0
        )));
        assert_eq!(
            komutlar
                .iter()
                .filter(|komut| matches!(
                    komut,
                    Komut::KesikliÇizgi { renk, .. } if renk == "rgb(76 175 80)"
                ))
                .count(),
            5
        );
        Ok(())
    }

    #[test]
    fn yakınlaştırma_görünürlüğü_ve_kırpmayı_yeniden_hesaplar() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = annotations_kartı()?;
        let mut grafik = Grafik::yeni(seçenekler, veri)?;
        assert!(grafik.seçim_yakınlaştır(0.34, 0.52)?);
        let (sol, sağ, _, _) = grafik.çizim_alanı_boyutta(1_920, 600);
        let sahne = grafik.çiz();
        assert!(
            !sahne
                .komutlar()
                .iter()
                .any(|komut| matches!(komut, Komut::Metin { içerik, .. } if içerik == "eqk_01"))
        );
        let dolgu = sahne.komutlar().iter().find_map(|komut| match komut {
            Komut::Dikdörtgen {
                konum,
                genişlik,
                dolgu,
                ..
            } if dolgu == "rgb(255 193 7 / 20%)" => Some((konum.x, *genişlik)),
            _ => None,
        });
        let (x, genişlik) =
            dolgu.ok_or_else(|| test_hatası("kırpılmış tornado dolgusu bulunamadı"))?;
        assert!(x >= sol);
        assert!(x + genişlik <= sağ);
        Ok(())
    }
}
