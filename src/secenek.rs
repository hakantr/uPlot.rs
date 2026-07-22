use crate::{Aralık, UplotHatası};

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
}

impl Default for EtkileşimSeçenekleri {
    fn default() -> Self {
        Self {
            tekerlek_etkileşimi: false,
            tekerlek_ayarları: TekerlekAyarları::default(),
            seçim_yakınlaştır: true,
            çift_tıkla_tam_görünüm: true,
            görünüm_geçmişi: false,
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
}

#[derive(Debug, Clone, PartialEq)]
pub struct SeriSeçenekleri {
    pub etiket: String,
    pub renk: String,
    pub çizgi_kalınlığı: f32,
    pub göster: bool,
}

impl SeriSeçenekleri {
    pub fn yeni(etiket: impl Into<String>) -> Self {
        Self {
            etiket: etiket.into(),
            renk: "#000000".to_string(),
            çizgi_kalınlığı: 1.0,
            göster: true,
        }
    }

    pub fn renk(mut self, renk: impl Into<String>) -> Self {
        self.renk = renk.into();
        self
    }

    pub fn çizgi_kalınlığı(mut self, kalınlık: f32) -> Self {
        self.çizgi_kalınlığı = kalınlık.max(0.0);
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

    pub fn etkileşimler(mut self, etkileşimler: EtkileşimSeçenekleri) -> Self {
        self.etkileşimler = etkileşimler;
        self
    }

    pub fn seri(mut self, seri: SeriSeçenekleri) -> Self {
        self.seriler.push(seri);
        self
    }
}
