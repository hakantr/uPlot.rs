use super::ortak_kart_etkileşimleri;
use crate::{
    Aralık, GrafikSeçenekleri, HizalıVeri, SayısalAralıkAyarları, SayısalAralıkParçası,
    SeriSeçenekleri, UplotHatası, YumuşakSınırKipi, YÖlçekEtiketBiçimi, YÖlçekSeçenekleri,
};

pub const NO_DATA_KART_TANIM_ÖRNEĞİ: &str = r##"// Tek katalog kartındaki 33 seçenekten biri tipli olarak seçilir.
let örnek = NoDataÖrneği::kimlikten("no-data-one-x-0-y-0")
    .unwrap_or(NoDataÖrneği::BOŞ_ÖZEL_ARALIK);
let (seçenekler, veri) = no_data_kartı(örnek)?;
// NoDataÖrneği::TÜMÜ, kaynak 33 yüzeyinin eksiksiz kanıt kümesidir.
// Adaptör aynı grafik yüzeyinde yalnız seçili tanımı değiştirerek 33 ayrı
// çizim yüzeyi ve etkileşim katmanı oluşturma maliyetini taşımaz.
let grafik = Grafik::yeni(seçenekler, veri)?;"##;

/// Resmî `demos/no-data.html` sayfasındaki 33 etkin uPlot yüzeyi.
///
/// İç indeks özeldir; böylece yalnız kaynakta gerçekten bulunan durumlar
/// üretilebilir ve geçersiz bir durum çalışma zamanında panik oluşturmaz.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoDataÖrneği(u8);

impl NoDataÖrneği {
    pub const BOŞ_ÖZEL_ARALIK: Self = Self(0);
    pub const BOŞ_SAYISAL: Self = Self(1);
    pub const TEK_ZAMAN: Self = Self(2);

    pub const TÜMÜ: [Self; 33] = [
        Self(0),
        Self(1),
        Self(2),
        Self(3),
        Self(4),
        Self(5),
        Self(6),
        Self(7),
        Self(8),
        Self(9),
        Self(10),
        Self(11),
        Self(12),
        Self(13),
        Self(14),
        Self(15),
        Self(16),
        Self(17),
        Self(18),
        Self(19),
        Self(20),
        Self(21),
        Self(22),
        Self(23),
        Self(24),
        Self(25),
        Self(26),
        Self(27),
        Self(28),
        Self(29),
        Self(30),
        Self(31),
        Self(32),
    ];

    pub fn kimlik(self) -> &'static str {
        self.kayıt().kimlik
    }

    pub fn başlık(self) -> &'static str {
        self.kayıt().başlık
    }

    pub fn nokta_sayısı(self) -> usize {
        self.kayıt().x.len()
    }

    pub fn kimlikten(kimlik: &str) -> Option<Self> {
        KAYITLAR
            .iter()
            .position(|kayıt| kayıt.kimlik == kimlik)
            .and_then(|indeks| u8::try_from(indeks).ok())
            .map(Self)
    }

    fn kayıt(self) -> &'static NoDataKaydı {
        KAYITLAR
            .get(usize::from(self.0))
            .map_or(&VARSAYILAN_KAYIT, |kayıt| kayıt)
    }
}

#[derive(Debug)]
struct NoDataKaydı {
    kimlik: &'static str,
    başlık: &'static str,
    x: &'static [f64],
    y: &'static [f64],
    zaman: bool,
    özel_boş_aralık: bool,
}

const BOŞ: &[f64] = &[];
const X_ZAMAN: &[f64] = &[1_566_453_600.0];
const X_NEG_1: &[f64] = &[-1.0];
const X_0: &[f64] = &[0.0];
const X_1: &[f64] = &[1.0];
const X_01: &[f64] = &[0.0, 1.0];
const Y_NEG_1: &[f64] = &[-1.0];
const Y_0: &[f64] = &[0.0];
const Y_1: &[f64] = &[1.0];
const VARSAYILAN_KAYIT: NoDataKaydı = kayıt(
    "no-data-empty-custom",
    "Plot without data",
    BOŞ,
    BOŞ,
    true,
    true,
);

