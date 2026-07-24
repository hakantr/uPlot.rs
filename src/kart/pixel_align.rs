use std::collections::VecDeque;

use super::ortak_kart_etkileşimleri;
use super::veri_uretici::KanıtRastgele;
use crate::{Aralık, GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası};

pub const PIXEL_ALIGN_KANIT_TOHUMU: u32 = 0x5049_5845;
pub const PIXEL_ALIGN_PENCERE_MS: f64 = 120_000.0;
pub const PIXEL_ALIGN_ARALIK_MS: f64 = 1_000.0;
const KANIT_BAŞLANGICI_MS: f64 = 1_700_000_000_000.0;
const PIXEL_ALIGN_HALKA_UZUNLUĞU: usize = 1_000;

pub const PIXEL_ALIGN_KART_TANIM_ÖRNEĞİ: &str = r##"for (örnek, seçenekler, veri) in pixel_align_kartları(121)? {
    // İki uPlot yüzeyi aynı veri örneğini paylaşır; yalnız pxAlign/pxSnap
    // geometrisi değişir. GPUI, SVG ve WASM aynı çekirdek sahnesini tüketir.
    let grafik = Grafik::yeni(seçenekler, veri)?;
}"##;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelAlignÖrneği {
    Varsayılan,
    Kapalı,
}

impl PixelAlignÖrneği {
    pub const TÜMÜ: [Self; 2] = [Self::Varsayılan, Self::Kapalı];

    pub const fn kimlik(self) -> &'static str {
        match self {
            Self::Varsayılan => "pixel-align-default",
            Self::Kapalı => "pixel-align-off",
        }
    }

    pub const fn başlık(self) -> &'static str {
        match self {
            Self::Varsayılan => "pxAlign: 1 (default)",
            Self::Kapalı => "pxAlign: 0 (off)",
        }
    }

    pub fn kimlikten(kimlik: &str) -> Option<Self> {
        Self::TÜMÜ
            .into_iter()
            .find(|örnek| örnek.kimlik() == kimlik)
    }
}

pub struct PixelAlignAkışı {
    rastgele: KanıtRastgele,
    sonraki_indeks: u64,
    şimdi_ms: f64,
    örnek_birikimi_ms: f64,
    x: VecDeque<f64>,
    seriler: [VecDeque<Option<f64>>; 3],
}

impl PixelAlignAkışı {
    pub fn yeni(başlangıç_adımı: usize) -> Result<Self, UplotHatası> {
        let mut akış = Self {
            rastgele: KanıtRastgele::yeni(PIXEL_ALIGN_KANIT_TOHUMU),
            sonraki_indeks: 0,
            şimdi_ms: KANIT_BAŞLANGICI_MS,
            örnek_birikimi_ms: 0.0,
            x: VecDeque::with_capacity(PIXEL_ALIGN_HALKA_UZUNLUĞU),
            seriler: std::array::from_fn(|_| VecDeque::with_capacity(PIXEL_ALIGN_HALKA_UZUNLUĞU)),
        };
        for _ in 0..başlangıç_adımı.clamp(1, 10_000) {
            akış.örnek_ekle();
        }
        akış.şimdi_ms = akış.x.back().copied().unwrap_or(KANIT_BAŞLANGICI_MS);
        Ok(akış)
    }

    fn örnek_ekle(&mut self) {
        if self.x.len() == PIXEL_ALIGN_HALKA_UZUNLUĞU {
            self.x.pop_front();
            for seri in &mut self.seriler {
                seri.pop_front();
            }
        }
        let x = KANIT_BAŞLANGICI_MS + self.sonraki_indeks as f64 * PIXEL_ALIGN_ARALIK_MS;
        self.sonraki_indeks = self.sonraki_indeks.saturating_add(1);
        self.x.push_back(x);
        for (seri, değer) in self.seriler.iter_mut().zip([
            0.0 - self.rastgele.sonraki(),
            -1.0 - self.rastgele.sonraki(),
            -2.0 - self.rastgele.sonraki(),
        ]) {
            seri.push_back(Some(değer));
        }
    }

    /// Tek animation-frame saatini ilerletir. Dönüş değeri bu karede yeni
    /// bir 1 Hz veri örneği eklendiğini bildirir.
    pub fn kareyi_ilerlet(&mut self, geçen_ms: f64) -> bool {
        if !geçen_ms.is_finite() || geçen_ms <= 0.0 {
            return false;
        }
        self.şimdi_ms += geçen_ms;
        self.örnek_birikimi_ms += geçen_ms;
        let mut veri_değişti = false;
        while self.örnek_birikimi_ms >= PIXEL_ALIGN_ARALIK_MS {
            self.örnek_birikimi_ms -= PIXEL_ALIGN_ARALIK_MS;
            self.örnek_ekle();
            veri_değişti = true;
        }
        veri_değişti
    }

