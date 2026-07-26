use std::collections::VecDeque;
use web_time::{SystemTime, UNIX_EPOCH};

use super::ortak_kart_etkileşimleri;
use super::veri_uretici::KanıtRastgele;
use crate::{Aralık, GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası};

pub const SINE_STREAM_KANIT_TOHUMU: u32 = 0x51A5_0045;
pub const SINE_STREAM_NOKTA_SAYISI: usize = 600;
pub const SINE_STREAM_KART_TANIM_ÖRNEĞİ: &str = r##"let mut akış = SineAkışı::yeni()?;
let (seçenekler, veri) = akış.kartı()?;
let mut grafik = Grafik::yeni(seçenekler, veri)?;

// Platform adaptörü her animation-frame adımında yalnız bunu çağırır.
grafik.veriyi_ayarla(akış.ilerlet()?)?;"##;

pub const SINE_STREAM_KANIT_BAŞLANGICI_SN: f64 = 1_700_000_000.0;
const ÖRNEK_ARALIĞI_SN: f64 = 5.0 * 60.0;

/// `sine-stream.html` içindeki Box–Muller yürüyüşlerini, kaynak başlangıç
/// yaşam döngüsünü ve 600 noktalı kayan pencereyi tutan platformdan bağımsız
/// canlı veri durumu.
pub struct SineAkışı {
    başlangıç_sn: f64,
    kaydırma: usize,
    x: VecDeque<f64>,
    seriler: [VecDeque<f64>; 6],
    normal: BoxMuller,
}

impl SineAkışı {
    pub fn yeni() -> Result<Self, UplotHatası> {
        let şimdi = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|hata| UplotHatası::GeçersizKaynakVeri {
                varlık: "SineAkışı",
                açıklama: format!("sistem saati Unix epoch öncesinde: {hata}"),
            })?;
        let başlangıç = şimdi.as_secs() as f64;
        let tohum = SINE_STREAM_KANIT_TOHUMU
            ^ şimdi.subsec_nanos()
            ^ (şimdi.as_secs() as u32).rotate_left(13);
        Self::başlangıçla(başlangıç, tohum)
    }

    /// Görsel regresyonlar ve karşılaştırılabilir ölçümler için sabit epoch
    /// ve tohum kullanan üretimle aynı akış.
    pub fn kanıt() -> Result<Self, UplotHatası> {
        Self::başlangıçla(SINE_STREAM_KANIT_BAŞLANGICI_SN, SINE_STREAM_KANIT_TOHUMU)
    }

    fn başlangıçla(başlangıç: f64, tohum: u32) -> Result<Self, UplotHatası> {
        let mut normal = BoxMuller::yeni(tohum);
        let x = (0..SINE_STREAM_NOKTA_SAYISI)
            .map(|indeks| başlangıç + indeks as f64 * ÖRNEK_ARALIĞI_SN)
            .collect();
        let sinüs = (0..SINE_STREAM_NOKTA_SAYISI)
            .map(|indeks| (indeks as f64 / 16.0).sin() * 5.0)
            .collect();
        let eksi_dört =
            rastgele_yürüyüş(SINE_STREAM_NOKTA_SAYISI, -4.0, -6.0, 1.0, &mut normal);
        let eksi_iki = rastgele_yürüyüş(SINE_STREAM_NOKTA_SAYISI, -2.0, -6.0, 1.0, &mut normal);
        let sıfır = rastgele_yürüyüş(SINE_STREAM_NOKTA_SAYISI, 0.0, -2.0, 2.0, &mut normal);
        let artı_iki = rastgele_yürüyüş(SINE_STREAM_NOKTA_SAYISI, 2.0, -1.0, 6.0, &mut normal);
        let artı_dört = rastgele_yürüyüş(SINE_STREAM_NOKTA_SAYISI, 4.0, -1.0, 6.0, &mut normal);
        let mut akış = Self {
            // Kaynak `getData(599)` ile grafiği kurar, ardından `update()`
            // senkron olarak 600. örneği uygular. İlk boyanan karedeki
            // yinelenen x=599 davranışı da bu iki adımla korunur.
            başlangıç_sn: başlangıç,
            kaydırma: SINE_STREAM_NOKTA_SAYISI - 1,
            x,
            seriler: [sinüs, eksi_dört, eksi_iki, sıfır, artı_iki, artı_dört],
            normal,
        };
        akış.adımı_uygula()?;
        akış.kaydırma = akış.kaydırma.saturating_add(1);
        akış.adımı_uygula()?;
        Ok(akış)
    }

    pub fn kartı(&self) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
        Ok((sine_stream_seçenekleri()?, self.veri()?))
    }

    pub fn veri(&self) -> Result<HizalıVeri, UplotHatası> {
        HizalıVeri::yeni(
            self.x.iter().copied().collect(),
            self.seriler
                .iter()
                .map(|seri| seri.iter().copied().map(Some).collect())
                .collect(),
        )
    }

    pub fn ilerlet(&mut self) -> Result<HizalıVeri, UplotHatası> {
        self.kaydırma = self.kaydırma.saturating_add(1);
        self.adımı_uygula()?;
        self.veri()
    }

    pub fn kaydırma(&self) -> usize {
        self.kaydırma
    }

    fn adımı_uygula(&mut self) -> Result<(), UplotHatası> {
        let son_değerler = [
            (self.kaydırma as f64 / 16.0).sin() * 5.0,
            sonraki_yürüyüş(self.seriler.get(1), -6.0, -1.0, &mut self.normal)?,
            sonraki_yürüyüş(self.seriler.get(2), -6.0, -1.0, &mut self.normal)?,
            sonraki_yürüyüş(self.seriler.get(3), -2.0, 2.0, &mut self.normal)?,
            sonraki_yürüyüş(self.seriler.get(4), -1.0, 6.0, &mut self.normal)?,
            sonraki_yürüyüş(self.seriler.get(5), -1.0, 6.0, &mut self.normal)?,
        ];
        self.x.pop_front();
        self.x
            .push_back(self.başlangıç_sn + self.kaydırma as f64 * ÖRNEK_ARALIĞI_SN);
        for (seri, değer) in self.seriler.iter_mut().zip(son_değerler) {
            seri.pop_front();
            seri.push_back(değer);
        }
        Ok(())
    }
}

