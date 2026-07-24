use std::collections::BTreeSet;

use crate::{Aralık, UplotHatası};

mod dagilim;
mod gradyan;
mod isi_haritasi;
mod seri;
mod timeline;
mod y_olcek;
mod zaman;

pub use dagilim::{DağılımDüzeni, DağılımNoktası, DağılımSerisi};
pub use gradyan::{GradyanDurağı, GradyanEkseni, GradyanKonumu, ÖlçekGradyanı};
pub use isi_haritasi::{IsıHaritasıDüzeni, IsıHücresi, IsıHücresiBoyutu};
pub use seri::{NoktaFiltreKipi, NoktaŞekli, SeriSeçenekleri, SeriÇizimTürü};
pub use timeline::{TimelineDüzeni, TimelineHücresi};
pub use y_olcek::{
    GüzelÖlçekDüzeni, YÖlçekDağılımı, YÖlçekEtiketBiçimi, YÖlçekSeçenekleri
};
pub use zaman::{TarihAdları, ZamanDilimi};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum XÖlçekDağılımı {
    Doğrusal,
    Logaritmik { taban: f64 },
}

/// Aynı X konumlarını kaydırılmış bir zaman aralığıyla karşı tarafta gösteren
/// uPlot `scales.{key}.from = "x"` ve ikinci X ekseni karşılığıdır.
#[derive(Debug, Clone, PartialEq)]
pub struct İkincilXEksen {
    pub zaman_kaydırması: f64,
    pub renk: String,
}

impl İkincilXEksen {
    pub fn yeni(zaman_kaydırması: f64, renk: impl Into<String>) -> Self {
        Self {
            zaman_kaydırması: if zaman_kaydırması.is_finite() {
                zaman_kaydırması
            } else {
                0.0
            },
            renk: renk.into(),
        }
    }
}

