use super::ortak_kart_etkileşimleri;
use super::veri_uretici::KanıtRastgele;
use crate::{
    Grafik, GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası, YÖlçekEtiketBiçimi,
    YÖlçekSeçenekleri,
};

pub const AXIS_AUTOSIZE_KANIT_TOHUMU: u32 = 0xA170_512E;
pub const AXIS_AUTOSIZE_ARALIK_MS: u64 = 500;
const NOKTA_SAYISI: usize = 501;

pub const AXIS_AUTOSIZE_KART_TANIM_ÖRNEĞİ: &str = r##"let çarpan = 1_000.0;
let (seçenekler, veri) = axis_autosize_kartı(çarpan)?;
let mut grafik = Grafik::yeni(seçenekler, veri)?;
let mut akış = AxisAutosizeAkışı::yeni();
if let Some((çarpan, veri)) = akış.ilerlet()? {
    grafik.canlı_veriyi_x_etiket_çarpanında_ayarla(veri, çarpan)?;
}"##;

#[derive(Debug, Clone)]
pub struct AxisAutosizeAkışı {
    aşama: u8,
    üretim: u64,
}

impl AxisAutosizeAkışı {
    pub fn yeni() -> Self {
        Self {
            aşama: 0,
            üretim: 0,
        }
    }

    pub fn aşama(&self) -> u8 {
        self.aşama
    }

    pub fn çarpanı_ayarla(&mut self, çarpan: f64) -> Result<(f64, HizalıVeri), UplotHatası> {
        doğrula_çarpan(çarpan)?;
        self.üretim = self.üretim.wrapping_add(1);
        self.aşama = çarpan.log10().round().clamp(0.0, 9.0) as u8;
        Ok((çarpan, axis_autosize_verisi(çarpan, self.üretim)?))
    }

    pub fn ilerlet(&mut self) -> Result<Option<(f64, HizalıVeri)>, UplotHatası> {
        if self.aşama >= 9 {
            return Ok(None);
        }
        let sonraki = self.aşama + 1;
        let çarpan = 10_f64.powi(i32::from(sonraki));
        self.çarpanı_ayarla(çarpan).map(Some)
    }

    pub fn grafiği_ilerlet(&mut self, grafik: &mut Grafik) -> Result<bool, UplotHatası> {
        let Some((çarpan, veri)) = self.ilerlet()? else {
            return Ok(false);
        };
        grafik.canlı_veriyi_x_etiket_çarpanında_ayarla(veri, çarpan)?;
        Ok(true)
    }
}

impl Default for AxisAutosizeAkışı {
    fn default() -> Self {
        Self::yeni()
    }
}

/// `demos/axis-autosize.html` içindeki veri formülünü ve 1…10⁹ dinamik
/// çarpan davranışını, belirlenimci görsel kanıt üretimiyle taşır.
pub fn axis_autosize_kartı(
    çarpan: f64,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    doğrula_çarpan(çarpan)?;
    let veri = axis_autosize_verisi(çarpan, 0)?;
    let seçenekler = GrafikSeçenekleri::yeni(1048, 600)?
        .başlık("Axis AutoSize")
        .arka_plan_rengi("#eeeeee")
        .çizim_alanı_arka_plan_rengi("#d6d6d6")
        .x_zaman(false)
        .x_eksen_etiket_biçimi(YÖlçekEtiketBiçimi::ArtımaGöre)
        .x_eksen_etiketi("X Axis Label")
        .y_eksen_etiketi("Y Axis Label")
        .y_ölçeği(YÖlçekSeçenekleri::yeni("y").etiket_biçimi(YÖlçekEtiketBiçimi::ArtımaGöre))
        .birincil_y_eksen_rengi("#ff0000")
        .x_eksen_değer_çarpanı(çarpan)
        .otomatik_x_sağ_pay(true)
        .otomatik_y_eksen_genişliği(true)
        .etkileşimler(ortak_kart_etkileşimleri())
        .seri(SeriSeçenekleri::yeni("sin(x)").renk("#ff0000"));
    Ok((seçenekler, veri))
}

fn doğrula_çarpan(çarpan: f64) -> Result<(), UplotHatası> {
    if çarpan.is_finite() && çarpan > 0.0 {
        Ok(())
    } else {
        Err(UplotHatası::GeçersizÇarpan { değer: çarpan })
    }
}

