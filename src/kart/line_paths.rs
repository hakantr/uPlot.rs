use super::ortak_kart_etkileşimleri;
use crate::{GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası};

pub const LINE_PATHS_KART_TANIM_ÖRNEĞİ: &str = r##"let (seçenekler, veri) =
    line_paths_kartı(LinePathsÖrneği::MonotonKübik)?;
// Yol seçimi, null boşluğu, dolgu ve çubuk hizası çekirdekte çözülür.
let grafik = Grafik::yeni(seçenekler, veri)?;"##;

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
    let x = (0..=100).map(f64::from).collect::<Vec<_>>();
    let y = kaynak_y();
    let veri = HizalıVeri::yeni(x, vec![y])?;
    let renk = örnek.renk();
    let mut seri = SeriSeçenekleri::yeni("Y")
        .renk(renk)
        .dolgu(format!("{renk}1A"));
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
    Ok((seçenekler, veri))
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
        for örnek in LinePathsÖrneği::TÜMÜ {
            let (seçenekler, veri) = line_paths_kartı(örnek)?;
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
                    Komut::Yol { .. } | Komut::Daire { .. } | Komut::Dikdörtgen { .. }
                )
            }));
        }
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
            .filter(|komut| matches!(komut, Komut::Daire { .. }))
            .count();
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
}