const KAYITLAR: [NoDataKaydı; 33] = [
    kayıt(
        "no-data-empty-custom",
        "Plot without data",
        BOŞ,
        BOŞ,
        true,
        true,
    ),
    kayıt(
        "no-data-empty",
        "Plot without data 2",
        BOŞ,
        BOŞ,
        false,
        false,
    ),
    kayıt(
        "no-data-one-time",
        "1 point (time)",
        X_ZAMAN,
        Y_1,
        true,
        false,
    ),
    kayıt(
        "no-data-one-x-neg1-y-neg1",
        "1 point - [[-1],[-1]]",
        X_NEG_1,
        Y_NEG_1,
        false,
        false,
    ),
    kayıt(
        "no-data-one-x-neg1-y-0",
        "1 point - [[-1],[0]]",
        X_NEG_1,
        Y_0,
        false,
        false,
    ),
    kayıt(
        "no-data-one-x-neg1-y-1",
        "1 point - [[-1],[1]]",
        X_NEG_1,
        Y_1,
        false,
        false,
    ),
    kayıt(
        "no-data-one-x-0-y-neg1",
        "1 point - [[0],[-1]]",
        X_0,
        Y_NEG_1,
        false,
        false,
    ),
    kayıt(
        "no-data-one-x-0-y-0",
        "1 point - [[0],[0]]",
        X_0,
        Y_0,
        false,
        false,
    ),
    kayıt(
        "no-data-one-x-0-y-1",
        "1 point - [[0],[1]]",
        X_0,
        Y_1,
        false,
        false,
    ),
    kayıt(
        "no-data-one-x-1-y-neg1",
        "1 point - [[1],[-1]]",
        X_1,
        Y_NEG_1,
        false,
        false,
    ),
    kayıt(
        "no-data-one-x-1-y-0",
        "1 point - [[1],[0]]",
        X_1,
        Y_0,
        false,
        false,
    ),
    kayıt(
        "no-data-one-x-1-y-1",
        "1 point - [[1],[1]]",
        X_1,
        Y_1,
        false,
        false,
    ),
    kayıt(
        "no-data-flatish-36-51",
        "1 point - [[0,1],[36,51]]",
        X_01,
        &[36.0, 51.0],
        false,
        false,
    ),
    kayıt(
        "no-data-flatish-10-close",
        "1 point - [[0,1],[9.999999,10.000001]]",
        X_01,
        &[9.999_999, 10.000_001],
        false,
        false,
    ),
    kayıt(
        "no-data-flatish-10-flat",
        "1 point - [[0,1],[10,10]]",
        X_01,
        &[10.0, 10.0],
        false,
        false,
    ),
    kayıt(
        "no-data-flatish-10-precision",
        "1 point - [[0,1],[9.9999999,10.0000001]]",
        X_01,
        &[9.999_999_9, 10.000_000_1],
        false,
        false,
    ),
    kayıt(
        "no-data-flatish-10m-precision",
        "1 point - [[0,1],[10000000.000027,9999999.999753]]",
        X_01,
        &[10_000_000.000_027, 9_999_999.999_753],
        false,
        false,
    ),
    kayıt(
        "no-data-flatish-1-precision",
        "1 point - [[0,1],[1,0.9999999]]",
        X_01,
        &[1.0, 0.999_999_9],
        false,
        false,
    ),
    kayıt(
        "no-data-flatish-neg36-neg51",
        "1 point - [[0,1],[-36,-51]]",
        X_01,
        &[-36.0, -51.0],
        false,
        false,
    ),
    kayıt(
        "no-data-flatish-neg10-close",
        "1 point - [[0,1],[-9.999999,-10.000001]]",
        X_01,
        &[-9.999_999, -10.000_001],
        false,
        false,
    ),
    kayıt(
        "no-data-flatish-neg10-flat",
        "1 point - [[0,1],[-10,-10]]",
        X_01,
        &[-10.0, -10.0],
        false,
        false,
    ),
    kayıt(
        "no-data-flatish-neg10-precision",
        "1 point - [[0,1],[-9.9999999,-10.0000001]]",
        X_01,
        &[-9.999_999_9, -10.000_000_1],
        false,
        false,
    ),
    kayıt(
        "no-data-flatish-neg10m-precision",
        "1 point - [[0,1],[-10000000.000027,-9999999.999753]]",
        X_01,
        &[-10_000_000.000_027, -9_999_999.999_753],
        false,
        false,
    ),
    kayıt(
        "no-data-flatish-neg1-precision",
        "1 point - [[0,1],[-1,-0.9999999]]",
        X_01,
        &[-1.0, -0.999_999_9],
        false,
        false,
    ),
    kayıt(
        "no-data-flat-neg100",
        "1 point - [[0,1],[-100,-100]]",
        X_01,
        &[-100.0, -100.0],
        false,
        false,
    ),
    kayıt(
        "no-data-flat-neg10",
        "1 point - [[0,1],[-10,-10]]",
        X_01,
        &[-10.0, -10.0],
        false,
        false,
    ),
    kayıt(
        "no-data-flat-neg1",
        "1 point - [[0,1],[-1,-1]]",
        X_01,
        &[-1.0, -1.0],
        false,
        false,
    ),
    kayıt(
        "no-data-flat-neg01",
        "1 point - [[0,1],[-0.1,-0.1]]",
        X_01,
        &[-0.1, -0.1],
        false,
        false,
    ),
    kayıt(
        "no-data-flat-0",
        "1 point - [[0,1],[0,0]]",
        X_01,
        &[0.0, 0.0],
        false,
        false,
    ),
    kayıt(
        "no-data-flat-01",
        "1 point - [[0,1],[0.1,0.1]]",
        X_01,
        &[0.1, 0.1],
        false,
        false,
    ),
    kayıt(
        "no-data-flat-1",
        "1 point - [[0,1],[1,1]]",
        X_01,
        &[1.0, 1.0],
        false,
        false,
    ),
    kayıt(
        "no-data-flat-10",
        "1 point - [[0,1],[10,10]]",
        X_01,
        &[10.0, 10.0],
        false,
        false,
    ),
    kayıt(
        "no-data-flat-100",
        "1 point - [[0,1],[100,100]]",
        X_01,
        &[100.0, 100.0],
        false,
        false,
    ),
];

