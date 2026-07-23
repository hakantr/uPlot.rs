use super::ortak_kart_etkileşimleri;
use crate::{
    GrafikSeçenekleri, HizalıVeri, OdakDüzeni, SeriSeçenekleri, UplotHatası, YÖlçekSeçenekleri,
};

pub const SCALES_DIR_ORI_KART_TANIM_ÖRNEĞİ: &str = r##"for örnek in ScalesDirOriÖrneği::TÜMÜ {
    let (seçenekler, veri) = scales_dir_ori_kartı(örnek)?;
    // Ölçek yönü, yönelimi ve eksen tarafları çekirdekte çözümlenir.
    let grafik = Grafik::yeni(seçenekler, veri)?;
}"##;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScalesDirOriÖrneği {
    XArtıAltYArtıSol,
    XArtıAltYEksiSol,
    XEksiAltYEksiSol,
    XEksiAltYArtıSol,
    XArtıÜstYArtıSağ,
    XArtıÜstYEksiSağ,
    XEksiÜstYEksiSağ,
    XEksiÜstYArtıSağ,
    XArtıSolYArtıÜst,
    XArtıSolYEksiÜst,
    XEksiSolYEksiÜst,
    XEksiSolYArtıÜst,
    XArtıSağYArtıAlt,
    XArtıSağYEksiAlt,
    XEksiSağYEksiAlt,
    XEksiSağYArtıAlt,
}

impl ScalesDirOriÖrneği {
    pub const TÜMÜ: [Self; 16] = [
        Self::XArtıAltYArtıSol,
        Self::XArtıAltYEksiSol,
        Self::XEksiAltYEksiSol,
        Self::XEksiAltYArtıSol,
        Self::XArtıÜstYArtıSağ,
        Self::XArtıÜstYEksiSağ,
        Self::XEksiÜstYEksiSağ,
        Self::XEksiÜstYArtıSağ,
        Self::XArtıSolYArtıÜst,
        Self::XArtıSolYEksiÜst,
        Self::XEksiSolYEksiÜst,
        Self::XEksiSolYArtıÜst,
        Self::XArtıSağYArtıAlt,
        Self::XArtıSağYEksiAlt,
        Self::XEksiSağYEksiAlt,
        Self::XEksiSağYArtıAlt,
    ];

