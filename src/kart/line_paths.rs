use std::sync::OnceLock;

use super::ortak_kart_etkileşimleri;
use crate::{GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası};

pub const LINE_PATHS_KART_TANIM_ÖRNEĞİ: &str = r##"for (örnek, seçenekler, veri) in line_paths_kartları()? {
    // Sekiz yüzey aynı Arc-backed veriyi paylaşır; cursor grup içinde senkronlanır.
    // Seçim/zoom geçmişi ise resmî sayfadaki gibi her Grafik için bağımsızdır.
    let grafik = Grafik::yeni(seçenekler, veri)?;
}"##;

static PAYLAŞILAN_VERİ: OnceLock<Result<HizalıVeri, UplotHatası>> = OnceLock::new();

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinePathsÖrneği {
    YalnızNoktalar,
    Doğrusal,
    MonotonKübik,
    BasamakSonra,
    BasamakÖnce,
    ÇubukOrta,
    ÇubukSol,
    ÇubukSağ,
}

impl LinePathsÖrneği {
    pub const TÜMÜ: [Self; 8] = [
        Self::YalnızNoktalar,
        Self::Doğrusal,
        Self::MonotonKübik,
        Self::BasamakSonra,
        Self::BasamakÖnce,
        Self::ÇubukOrta,
        Self::ÇubukSol,
        Self::ÇubukSağ,
    ];

    pub const fn kimlik(self) -> &'static str {
        match self {
            Self::YalnızNoktalar => "line-paths-points-only",
            Self::Doğrusal => "line-paths-linear",
            Self::MonotonKübik => "line-paths-monotone-cubic",
            Self::BasamakSonra => "line-paths-step-after",
            Self::BasamakÖnce => "line-paths-step-before",
            Self::ÇubukOrta => "line-paths-bars-center",
            Self::ÇubukSol => "line-paths-bars-left",
            Self::ÇubukSağ => "line-paths-bars-right",
        }
    }

    pub const fn başlık(self) -> &'static str {
        match self {
            Self::YalnızNoktalar => "null path (points only)",
            Self::Doğrusal => "linear",
            Self::MonotonKübik => "spline (Monotone Cubic)",
            Self::BasamakSonra => "stepped {align: 1}",
            Self::BasamakÖnce => "stepped {align: -1}",
            Self::ÇubukOrta => "bars {align: 0}",
            Self::ÇubukSol => "bars {align: 1}",
            Self::ÇubukSağ => "bars {align: -1}",
        }
    }

    pub const fn renk(self) -> &'static str {
        match self {
            Self::YalnızNoktalar => "#FFFFFF",
            Self::Doğrusal => "#7EB26D",
            Self::MonotonKübik => "#1F78C1",
            Self::BasamakSonra => "#6ED0E0",
            Self::BasamakÖnce => "#EF843C",
            Self::ÇubukOrta => "#E24D42",
            Self::ÇubukSol => "#008080",
            Self::ÇubukSağ => "#DA70D6",
        }
    }

    pub fn kimlikten(kimlik: &str) -> Option<Self> {
        Self::TÜMÜ
            .into_iter()
            .find(|örnek| örnek.kimlik() == kimlik)
    }
}

/// Resmî line-paths.html sayfasındaki sekiz etkin alt grafikten birini üretir.
pub fn line_paths_kartı(
    örnek: LinePathsÖrneği,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let veri = paylaşılan_veri()?;
    Ok((line_paths_seçenekleri(örnek)?, veri))
}

/// Resmî sayfadaki sekiz yüzeyi kaynak sırasıyla ve tek immutable veri deposuyla üretir.
pub fn line_paths_kartları()
-> Result<Vec<(LinePathsÖrneği, GrafikSeçenekleri, HizalıVeri)>, UplotHatası> {
    let veri = paylaşılan_veri()?;
    LinePathsÖrneği::TÜMÜ
        .into_iter()
        .map(|örnek| Ok((örnek, line_paths_seçenekleri(örnek)?, veri.clone())))
        .collect()
}