const fn kayıt(
    kimlik: &'static str,
    başlık: &'static str,
    x: &'static [f64],
    y: &'static [f64],
    zaman: bool,
    özel_boş_aralık: bool,
) -> NoDataKaydı {
    NoDataKaydı {
        kimlik,
        başlık,
        x,
        y,
        zaman,
        özel_boş_aralık,
    }
}

/// Kaynaktaki seçili boş/kenar durum yüzeyini aynı veri ve ölçek kurallarıyla
/// çekirdek grafik tanımına dönüştürür.
pub fn no_data_kartı(
    örnek: NoDataÖrneği,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let kayıt = örnek.kayıt();
    let mut seçenekler = GrafikSeçenekleri::yeni(800, 400)?
        .başlık(kayıt.başlık)
        .x_zaman(kayıt.zaman)
        .x_eksen_etiket_biçimi(YÖlçekEtiketBiçimi::ArtımaGöre)
        .y_ölçeği(
            YÖlçekSeçenekleri::yeni("y")
                .etiket_biçimi(YÖlçekEtiketBiçimi::ArtımaGöre)
                .sayısal_aralık(SayısalAralıkAyarları::yeni(
                    SayısalAralıkParçası::yeni(0.1, Some(0.0), YumuşakSınırKipi::Koşullu),
                    SayısalAralıkParçası::yeni(0.1, Some(0.0), YumuşakSınırKipi::Koşullu),
                )),
        )
        .etkileşimler(ortak_kart_etkileşimleri())
        .seri(SeriSeçenekleri::yeni("Value").renk("#000000"));

    if kayıt.özel_boş_aralık {
        seçenekler = seçenekler
            .boş_x_aralığı(Aralık::yeni(1_566_453_600.0, 1_566_497_660.0)?)
            .boş_y_aralığı(Aralık::yeni(0.0, 100.0)?);
    } else if let (Some(x_alt), Some(x_üst)) = (
        kayıt.x.iter().copied().reduce(f64::min),
        kayıt.x.iter().copied().reduce(f64::max),
    ) {
        let x_aralığı = if x_alt == x_üst {
            if kayıt.zaman {
                Aralık::yeni(x_alt, x_üst + 86_400.0)?
            } else {
                Aralık::uplot_sayısal(x_alt, x_üst, 0.1, true)?
            }
        } else {
            Aralık::yeni(x_alt, x_üst)?
        };
        seçenekler = seçenekler.x_aralığı(x_aralığı);
    }

    let y = kayıt.y.iter().copied().map(Some).collect();
    Ok((seçenekler, HizalıVeri::yeni(kayıt.x.to_vec(), vec![y])?))
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut};

    #[test]
    fn kaynak_33_yüzeyi_eksiksiz_ve_benzersizdir() -> Result<(), UplotHatası> {
        let mut kimlikler = NoDataÖrneği::TÜMÜ
            .into_iter()
            .map(NoDataÖrneği::kimlik)
            .collect::<Vec<_>>();
        kimlikler.sort_unstable();
        kimlikler.dedup();
        assert_eq!(kimlikler.len(), 33);

        for örnek in NoDataÖrneği::TÜMÜ {
            assert_eq!(NoDataÖrneği::kimlikten(örnek.kimlik()), Some(örnek));
            let (seçenekler, veri) = no_data_kartı(örnek)?;
            assert_eq!(seçenekler.başlık, örnek.başlık());
            assert_eq!(seçenekler.seriler.len(), 1);
            assert_eq!(
                veri.x().len(),
                veri.seriler().first().map_or(usize::MAX, Vec::len)
            );
            let _grafik = Grafik::yeni(seçenekler, veri)?;
        }
        Ok(())
    }

    #[test]
    fn boş_ölçekler_kaynak_null_davranışını_korur() -> Result<(), UplotHatası> {
        let (özel_seçenekler, özel_veri) = no_data_kartı(NoDataÖrneği::BOŞ_ÖZEL_ARALIK)?;
        let özel = Grafik::yeni(özel_seçenekler, özel_veri)?;
        assert_eq!(özel.görünür_x_aralığı().en_az, 1_566_453_600.0);
        assert_eq!(özel.görünür_y_aralığı().en_çok, 100.0);
        assert!(
            özel
                .çiz()
                .komutlar()
                .iter()
                .any(|komut| matches!(komut, Komut::Çizgi { .. }))
        );

        let (boş_seçenekler, boş_veri) = no_data_kartı(NoDataÖrneği::BOŞ_SAYISAL)?;
        let boş = Grafik::yeni(boş_seçenekler, boş_veri)?;
        assert_eq!(boş.çiz().komutlar().len(), 2);
        Ok(())
    }

    #[test]
    fn tek_ve_düz_noktalar_uplot_aralıklarını_korur() -> Result<(), UplotHatası> {
        let tek_sıfır = NoDataÖrneği::kimlikten("no-data-one-x-0-y-0").ok_or_else(|| {
            UplotHatası::BilinmeyenKart {
                kimlik: "no-data-one-x-0-y-0".to_string(),
            }
        })?;
        let (seçenekler, veri) = no_data_kartı(tek_sıfır)?;
        let grafik = Grafik::yeni(seçenekler, veri)?;
        assert_eq!(grafik.görünür_x_aralığı(), Aralık::yeni(0.0, 100.0)?);
        assert_eq!(grafik.görünür_y_aralığı(), Aralık::yeni(0.0, 100.0)?);

        let hassas = NoDataÖrneği::kimlikten("no-data-flatish-10-precision").ok_or_else(|| {
            UplotHatası::BilinmeyenKart {
                kimlik: "no-data-flatish-10-precision".to_string(),
            }
        })?;
        let (seçenekler, veri) = no_data_kartı(hassas)?;
        let grafik = Grafik::yeni(seçenekler, veri)?;
        assert_eq!(grafik.görünür_y_aralığı(), Aralık::yeni(0.0, 20.0)?);
        let sahne = grafik.çiz();
        let metinler = sahne
            .komutlar()
            .iter()
            .filter_map(|komut| match komut {
                Komut::Metin { içerik, .. } => Some(içerik.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert!(metinler.contains(&"0.1"));
        assert!(metinler.contains(&"0"));
        assert!(!metinler.contains(&"0.10"));
        assert!(!metinler.contains(&"0.0"));
        Ok(())
    }

    #[test]
    fn kaynak_30_sayısal_yüzeyinin_x_y_aralıkları_altın_matrisi_korunur() -> Result<(), UplotHatası>
    {
        let beklenen_y: [(f64, f64); 30] = [
            (-2.0, 0.0),
            (0.0, 100.0),
            (0.0, 2.0),
            (-2.0, 0.0),
            (0.0, 100.0),
            (0.0, 2.0),
            (-2.0, 0.0),
            (0.0, 100.0),
            (0.0, 2.0),
            (34.0, 53.0),
            (9.999_998_800_000_002, 10.000_001_200_000_002),
            (0.0, 20.0),
            (0.0, 20.0),
            (0.0, 20_000_000.0),
            (0.0, 2.0),
            (-53.0, -34.0),
            (-10.000_001_200_000_002, -9.999_998_800_000_002),
            (-20.0, 0.0),
            (-20.0, 0.0),
            (-20_000_000.0, 0.0),
            (-2.0, 0.0),
            (-200.0, 0.0),
            (-20.0, 0.0),
            (-2.0, 0.0),
            (-0.200_000_000_000_000_07, 0.0),
            (0.0, 100.0),
            (0.0, 0.200_000_000_000_000_07),
            (0.0, 2.0),
            (0.0, 20.0),
            (0.0, 200.0),
        ];
        for (örnek, (beklenen_alt, beklenen_üst)) in
            NoDataÖrneği::TÜMÜ.into_iter().skip(3).zip(beklenen_y)
        {
            let (seçenekler, veri) = no_data_kartı(örnek)?;
            let grafik = Grafik::yeni(seçenekler, veri)?;
            let beklenen_x = if örnek.nokta_sayısı() == 1 {
                match grafik.hizalı_veri().x().first().copied() {
                    Some(-1.0) => Aralık::yeni(-2.0, 0.0)?,
                    Some(0.0) => Aralık::yeni(0.0, 100.0)?,
                    Some(_) => Aralık::yeni(0.0, 2.0)?,
                    None => return Err(UplotHatası::YetersizVeri { uzunluk: 0 }),
                }
            } else {
                Aralık::yeni(0.0, 1.0)?
            };
            assert_eq!(grafik.görünür_x_aralığı(), beklenen_x, "{}", örnek.kimlik());
            let gerçekleşen = grafik.görünür_y_aralığı();
            let tolerans = beklenen_alt.abs().max(beklenen_üst.abs()).max(1.0) * 1e-12;
            assert!(
                (gerçekleşen.en_az - beklenen_alt).abs() <= tolerans
                    && (gerçekleşen.en_çok - beklenen_üst).abs() <= tolerans,
                "{}: {gerçekleşen:?}",
                örnek.kimlik()
            );
        }
        Ok(())
    }

    #[test]
    fn özel_boş_aralık_set_data_sonrası_otomatik_aralığa_döner() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = no_data_kartı(NoDataÖrneği::BOŞ_ÖZEL_ARALIK)?;
        let mut grafik = Grafik::yeni(seçenekler, veri)?;
        assert_eq!(
            grafik.görünür_x_aralığı(),
            Aralık::yeni(1_566_453_600.0, 1_566_497_660.0)?
        );
        assert_eq!(grafik.görünür_y_aralığı(), Aralık::yeni(0.0, 100.0)?);

        grafik.veriyi_ayarla(HizalıVeri::yeni(
            vec![10.0, 20.0],
            vec![vec![Some(36.0), Some(51.0)]],
        )?)?;
        assert_eq!(grafik.görünür_x_aralığı(), Aralık::yeni(10.0, 20.0)?);
        assert_eq!(grafik.görünür_y_aralığı(), Aralık::yeni(34.0, 53.0)?);
        Ok(())
    }
}