    pub fn veri(&self) -> Result<HizalıVeri, UplotHatası> {
        HizalıVeri::yeni(
            self.x.iter().copied().collect(),
            self.seriler
                .iter()
                .map(|seri| seri.iter().copied().collect())
                .collect(),
        )
    }

    pub fn görünür_x_aralığı(&self) -> Result<Aralık, UplotHatası> {
        Aralık::yeni(self.şimdi_ms - PIXEL_ALIGN_PENCERE_MS, self.şimdi_ms)
    }

    pub fn örnek_sayısı(&self) -> usize {
        self.x.len()
    }
}

/// `demos/pixel-align.html` içindeki 120 saniyelik pencereyi, bir saniyelik
/// örnekleme aralığını ve üç rastgele seri üretecini yeniden kurar.
///
/// Tarayıcıdaki `Date.now()` yerine görsel kanıtların tekrar üretilebilmesi
/// için sabit bir başlangıç; `Math.random()` yerine açık bir kanıt tohumu
/// kullanılır. Üreteç ve parametreler kaynakla aynıdır.
pub fn pixel_align_kartı(
    örnek: PixelAlignÖrneği,
    adım_sayısı: usize,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let (veri, şimdi) = pixel_align_verisi(adım_sayısı)?;
    Ok((pixel_align_seçenekleri(örnek, şimdi)?, veri))
}

/// Kaynak sayfadaki iki canlı yüzeyi tek aile olarak ve aynı veri anlık
/// görüntüsüyle üretir. Yüzeyler ayrı görünüm durumuna, ortak veri/tick
/// zamanına sahiptir.
pub fn pixel_align_kartları(
    adım_sayısı: usize,
) -> Result<Vec<(PixelAlignÖrneği, GrafikSeçenekleri, HizalıVeri)>, UplotHatası> {
    let (veri, şimdi) = pixel_align_verisi(adım_sayısı)?;
    PixelAlignÖrneği::TÜMÜ
        .into_iter()
        .map(|örnek| {
            pixel_align_seçenekleri(örnek, şimdi)
                .map(|seçenekler| (örnek, seçenekler, veri.clone()))
        })
        .collect()
}

fn pixel_align_verisi(adım_sayısı: usize) -> Result<(HizalıVeri, f64), UplotHatası> {
    let akış = PixelAlignAkışı::yeni(adım_sayısı)?;
    let şimdi = akış.görünür_x_aralığı()?.en_çok;
    Ok((akış.veri()?, şimdi))
}