fn paylaşılan_veri() -> Result<HizalıVeri, UplotHatası> {
    PAYLAŞILAN_VERİ
        .get_or_init(|| {
            HizalıVeri::yeni(
                (0..=100).map(f64::from).collect::<Vec<_>>(),
                vec![kaynak_y()],
            )
        })
        .clone()
}

fn line_paths_seçenekleri(örnek: LinePathsÖrneği) -> Result<GrafikSeçenekleri, UplotHatası> {
    let renk = örnek.renk();
    let mut seri = SeriSeçenekleri::yeni("Y")
        .renk(renk)
        .dolgu(format!("{renk}1A"));
    let imleç_nokta_boyutu = seri.nokta_boyutu * 2.5;
    seri = seri.imleç_nokta_stili(
        imleç_nokta_boyutu,
        imleç_nokta_boyutu / 4.0,
        "#ffffff",
        format!("{renk}90"),
    );
    seri = match örnek {
        LinePathsÖrneği::YalnızNoktalar => seri.yalnız_noktalar(),
        LinePathsÖrneği::Doğrusal => seri,
        LinePathsÖrneği::MonotonKübik => seri.eğri(),
        LinePathsÖrneği::BasamakSonra => seri.basamak_sonra(),
        LinePathsÖrneği::BasamakÖnce => seri.basamak_önce(),
        LinePathsÖrneği::ÇubukOrta => {
            seri.çubuk(true).çubuk_boyutu(0.6, 100.0).çubuk_hizası(0)
        }
        LinePathsÖrneği::ÇubukSol => {
            seri.çubuk(true).çubuk_boyutu(1.0, f32::MAX).çubuk_hizası(1)
        }
        LinePathsÖrneği::ÇubukSağ => seri
            .çubuk(true)
            .çubuk_boyutu(1.0, f32::MAX)
            .çubuk_hizası(-1),
    };
    let seçenekler = GrafikSeçenekleri::yeni(2_400, 600)?
        .başlık(örnek.başlık())
        .arka_plan_rengi("#141619")
        .başlık_rengi("#c7d0d9")
        .x_eksen_rengi("#c7d0d9")
        .birincil_y_eksen_rengi("#c7d0d9")
        .ızgara_rengi("#2c3235")
        .x_zaman(false)
        .seri(seri)
        .etkileşimler(ortak_kart_etkileşimleri());
    Ok(seçenekler)
}

