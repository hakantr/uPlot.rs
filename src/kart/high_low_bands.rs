use std::sync::OnceLock;

use serde::Deserialize;

use super::ortak_kart_etkileşimleri;
use crate::{
    BantYönü, GrafikSeçenekleri, HizalıVeri, SeriBandı, SeriSeçenekleri, UplotHatası,
    YÖlçekSeçenekleri,
};

const KAYNAK_JSON: &str = include_str!("veri/high_low_bands.json");
pub const HIGH_LOW_BANDS_KANIT_TOHUMU: u32 = 0x4849_4241;
pub const HIGH_LOW_BANDS_KART_TANIM_ÖRNEĞİ: &str = r##"let (seçenekler, veri) =
    high_low_bands_kartı(HighLowBandsÖrneği::FarklıYollar)?;
// Bant yönü, boşluk kırpması ve yol geometrisi çekirdekte çözülür.
let grafik = Grafik::yeni(seçenekler, veri)?;"##;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighLowBandsÖrneği {
    YıllıkSıcaklık,
    FarklıYollar,
    Çubuklar,
    BasitBant,
    KesişenBant,
    KısmiSıcaklık,
    YalnızOrtalama,
    TersÇizgiler,
    TersÇubuklar,
    HizalanmamışÇubuklar,
    HizalanmamışÇubukVuruşu,
    ÇokİnceÇubuklar,
}

impl HighLowBandsÖrneği {
    pub const TÜMÜ: [Self; 12] = [
        Self::YıllıkSıcaklık,
        Self::FarklıYollar,
        Self::Çubuklar,
        Self::BasitBant,
        Self::KesişenBant,
        Self::KısmiSıcaklık,
        Self::YalnızOrtalama,
        Self::TersÇizgiler,
        Self::TersÇubuklar,
        Self::HizalanmamışÇubuklar,
        Self::HizalanmamışÇubukVuruşu,
        Self::ÇokİnceÇubuklar,
    ];

    fn indeks(self) -> usize {
        match self {
            Self::YıllıkSıcaklık => 0,
            Self::FarklıYollar => 1,
            Self::Çubuklar => 2,
            Self::BasitBant => 3,
            Self::KesişenBant => 4,
            Self::KısmiSıcaklık => 5,
            Self::YalnızOrtalama => 6,
            Self::TersÇizgiler => 7,
            Self::TersÇubuklar => 8,
            Self::HizalanmamışÇubuklar => 9,
            Self::HizalanmamışÇubukVuruşu => 10,
            Self::ÇokİnceÇubuklar => 11,
        }
    }

    pub fn kimlik(self) -> &'static str {
        match self {
            Self::YıllıkSıcaklık => "high-low-bands-temps-year",
            Self::FarklıYollar => "high-low-bands-differing-paths",
            Self::Çubuklar => "high-low-bands-bars",
            Self::BasitBant => "high-low-bands-simple",
            Self::KesişenBant => "high-low-bands-crossing",
            Self::KısmiSıcaklık => "high-low-bands-temps-partial",
            Self::YalnızOrtalama => "high-low-bands-average-only",
            Self::TersÇizgiler => "high-low-bands-inverted-lines",
            Self::TersÇubuklar => "high-low-bands-inverted-bars",
            Self::HizalanmamışÇubuklar => "high-low-bands-unaligned-bars",
            Self::HizalanmamışÇubukVuruşu => "high-low-bands-unaligned-stroke",
            Self::ÇokİnceÇubuklar => "high-low-bands-very-thin-bars",
        }
    }

    pub fn başlık(self) -> &'static str {
        match self {
            Self::YıllıkSıcaklık => "Temps · 365 days",
            Self::FarklıYollar => "Differing Paths",
            Self::Çubuklar => "Bars",
            Self::BasitBant => "High/Low Band",
            Self::KesişenBant => "Crossing High/Low Band",
            Self::KısmiSıcaklık => "Temps · partial data",
            Self::YalnızOrtalama => "Temps · average only",
            Self::TersÇizgiler => "Inverted bands · lines",
            Self::TersÇubuklar => "Inverted bands · bars",
            Self::HizalanmamışÇubuklar => "Unaligned bars + band clipping issue",
            Self::HizalanmamışÇubukVuruşu => "Unaligned bars · stroke + zero-alpha fill",
            Self::ÇokİnceÇubuklar => "Very thin bars",
        }
    }

    pub fn kimlikten(kimlik: &str) -> Option<Self> {
        Self::TÜMÜ
            .into_iter()
            .find(|örnek| örnek.kimlik() == kimlik)
    }

    pub fn nokta_sayısı(self) -> usize {
        [365, 101, 101, 10, 4, 83, 3, 6, 6, 363, 363, 42]
            .get(self.indeks())
            .copied()
            .unwrap_or_default()
    }
}

#[derive(Debug, Clone, Deserialize)]
struct KaynakKök {
    charts: Vec<KaynakGrafik>,
}

