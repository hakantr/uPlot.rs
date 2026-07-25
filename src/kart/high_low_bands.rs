use std::sync::OnceLock;

use serde::Deserialize;

use super::ortak_kart_etkileşimleri;
use crate::{
    BantYönü, GrafikSeçenekleri, HizalıVeri, SeriBandı, SeriSeçenekleri, UplotHatası,
    YÖlçekSeçenekleri,
};

const KAYNAK_JSON: &str = include_str!("veri/high_low_bands.json");
pub const HIGH_LOW_BANDS_KANIT_TOHUMU: u32 = 0x4849_4241;
pub const HIGH_LOW_BANDS_KART_TANIM_ÖRNEĞİ: &str = r##"for (örnek, seçenekler, veri) in high_low_bands_kartları()? {
    // Bant yönü, boşluk kırpması ve yol geometrisi çekirdekte çözülür.
    let grafik = Grafik::yeni(seçenekler, veri)?;
}"##;

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

    pub fn durum(self) -> &'static str {
        match self {
            Self::YıllıkSıcaklık => "365 gün · null boşlukları",
            Self::FarklıYollar => "101 nokta · line/step/spline",
            Self::Çubuklar => "aynı 101×4 veri · bar + yuvarlak dış uç",
            Self::BasitBant => "10 nokta · basit bant",
            Self::KesişenBant => "4 nokta · üç kesin kesişim",
            Self::KısmiSıcaklık => "83 nokta · eksik uçlar",
            Self::YalnızOrtalama => "3 nokta · yalnız ortalama",
            Self::TersÇizgiler => "6 nokta · iki bant yönü",
            Self::TersÇubuklar => "aynı 6×4 veri · yuvarlak dış uçlar",
            Self::HizalanmamışÇubuklar => "363 seyrek nokta · fill",
            Self::HizalanmamışÇubukVuruşu => "aynı 363 nokta · stroke",
            Self::ÇokİnceÇubuklar => "42 milisaniye noktası · ince bar",
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
struct KaynakKök {
    charts: Vec<KaynakGrafik>,
}

#[derive(Debug, Clone, Deserialize)]
struct KaynakGrafik {
    title: String,
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
    points: bool,
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

fn kaynak_verileri() -> Result<&'static [HizalıVeri], UplotHatası> {
    static VERİLER: OnceLock<Result<Vec<HizalıVeri>, String>> = OnceLock::new();
    match VERİLER.get_or_init(|| {
        let kaynaklar = kaynak_grafikler().map_err(|hata| hata.to_string())?;
        let mut veriler = Vec::<HizalıVeri>::with_capacity(kaynaklar.len());
        for (indeks, kaynak) in kaynaklar.iter().enumerate() {
            if let Some(önceki) = kaynaklar
                .get(..indeks)
                .and_then(|öncekiler| öncekiler.iter().position(|aday| aday.data == kaynak.data))
                .and_then(|önceki| veriler.get(önceki))
            {
                veriler.push(önceki.clone());
                continue;
            }
            let x_kaynağı = kaynak
                .data
                .first()
                .ok_or_else(|| "X veri sütunu bulunamadı".to_string())?;
            let x = x_kaynağı
                .iter()
                .enumerate()
                .map(|(indeks, değer)| {
                    değer.ok_or_else(|| format!("{indeks}. X değeri sonlu değil"))
                })
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
            veriler.push(HizalıVeri::yeni(x, seriler).map_err(|hata| hata.to_string())?);
        }
        Ok(veriler)
    }) {
        Ok(veriler) => Ok(veriler),
        Err(açıklama) => Err(UplotHatası::GeçersizKaynakVeri {
            varlık: "src/kart/veri/high_low_bands.json",
            açıklama: açıklama.clone(),
        }),
    }
}

/// Resmî sayfadaki on iki bağımsız yüzeyi kaynak sırasıyla döndürür.
///
/// Differing Paths/Bars, inverted lines/bars ve iki unaligned yüzey aynı
/// immutable aligned veri depolarını paylaşır.
pub fn high_low_bands_kartları()
-> Result<Vec<(HighLowBandsÖrneği, GrafikSeçenekleri, HizalıVeri)>, UplotHatası> {
    HighLowBandsÖrneği::TÜMÜ
        .into_iter()
        .map(|örnek| {
            let (seçenekler, veri) = high_low_bands_kartı(örnek)?;
            Ok((örnek, seçenekler, veri))
        })
        .collect()
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
    let veri =
        kaynak_verileri()?
            .get(örnek.indeks())
            .cloned()
            .ok_or(UplotHatası::YetersizVeri {
                uzunluk: örnek.indeks(),
            })?;
    let mut seçenekler = GrafikSeçenekleri::yeni(kaynak.width, kaynak.height)?
        .başlık(&kaynak.title)
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
    for (seri_indeksi, kaynak_seri) in kaynak.series.iter().enumerate() {
        seçenekler = seçenekler.seri(kaynak_serisini_oluştur(örnek, seri_indeksi, kaynak_seri));
    }
    for kaynak_bant in &kaynak.bands {
        seçenekler = seçenekler.bant(kaynak_bandı_oluştur(kaynak, kaynak_bant)?);
    }
    Ok((seçenekler, veri))
}

fn kaynak_serisini_oluştur(
    örnek: HighLowBandsÖrneği,
    seri_indeksi: usize,
    kaynak: &KaynakSeri,
) -> SeriSeçenekleri {
    let mut seri = SeriSeçenekleri::yeni(&kaynak.label)
        .renk(css_rengini_hex(
            kaynak.stroke.as_deref().unwrap_or("#00000000"),
        ))
        .çizgi_kalınlığı(kaynak.width)
        .noktaları_göster(kaynak.points);
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
            let yuvarlatılmış = (örnek == HighLowBandsÖrneği::Çubuklar && seri_indeksi == 3)
                || (örnek == HighLowBandsÖrneği::TersÇubuklar && matches!(seri_indeksi, 0 | 3));
            seri = seri.çubuk(true).çubuk_boyutu(oran, azami);
            if yuvarlatılmış {
                seri = seri.çubuk_uç_yarıçap_oranı(0.3);
            }
            seri
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
        let kartlar = high_low_bands_kartları()?;
        assert_eq!(kartlar.len(), 12);
        assert_eq!(
            kartlar
                .iter()
                .map(|(örnek, _, _)| *örnek)
                .collect::<Vec<_>>(),
            HighLowBandsÖrneği::TÜMÜ
        );
        for ((örnek, seçenekler, veri), uzunluk) in kartlar.into_iter().zip(beklenen) {
            assert_eq!(veri.uzunluk(), uzunluk);
            assert!(!seçenekler.bantlar.is_empty());
            let bant_geometrisi_var =
                Grafik::yeni(seçenekler, veri)?
                    .çiz()
                    .komutlar()
                    .iter()
                    .any(|komut| {
                        matches!(
                            komut,
                            Komut::Alan { .. } | Komut::YuvarlatılmışDikdörtgen { .. }
                        )
                    });
            assert_eq!(
                bant_geometrisi_var,
                örnek != HighLowBandsÖrneği::YalnızOrtalama
            );
        }
        let (_, farklı) = high_low_bands_kartı(HighLowBandsÖrneği::FarklıYollar)?;
        let (_, çubuklar) = high_low_bands_kartı(HighLowBandsÖrneği::Çubuklar)?;
        let (_, ters_çizgiler) = high_low_bands_kartı(HighLowBandsÖrneği::TersÇizgiler)?;
        let (_, ters_çubuklar) = high_low_bands_kartı(HighLowBandsÖrneği::TersÇubuklar)?;
        let (_, hizalanmamış) = high_low_bands_kartı(HighLowBandsÖrneği::HizalanmamışÇubuklar)?;
        let (_, hizalanmamış_vuruş) =
            high_low_bands_kartı(HighLowBandsÖrneği::HizalanmamışÇubukVuruşu)?;
        assert!(farklı.aynı_depolamayı_paylaşıyor(&çubuklar));
        assert!(ters_çizgiler.aynı_depolamayı_paylaşıyor(&ters_çubuklar));
        assert!(hizalanmamış.aynı_depolamayı_paylaşıyor(&hizalanmamış_vuruş));
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
        let (çubuklar, veri) = high_low_bands_kartı(HighLowBandsÖrneği::Çubuklar)?;
        assert!(
            çubuklar
                .seriler
                .iter()
                .all(|seri| seri.noktaları_göster == Some(false))
        );
        assert_eq!(
            çubuklar
                .seriler
                .last()
                .map(|seri| seri.çubuk_uç_yarıçap_oranı),
            Some(0.3)
        );
        assert!(
            Grafik::yeni(çubuklar, veri)?
                .çiz()
                .komutlar()
                .iter()
                .any(|komut| matches!(komut, Komut::YuvarlatılmışDikdörtgen { .. }))
        );
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
        assert!(
            çokgen_sayısı > 1 && çokgen_sayısı < 50,
            "birleştirilmiş bant çokgeni sayısı: {çokgen_sayısı}"
        );
        let (_, veri) = high_low_bands_kartı(HighLowBandsÖrneği::YıllıkSıcaklık)?;
        let low = veri.seriler().first().ok_or(UplotHatası::YetersizVeri {
            uzunluk: veri.seriler().len(),
        })?;
        let high = veri.seriler().get(1).ok_or(UplotHatası::YetersizVeri {
            uzunluk: veri.seriler().len(),
        })?;
        let avg = veri.seriler().get(2).ok_or(UplotHatası::YetersizVeri {
            uzunluk: veri.seriler().len(),
        })?;
        assert!(
            low.get(50..60)
                .is_some_and(|koşu| koşu.iter().all(Option::is_none))
        );
        assert!(
            high.get(50..60)
                .is_some_and(|koşu| koşu.iter().all(Option::is_none))
        );
        assert!(
            low.get(100..110)
                .is_some_and(|koşu| koşu.iter().all(Option::is_none))
        );
        assert!(
            high.get(200..210)
                .is_some_and(|koşu| koşu.iter().all(Option::is_none))
        );
        assert!(
            avg.get(300..310)
                .is_some_and(|koşu| koşu.iter().all(Option::is_none))
        );
        assert_eq!(low.last(), Some(&None));
        assert_eq!(high.last(), Some(&None));
        assert_eq!(avg.last(), Some(&Some(38.0)));
        Ok(())
    }
}
