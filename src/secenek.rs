use crate::{Aralık, UplotHatası};

mod gradyan;
mod isi_haritasi;
mod seri;
mod y_olcek;
mod zaman;

pub use gradyan::{GradyanDurağı, GradyanEkseni, GradyanKonumu, ÖlçekGradyanı};
pub use isi_haritasi::{IsıHaritasıDüzeni, IsıHücresi, IsıHücresiBoyutu};
pub use seri::{NoktaFiltreKipi, SeriSeçenekleri, SeriÇizimTürü};
pub use y_olcek::{
    GüzelÖlçekDüzeni, YÖlçekDağılımı, YÖlçekEtiketBiçimi, YÖlçekSeçenekleri
};
pub use zaman::TarihAdları;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum XÖlçekDağılımı {
    Doğrusal,
    Logaritmik { taban: f64 },
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ÇubukDüzeni {
    pub yön: ÇubukYönü,
    pub yığılmış: bool,
    pub ters: bool,
    pub değer_etiketi_otomatik: bool,
}

impl ÇubukDüzeni {
    pub fn yeni(yön: ÇubukYönü) -> Self {
        Self {
            yön,
            yığılmış: false,
            ters: false,
            değer_etiketi_otomatik: false,
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
}

#[derive(Debug, Clone, PartialEq)]
pub struct GrafikSeçenekleri {
    pub başlık: String,
    pub arka_plan_rengi: String,
    pub başlık_rengi: String,
    pub genişlik: u32,
    pub yükseklik: u32,
    pub x_zaman: bool,
    pub x_zaman_milisaniye: bool,
    pub x_tarih_adları: TarihAdları,
    pub x_dağılımı: XÖlçekDağılımı,
    pub x_ters_yön: bool,
    /// uPlot `scales.x.ori = 1` karşılığıdır. Etkin olduğunda X dikey,
    /// Y yatay çizilir.
    pub x_dikey: bool,
    /// X eksenini yönelimine göre karşı tarafa taşır: standart düzende üst,
    /// değiştirilmiş düzende sağ.
    pub x_eksen_karşıda: bool,
    pub x_aralığı: Option<Aralık>,
    pub y_aralığı: Option<Aralık>,
    pub y_ölçekleri: Vec<YÖlçekSeçenekleri>,
    pub birincil_y_ölçeği: String,
    pub x_eksen_etiketi: String,
    pub x_eksen_rengi: String,
    pub x_eksen_etiket_biçimi: YÖlçekEtiketBiçimi,
    pub y_eksen_etiketi: String,
    pub birincil_y_sağda: bool,
    /// Birincil Y eksenini yönelimine göre karşı tarafa taşır. Standart
    /// düzende sağ, değiştirilmiş düzende alt taraftır.
    pub birincil_y_karşıda: bool,
    pub birincil_y_eksen_rengi: String,
    pub x_eksen_değer_çarpanı: f64,
    pub otomatik_x_sağ_pay: bool,
    pub otomatik_y_eksen_genişliği: bool,
    pub eksen_göstergeleri: bool,
    pub kategoriler: Vec<String>,
    pub çubuk_düzeni: Option<ÇubukDüzeni>,
    pub kutu_bıyık_düzeni: Option<KutuBıyıkDüzeni>,
    pub mum_düzeni: Option<MumDüzeni>,
    pub ısı_haritası_düzeni: Option<IsıHaritasıDüzeni>,
    pub bantlar: Vec<SeriBandı>,
    pub nokta_katmanları: Vec<NoktaKatmanı>,
    pub çizim_kancaları: Option<ÇizimKancasıDüzeni>,
    pub odak: Option<OdakDüzeni>,
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
            x_zaman: true,
            x_zaman_milisaniye: false,
            x_tarih_adları: TarihAdları::default(),
            x_dağılımı: XÖlçekDağılımı::Doğrusal,
            x_ters_yön: false,
            x_dikey: false,
            x_eksen_karşıda: false,
            x_aralığı: None,
            y_aralığı: None,
            y_ölçekleri: Vec::new(),
            birincil_y_ölçeği: "y".to_string(),
            x_eksen_etiketi: String::new(),
            x_eksen_rengi: "#4b5563".to_string(),
            x_eksen_etiket_biçimi: YÖlçekEtiketBiçimi::Otomatik,
            y_eksen_etiketi: String::new(),
            birincil_y_sağda: false,
            birincil_y_karşıda: false,
            birincil_y_eksen_rengi: "#4b5563".to_string(),
            x_eksen_değer_çarpanı: 1.0,
            otomatik_x_sağ_pay: false,
            otomatik_y_eksen_genişliği: false,
            eksen_göstergeleri: false,
            kategoriler: Vec::new(),
            çubuk_düzeni: None,
            kutu_bıyık_düzeni: None,
            mum_düzeni: None,
            ısı_haritası_düzeni: None,
            bantlar: Vec::new(),
            nokta_katmanları: Vec::new(),
            çizim_kancaları: None,
            odak: None,
            çizim_sırası: ÇizimSırası::EksenlerSeriler,
            piksel_hizası: 1.0,
            ızgara_rengi: "#e5e7eb".to_string(),
            imleç_ızgara_adımı: None,
            etkileşimler: EtkileşimSeçenekleri::default(),
            seriler: Vec::new(),
        })
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

    pub fn bant(mut self, bant: SeriBandı) -> Self {
        self.bantlar.push(bant);
        self
    }

    pub fn nokta_katmanı(mut self, katman: NoktaKatmanı) -> Self {
        self.nokta_katmanları.push(katman);
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

    pub fn x_aralığı(mut self, aralık: Aralık) -> Self {
        self.x_aralığı = Some(aralık);
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