#[derive(Debug, Clone, Deserialize)]
struct KaynakGrafik {
    width: u32,
    height: u32,
    x_time: bool,
    milliseconds: bool,
    series: Vec<KaynakSeri>,
    bands: Vec<KaynakBant>,
    data: Vec<Vec<Option<f64>>>,
}

#[derive(Debug, Clone, Deserialize)]
struct KaynakSeri {
    label: String,
    stroke: Option<String>,
    fill: Option<String>,
    width: f32,
    dash: Option<[f32; 2]>,
    path: KaynakYol,
}

#[derive(Debug, Clone, Deserialize)]
struct KaynakYol {
    kind: String,
    size: Option<[f32; 2]>,
}

#[derive(Debug, Clone, Deserialize)]
struct KaynakBant {
    series: [usize; 2],
    dir: i8,
    fill: Option<String>,
}

fn kaynak_grafikler() -> Result<&'static [KaynakGrafik], UplotHatası> {
    static KAYNAKLAR: OnceLock<Result<Vec<KaynakGrafik>, String>> = OnceLock::new();
    match KAYNAKLAR.get_or_init(|| {
        serde_json::from_str::<KaynakKök>(KAYNAK_JSON)
            .map(|kök| kök.charts)
            .map_err(|hata| hata.to_string())
    }) {
        Ok(grafikler) => Ok(grafikler),
        Err(açıklama) => Err(UplotHatası::GeçersizKaynakVeri {
            varlık: "src/kart/veri/high_low_bands.json",
            açıklama: açıklama.clone(),
        }),
    }
}

pub fn high_low_bands_kartı(
    örnek: HighLowBandsÖrneği,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let kaynak = kaynak_grafikler()?.get(örnek.indeks()).ok_or_else(|| {
        UplotHatası::GeçersizKaynakVeri {
            varlık: "src/kart/veri/high_low_bands.json",
            açıklama: format!("{}. kaynak grafik bulunamadı", örnek.indeks()),
        }
    })?;
    let x_kaynağı = kaynak
        .data
        .first()
        .ok_or(UplotHatası::YetersizVeri { uzunluk: 0 })?;
    let x = x_kaynağı
        .iter()
        .enumerate()
        .map(|(indeks, değer)| değer.ok_or(UplotHatası::SonluOlmayanX { indeks }))
        .collect::<Result<Vec<_>, _>>()?;
    let seriler = kaynak
        .data
        .iter()
        .skip(1)
        .cloned()
        .map(|mut seri| {
            seri.resize(x.len(), None);
            seri
        })
        .collect::<Vec<_>>();
    let mut seçenekler = GrafikSeçenekleri::yeni(kaynak.width, kaynak.height)?
        .başlık(örnek.başlık())
        .x_zaman(kaynak.x_time)
        .x_zaman_milisaniye(kaynak.milliseconds)
        .etkileşimler(ortak_kart_etkileşimleri());
    if matches!(
        örnek,
        HighLowBandsÖrneği::YıllıkSıcaklık
            | HighLowBandsÖrneği::KısmiSıcaklık
            | HighLowBandsÖrneği::YalnızOrtalama
    ) {
        seçenekler = seçenekler.y_ölçeği(YÖlçekSeçenekleri::yeni("y").birim("°F"));
    }
    for kaynak_seri in &kaynak.series {
        seçenekler = seçenekler.seri(kaynak_serisini_oluştur(kaynak_seri));
    }
    for kaynak_bant in &kaynak.bands {
        seçenekler = seçenekler.bant(kaynak_bandı_oluştur(kaynak, kaynak_bant)?);
    }
    Ok((seçenekler, HizalıVeri::yeni(x, seriler)?))
}

fn kaynak_serisini_oluştur(kaynak: &KaynakSeri) -> SeriSeçenekleri {
    let mut seri = SeriSeçenekleri::yeni(&kaynak.label)
        .renk(css_rengini_hex(
            kaynak.stroke.as_deref().unwrap_or("#00000000"),
        ))
        .çizgi_kalınlığı(kaynak.width);
    if let Some(dolgu) = kaynak.fill.as_deref() {
        seri = seri.dolgu(css_rengini_hex(dolgu));
    }
    if let Some([çizgi, boşluk]) = kaynak.dash {
        seri = seri.çizgi_kesik(çizgi, boşluk);
    }
    match kaynak.path.kind.as_str() {
        "step-before" => seri.basamak_önce(),
        "step-after" => seri.basamak_sonra(),
        "spline" => seri.eğri(),
        "bars" => {
            let [oran, azami] = kaynak.path.size.unwrap_or([0.6, 100.0]);
            seri.çubuk(true).çubuk_boyutu(oran, azami)
        }
        _ => seri,
    }
}