/// uPlot `drawOrder` içindeki iki yerleşik çizim katmanının sırası.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ÇizimSırası {
    EksenlerSeriler,
    SerilerEksenler,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ÇubukYönü {
    Dikey,
    Yatay,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KutuBıyıkDüzeni {
    pub ayrık_değerler: Vec<Vec<f64>>,
    pub gövde_genişlik_oranı: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MumDüzeni {
    pub zamanlar: Vec<f64>,
    pub yükseliş_rengi: String,
    pub düşüş_rengi: String,
    pub azami_gövde_genişliği: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SeriBandı {
    pub üst_seri: usize,
    pub alt_seri: usize,
    pub dolgu: String,
    pub yön: BantYönü,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BantYönü {
    EnAza,
    EnÇoğa,
}

impl SeriBandı {
    pub fn yeni(üst_seri: usize, alt_seri: usize, dolgu: impl Into<String>) -> Self {
        Self {
            üst_seri,
            alt_seri,
            dolgu: dolgu.into(),
            yön: BantYönü::EnAza,
        }
    }

    pub fn yön(mut self, yön: BantYönü) -> Self {
        self.yön = yön;
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NoktaKatmanı {
    pub noktalar: Vec<(f64, f64)>,
    pub renk: String,
    pub boyut: f32,
}

/// `annotations.html` eklentisindeki etiketin çizim alanının hangi kenarına
/// yerleşeceğini belirtir.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AçıklamaHizası {
    Üst,
    Alt,
}

/// Aynı türdeki annotation işaretlerinin ortak görünümüdür.
#[derive(Debug, Clone, PartialEq)]
pub struct AçıklamaStili {
    pub tür: String,
    pub kalınlık: f32,
    pub kesik: f32,
    pub çizgi: String,
    pub dolgu: String,
    pub hiza: AçıklamaHizası,
}

impl AçıklamaStili {
    pub fn yeni(
        tür: impl Into<String>,
        çizgi: impl Into<String>,
        dolgu: impl Into<String>,
        hiza: AçıklamaHizası,
    ) -> Self {
        Self {
            tür: tür.into(),
            kalınlık: 2.0,
            kesik: 5.0,
            çizgi: çizgi.into(),
            dolgu: dolgu.into(),
            hiza,
        }
    }

    pub fn çizgi_biçimi(mut self, kalınlık: f32, kesik: f32) -> Self {
        if kalınlık.is_finite() && kalınlık > 0.0 {
            self.kalınlık = kalınlık;
        }
        if kesik.is_finite() && kesik > 0.0 {
            self.kesik = kesik;
        }
        self
    }
}

/// X ölçeğine bağlı tek bir annotation çizgisi veya aralığıdır.
#[derive(Debug, Clone, PartialEq)]
pub struct Açıklamaİşareti {
    pub tür: String,
    pub başlangıç: f64,
    pub bitiş: f64,
    pub etiket: String,
    pub açıklama: String,
    /// Kaynak demo bu alanı taşır ancak bir tıklama davranışına bağlamaz.
    pub bağlantı: Option<String>,
}

impl Açıklamaİşareti {
    pub fn yeni(
        tür: impl Into<String>,
        başlangıç: f64,
        bitiş: f64,
        etiket: impl Into<String>,
    ) -> Self {
        Self {
            tür: tür.into(),
            başlangıç,
            bitiş,
            etiket: etiket.into(),
            açıklama: String::new(),
            bağlantı: None,
        }
    }

    pub fn açıklama(mut self, açıklama: impl Into<String>) -> Self {
        self.açıklama = açıklama.into();
        self
    }

    pub fn bağlantı(mut self, bağlantı: impl Into<String>) -> Self {
        self.bağlantı = Some(bağlantı.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AçıklamaDüzeni {
    pub stiller: Vec<AçıklamaStili>,
    pub işaretler: Vec<Açıklamaİşareti>,
}

impl AçıklamaDüzeni {
    pub fn stil(mut self, stil: AçıklamaStili) -> Self {
        self.stiller.push(stil);
        self
    }

    pub fn işaret(mut self, işaret: Açıklamaİşareti) -> Self {
        self.işaretler.push(işaret);
        self
    }
}

/// `wind-direction.html` özel `series.paths` fonksiyonunun çekirdek
/// karşılığıdır. Hız serisi vektör başlangıcını, yön serisi dereceyi taşır.
#[derive(Debug, Clone, PartialEq)]
pub struct RüzgarYönüDüzeni {
    pub hız_serisi: usize,
    pub yön_serisi: usize,
    pub ölçek: String,
    pub uzunluk: f32,
    pub renk: String,
    pub kalınlık: f32,
}

impl RüzgarYönüDüzeni {
    pub fn yeni(hız_serisi: usize, yön_serisi: usize, ölçek: impl Into<String>) -> Self {
        Self {
            hız_serisi,
            yön_serisi,
            ölçek: ölçek.into(),
            uzunluk: 15.0,
            renk: "blue".to_string(),
            kalınlık: 1.0,
        }
    }

    pub fn stil(mut self, uzunluk: f32, renk: impl Into<String>, kalınlık: f32) -> Self {
        if uzunluk.is_finite() && uzunluk > 0.0 {
            self.uzunluk = uzunluk;
        }
        self.renk = renk.into();
        if kalınlık.is_finite() && kalınlık > 0.0 {
            self.kalınlık = kalınlık;
        }
        self
    }
}

/// uPlot çizim kancalarının sahneye eklediği, yüzeyden bağımsız katmanlar.
#[derive(Debug, Clone, PartialEq)]
pub struct ÇizimKancasıDüzeni {
    pub gradyan_durakları: Option<(String, String)>,
    pub seri_medyanları: bool,
    pub medyan_kalınlığı: f32,
    pub medyan_bulanıklığı: f32,
    pub yıldız_uçları: Option<usize>,
    pub yıldız_dış_yarıçapı: f32,
    pub yıldız_iç_yarıçapı: f32,
    pub çizim_süresi_metni: bool,
    pub seri_uç_trendleri: bool,
    pub trend_kesik: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OdakStili {
    Opaklık,
    OdakDışıSiyah,
    OdaklıMacenta,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OdakDüzeni {
    pub alfa: f32,
    pub yakınlık: f32,
    pub yön_eğilimi: i8,
    pub odak_kalınlığı: Option<f32>,
    pub stil: OdakStili,
}

impl OdakDüzeni {
    pub fn yeni(alfa: f32, yakınlık: f32) -> Self {
        Self {
            alfa: if alfa.is_finite() {
                alfa.clamp(0.0, 1.0)
            } else {
                0.3
            },
            yakınlık: if yakınlık.is_finite() {
                yakınlık.max(-1.0)
            } else {
                -1.0
            },
            yön_eğilimi: 0,
            odak_kalınlığı: None,
            stil: OdakStili::Opaklık,
        }
    }

    pub fn yön_eğilimi(mut self, eğilim: i8) -> Self {
        self.yön_eğilimi = eğilim.clamp(-1, 1);
        self
    }

    pub fn odak_kalınlığı(mut self, kalınlık: f32) -> Self {
        if kalınlık.is_finite() && kalınlık > 0.0 {
            self.odak_kalınlığı = Some(kalınlık);
        }
        self
    }

    pub fn stil(mut self, stil: OdakStili) -> Self {
        self.stil = stil;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnYakınTooltipDüzeni {
    pub commitler: Vec<String>,
    pub interpolasyonlar: BTreeSet<usize>,
    pub stat: String,
    pub interpolasyon_rengi: String,
}

impl EnYakınTooltipDüzeni {
    pub fn yeni(
        commitler: Vec<String>,
        interpolasyonlar: impl IntoIterator<Item = usize>,
        stat: impl Into<String>,
    ) -> Self {
        Self {
            commitler,
            interpolasyonlar: interpolasyonlar.into_iter().collect(),
            stat: stat.into(),
            interpolasyon_rengi: "#fcb0f1".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnYakınTooltipBilgisi {
    pub zaman: f64,
    pub commit: String,
    pub önceki_commit: Option<String>,
    pub seri: usize,
    pub değer: f64,
    pub başlangıçtan_yüzde: f64,
    pub interpolasyon: bool,
    pub kenarlık_rengi: String,
    pub karşılaştırma_url: String,
    pub metin: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TooltipDüzeni {
    pub imleç_değeri: bool,
    pub seri_değerleri: bool,
    pub imleç_durumunu_koru: bool,
    pub yeniden_kurma_ms: Option<u64>,
}

impl TooltipDüzeni {
    pub const fn yeni() -> Self {
        Self {
            imleç_değeri: true,
            seri_değerleri: true,
            imleç_durumunu_koru: false,
            yeniden_kurma_ms: None,
        }
    }

    pub const fn imleç_durumunu_koru(mut self, koru: bool) -> Self {
        self.imleç_durumunu_koru = koru;
        self
    }

    pub const fn yeniden_kurma_ms(mut self, milisaniye: u64) -> Self {
        self.yeniden_kurma_ms = Some(milisaniye);
        self
    }
}

impl Default for TooltipDüzeni {
    fn default() -> Self {
        Self::yeni()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TooltipBilgisi {
    pub seri: Option<usize>,
    pub metin: String,
    pub yatay_oran: f64,
    pub dikey_oran: f64,
    pub arka_plan_rengi: String,
    pub metin_rengi: String,
}

/// uPlot `setSize()` sırasında kalıcı seçim, kilitli imleç ve hover
/// noktasını yeni çizim alanına oransal olarak taşıyan düzen.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoyutSenkronDüzeni {
    pub imleç_x_oranı: f32,
    pub imleç_y_oranı: f32,
    pub seçim_x_oranı: f32,
    pub seçim_y_oranı: f32,
    pub seçim_genişlik_oranı: f32,
    pub seçim_yükseklik_oranı: f32,
    pub hover_x_oranı: f32,
    pub hover_y_oranı: f32,
}

impl BoyutSenkronDüzeni {
    #[allow(clippy::too_many_arguments)]
    pub fn piksel_değerlerinden(
        çizim_genişliği: f32,
        çizim_yüksekliği: f32,
        imleç_x: f32,
        imleç_y: f32,
        seçim_x: f32,
        seçim_y: f32,
        seçim_genişliği: f32,
        seçim_yüksekliği: f32,
        hover_x: f32,
        hover_y: f32,
    ) -> Option<Self> {
        if !çizim_genişliği.is_finite()
            || !çizim_yüksekliği.is_finite()
            || çizim_genişliği <= 0.0
            || çizim_yüksekliği <= 0.0
        {
            return None;
        }
        let değerler = [
            imleç_x,
            imleç_y,
            seçim_x,
            seçim_y,
            seçim_genişliği,
            seçim_yüksekliği,
            hover_x,
            hover_y,
        ];
        if değerler.iter().any(|değer| !değer.is_finite()) {
            return None;
        }
        Some(Self {
            imleç_x_oranı: (imleç_x / çizim_genişliği).clamp(0.0, 1.0),
            imleç_y_oranı: (imleç_y / çizim_yüksekliği).clamp(0.0, 1.0),
            seçim_x_oranı: (seçim_x / çizim_genişliği).clamp(0.0, 1.0),
            seçim_y_oranı: (seçim_y / çizim_yüksekliği).clamp(0.0, 1.0),
            seçim_genişlik_oranı: (seçim_genişliği / çizim_genişliği).clamp(0.0, 1.0),
            seçim_yükseklik_oranı: (seçim_yüksekliği / çizim_yüksekliği).clamp(0.0, 1.0),
            hover_x_oranı: (hover_x / çizim_genişliği).clamp(0.0, 1.0),
            hover_y_oranı: (hover_y / çizim_yüksekliği).clamp(0.0, 1.0),
        })
    }
}

impl Default for ÇizimKancasıDüzeni {
    fn default() -> Self {
        Self {
            gradyan_durakları: None,
            seri_medyanları: false,
            medyan_kalınlığı: 50.0,
            medyan_bulanıklığı: 6.0,
            yıldız_uçları: None,
            yıldız_dış_yarıçapı: 8.0,
            yıldız_iç_yarıçapı: 4.0,
            çizim_süresi_metni: false,
            seri_uç_trendleri: false,
            trend_kesik: 5.0,
        }
    }
}

impl ÇizimKancasıDüzeni {
    pub fn gradyan(mut self, üst: impl Into<String>, alt: impl Into<String>) -> Self {
        self.gradyan_durakları = Some((üst.into(), alt.into()));
        self
    }

    pub fn seri_medyanları(mut self, kalınlık: f32, bulanıklık: f32) -> Self {
        if kalınlık.is_finite() && kalınlık > 0.0 {
            self.medyan_kalınlığı = kalınlık;
        }
        if bulanıklık.is_finite() && bulanıklık >= 0.0 {
            self.medyan_bulanıklığı = bulanıklık;
        }
        self.seri_medyanları = true;
        self
    }

    pub fn yıldız_noktalar(mut self, uçlar: usize, dış: f32, iç: f32) -> Self {
        if (2..=32).contains(&uçlar)
            && dış.is_finite()
            && iç.is_finite()
            && dış > 0.0
            && iç > 0.0
            && iç <= dış
        {
            self.yıldız_uçları = Some(uçlar);
            self.yıldız_dış_yarıçapı = dış;
            self.yıldız_iç_yarıçapı = iç;
        }
        self
    }

    pub fn çizim_süresi_metni(mut self, etkin: bool) -> Self {
        self.çizim_süresi_metni = etkin;
        self
    }

    pub fn seri_uç_trendleri(mut self, kesik: f32) -> Self {
        if kesik.is_finite() && kesik > 0.0 {
            self.trend_kesik = kesik;
            self.seri_uç_trendleri = true;
        }
        self
    }
}

impl NoktaKatmanı {
    pub fn yeni(noktalar: Vec<(f64, f64)>) -> Self {
        Self {
            noktalar,
            renk: "#000000".to_string(),
            boyut: 5.0,
        }
    }
}

impl MumDüzeni {
    pub fn yeni(zamanlar: Vec<f64>) -> Self {
        Self {
            zamanlar,
            yükseliş_rengi: "#4ab650".to_string(),
            düşüş_rengi: "#e54245".to_string(),
            azami_gövde_genişliği: 20.0,
        }
    }
}

impl KutuBıyıkDüzeni {
    pub fn yeni(ayrık_değerler: Vec<Vec<f64>>) -> Self {
        Self {
            ayrık_değerler,
            gövde_genişlik_oranı: 0.7,
        }
    }

    pub fn gövde_genişlik_oranı(mut self, oran: f32) -> Self {
        if oran.is_finite() {
            self.gövde_genişlik_oranı = oran.clamp(0.1, 1.0);
        }
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ÇubukDüzeni {
    pub yön: ÇubukYönü,
    pub yığılmış: bool,
    pub ters: bool,
    pub değer_etiketi_otomatik: bool,
    pub değer_etiketleri: bool,
    pub genişlik_oranı: f32,
    pub ek_boşluk: f32,
    pub hizalama: i8,
    pub x_kenar_paylı: bool,
}

impl ÇubukDüzeni {
    pub fn yeni(yön: ÇubukYönü) -> Self {
        Self {
            yön,
            yığılmış: false,
            ters: false,
            değer_etiketi_otomatik: false,
            değer_etiketleri: true,
            genişlik_oranı: 0.9,
            ek_boşluk: 0.0,
            hizalama: 0,
            x_kenar_paylı: true,
        }
    }

    pub fn yığılmış(mut self, etkin: bool) -> Self {
        self.yığılmış = etkin;
        self
    }

    pub fn ters(mut self, etkin: bool) -> Self {
        self.ters = etkin;
        self
    }

    pub fn değer_etiketi_otomatik(mut self, etkin: bool) -> Self {
        self.değer_etiketi_otomatik = etkin;
        self
    }

    pub fn değer_etiketleri(mut self, etkin: bool) -> Self {
        self.değer_etiketleri = etkin;
        self
    }

    pub fn genişlik_oranı(mut self, oran: f32) -> Self {
        if oran.is_finite() {
            self.genişlik_oranı = oran.clamp(0.0, 1.0);
        }
        self
    }

    pub fn ek_boşluk(mut self, piksel: f32) -> Self {
        if piksel.is_finite() {
            self.ek_boşluk = piksel.max(0.0);
        }
        self
    }

    pub fn hizalama(mut self, hizalama: i8) -> Self {
        self.hizalama = hizalama.clamp(-1, 1);
        self
    }

    pub fn x_kenar_paylı(mut self, etkin: bool) -> Self {
        self.x_kenar_paylı = etkin;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TekerlekKipi {
    /// Piksel ve satır olaylarını giriş aygıtına göre ayrı işler.
    Otomatik,
    /// Bütün olayları klasik, ayrık tekerlek adımları olarak işler.
    Ayrık,
    /// Bütün olayları hassas piksel akışı olarak işler.
    Hassas,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TekerlekAyarları {
    pub kip: TekerlekKipi,
    pub ayrık_katsayı: f64,
    pub hassas_piksel_adımı: f64,
    pub hassas_ölü_bölge: f64,
    pub azami_hassas_delta: f64,
    pub hareket_birleştirme_ms: u64,
}

impl Default for TekerlekAyarları {
    fn default() -> Self {
        Self {
            kip: TekerlekKipi::Otomatik,
            ayrık_katsayı: 0.75,
            hassas_piksel_adımı: 100.0,
            hassas_ölü_bölge: 1.5,
            azami_hassas_delta: 100.0,
            hareket_birleştirme_ms: 140,
        }
    }
}

impl TekerlekAyarları {
    pub fn kip(mut self, kip: TekerlekKipi) -> Self {
        self.kip = kip;
        self
    }

    pub fn ayrık_katsayı(mut self, katsayı: f64) -> Self {
        if katsayı.is_finite() {
            self.ayrık_katsayı = katsayı.clamp(0.1, 0.99);
        }
        self
    }

    pub fn hassas_piksel_adımı(mut self, piksel: f64) -> Self {
        if piksel.is_finite() {
            self.hassas_piksel_adımı = piksel.clamp(10.0, 1_000.0);
        }
        self
    }

    pub fn hassas_ölü_bölge(mut self, piksel: f64) -> Self {
        if piksel.is_finite() {
            self.hassas_ölü_bölge = piksel.clamp(0.0, 20.0);
        }
        self
    }

    pub fn azami_hassas_delta(mut self, piksel: f64) -> Self {
        if piksel.is_finite() {
            self.azami_hassas_delta = piksel.clamp(10.0, 1_000.0);
        }
        self
    }

    pub fn hareket_birleştirme_ms(mut self, milisaniye: u64) -> Self {
        self.hareket_birleştirme_ms = milisaniye.clamp(40, 1_000);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EtkileşimSeçenekleri {
    /// uPlot'un resmi `wheelZoomPlugin` portunu etkinleştirir. Varsayılan: kapalı.
    pub tekerlek_etkileşimi: bool,
    /// Ayrık tekerlek ve hassas kaydırma yüzeylerinin normalizasyon ayarları.
    pub tekerlek_ayarları: TekerlekAyarları,
    /// uPlot çekirdeğinin sürükleyerek X aralığı seçme davranışı.
    pub seçim_yakınlaştır: bool,
    /// uPlot çekirdeğinin çift tıklamayla tam X aralığına dönme davranışı.
    pub çift_tıkla_tam_görünüm: bool,
    /// uPlot.rs'e özgü adımlı görünüm geçmişi. Varsayılan: kapalı.
    pub görünüm_geçmişi: bool,
    /// Resmî `zoom-touch` demosundaki kıstırarak yakınlaştırma ve tek parmakla
    /// taşıma davranışlarını etkinleştirir. Varsayılan: kapalı.
    pub dokunma_etkileşimi: bool,
    /// `cursor-bind` demosundaki Ctrl + sürükleme açıklama seçim bağını etkinleştirir.
    pub ctrl_açıklama: bool,
    /// `cursor-tooltip` demosundaki imleç bilgi kutusunu etkinleştirir.
    pub imleç_bilgi_kutusu: bool,
    /// `y-scale-drag` demosundaki ekseni sürükleyerek ölçeği kaydırma ve
    /// Shift ile büyütüp daraltma davranışını etkinleştirir.
    pub eksen_sürükleme: bool,
}

impl Default for EtkileşimSeçenekleri {
    fn default() -> Self {
        Self {
            tekerlek_etkileşimi: false,
            tekerlek_ayarları: TekerlekAyarları::default(),
            seçim_yakınlaştır: true,
            çift_tıkla_tam_görünüm: true,
            görünüm_geçmişi: false,
            dokunma_etkileşimi: false,
            ctrl_açıklama: false,
            imleç_bilgi_kutusu: false,
            eksen_sürükleme: false,
        }
    }
}

impl EtkileşimSeçenekleri {
    /// Resmi `wheelZoomPlugin` portunu kart için açar veya kapatır.
    pub fn tekerlek_etkileşimi(mut self, etkin: bool) -> Self {
        self.tekerlek_etkileşimi = etkin;
        self
    }

    pub fn tekerlek_ayarları(mut self, ayarlar: TekerlekAyarları) -> Self {
        self.tekerlek_ayarları = ayarlar;
        self
    }

    pub fn seçim_yakınlaştır(mut self, etkin: bool) -> Self {
        self.seçim_yakınlaştır = etkin;
        self
    }

    pub fn çift_tıkla_tam_görünüm(mut self, etkin: bool) -> Self {
        self.çift_tıkla_tam_görünüm = etkin;
        self
    }

    pub fn görünüm_geçmişi(mut self, etkin: bool) -> Self {
        self.görünüm_geçmişi = etkin;
        self
    }

    pub fn dokunma_etkileşimi(mut self, etkin: bool) -> Self {
        self.dokunma_etkileşimi = etkin;
        self
    }

    pub fn ctrl_açıklama(mut self, etkin: bool) -> Self {
        self.ctrl_açıklama = etkin;
        self
    }

    pub fn imleç_bilgi_kutusu(mut self, etkin: bool) -> Self {
        self.imleç_bilgi_kutusu = etkin;
        self
    }

    pub fn eksen_sürükleme(mut self, etkin: bool) -> Self {
        self.eksen_sürükleme = etkin;
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GrafikSeçenekleri {
    pub başlık: String,
    pub arka_plan_rengi: String,
    pub başlık_rengi: String,
    pub genişlik: u32,
    pub yükseklik: u32,
    /// Eksenleri gizli, çok küçük sparkline yüzeylerinde kaynak piksel
    /// boyutunu ve sıfır kenar payını korur.
    pub kompakt_yüzey: bool,
    pub x_zaman: bool,
    pub x_zaman_milisaniye: bool,
    pub x_tarih_adları: TarihAdları,
    pub x_zaman_dilimi: ZamanDilimi,
    pub x_dağılımı: XÖlçekDağılımı,
    pub x_ters_yön: bool,
    /// uPlot `scales.x.ori = 1` karşılığıdır. Etkin olduğunda X dikey,
    /// Y yatay çizilir.
    pub x_dikey: bool,
    /// X eksenini yönelimine göre karşı tarafa taşır: standart düzende üst,
    /// değiştirilmiş düzende sağ.
    pub x_eksen_karşıda: bool,
    pub x_eksen_görünür: bool,
    pub x_ızgara_görünür: bool,
    pub x_aralığı: Option<Aralık>,
    /// uPlot `scales.x.range` içinde `valToIdx` ile görünür uçları veri
    /// değerlerine yapıştıran aralık callback'inin karşılığıdır.
    pub x_aralığı_veriye_yapışık: bool,
    pub y_aralığı: Option<Aralık>,
    /// `mass-spectrum.html` içindeki veri min/max'ını doğrudan kullanan ve
    /// düz görünümde sıfır için 0..100, diğer değerler için 0..2v üreten kip.
    pub kütle_spektrumu_y_aralığı: bool,
    pub y_ölçekleri: Vec<YÖlçekSeçenekleri>,
    pub birincil_y_ölçeği: String,
    pub x_eksen_etiketi: String,
    pub x_eksen_rengi: String,
    pub x_eksen_etiket_biçimi: YÖlçekEtiketBiçimi,
    pub ikincil_x_eksen: Option<İkincilXEksen>,
    /// uPlot `axes[0].space` karşılığı asgari X etiketi piksel boşluğu.
    pub x_eksen_asgari_etiket_boşluğu: f32,
    /// Kaynak demonun otomatik olarak seçtiği zaman artımını duyarlı
    /// yüzeylerde de sabit tutmak için saniye cinsinden eksen artımı.
    pub x_zaman_sabit_artımı: Option<f64>,
    pub y_eksen_etiketi: String,
    pub birincil_y_sağda: bool,
    /// Birincil Y eksenini yönelimine göre karşı tarafa taşır. Standart
    /// düzende sağ, değiştirilmiş düzende alt taraftır.
    pub birincil_y_karşıda: bool,
    pub birincil_y_eksen_görünür: bool,
    pub birincil_y_ızgara_görünür: bool,
    pub birincil_y_sabit_bölmeler: Option<Vec<f64>>,
    pub birincil_y_özel_etiketler: Vec<(f64, String)>,
    pub birincil_y_ızgara_kesik: Option<f32>,
    pub birincil_y_eksen_rengi: String,
    /// uPlot `axes[1].size` karşılığı sabit Y ekseni payı.
    pub birincil_y_eksen_genişliği: Option<f32>,
    pub x_eksen_değer_çarpanı: f64,
    pub otomatik_x_sağ_pay: bool,
    pub otomatik_y_eksen_genişliği: bool,
    pub eksen_göstergeleri: bool,
    pub kategoriler: Vec<String>,
    pub çubuk_düzeni: Option<ÇubukDüzeni>,
    pub kutu_bıyık_düzeni: Option<KutuBıyıkDüzeni>,
    pub mum_düzeni: Option<MumDüzeni>,
    pub ısı_haritası_düzeni: Option<IsıHaritasıDüzeni>,
    pub timeline_düzeni: Option<TimelineDüzeni>,
    pub dağılım_düzeni: Option<DağılımDüzeni>,
    pub bantlar: Vec<SeriBandı>,
    pub nokta_katmanları: Vec<NoktaKatmanı>,
    pub açıklama_düzeni: Option<AçıklamaDüzeni>,
    pub ölçüm_datumları: bool,
    pub rüzgar_yönü_düzeni: Option<RüzgarYönüDüzeni>,
    pub çizim_kancaları: Option<ÇizimKancasıDüzeni>,
    pub odak: Option<OdakDüzeni>,
    pub en_yakın_tooltip: Option<EnYakınTooltipDüzeni>,
    pub tooltip: Option<TooltipDüzeni>,
    pub boyut_senkron_düzeni: Option<BoyutSenkronDüzeni>,
    pub lejant_canlı: bool,
    pub çizim_sırası: ÇizimSırası,
    /// uPlot `opts.pxAlign` karşılığıdır. `1`, koordinatları tam piksele
    /// yuvarlar; `0`, canlı akışlarda alt piksel hareketini korur.
    pub piksel_hizası: f32,
    pub ızgara_rengi: String,
    /// uPlot `cursor.move` ile eşdeğer, çizim alanı piksel koordinatlarında
    /// imleci kare ızgaraya oturtan isteğe bağlı adım.
    pub imleç_ızgara_adımı: Option<f32>,
    pub etkileşimler: EtkileşimSeçenekleri,
    pub seriler: Vec<SeriSeçenekleri>,
}

impl GrafikSeçenekleri {
    pub fn yeni(genişlik: u32, yükseklik: u32) -> Result<Self, UplotHatası> {
        if genişlik < 160 || yükseklik < 120 {
            return Err(UplotHatası::GeçersizBoyut {
                genişlik,
                yükseklik,
            });
        }
        Ok(Self {
            başlık: String::new(),
            arka_plan_rengi: "#ffffff".to_string(),
            başlık_rengi: "#111111".to_string(),
            genişlik,
            yükseklik,
            kompakt_yüzey: false,
            x_zaman: true,
            x_zaman_milisaniye: false,
            x_tarih_adları: TarihAdları::default(),
            x_zaman_dilimi: ZamanDilimi::Utc,
            x_dağılımı: XÖlçekDağılımı::Doğrusal,
            x_ters_yön: false,
            x_dikey: false,
            x_eksen_karşıda: false,
            x_eksen_görünür: true,
            x_ızgara_görünür: true,
            x_aralığı: None,
            x_aralığı_veriye_yapışık: false,
            y_aralığı: None,
            kütle_spektrumu_y_aralığı: false,
            y_ölçekleri: Vec::new(),
            birincil_y_ölçeği: "y".to_string(),
            x_eksen_etiketi: String::new(),
            x_eksen_rengi: "#4b5563".to_string(),
            x_eksen_etiket_biçimi: YÖlçekEtiketBiçimi::Otomatik,
            ikincil_x_eksen: None,
            x_eksen_asgari_etiket_boşluğu: 50.0,
            x_zaman_sabit_artımı: None,
            y_eksen_etiketi: String::new(),
            birincil_y_sağda: false,
            birincil_y_karşıda: false,
            birincil_y_eksen_görünür: true,
            birincil_y_ızgara_görünür: true,
            birincil_y_sabit_bölmeler: None,
            birincil_y_özel_etiketler: Vec::new(),
            birincil_y_ızgara_kesik: None,
            birincil_y_eksen_rengi: "#4b5563".to_string(),
            birincil_y_eksen_genişliği: None,
            x_eksen_değer_çarpanı: 1.0,
            otomatik_x_sağ_pay: false,
            otomatik_y_eksen_genişliği: false,
            eksen_göstergeleri: false,
            kategoriler: Vec::new(),
            çubuk_düzeni: None,
            kutu_bıyık_düzeni: None,
            mum_düzeni: None,
            ısı_haritası_düzeni: None,
            timeline_düzeni: None,
            dağılım_düzeni: None,
            bantlar: Vec::new(),
            nokta_katmanları: Vec::new(),
            açıklama_düzeni: None,
            ölçüm_datumları: false,
            rüzgar_yönü_düzeni: None,
            çizim_kancaları: None,
            odak: None,
            en_yakın_tooltip: None,
            tooltip: None,
            boyut_senkron_düzeni: None,
            lejant_canlı: true,
            çizim_sırası: ÇizimSırası::EksenlerSeriler,
            piksel_hizası: 1.0,
            ızgara_rengi: "#e5e7eb".to_string(),
            imleç_ızgara_adımı: None,
            etkileşimler: EtkileşimSeçenekleri::default(),
            seriler: Vec::new(),
        })
    }

    /// uPlot sparkline örneklerindeki gibi küçük, eksensiz bir yüzey kurar.
    pub fn kompakt(genişlik: u32, yükseklik: u32) -> Result<Self, UplotHatası> {
        if genişlik < 2 || yükseklik < 2 {
            return Err(UplotHatası::GeçersizBoyut {
                genişlik,
                yükseklik,
            });
        }
        let mut seçenekler = Self::yeni(genişlik.max(160), yükseklik.max(120))?;
        seçenekler.genişlik = genişlik;
        seçenekler.yükseklik = yükseklik;
        seçenekler.kompakt_yüzey = true;
        Ok(seçenekler)
    }

    pub fn başlık(mut self, başlık: impl Into<String>) -> Self {
        self.başlık = başlık.into();
        self
    }

    pub fn arka_plan_rengi(mut self, renk: impl Into<String>) -> Self {
        self.arka_plan_rengi = renk.into();
        self
    }

    pub fn başlık_rengi(mut self, renk: impl Into<String>) -> Self {
        self.başlık_rengi = renk.into();
        self
    }

    pub fn x_eksen_rengi(mut self, renk: impl Into<String>) -> Self {
        self.x_eksen_rengi = renk.into();
        self
    }

    pub fn x_eksen_etiket_biçimi(mut self, biçim: YÖlçekEtiketBiçimi) -> Self {
        self.x_eksen_etiket_biçimi = biçim;
        self
    }

    pub fn ikincil_x_ekseni(mut self, eksen: İkincilXEksen) -> Self {
        self.ikincil_x_eksen = Some(eksen);
        self
    }

    pub fn x_zaman(mut self, zaman: bool) -> Self {
        self.x_zaman = zaman;
        self
    }

    pub fn x_zaman_milisaniye(mut self, milisaniye: bool) -> Self {
        self.x_zaman_milisaniye = milisaniye;
        self
    }

    pub fn x_tarih_adları(mut self, adlar: TarihAdları) -> Self {
        self.x_tarih_adları = adlar;
        self
    }

    pub fn x_zaman_dilimi(mut self, zaman_dilimi: ZamanDilimi) -> Self {
        self.x_zaman_dilimi = zaman_dilimi;
        self
    }

    pub fn x_logaritmik(mut self, taban: f64) -> Self {
        if taban.is_finite() && taban > 1.0 {
            self.x_dağılımı = XÖlçekDağılımı::Logaritmik { taban };
        }
        self
    }

    /// uPlot `scales.x.dir = -1` karşılığıdır.
    pub fn x_ters_yön(mut self, ters: bool) -> Self {
        self.x_ters_yön = ters;
        self
    }

    /// uPlot ölçek yönelim çiftini `x.ori = 1`, `y.ori = 0` yapar.
    pub fn x_dikey(mut self, dikey: bool) -> Self {
        self.x_dikey = dikey;
        self
    }

    pub fn x_eksen_karşıda(mut self, karşıda: bool) -> Self {
        self.x_eksen_karşıda = karşıda;
        self
    }

    pub fn x_ekseni_göster(mut self, görünür: bool) -> Self {
        self.x_eksen_görünür = görünür;
        self
    }

    pub fn x_ızgarası_göster(mut self, görünür: bool) -> Self {
        self.x_ızgara_görünür = görünür;
        self
    }

    pub fn y_ekseni_göster(mut self, görünür: bool) -> Self {
        self.birincil_y_eksen_görünür = görünür;
        self
    }

    pub fn y_ızgarası_göster(mut self, görünür: bool) -> Self {
        self.birincil_y_ızgara_görünür = görünür;
        self
    }

    pub fn y_sabit_bölmeler(mut self, bölmeler: Vec<f64>) -> Self {
        if bölmeler.iter().all(|değer| değer.is_finite()) {
            self.birincil_y_sabit_bölmeler = Some(bölmeler);
        }
        self
    }

    pub fn y_özel_etiketler<I, S>(mut self, etiketler: I) -> Self
    where
        I: IntoIterator<Item = (f64, S)>,
        S: Into<String>,
    {
        self.birincil_y_özel_etiketler = etiketler
            .into_iter()
            .filter(|(değer, _)| değer.is_finite())
            .map(|(değer, etiket)| (değer, etiket.into()))
            .collect();
        self
    }

    pub fn y_ızgara_kesik(mut self, kesik: f32) -> Self {
        if kesik.is_finite() && kesik > 0.0 {
            self.birincil_y_ızgara_kesik = Some(kesik);
        }
        self
    }

    pub fn x_eksen_asgari_etiket_boşluğu(mut self, boşluk: f32) -> Self {
        if boşluk.is_finite() && boşluk > 0.0 {
            self.x_eksen_asgari_etiket_boşluğu = boşluk;
        }
        self
    }

    pub fn x_zaman_sabit_artımı(mut self, saniye: f64) -> Self {
        if saniye.is_finite() && saniye > 0.0 {
            self.x_zaman_sabit_artımı = Some(saniye);
        }
        self
    }

    pub fn bant(mut self, bant: SeriBandı) -> Self {
        self.bantlar.push(bant);
        self
    }

    pub fn nokta_katmanı(mut self, katman: NoktaKatmanı) -> Self {
        self.nokta_katmanları.push(katman);
        self
    }

    pub fn açıklamalar(mut self, düzen: AçıklamaDüzeni) -> Self {
        self.açıklama_düzeni = Some(düzen);
        self
    }

    pub fn ölçüm_datumları(mut self, etkin: bool) -> Self {
        self.ölçüm_datumları = etkin;
        self
    }

    pub fn rüzgar_yönü(mut self, düzen: RüzgarYönüDüzeni) -> Self {
        self.rüzgar_yönü_düzeni = Some(düzen);
        self
    }

    pub fn dağılım_düzeni(mut self, düzen: DağılımDüzeni) -> Self {
        self.dağılım_düzeni = Some(düzen);
        self
    }

    pub fn çizim_kancaları(mut self, düzen: ÇizimKancasıDüzeni) -> Self {
        self.çizim_kancaları = Some(düzen);
        self
    }

    pub fn odak(mut self, düzen: OdakDüzeni) -> Self {
        self.odak = Some(düzen);
        self
    }

    pub fn en_yakın_tooltip(mut self, düzen: EnYakınTooltipDüzeni) -> Self {
        self.en_yakın_tooltip = Some(düzen);
        self
    }

    pub fn tooltip(mut self, düzen: TooltipDüzeni) -> Self {
        self.tooltip = Some(düzen);
        self
    }

    pub fn boyut_senkronu(mut self, düzen: BoyutSenkronDüzeni) -> Self {
        self.boyut_senkron_düzeni = Some(düzen);
        self
    }

    pub fn lejant_canlı(mut self, canlı: bool) -> Self {
        self.lejant_canlı = canlı;
        self
    }

    pub fn çizim_sırası(mut self, sıra: ÇizimSırası) -> Self {
        self.çizim_sırası = sıra;
        self
    }

    /// Grafik düzeyindeki uPlot `pxAlign` değerini belirler.
    pub fn piksel_hizası(mut self, adım: f32) -> Self {
        if adım.is_finite() && adım >= 0.0 {
            self.piksel_hizası = adım;
        }
        self
    }

    pub fn ızgara_rengi(mut self, renk: impl Into<String>) -> Self {
        self.ızgara_rengi = renk.into();
        self
    }

    pub fn y_aralığı(mut self, aralık: Aralık) -> Self {
        self.y_aralığı = Some(aralık);
        self
    }

    pub fn kütle_spektrumu_y_aralığı(mut self, etkin: bool) -> Self {
        self.kütle_spektrumu_y_aralığı = etkin;
        self
    }

    pub fn x_aralığı(mut self, aralık: Aralık) -> Self {
        self.x_aralığı = Some(aralık);
        self
    }

    pub fn x_aralığını_veriye_yapıştır(mut self, etkin: bool) -> Self {
        self.x_aralığı_veriye_yapışık = etkin;
        self
    }

    pub fn y_ölçeği(mut self, ölçek: YÖlçekSeçenekleri) -> Self {
        if let Some(mevcut) = self
            .y_ölçekleri
            .iter_mut()
            .find(|mevcut| mevcut.anahtar == ölçek.anahtar)
        {
            *mevcut = ölçek;
        } else {
            self.y_ölçekleri.push(ölçek);
        }
        self
    }

    pub fn birincil_y_ölçeği(mut self, anahtar: impl Into<String>) -> Self {
        self.birincil_y_ölçeği = anahtar.into();
        self
    }

    pub fn x_eksen_etiketi(mut self, etiket: impl Into<String>) -> Self {
        self.x_eksen_etiketi = etiket.into();
        self
    }

    pub fn y_eksen_etiketi(mut self, etiket: impl Into<String>) -> Self {
        self.y_eksen_etiketi = etiket.into();
        self
    }

    pub fn birincil_y_sağda(mut self, sağda: bool) -> Self {
        self.birincil_y_sağda = sağda;
        self.birincil_y_karşıda = sağda;
        self
    }

    pub fn birincil_y_karşıda(mut self, karşıda: bool) -> Self {
        self.birincil_y_karşıda = karşıda;
        if !self.x_dikey {
            self.birincil_y_sağda = karşıda;
        }
        self
    }

    pub fn birincil_y_eksen_rengi(mut self, renk: impl Into<String>) -> Self {
        self.birincil_y_eksen_rengi = renk.into();
        self
    }

    pub fn birincil_y_eksen_genişliği(mut self, genişlik: f32) -> Self {
        if genişlik.is_finite() && genişlik >= 0.0 {
            self.birincil_y_eksen_genişliği = Some(genişlik);
        }
        self
    }

    pub fn x_eksen_değer_çarpanı(mut self, çarpan: f64) -> Self {
        if çarpan.is_finite() && çarpan > 0.0 {
            self.x_eksen_değer_çarpanı = çarpan;
        }
        self
    }

    pub fn otomatik_x_sağ_pay(mut self, etkin: bool) -> Self {
        self.otomatik_x_sağ_pay = etkin;
        self
    }

    pub fn otomatik_y_eksen_genişliği(mut self, etkin: bool) -> Self {
        self.otomatik_y_eksen_genişliği = etkin;
        self
    }

    pub fn eksen_göstergeleri(mut self, etkin: bool) -> Self {
        self.eksen_göstergeleri = etkin;
        self
    }

    pub fn kategoriler<I, S>(mut self, kategoriler: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.kategoriler = kategoriler.into_iter().map(Into::into).collect();
        self
    }

    pub fn çubuk_düzeni(mut self, düzen: ÇubukDüzeni) -> Self {
        self.çubuk_düzeni = Some(düzen);
        self
    }

    pub fn kutu_bıyık_düzeni(mut self, düzen: KutuBıyıkDüzeni) -> Self {
        self.kutu_bıyık_düzeni = Some(düzen);
        self
    }

    pub fn mum_düzeni(mut self, düzen: MumDüzeni) -> Self {
        self.mum_düzeni = Some(düzen);
        self
    }

    pub fn ısı_haritası_düzeni(mut self, düzen: IsıHaritasıDüzeni) -> Self {
        self.ısı_haritası_düzeni = Some(düzen);
        self
    }

    pub fn timeline_düzeni(mut self, düzen: TimelineDüzeni) -> Self {
        self.timeline_düzeni = Some(düzen);
        self
    }

    pub fn imleç_ızgara_adımı(mut self, piksel: f32) -> Self {
        if piksel.is_finite() && piksel > 0.0 {
            self.imleç_ızgara_adımı = Some(piksel);
        }
        self
    }

    pub fn etkileşimler(mut self, etkileşimler: EtkileşimSeçenekleri) -> Self {
        self.etkileşimler = etkileşimler;
        self
    }

    pub fn seri(mut self, seri: SeriSeçenekleri) -> Self {
        self.seriler.push(seri);
        self
    }
}