pub fn sine_stream_kartı() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    SineAkışı::yeni()?.kartı()
}

fn sine_stream_seçenekleri() -> Result<GrafikSeçenekleri, UplotHatası> {
    let stiller = [
        ("Sine", "red", "rgba(255,0,0,0.1)"),
        ("Value", "green", "#4caf505e"),
        ("Value", "blue", "#0000ff20"),
        ("Value", "orange", "#ffa5004f"),
        ("Value", "magenta", "#ff00ff20"),
        ("Value", "purple", "#80008020"),
    ];
    let mut seçenekler = GrafikSeçenekleri::yeni(1_920, 600)?
        .başlık("6 series x 600 points @ 60fps")
        .x_zaman(true)
        .x_eksen_asgari_etiket_boşluğu(300.0)
        .y_aralığı(Aralık::yeni(-6.0, 6.0)?)
        .piksel_hizası(0.0)
        .etkileşimler(ortak_kart_etkileşimleri());
    for (etiket, renk, dolgu) in stiller {
        seçenekler = seçenekler.seri(
            SeriSeçenekleri::yeni(etiket)
                .renk(renk)
                .dolgu(dolgu)
                .piksel_hizası(0.0),
        );
    }
    Ok(seçenekler)
}

fn sonraki_yürüyüş(
    seri: Option<&VecDeque<f64>>,
    en_az: f64,
    en_çok: f64,
    normal: &mut BoxMuller,
) -> Result<f64, UplotHatası> {
    let Some(son) = seri.and_then(|değerler| değerler.back()).copied() else {
        return Err(UplotHatası::GeçersizKaynakVeri {
            varlık: "SineAkışı",
            açıklama: "rastgele yürüyüşün son değeri bulunamadı".to_string(),
        });
    };
    Ok(yürüyüş_adımı(son, en_az, en_çok, normal.sonraki()))
}

fn rastgele_yürüyüş(
    adım: usize,
    mut değer: f64,
    en_az: f64,
    en_çok: f64,
    normal: &mut BoxMuller,
) -> VecDeque<f64> {
    (0..adım)
        .map(|_| {
            değer = yürüyüş_adımı(değer, en_az, en_çok, normal.sonraki());
            değer
        })
        .collect()
}

fn yürüyüş_adımı(değer: f64, en_az: f64, en_çok: f64, ek: f64) -> f64 {
    let yeni = değer + ek;
    if yeni > en_çok || yeni < en_az {
        (değer - ek).clamp(en_az, en_çok)
    } else {
        yeni
    }
}

struct BoxMuller {
    rastgele: KanıtRastgele,
    ikinci: Option<f64>,
}

impl BoxMuller {
    fn yeni(tohum: u32) -> Self {
        Self {
            rastgele: KanıtRastgele::yeni(tohum),
            ikinci: None,
        }
    }

    fn sonraki(&mut self) -> f64 {
        if let Some(değer) = self.ikinci.take() {
            return değer;
        }
        loop {
            let x1 = 2.0 * self.rastgele.sonraki() - 1.0;
            let x2 = 2.0 * self.rastgele.sonraki() - 1.0;
            let w = x1 * x1 + x2 * x2;
            if !(f64::EPSILON..1.0).contains(&w) {
                continue;
            }
            let çarpan = (-2.0 * w.ln() / w).sqrt();
            self.ikinci = Some(x2 * çarpan);
            return x1 * çarpan;
        }
    }
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::Grafik;

