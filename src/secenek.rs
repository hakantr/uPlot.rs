use crate::{Aralık, UplotHatası};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EtkileşimSeçenekleri {
    /// uPlot'un resmi `wheelZoomPlugin` portunu etkinleştirir. Varsayılan: kapalı.
    pub tekerlek_etkileşimi: bool,
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
