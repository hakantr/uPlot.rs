use super::ortak_kart_etkileşimleri;
use crate::{
    GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası, YÖlçekSeçenekleri
};

const VARLIK: &str = "assets/data/missing-data.csv";

pub const MISSING_DATA_KART_TANIM_ÖRNEĞİ: &str = r##"let (seçenekler, veri) = missing_data_null_kartı()?;
let null_yüzeyi = Grafik::yeni(seçenekler, veri)?;
let (seçenekler, veri) = missing_data_x_boşluğu_kartı()?;
let x_boşluğu_yüzeyi = Grafik::yeni(seçenekler, veri)?;

// İki bağımsız Grafik aynı resmî sayfanın tek karşılaştırma grubudur.
let yüzeyler = missing_data_kartları()?;"##;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MissingDataÖrneği {
    NullDeğerler,
    KomşuXBoşluğu,
}

impl MissingDataÖrneği {
    pub const TÜMÜ: [Self; 2] = [Self::NullDeğerler, Self::KomşuXBoşluğu];

    pub const fn kimlik(self) -> &'static str {
        match self {
            Self::NullDeğerler => "missing-data-null",
            Self::KomşuXBoşluğu => "missing-data-x-gap",
        }
    }

    pub const fn başlık(self) -> &'static str {
        match self {
            Self::NullDeğerler => "Missing Data (null values)",
            Self::KomşuXBoşluğu => "Insert gaps when adjacent points > delta",
        }
    }

    pub const fn açıklama(self) -> &'static str {
        match self {
            Self::NullDeğerler => {
                "Gerçek null örnekler CPU/RAM yollarını böler; TCP Out bağımsız MB ölçeğindedir."
            }
            Self::KomşuXBoşluğu => {
                "Değerler dolu olsa da komşu X farkı 1'i aşınca series.gaps yolu böler."
            }
        }
    }
}

pub fn missing_data_kartları()
-> Result<Vec<(MissingDataÖrneği, GrafikSeçenekleri, HizalıVeri)>, UplotHatası> {
    MissingDataÖrneği::TÜMÜ
        .into_iter()
        .map(|örnek| {
            let (seçenekler, veri) = match örnek {
                MissingDataÖrneği::NullDeğerler => missing_data_null_kartı()?,
                MissingDataÖrneği::KomşuXBoşluğu => missing_data_x_boşluğu_kartı()?,
            };
            Ok((örnek, seçenekler, veri))
        })
        .collect()
}

/// `demos/missing-data.html` içindeki null CPU/RAM noktalarını, bağımsız `%`
/// ve `mb` ölçeklerini ve özgün veri kümesini kurar.
pub fn missing_data_null_kartı() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let veri = csv_verisi()?;
    let seçenekler = GrafikSeçenekleri::yeni(1920, 600)?
        .başlık("Missing Data (null values)")
        .birincil_y_ölçeği("%")
        .y_ölçeği(YÖlçekSeçenekleri::yeni("%").birim("%"))
        .y_ölçeği(
            YÖlçekSeçenekleri::yeni("mb")
                .sağda(true)
                .ızgara(false)
                .birim("MB"),
        )
        .etkileşimler(ortak_kart_etkileşimleri())
        .seri(
            SeriSeçenekleri::yeni("CPU")
                .renk("#ff0000")
                .dolgu("#ff00000d")
                .ölçek("%"),
        )
        .seri(
            SeriSeçenekleri::yeni("RAM")
                .renk("#0000ff")
                .dolgu("#0000ff0d")
                .ölçek("%"),
        )
        .seri(
            SeriSeçenekleri::yeni("TCP Out")
                .renk("#008000")
                .dolgu("#00ff000d")
                .ölçek("mb"),
        );
    Ok((seçenekler, veri))
}

/// Kaynak demonun `series.gaps` callback'ini, komşu X değerleri arasındaki
/// fark birden büyük olduğunda yolu bölerek taşır.
pub fn missing_data_x_boşluğu_kartı() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let x = vec![0.0, 1.0, 2.0, 3.0, 25.0, 26.0, 27.0, 28.0];
    let y = (0_u32..=7).map(|değer| Some(f64::from(değer))).collect();
    let seçenekler = GrafikSeçenekleri::yeni(1920, 600)?
        .başlık("Insert gaps when adjacent points > delta")
        .x_zaman(false)
        .etkileşimler(ortak_kart_etkileşimleri())
        .seri(
            SeriSeçenekleri::yeni("Value")
                .renk("#ff0000")
                .azami_x_boşluğu(1.0),
        );
    Ok((seçenekler, HizalıVeri::yeni(x, vec![y])?))
}

