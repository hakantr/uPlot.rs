use super::{ortak_kart_etkileşimleri, veri_uretici::KanıtRastgele};
use crate::{Aralık, GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası};

pub const TIMESERIES_DISCRETE_KANIT_TOHUMU: u32 = 0x5453_4449;
pub const TIMESERIES_DISCRETE_KART_TANIM_ÖRNEĞİ: &str = r##"let yüzeyler = TimeseriesDiscreteÖrneği::TÜMÜ
    .map(timeseries_discrete_kartı);
let mut grup = TimeseriesDiscreteGrubu::yeni();
// İki yüzey aynı X imlecini paylaşır; birleşik lejant değerleri çekirdektedir.
grup.imleci_güncelle(0.5);
"##;

const BAŞLANGIÇ: f64 = 1_704_067_200.0;
const ADIM: f64 = 15.0;
const NOKTA_SAYISI: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeseriesDiscreteÖrneği {
    ZamanSerisi,
    AyrıkDurumlar,
}

impl TimeseriesDiscreteÖrneği {
    pub const TÜMÜ: [Self; 2] = [Self::ZamanSerisi, Self::AyrıkDurumlar];

    pub const fn kimlik(self) -> &'static str {
        match self {
            Self::ZamanSerisi => "timeseries-discrete-floats",
            Self::AyrıkDurumlar => "timeseries-discrete-devices",
        }
    }

    pub const fn başlık(self) -> &'static str {
        match self {
            Self::ZamanSerisi => "TimeSeries · float values",
            Self::AyrıkDurumlar => "Discrete device states",
        }
    }

    pub const fn boyut(self) -> (u32, u32) {
        match self {
            Self::ZamanSerisi => (1_920, 600),
            Self::AyrıkDurumlar => (1_920, 200),
        }
    }

    pub fn kimlikten(kimlik: &str) -> Option<Self> {
        Self::TÜMÜ
            .into_iter()
            .find(|örnek| örnek.kimlik() == kimlik)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct TimeseriesDiscreteGrubu {
    yatay_oran: Option<f64>,
}

impl TimeseriesDiscreteGrubu {
    pub const fn yeni() -> Self {
        Self { yatay_oran: None }
    }

    pub fn imleci_güncelle(&mut self, yatay_oran: f64) -> bool {
        if !yatay_oran.is_finite() {
            return false;
        }
        let yeni = yatay_oran.clamp(0.0, 1.0);
        let değişti = self
            .yatay_oran
            .is_none_or(|önceki| (önceki - yeni).abs() > f64::EPSILON);
        self.yatay_oran = Some(yeni);
        değişti
    }

    pub const fn yatay_oran(&self) -> Option<f64> {
        self.yatay_oran
    }

    pub fn birleşik_lejant(
        &self,
        zaman_serisi: &crate::Grafik,
        ayrık_seriler: &crate::Grafik,
    ) -> Option<(f64, Vec<Option<f64>>)> {
        let oran = self.yatay_oran?;
        let (x, mut değerler) = zaman_serisi.en_yakın_noktalar(oran)?;
        let (_, ayrık_değerler) = ayrık_seriler.en_yakın_noktalar(oran)?;
        değerler.extend(ayrık_değerler);
        Some((x, değerler))
    }
}

pub fn timeseries_discrete_kartı(
    örnek: TimeseriesDiscreteÖrneği,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let (x, floatlar, ayrıklar) = kaynak_veri();
    match örnek {
        TimeseriesDiscreteÖrneği::ZamanSerisi => {
            let seçenekler = GrafikSeçenekleri::yeni(1_920, 600)?
                .x_ekseni_göster(false)
                .seri(
                    SeriSeçenekleri::yeni("Value")
                        .renk("red")
                        .noktaları_göster(false),
                )
                .etkileşimler(ortak_kart_etkileşimleri());
            Ok((
                seçenekler,
                HizalıVeri::yeni(x, vec![floatlar.into_iter().map(Some).collect()])?,
            ))
        }
        TimeseriesDiscreteÖrneği::AyrıkDurumlar => {
            let renkler = ["green", "blue", "magenta"];
            let mut seçenekler = GrafikSeçenekleri::yeni(1_920, 200)?
                .y_aralığı(Aralık::yeni(0.0, 5.0)?)
                .y_sabit_bölmeler(vec![0.0, 2.0, 4.0])
                .y_özel_etiketler([(0.0, "DEV1"), (2.0, "DEV2"), (4.0, "DEV3")])
                .etkileşimler(ortak_kart_etkileşimleri());
            let mut çizim_serileri = Vec::with_capacity(3);
            for (indeks, ham) in ayrıklar.into_iter().enumerate() {
                let kaydırma = indeks as f64 * 2.0;
                seçenekler = seçenekler.seri(
                    SeriSeçenekleri::yeni(format!("DEV{}", indeks + 1))
                        .renk(renkler.get(indeks).copied().unwrap_or("black"))
                        .basamak_sonra()
                        .lejant_değerleri(ham.iter().map(|değer| Some(*değer)).collect())
                        .noktaları_göster(false),
                );
                çizim_serileri.push(
                    ham.into_iter()
                        .map(|değer| Some(değer + kaydırma))
                        .collect(),
                );
            }
            Ok((seçenekler, HizalıVeri::yeni(x, çizim_serileri)?))
        }
    }
}

fn kaynak_veri() -> (Vec<f64>, Vec<f64>, [Vec<f64>; 3]) {
    let x = (0..NOKTA_SAYISI)
        .map(|indeks| BAŞLANGIÇ + ADIM * indeks as f64)
        .collect();
    let mut rastgele = KanıtRastgele::yeni(TIMESERIES_DISCRETE_KANIT_TOHUMU);
    let floatlar = (0..NOKTA_SAYISI)
        .map(|_| rastgele.sonraki() * 100.0)
        .collect();
    let ayrıklar = [0.5, 0.2, 0.1].map(|eşik| {
        (0..NOKTA_SAYISI)
            .map(|_| if rastgele.sonraki() > eşik { 0.0 } else { 1.0 })
            .collect()
    });
    (x, floatlar, ayrıklar)
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut, SeriÇizimTürü};

    #[test]
    fn iki_kaynak_yüzeyi_aynı_elli_zaman_noktasını_kullanır() -> Result<(), UplotHatası> {
        let (üst_seçenekler, üst_veri) =
            timeseries_discrete_kartı(TimeseriesDiscreteÖrneği::ZamanSerisi)?;
        let (alt_seçenekler, alt_veri) =
            timeseries_discrete_kartı(TimeseriesDiscreteÖrneği::AyrıkDurumlar)?;
        assert_eq!(üst_veri.x(), alt_veri.x());
        assert!(!üst_seçenekler.x_eksen_görünür);
        assert_eq!(üst_veri.uzunluk(), 50);
        assert_eq!(üst_veri.x().first(), Some(&1_704_067_200.0));
        assert_eq!(üst_veri.x().last(), Some(&1_704_067_935.0));
        assert_eq!(
            alt_seçenekler
                .seriler
                .iter()
                .map(|seri| seri.çizim_türü)
                .collect::<Vec<_>>(),
            vec![SeriÇizimTürü::BasamakSonra; 3]
        );
        let sahne = Grafik::yeni(üst_seçenekler, üst_veri)?.çiz();
        assert!(
            sahne
                .komutlar()
                .iter()
                .any(|komut| { matches!(komut, Komut::Yol { renk, .. } if renk == "red") })
        );
        Ok(())
    }

    #[test]
    fn ayrık_değerler_çizimde_kaydırılır_lejantta_sıfır_bir_kalır() -> Result<(), UplotHatası> {
        let (seçenekler, veri) =
            timeseries_discrete_kartı(TimeseriesDiscreteÖrneği::AyrıkDurumlar)?;
        assert!(veri.seriler().get(1).is_some_and(|seri| {
            seri.iter()
                .flatten()
                .all(|değer| (2.0..=3.0).contains(değer))
        }));
        assert!(seçenekler.seriler.get(2).is_some_and(|seri| {
            seri.lejant_değerleri.as_ref().is_some_and(|değerler| {
                değerler
                    .iter()
                    .flatten()
                    .all(|değer| *değer == 0.0 || *değer == 1.0)
            })
        }));
        Ok(())
    }

    #[test]
    fn birleşik_lejant_iki_yüzeyin_dört_serisini_döndürür() -> Result<(), UplotHatası> {
        let (üst_seçenekler, üst_veri) =
            timeseries_discrete_kartı(TimeseriesDiscreteÖrneği::ZamanSerisi)?;
        let (alt_seçenekler, alt_veri) =
            timeseries_discrete_kartı(TimeseriesDiscreteÖrneği::AyrıkDurumlar)?;
        let üst = Grafik::yeni(üst_seçenekler, üst_veri)?;
        let alt = Grafik::yeni(alt_seçenekler, alt_veri)?;
        let mut grup = TimeseriesDiscreteGrubu::yeni();
        assert!(grup.imleci_güncelle(0.5));
        let (_, değerler) = grup
            .birleşik_lejant(&üst, &alt)
            .ok_or(UplotHatası::YetersizVeri { uzunluk: 0 })?;
        assert_eq!(değerler.len(), 4);
        assert!(değerler.iter().all(Option::is_some));
        Ok(())
    }
}