fn kaynak_y() -> Vec<Option<f64>> {
    let değerler = [
        109.0, 117.0, 122.0, 104.0, 105.0, 117.0, 119.0, 121.0, 117.0, 121.0, 122.0, 129.0, 119.0,
        113.0, 113.0, 121.0, 108.0, 108.0, 100.0, 103.0, 113.0, 110.0, 107.0, 105.0, 99.0, 93.0,
        87.0, 83.0, 91.0, 85.0, 81.0, 69.0, 76.0, 61.0, 63.0, 74.0, 76.0, 68.0, 55.0, 61.0, 48.0,
        39.0, 54.0, 44.0, 37.0, 30.0, 22.0, 33.0, 29.0, 21.0, 22.0, 43.0, 47.0, 33.0, 47.0, 28.0,
        29.0, 31.0, 32.0, 35.0, 37.0, 25.0, -5.0, -14.0, -7.0, -14.0, -7.0, -18.0, -18.0, -18.0,
        -16.0, -41.0, -22.0, -30.0, -27.0, -30.0, -47.0, -49.0, -47.0, -42.0, -55.0, -34.0, -27.0,
        -22.0, -23.0, -34.0, -23.0, -32.0, -36.0, -47.0, -33.0, -32.0, -18.0, -23.0, -21.0, -33.0,
        -39.0, -21.0, -18.0, -27.0, -5.0,
    ];
    değerler
        .into_iter()
        .enumerate()
        .map(|(indeks, değer)| {
            if (22..26).contains(&indeks) {
                None
            } else {
                Some(değer)
            }
        })
        .collect()
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut, SeriÇizimTürü};

    #[test]
    fn sekiz_etkin_yol_aynı_101_noktalı_veriyi_kullanır() -> Result<(), UplotHatası> {
        let kartlar = line_paths_kartları()?;
        assert_eq!(kartlar.len(), 8);
        for ((örnek, seçenekler, veri), beklenen) in kartlar.into_iter().zip(LinePathsÖrneği::TÜMÜ)
        {
            assert_eq!(örnek, beklenen);
            assert_eq!(veri.uzunluk(), 101);
            assert_eq!(
                veri.seriler()
                    .first()
                    .map(|seri| seri.iter().filter(|değer| değer.is_none()).count()),
                Some(4)
            );
            let sahne = Grafik::yeni(seçenekler, veri)?.çiz();
            assert!(sahne.komutlar().iter().any(|komut| {
                matches!(
                    komut,
                    Komut::Yol { .. }
                        | Komut::Daire { .. }
                        | Komut::Daireler { .. }
                        | Komut::Dikdörtgen { .. }
                )
            }));
        }
        Ok(())
    }

    #[test]
    fn sekiz_yüzey_tek_arc_backed_veriyi_paylaşır() -> Result<(), UplotHatası> {
        let kartlar = line_paths_kartları()?;
        let Some((_, _, ilk)) = kartlar.first() else {
            return Err(UplotHatası::YetersizVeri { uzunluk: 0 });
        };
        assert!(
            kartlar
                .iter()
                .skip(1)
                .all(|(_, _, veri)| ilk.aynı_depolamayı_paylaşıyor(veri))
        );
        Ok(())
    }

    #[test]
    fn yorumlu_spline2_kaynağı_catmull_rom_yeteneği_olarak_korunur() {
        let seri = SeriSeçenekleri::yeni("spline2").catmull_rom();
        assert_eq!(seri.çizim_türü, SeriÇizimTürü::CatmullRom);
    }

    #[test]
    fn null_path_geçerli_tüm_kaynak_noktalarını_çizer() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = line_paths_kartı(LinePathsÖrneği::YalnızNoktalar)?;
        let sahne = Grafik::yeni(seçenekler, veri)?.çiz();
        let daireler = sahne
            .komutlar()
            .iter()
            .map(|komut| match komut {
                Komut::Daire { .. } => 1,
                Komut::Daireler { merkezler, .. } => merkezler.len(),
                _ => 0,
            })
            .sum::<usize>();
        assert_eq!(daireler, 97);
        assert!(
            !sahne
                .komutlar()
                .iter()
                .any(|komut| matches!(komut, Komut::Yol { .. }))
        );
        Ok(())
    }

    #[test]
    fn çubuk_hizaları_kaynak_align_değerlerini_korur() -> Result<(), UplotHatası> {
        for (örnek, beklenen) in [
            (LinePathsÖrneği::ÇubukOrta, 0),
            (LinePathsÖrneği::ÇubukSol, 1),
            (LinePathsÖrneği::ÇubukSağ, -1),
        ] {
            let (seçenekler, _) = line_paths_kartı(örnek)?;
            assert_eq!(
                seçenekler.seriler.first().map(|seri| seri.çubuk_hizası),
                Some(beklenen)
            );
        }
        Ok(())
    }

    #[test]
    fn cursor_noktası_kaynak_callback_sunumunu_korur() -> Result<(), UplotHatası> {
        for örnek in LinePathsÖrneği::TÜMÜ {
            let (seçenekler, _) = line_paths_kartı(örnek)?;
            let Some(seri) = seçenekler.seriler.first() else {
                return Err(UplotHatası::YetersizVeri { uzunluk: 0 });
            };
            assert_eq!(seri.imleç_nokta_boyutu, Some(12.5));
            assert_eq!(seri.imleç_nokta_kalınlığı, Some(3.125));
            assert_eq!(seri.imleç_nokta_dolgusu.as_deref(), Some("#ffffff"));
            assert_eq!(
                seri.imleç_nokta_çizgisi.as_deref(),
                Some(format!("{}90", örnek.renk()).as_str())
            );
        }
        Ok(())
    }
}