    pub const fn kimlik(self) -> &'static str {
        match self {
            Self::XArtıAltYArtıSol => "scales-dir-ori-x-plus-bottom-y-plus-left",
            Self::XArtıAltYEksiSol => "scales-dir-ori-x-plus-bottom-y-minus-left",
            Self::XEksiAltYEksiSol => "scales-dir-ori-x-minus-bottom-y-minus-left",
            Self::XEksiAltYArtıSol => "scales-dir-ori-x-minus-bottom-y-plus-left",
            Self::XArtıÜstYArtıSağ => "scales-dir-ori-x-plus-top-y-plus-right",
            Self::XArtıÜstYEksiSağ => "scales-dir-ori-x-plus-top-y-minus-right",
            Self::XEksiÜstYEksiSağ => "scales-dir-ori-x-minus-top-y-minus-right",
            Self::XEksiÜstYArtıSağ => "scales-dir-ori-x-minus-top-y-plus-right",
            Self::XArtıSolYArtıÜst => "scales-dir-ori-x-plus-left-y-plus-top",
            Self::XArtıSolYEksiÜst => "scales-dir-ori-x-plus-left-y-minus-top",
            Self::XEksiSolYEksiÜst => "scales-dir-ori-x-minus-left-y-minus-top",
            Self::XEksiSolYArtıÜst => "scales-dir-ori-x-minus-left-y-plus-top",
            Self::XArtıSağYArtıAlt => "scales-dir-ori-x-plus-right-y-plus-bottom",
            Self::XArtıSağYEksiAlt => "scales-dir-ori-x-plus-right-y-minus-bottom",
            Self::XEksiSağYEksiAlt => "scales-dir-ori-x-minus-right-y-minus-bottom",
            Self::XEksiSağYArtıAlt => "scales-dir-ori-x-minus-right-y-plus-bottom",
        }
    }

    pub const fn başlık(self) -> &'static str {
        match self {
            Self::XArtıAltYArtıSol => "+x bottom, +y left",
            Self::XArtıAltYEksiSol => "+x bottom, -y left",
            Self::XEksiAltYEksiSol => "-x bottom, -y left",
            Self::XEksiAltYArtıSol => "-x bottom, +y left",
            Self::XArtıÜstYArtıSağ => "+x top, +y right",
            Self::XArtıÜstYEksiSağ => "+x top, -y right",
            Self::XEksiÜstYEksiSağ => "-x top, -y right",
            Self::XEksiÜstYArtıSağ => "-x top, +y right",
            Self::XArtıSolYArtıÜst => "+x left, +y top",
            Self::XArtıSolYEksiÜst => "+x left, -y top",
            Self::XEksiSolYEksiÜst => "-x left, -y top",
            Self::XEksiSolYArtıÜst => "-x left, +y top",
            Self::XArtıSağYArtıAlt => "+x right, +y bottom",
            Self::XArtıSağYEksiAlt => "+x right, -y bottom",
            Self::XEksiSağYEksiAlt => "-x right, -y bottom",
            Self::XEksiSağYArtıAlt => "-x right, +y bottom",
        }
    }

    pub const fn x_dikey(self) -> bool {
        matches!(
            self,
            Self::XArtıSolYArtıÜst
                | Self::XArtıSolYEksiÜst
                | Self::XEksiSolYEksiÜst
                | Self::XEksiSolYArtıÜst
                | Self::XArtıSağYArtıAlt
                | Self::XArtıSağYEksiAlt
                | Self::XEksiSağYEksiAlt
                | Self::XEksiSağYArtıAlt
        )
    }

    pub const fn x_ters(self) -> bool {
        matches!(
            self,
            Self::XEksiAltYEksiSol
                | Self::XEksiAltYArtıSol
                | Self::XEksiÜstYEksiSağ
                | Self::XEksiÜstYArtıSağ
                | Self::XEksiSolYEksiÜst
                | Self::XEksiSolYArtıÜst
                | Self::XEksiSağYEksiAlt
                | Self::XEksiSağYArtıAlt
        )
    }

    pub const fn y_ters(self) -> bool {
        matches!(
            self,
            Self::XArtıAltYEksiSol
                | Self::XEksiAltYEksiSol
                | Self::XArtıÜstYEksiSağ
                | Self::XEksiÜstYEksiSağ
                | Self::XArtıSolYEksiÜst
                | Self::XEksiSolYEksiÜst
                | Self::XArtıSağYEksiAlt
                | Self::XEksiSağYEksiAlt
        )
    }

    pub const fn x_eksen_karşıda(self) -> bool {
        matches!(
            self,
            Self::XArtıÜstYArtıSağ
                | Self::XArtıÜstYEksiSağ
                | Self::XEksiÜstYEksiSağ
                | Self::XEksiÜstYArtıSağ
                | Self::XArtıSağYArtıAlt
                | Self::XArtıSağYEksiAlt
                | Self::XEksiSağYEksiAlt
                | Self::XEksiSağYArtıAlt
        )
    }

    pub const fn y_eksen_karşıda(self) -> bool {
        self.x_eksen_karşıda()
    }

    pub const fn boyut(self) -> (u32, u32) {
        if self.x_dikey() {
            (320, 600)
        } else {
            (600, 300)
        }
    }

    pub fn kimlikten(kimlik: &str) -> Option<Self> {
        Self::TÜMÜ
            .into_iter()
            .find(|örnek| örnek.kimlik() == kimlik)
    }
}