    #[test]
    fn kaynak_altı_seriyi_ve_sabit_aralığı_korur() -> Result<(), UplotHatası> {
        let mut akış = SineAkışı::kanıt()?;
        let (seçenekler, ilk) = akış.kartı()?;
        assert_eq!(ilk.uzunluk(), 600);
        assert_eq!(ilk.seriler().len(), 6);
        assert_eq!(seçenekler.y_aralığı, Some(Aralık::yeni(-6.0, 6.0)?));
        assert_eq!(seçenekler.piksel_hizası, 0.0);
        assert_eq!(seçenekler.x_eksen_asgari_etiket_boşluğu, 300.0);
        assert_eq!(akış.kaydırma(), 600);
        assert_eq!(
            ilk.x()
                .windows(2)
                .filter(|çift| matches!(*çift, [a, b] if a == b))
                .count(),
            1
        );
        assert_eq!(
            ilk.x()
                .windows(2)
                .find(|çift| matches!(*çift, [a, b] if a == b)),
            Some(
                &[
                    SINE_STREAM_KANIT_BAŞLANGICI_SN + 599.0 * ÖRNEK_ARALIĞI_SN,
                    SINE_STREAM_KANIT_BAŞLANGICI_SN + 599.0 * ÖRNEK_ARALIĞI_SN,
                ][..]
            )
        );
        assert_eq!(
            seçenekler.seriler.get(1).map(|seri| seri.etiket.as_str()),
            Some("Value")
        );
        assert_eq!(
            seçenekler
                .seriler
                .first()
                .and_then(|seri| seri.dolgu.as_deref()),
            Some("rgba(255,0,0,0.1)")
        );

        let önceki_son = ilk.x().last().copied();
        let sonraki = akış.ilerlet()?;
        assert_eq!(sonraki.x().first().copied(), ilk.x().get(1).copied());
        assert!(
            önceki_son
                .zip(sonraki.x().last().copied())
                .is_some_and(|(önce, sonra)| (sonra - önce - ÖRNEK_ARALIĞI_SN).abs() < f64::EPSILON)
        );
        Ok(())
    }

    #[test]
    fn set_data_akışı_grafiği_yeniler() -> Result<(), UplotHatası> {
        let mut akış = SineAkışı::kanıt()?;
        let (seçenekler, veri) = akış.kartı()?;
        let mut grafik = Grafik::yeni(seçenekler, veri)?;
        let önce = grafik.çiz();
        grafik.veriyi_ayarla(akış.ilerlet()?)?;
        assert_ne!(grafik.çiz(), önce);
        Ok(())
    }

    #[test]
    fn ardışık_kareler_tek_akışta_yalnız_ilki_düşürüp_sona_bir_değer_ekler()
    -> Result<(), UplotHatası> {
        let mut akış = SineAkışı::kanıt()?;
        let (_, ilk) = akış.kartı()?;
        let ikinci = akış.ilerlet()?;
        let üçüncü = akış.ilerlet()?;

        for (önceki, sonraki) in [(&ilk, &ikinci), (&ikinci, &üçüncü)] {
            assert_eq!(sonraki.x().get(..599), önceki.x().get(1..));
            assert_eq!(
                sonraki
                    .x()
                    .last()
                    .zip(önceki.x().last())
                    .map(|(sonraki, önceki)| sonraki - önceki),
                Some(ÖRNEK_ARALIĞI_SN)
            );
            for (önceki_seri, sonraki_seri) in önceki.seriler().iter().zip(sonraki.seriler()) {
                assert_eq!(sonraki_seri.get(..599), önceki_seri.get(1..));
            }
        }
        assert_eq!(
            ikinci
                .x()
                .windows(2)
                .filter(|çift| matches!(**çift, [a, b] if a == b))
                .count(),
            1,
            "kaynak x599 tekrarı kayan pencereden doğal olarak çıkana kadar korunmalı"
        );
        Ok(())
    }

    #[test]
    fn kaynak_set_data_varsayılanı_yakınlaştırmayı_sonraki_karede_sıfırlar()
    -> Result<(), UplotHatası> {
        let mut akış = SineAkışı::kanıt()?;
        let (seçenekler, veri) = akış.kartı()?;
        let mut grafik = Grafik::yeni(seçenekler, veri)?;
        assert!(grafik.seçim_yakınlaştır(0.25, 0.75)?);
        assert!(grafik.yakınlaştırılmış());

        let sonraki = akış.ilerlet()?;
        let beklenen_x = Aralık::yeni(
            *sonraki
                .x()
                .first()
                .ok_or(UplotHatası::YetersizVeri { uzunluk: 0 })?,
            *sonraki
                .x()
                .last()
                .ok_or(UplotHatası::YetersizVeri { uzunluk: 0 })?,
        )?;
        grafik.veriyi_ayarla(sonraki)?;

        assert_eq!(grafik.görünür_x_aralığı(), beklenen_x);
        assert_eq!(grafik.görünür_y_aralığı(), Aralık::yeni(-6.0, 6.0)?);
        assert!(!grafik.yakınlaştırılmış());
        assert!(!grafik.geri_var());
        Ok(())
    }
}