fn pixel_align_seçenekleri(
    örnek: PixelAlignÖrneği,
    şimdi: f64,
) -> Result<GrafikSeçenekleri, UplotHatası> {
    let hizalama = match örnek {
        PixelAlignÖrneği::Varsayılan => 1.0,
        PixelAlignÖrneği::Kapalı => 0.0,
    };
    let ortak_seri = |etiket: &str, renk: &str| {
        SeriSeçenekleri::yeni(etiket)
            .renk(renk)
            .boşlukları_birleştir(true)
            .noktaları_göster(true)
            .piksel_hizası(hizalama)
    };
    let seçenekler = GrafikSeçenekleri::yeni(1_200, 400)?
        .başlık(örnek.başlık())
        .duyarlı_boyut(true)
        .x_zaman(true)
        .x_zaman_milisaniye(true)
        .x_aralığı(Aralık::yeni(şimdi - PIXEL_ALIGN_PENCERE_MS, şimdi)?)
        .y_aralığı(Aralık::yeni(-3.5, 1.5)?)
        .piksel_hizası(hizalama)
        .seri(ortak_seri("Linear", "red"))
        .seri(ortak_seri("Spline", "blue").eğri())
        .seri(ortak_seri("Stepped", "purple").basamak_sonra())
        .etkileşimler(ortak_kart_etkileşimleri());
    Ok(seçenekler)
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut};

    fn doğrusal_yol_noktaları(grafik: &Grafik) -> Vec<crate::Nokta> {
        grafik
            .çiz()
            .komutlar()
            .iter()
            .filter_map(|komut| match komut {
                Komut::Yol {
                    parçalar, renk, ..
                } if renk == "red" => Some(parçalar),
                _ => None,
            })
            .flat_map(|parçalar| parçalar.iter().flatten().copied())
            .collect()
    }

    #[test]
    fn kaynak_penceresi_ve_üç_yol_türü_korunur() -> Result<(), UplotHatası> {
        let kartlar = pixel_align_kartları(140)?;
        assert_eq!(kartlar.len(), 2);
        for (örnek, seçenekler, veri) in kartlar {
            assert_eq!(veri.uzunluk(), 140);
            assert_eq!(veri.seriler().len(), 3);
            assert_eq!(seçenekler.başlık, örnek.başlık());
            assert!(seçenekler.duyarlı_boyut);
            assert_eq!(seçenekler.y_aralığı, Some(Aralık::yeni(-3.5, 1.5)?));
            assert!(
                seçenekler
                    .seriler
                    .iter()
                    .all(|seri| seri.noktaları_göster == Some(true))
            );
        }
        Ok(())
    }

    #[test]
    fn varsayılan_tam_piksele_hizalanır_kapalı_alt_pikseli_korur() -> Result<(), UplotHatası> {
        let (hizalı_seçenekler, hizalı_veri) =
            pixel_align_kartı(PixelAlignÖrneği::Varsayılan, 121)?;
        let hizalı_grafik = Grafik::yeni(hizalı_seçenekler, hizalı_veri)?;
        let hizalı = doğrusal_yol_noktaları(&hizalı_grafik);
        assert!(!hizalı.is_empty());
        assert!(
            hizalı
                .iter()
                .all(|nokta| nokta.x.fract() == 0.0 && nokta.y.fract() == 0.0)
        );

        let (serbest_seçenekler, serbest_veri) = pixel_align_kartı(PixelAlignÖrneği::Kapalı, 121)?;
        let serbest_grafik = Grafik::yeni(serbest_seçenekler, serbest_veri)?;
        let serbest = doğrusal_yol_noktaları(&serbest_grafik);
        assert!(
            serbest
                .iter()
                .any(|nokta| nokta.x.fract() != 0.0 || nokta.y.fract() != 0.0)
        );
        Ok(())
    }

    #[test]
    fn aynı_adım_sayısı_aynı_kanıt_verisini_üretir() -> Result<(), UplotHatası> {
        let (_, sol) = pixel_align_kartı(PixelAlignÖrneği::Varsayılan, 257)?;
        let (_, sağ) = pixel_align_kartı(PixelAlignÖrneği::Kapalı, 257)?;
        assert_eq!(sol, sağ);
        Ok(())
    }

    #[test]
    fn kaynak_döngüsü_en_fazla_bin_örneği_tutar() -> Result<(), UplotHatası> {
        let (_, veri) = pixel_align_kartı(PixelAlignÖrneği::Varsayılan, 1_200)?;
        assert_eq!(veri.uzunluk(), 1_000);
        Ok(())
    }

    #[test]
    fn canlı_akış_on_bin_adımdan_sonra_donmadan_halkayı_sürdürür() -> Result<(), UplotHatası> {
        let mut akış = PixelAlignAkışı::yeni(140)?;
        let ilk_son = akış.veri()?.x().last().copied();
        for _ in 0..10_100 {
            assert!(akış.kareyi_ilerlet(PIXEL_ALIGN_ARALIK_MS));
        }
        let veri = akış.veri()?;
        assert_eq!(akış.örnek_sayısı(), 1_000);
        assert!(veri.x().last().copied() > ilk_son);
        assert_eq!(
            akış.görünür_x_aralığı()?.en_çok - akış.görünür_x_aralığı()?.en_az,
            PIXEL_ALIGN_PENCERE_MS
        );
        Ok(())
    }

    #[test]
    fn frame_saati_veri_eklemeden_kayan_x_aralığını_ilerletir() -> Result<(), UplotHatası> {
        let mut akış = PixelAlignAkışı::yeni(140)?;
        let önce = akış.görünür_x_aralığı()?;
        assert!(!akış.kareyi_ilerlet(16.0));
        let sonra = akış.görünür_x_aralığı()?;
        assert_eq!(sonra.en_çok - önce.en_çok, 16.0);
        assert_eq!(akış.örnek_sayısı(), 140);
        Ok(())
    }

    #[test]
    fn canlı_x_penceresi_grafiği_yeniden_kurmadan_ilerler_ve_zoomu_korur() -> Result<(), UplotHatası>
    {
        let (seçenekler, veri) = pixel_align_kartı(PixelAlignÖrneği::Kapalı, 140)?;
        let mut grafik = Grafik::yeni(seçenekler, veri)?;
        let ilk_yol = doğrusal_yol_noktaları(&grafik);
        let ilk = grafik.görünür_x_aralığı();
        let kayan = Aralık::yeni(ilk.en_az + 16.0, ilk.en_çok + 16.0)?;
        assert!(grafik.canlı_x_aralığını_ayarla(kayan));
        assert_eq!(grafik.görünür_x_aralığı(), kayan);
        assert_ne!(doğrusal_yol_noktaları(&grafik), ilk_yol);
        assert!(matches!(
            grafik.seçimi_bitir(0.25, 0.75, false)?,
            crate::SeçimEylemi::Yakınlaştırıldı
        ));
        let yakın = grafik.görünür_x_aralığı();
        let sonraki = Aralık::yeni(kayan.en_az + 16.0, kayan.en_çok + 16.0)?;
        assert!(!grafik.canlı_x_aralığını_ayarla(sonraki));
        assert_eq!(grafik.görünür_x_aralığı(), yakın);
        assert!(grafik.tam_görünüm());
        assert_eq!(grafik.görünür_x_aralığı(), sonraki);
        Ok(())
    }
}