pub fn scales_dir_ori_kartı(
    örnek: ScalesDirOriÖrneği,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let (genişlik, yükseklik) = örnek.boyut();
    let seçenekler = GrafikSeçenekleri::yeni(genişlik, yükseklik)?
        .başlık(örnek.başlık())
        .x_zaman(false)
        .x_ters_yön(örnek.x_ters())
        .x_dikey(örnek.x_dikey())
        .x_eksen_karşıda(örnek.x_eksen_karşıda())
        .birincil_y_karşıda(örnek.y_eksen_karşıda())
        .y_ölçeği(YÖlçekSeçenekleri::yeni("y").ters_yön(örnek.y_ters()))
        .odak(OdakDüzeni::yeni(0.3, 30.0))
        .etkileşimler(ortak_kart_etkileşimleri())
        .seri(SeriSeçenekleri::yeni("Red").renk("red").dolgu("#ff00001a"))
        .seri(
            SeriSeçenekleri::yeni("Blue")
                .renk("blue")
                .dolgu("#0000ff1a"),
        );
    let x = vec![-3.0, -2.0, -1.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let kırmızı = vec![
        Some(-10.0),
        Some(-4.0),
        Some(-2.0),
        Some(-1.0),
        Some(0.0),
        Some(2.0),
        Some(3.0),
        None,
        Some(5.0),
        Some(6.0),
    ];
    let mavi = vec![
        Some(-2.0),
        Some(2.0),
        Some(-2.0),
        Some(2.0),
        Some(-2.0),
        Some(2.0),
        Some(-2.0),
        Some(2.0),
        Some(-2.0),
        Some(2.0),
    ];
    Ok((seçenekler, HizalıVeri::yeni(x, vec![kırmızı, mavi])?))
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut};

    #[test]
    fn on_altı_kaynak_yüzeyi_aynı_veriyi_korur() -> Result<(), UplotHatası> {
        for örnek in ScalesDirOriÖrneği::TÜMÜ {
            let (seçenekler, veri) = scales_dir_ori_kartı(örnek)?;
            assert_eq!((seçenekler.genişlik, seçenekler.yükseklik), örnek.boyut());
            assert_eq!(veri.uzunluk(), 10);
            assert_eq!(veri.seriler().len(), 2);
            assert!(
                veri.seriler()
                    .first()
                    .and_then(|seri| seri.get(7))
                    .is_some_and(Option::is_none)
            );
        }
        Ok(())
    }

    #[test]
    fn yönelim_ızgara_ve_seri_geometrisini_birlikte_döndürür() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = scales_dir_ori_kartı(ScalesDirOriÖrneği::XArtıSolYArtıÜst)?;
        let sahne = Grafik::yeni(seçenekler, veri)?.çiz();
        let yatay_ızgara = sahne
            .komutlar()
            .iter()
            .filter(|komut| {
                matches!(
                    komut,
                    Komut::Çizgi { başlangıç, bitiş, .. }
                        if (başlangıç.y - bitiş.y).abs() <= f32::EPSILON
                )
            })
            .count();
        let dikey_ızgara = sahne
            .komutlar()
            .iter()
            .filter(|komut| {
                matches!(
                    komut,
                    Komut::Çizgi { başlangıç, bitiş, .. }
                        if (başlangıç.x - bitiş.x).abs() <= f32::EPSILON
                )
            })
            .count();
        assert!(yatay_ızgara > 0);
        assert!(dikey_ızgara > 0);
        assert!(
            sahne
                .komutlar()
                .iter()
                .filter(|komut| matches!(komut, Komut::Yol { .. }))
                .count()
                >= 2
        );
        Ok(())
    }

    #[test]
    fn fiziksel_etkileşim_oranları_yönlere_göre_dönüşür() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = scales_dir_ori_kartı(ScalesDirOriÖrneği::XEksiSağYEksiAlt)?;
        let mut grafik = Grafik::yeni(seçenekler, veri)?;
        let (x, y) = grafik.fiziksel_oranları_mantıksala(0.25, 0.75);
        assert!((x - 0.75).abs() <= f64::EPSILON);
        assert!((y - 0.25).abs() <= f64::EPSILON);
        assert!(grafik.seçim_yakınlaştır(0.2, 0.4)?);
        let görünür = grafik.görünür_x_aralığı();
        assert!((görünür.en_az - 2.4).abs() < 1e-9);
        assert!((görünür.en_çok - 4.2).abs() < 1e-9);
        Ok(())
    }
}