fn axis_autosize_verisi(çarpan: f64, üretim: u64) -> Result<HizalıVeri, UplotHatası> {
    // Kaynak her `setData(getData(...))` çağrısında Math.random akışını
    // ilerletir. Güncelleme sayacını tohuma katarak aynı çarpana dönülse
    // bile yeni, testlerde yeniden üretilebilir bir geometri elde edilir.
    let çarpan_bitleri = çarpan.to_bits();
    let aşama_tohumu = AXIS_AUTOSIZE_KANIT_TOHUMU
        ^ çarpan_bitleri as u32
        ^ (çarpan_bitleri >> 32) as u32
        ^ üretim as u32
        ^ (üretim >> 32) as u32;
    let mut rastgele = KanıtRastgele::yeni(aşama_tohumu);
    let mut x = Vec::with_capacity(NOKTA_SAYISI);
    let mut y = Vec::with_capacity(NOKTA_SAYISI);
    for indeks in 0..NOKTA_SAYISI {
        let değer = indeks as f64;
        x.push(değer);
        y.push(Some(
            (rastgele.sonraki() - 0.5
                + (değer * 0.00002).sin() * 40.0
                + (değer * 0.001).sin() * 5.0
                + (değer * 0.1).sin() * 2.0)
                * çarpan,
        ));
    }
    HizalıVeri::yeni(x, vec![y])
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Aralık, Grafik, Komut};

    #[test]
    fn çarpan_veriyi_etiketleri_ve_eksen_paylarını_büyütür() -> Result<(), UplotHatası> {
        let (küçük_seçenekler, küçük_veri) = axis_autosize_kartı(1.0)?;
        let küçük_normalize = ilk_y(&küçük_veri);
        let küçük = Grafik::yeni(küçük_seçenekler, küçük_veri)?;
        let (büyük_seçenekler, büyük_veri) = axis_autosize_kartı(1e9)?;
        let büyük_normalize = ilk_y(&büyük_veri).map(|değer| değer / 1e9);
        let büyük = Grafik::yeni(büyük_seçenekler, büyük_veri)?;
        assert!(
            küçük_normalize
                .zip(büyük_normalize)
                .is_some_and(|(a, b)| a != b)
        );
        assert_eq!(küçük_veri_uzunluğu(&küçük), NOKTA_SAYISI);
        let (küçük_sol, küçük_sağ, _, _) = küçük.çizim_alanı_boyutta(1048, 600);
        let (büyük_sol, büyük_sağ, _, _) = büyük.çizim_alanı_boyutta(1048, 600);
        assert!(büyük_sol > küçük_sol);
        assert!(büyük_sağ < küçük_sağ);
        assert!(
            büyük.çiz().komutlar().iter().any(
                |komut| matches!(komut, Komut::Metin { içerik, .. } if içerik == "500000000000")
            )
        );
        assert!(büyük.çiz().komutlar().iter().any(
            |komut| matches!(komut, Komut::DöndürülmüşMetin { içerik, .. } if içerik == "Y Axis Label")
        ));
        assert!(matches!(
            büyük.çiz().komutlar().first(),
            Some(Komut::ArkaPlan { renk }) if renk == "#eeeeee"
        ));
        assert!(
            büyük.çiz().komutlar().iter().any(
                |komut| matches!(komut, Komut::Dikdörtgen { dolgu, .. } if dolgu == "#d6d6d6")
            )
        );
        Ok(())
    }

    #[test]
    fn aynı_grafikte_set_data_görünüm_seri_ve_yeni_rastgele_geometriyi_korur()
    -> Result<(), UplotHatası> {
        let (seçenekler, veri) = axis_autosize_kartı(1.0)?;
        let mut grafik = Grafik::yeni(seçenekler, veri)?;
        assert!(grafik.seçim_yakınlaştır(0.2, 0.8)?);
        let görünür_x = grafik.görünür_x_aralığı();
        assert!(grafik.seri_görünürlüğünü_ayarla(0, false)?);

        let mut akış = AxisAutosizeAkışı::yeni();
        let Some((çarpan, veri)) = akış.ilerlet()? else {
            return Err(UplotHatası::GeçersizKaynakVeri {
                varlık: "AxisAutosizeAkışı",
                açıklama: "ilk 500 ms aşaması üretilemedi".to_string(),
            });
        };
        grafik.canlı_veriyi_x_etiket_çarpanında_ayarla(veri, çarpan)?;
        assert_eq!(grafik.görünür_x_aralığı(), görünür_x);
        assert!(!grafik.seri_görünür_mü(0));
        assert_eq!(akış.aşama(), 1);

        let (_, ilk) = akış.çarpanı_ayarla(100.0)?;
        let (_, ikinci) = akış.çarpanı_ayarla(100.0)?;
        assert_ne!(ilk_y(&ilk), ilk_y(&ikinci));
        assert!(grafik.görünür_x_aralığı().en_az >= Aralık::yeni(0.0, 500.0)?.en_az);
        Ok(())
    }

    fn ilk_y(veri: &HizalıVeri) -> Option<f64> {
        veri.seriler().first()?.first().copied().flatten()
    }

    fn küçük_veri_uzunluğu(grafik: &Grafik) -> usize {
        grafik
            .çiz()
            .komutlar()
            .iter()
            .find_map(|komut| match komut {
                Komut::Yol { parçalar, .. } => Some(parçalar.iter().map(Vec::len).sum()),
                _ => None,
            })
            .unwrap_or(0)
    }
}