fn csv_verisi() -> Result<HizalıVeri, UplotHatası> {
    let mut x = Vec::new();
    let mut cpu = Vec::new();
    let mut ram = Vec::new();
    let mut tcp = Vec::new();

    for (indeks, satır) in include_str!("../../../assets/data/missing-data.csv")
        .lines()
        .skip(1)
        .enumerate()
    {
        let satır_no = indeks.saturating_add(2);
        let mut sütunlar = satır.split(',');
        let zaman = sayı(sütunlar.next(), satır_no)?;
        let cpu_değeri = isteğe_bağlı_sayı(sütunlar.next(), satır_no)?;
        let ram_değeri = isteğe_bağlı_sayı(sütunlar.next(), satır_no)?;
        let tcp_değeri = isteğe_bağlı_sayı(sütunlar.next(), satır_no)?;
        if sütunlar.next().is_some() {
            return Err(UplotHatası::GeçersizVarlıkSatırı {
                varlık: VARLIK,
                satır: satır_no,
            });
        }
        x.push(zaman);
        cpu.push(cpu_değeri);
        ram.push(ram_değeri);
        tcp.push(tcp_değeri);
    }
    HizalıVeri::yeni(x, vec![cpu, ram, tcp])
}

fn sayı(değer: Option<&str>, satır: usize) -> Result<f64, UplotHatası> {
    değer
        .and_then(|metin| metin.parse::<f64>().ok())
        .filter(|sayı| sayı.is_finite())
        .ok_or(UplotHatası::GeçersizVarlıkSatırı {
            varlık: VARLIK,
            satır,
        })
}

fn isteğe_bağlı_sayı(değer: Option<&str>, satır: usize) -> Result<Option<f64>, UplotHatası> {
    match değer {
        Some("") => Ok(None),
        Some(metin) => sayı(Some(metin), satır).map(Some),
        None => Err(UplotHatası::GeçersizVarlıkSatırı {
            varlık: VARLIK,
            satır,
        }),
    }
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut};

    #[test]
    fn özgün_null_verisi_ve_iki_ölçek_korunur() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = missing_data_null_kartı()?;
        assert_eq!(veri.uzunluk(), 200);
        assert_eq!(
            veri.seriler()
                .first()
                .map(|seri| seri.iter().filter(|v| v.is_none()).count()),
            Some(6)
        );
        assert_eq!(
            veri.seriler()
                .get(1)
                .map(|seri| seri.iter().filter(|v| v.is_none()).count()),
            Some(6)
        );
        assert_eq!(
            seçenekler.seriler.get(2).map(|seri| seri.renk.as_str()),
            Some("#008000")
        );
        let mut grafik = Grafik::yeni(seçenekler, veri)?;
        assert!(
            grafik
                .seri_görünür_y_aralığı(2)
                .is_some_and(|aralık| aralık.en_çok < 0.04)
        );
        let sahne = grafik.çiz();
        let yol_parçaları = sahne
            .komutlar()
            .iter()
            .filter_map(|komut| match komut {
                Komut::Yol { parçalar, .. } => Some(parçalar.len()),
                _ => None,
            })
            .sum::<usize>();
        assert_eq!(yol_parçaları, 7);
        assert!(
            sahne.komutlar().iter().any(
                |komut| matches!(komut, Komut::Metin { içerik, .. } if içerik.ends_with("MB"))
            )
        );
        assert!(sahne.komutlar().iter().any(
            |komut| matches!(komut, Komut::Metin { içerik, .. } if içerik.ends_with('%') && !içerik.contains(" %"))
        ));
        assert!(grafik.seri_görünürlüğünü_ayarla(1, false)?);
        assert!(
            grafik
                .seri_görünür_y_aralığı(0)
                .is_some_and(|aralık| aralık.en_çok < 6.0)
        );
        Ok(())
    }

    #[test]
    fn büyük_x_farkı_yolu_ikiye_böler() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = missing_data_x_boşluğu_kartı()?;
        let sahne = Grafik::yeni(seçenekler, veri)?.çiz();
        assert!(
            sahne
                .komutlar()
                .iter()
                .any(|komut| matches!(komut, Komut::Yol { parçalar, .. } if parçalar.len() == 2))
        );
        Ok(())
    }

    #[test]
    fn aynı_kaynak_sayfasının_iki_yüzeyi_tek_grupta_üretilir() -> Result<(), UplotHatası> {
        let kartlar = missing_data_kartları()?;
        assert_eq!(kartlar.len(), 2);
        assert_eq!(
            kartlar
                .iter()
                .map(|(örnek, _, _)| örnek.kimlik())
                .collect::<Vec<_>>(),
            vec!["missing-data-null", "missing-data-x-gap"]
        );
        Ok(())
    }

    #[test]
    fn seyrekleştirme_gerçek_olmayan_komşu_x_boşluğu_üretmez() -> Result<(), UplotHatası> {
        let x = (0_u32..10_000).map(f64::from).collect::<Vec<_>>();
        let y = x.iter().map(|değer| Some(değer.sin())).collect();
        let seçenekler = GrafikSeçenekleri::yeni(320, 200)?.x_zaman(false).seri(
            SeriSeçenekleri::yeni("Value")
                .renk("red")
                .azami_x_boşluğu(1.0),
        );
        let sahne = Grafik::yeni(seçenekler, HizalıVeri::yeni(x, vec![y])?)?.çiz();
        assert!(sahne.komutlar().iter().any(
            |komut| matches!(komut, Komut::Yol { parçalar, renk, .. } if renk == "red" && parçalar.len() == 1)
        ));
        Ok(())
    }
}