fn kaynak_bandı_oluştur(
    grafik: &KaynakGrafik,
    kaynak: &KaynakBant,
) -> Result<SeriBandı, UplotHatası> {
    let çöz = |indeks: usize, ad: &str| {
        indeks
            .checked_sub(1)
            .ok_or_else(|| UplotHatası::GeçersizKaynakVeri {
                varlık: "src/kart/veri/high_low_bands.json",
                açıklama: format!("bant {ad} serisi X serisini gösteriyor"),
            })
    };
    let üst_seri = çöz(kaynak.series[0], "üst")?;
    let alt_seri = çöz(kaynak.series[1], "alt")?;
    let dolgu = kaynak
        .fill
        .as_deref()
        .or_else(|| grafik.series.get(üst_seri)?.fill.as_deref())
        .map(css_rengini_hex)
        .unwrap_or_else(|| "#00000000".to_string());
    let yön = if kaynak.dir == 1 {
        BantYönü::EnÇoğa
    } else {
        BantYönü::EnAza
    };
    Ok(SeriBandı::yeni(üst_seri, alt_seri, dolgu).yön(yön))
}

fn css_rengini_hex(renk: &str) -> String {
    match renk.trim().to_ascii_lowercase().as_str() {
        "red" => "#ff0000".to_string(),
        "green" => "#008000".to_string(),
        "blue" => "#0000ff".to_string(),
        "magenta" => "#ff00ff".to_string(),
        "orange" => "#ffa500".to_string(),
        küçük if küçük.starts_with('#') => küçük.to_string(),
        küçük => rgba_rengini_çöz(küçük).unwrap_or_else(|| "#000000".to_string()),
    }
}

fn rgba_rengini_çöz(renk: &str) -> Option<String> {
    let içerik = renk
        .strip_prefix("rgba(")
        .or_else(|| renk.strip_prefix("rgb("))?
        .strip_suffix(')')?;
    let parçalar = içerik
        .split(',')
        .map(str::trim)
        .map(str::parse::<f32>)
        .collect::<Result<Vec<_>, _>>()
        .ok()?;
    let r = parçalar.first().copied()?.round().clamp(0.0, 255.0) as u8;
    let g = parçalar.get(1).copied()?.round().clamp(0.0, 255.0) as u8;
    let b = parçalar.get(2).copied()?.round().clamp(0.0, 255.0) as u8;
    let alfa = parçalar
        .get(3)
        .copied()
        .map_or(255, |değer| (değer.clamp(0.0, 1.0) * 255.0).round() as u8);
    Some(format!("#{r:02x}{g:02x}{b:02x}{alfa:02x}"))
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut, SeriÇizimTürü};

    #[test]
    fn on_iki_kaynak_grafiğin_veri_ve_bant_sayıları_korunur() -> Result<(), UplotHatası> {
        let beklenen = [365, 101, 101, 10, 4, 83, 3, 6, 6, 363, 363, 42];
        for (örnek, uzunluk) in HighLowBandsÖrneği::TÜMÜ.into_iter().zip(beklenen) {
            let (seçenekler, veri) = high_low_bands_kartı(örnek)?;
            assert_eq!(veri.uzunluk(), uzunluk);
            assert!(!seçenekler.bantlar.is_empty());
            let bant_alanı_var = Grafik::yeni(seçenekler, veri)?
                .çiz()
                .komutlar()
                .iter()
                .any(|komut| matches!(komut, Komut::Alan { .. }));
            assert_eq!(bant_alanı_var, örnek != HighLowBandsÖrneği::YalnızOrtalama);
        }
        Ok(())
    }

    #[test]
    fn farklı_yollar_ve_ters_bant_yönü_çekirdeğe_taşınır() -> Result<(), UplotHatası> {
        let (seçenekler, _) = high_low_bands_kartı(HighLowBandsÖrneği::FarklıYollar)?;
        assert_eq!(
            seçenekler.seriler.get(1).map(|seri| seri.çizim_türü),
            Some(SeriÇizimTürü::BasamakÖnce)
        );
        assert_eq!(
            seçenekler.seriler.get(2).map(|seri| seri.çizim_türü),
            Some(SeriÇizimTürü::Eğri)
        );
        assert_eq!(
            seçenekler.seriler.get(3).map(|seri| seri.çizim_türü),
            Some(SeriÇizimTürü::BasamakSonra)
        );
        let (ters, _) = high_low_bands_kartı(HighLowBandsÖrneği::TersÇizgiler)?;
        assert!(ters.bantlar.iter().any(|bant| bant.yön == BantYönü::EnÇoğa));
        Ok(())
    }

    #[test]
    fn kaynak_boşlukları_bantla_birleştirilmez() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = high_low_bands_kartı(HighLowBandsÖrneği::YıllıkSıcaklık)?;
        let sahne = Grafik::yeni(seçenekler, veri)?.çiz();
        let çokgen_sayısı = sahne
            .komutlar()
            .iter()
            .filter_map(|komut| match komut {
                Komut::Alan { çokgenler, dolgu }
                    if dolgu == "#ff00001a" || dolgu == "#00ff001a" =>
                {
                    Some(çokgenler.len())
                }
                _ => None,
            })
            .sum::<usize>();
        assert!(çokgen_sayısı > 500);
        Ok(())
    }
}
