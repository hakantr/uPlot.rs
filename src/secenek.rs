use crate::{Aralık, UplotHatası};

mod seri;
mod y_olcek;

pub use seri::SeriSeçenekleri;
pub use y_olcek::{YÖlçekDağılımı, YÖlçekSeçenekleri};

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
}

#[derive(Debug, Clone, PartialEq)]
pub struct GrafikSeçenekleri {
    pub başlık: String,
    pub genişlik: u32,
    pub yükseklik: u32,
    pub x_zaman: bool,
    pub y_aralığı: Option<Aralık>,
    pub y_ölçekleri: Vec<YÖlçekSeçenekleri>,
    pub birincil_y_ölçeği: String,
    pub x_eksen_etiketi: String,
    pub y_eksen_etiketi: String,
    pub birincil_y_sağda: bool,
    pub birincil_y_eksen_rengi: String,
    pub x_eksen_değer_çarpanı: f64,
    pub otomatik_x_sağ_pay: bool,
    pub otomatik_y_eksen_genişliği: bool,
    pub eksen_göstergeleri: bool,
    pub kategoriler: Vec<String>,
    pub çubuk_düzeni: Option<ÇubukDüzeni>,
    pub kutu_bıyık_düzeni: Option<KutuBıyıkDüzeni>,
    pub mum_düzeni: Option<MumDüzeni>,
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
            genişlik,
            yükseklik,
            x_zaman: true,
            y_aralığı: None,
            y_ölçekleri: Vec::new(),
            birincil_y_ölçeği: "y".to_string(),
            x_eksen_etiketi: String::new(),
            y_eksen_etiketi: String::new(),
            birincil_y_sağda: false,
            birincil_y_eksen_rengi: "#4b5563".to_string(),
            x_eksen_değer_çarpanı: 1.0,
            otomatik_x_sağ_pay: false,
            otomatik_y_eksen_genişliği: false,
            eksen_göstergeleri: false,
            kategoriler: Vec::new(),
            çubuk_düzeni: None,
            kutu_bıyık_düzeni: None,
            mum_düzeni: None,
            imleç_ızgara_adımı: None,
            etkileşimler: EtkileşimSeçenekleri::default(),
            seriler: Vec::new(),
        })
    }

    pub fn başlık(mut self, başlık: impl Into<String>) -> Self {
        self.başlık = başlık.into();
        self
    }

    pub fn x_zaman(mut self, zaman: bool) -> Self {
        self.x_zaman = zaman;
        self
    }

    pub fn y_aralığı(mut self, aralık: Aralık) -> Self {
        self.y_aralığı = Some(aralık);
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
