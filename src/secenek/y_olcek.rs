use crate::{Aralık, UplotHatası};

/// `nice-scale.html` içindeki piksel yüksekliğine bağlı güzel sayı
/// algoritmasının doğrulanmış ayarları.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GüzelÖlçekDüzeni {
    pub(crate) en_az_etiket_boşluğu: f32,
}

impl GüzelÖlçekDüzeni {
    pub fn yeni(en_az_etiket_boşluğu: f32) -> Result<Self, UplotHatası> {
        if !en_az_etiket_boşluğu.is_finite() || en_az_etiket_boşluğu <= 0.0 {
            return Err(UplotHatası::GeçersizEksenBoşluğu {
                değer: en_az_etiket_boşluğu,
            });
        }
        Ok(Self {
            en_az_etiket_boşluğu,
        })
    }

    pub fn en_az_etiket_boşluğu(self) -> f32 {
        self.en_az_etiket_boşluğu
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum YÖlçekDağılımı {
    Doğrusal,
    Logaritmik {
        taban: f64,
    },
    /// `log(-log(1-y))` ileri ve `1-exp(-exp(v))` geri dönüşümü.
    Weibull,
    ArcSinh {
        eşik: f64,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YÖlçekEtiketBiçimi {
    Otomatik,
    ArtımaGöre,
    Bilimsel,
    İkiliÜs,
    İkiliŞapka,
    Kompakt,
}

/// uPlot'un adlandırılmış Y ölçekleri ve bunlara bağlı eksenlerinin çekirdek
/// karşılığıdır. `y` anahtarı birincil sol eksendir; diğer ölçekler sağda
/// gösterilebilir.
#[derive(Debug, Clone, PartialEq)]
pub struct YÖlçekSeçenekleri {
    pub anahtar: String,
    pub aralık: Option<Aralık>,
    pub sağda: bool,
    pub ızgara: bool,
    pub eksen_görünür: bool,
    pub eksen_rengi: String,
    pub eksen_etiketi: String,
    pub birim: String,
    pub kaynak: Option<String>,
    pub dönüşüm_çarpanı: f64,
    pub dönüşüm_kaydırması: f64,
    pub dağılım: YÖlçekDağılımı,
    pub ters_yön: bool,
    pub eksen_değer_çarpanı: f64,
    pub etiket_biçimi: YÖlçekEtiketBiçimi,
    pub log_tam_büyüklükler: bool,
    pub güzel_ölçek: Option<GüzelÖlçekDüzeni>,
}

impl YÖlçekSeçenekleri {
    pub fn yeni(anahtar: impl Into<String>) -> Self {
        Self {
            anahtar: anahtar.into(),
            aralık: None,
            sağda: false,
            ızgara: true,
            eksen_görünür: false,
            eksen_rengi: "#4b5563".to_string(),
            eksen_etiketi: String::new(),
            birim: String::new(),
            kaynak: None,
            dönüşüm_çarpanı: 1.0,
            dönüşüm_kaydırması: 0.0,
            dağılım: YÖlçekDağılımı::Doğrusal,
            ters_yön: false,
            eksen_değer_çarpanı: 1.0,
            etiket_biçimi: YÖlçekEtiketBiçimi::Otomatik,
            log_tam_büyüklükler: true,
            güzel_ölçek: None,
        }
    }

    pub fn aralık(mut self, aralık: Aralık) -> Self {
        self.aralık = Some(aralık);
        self
    }

    pub fn sağda(mut self, sağda: bool) -> Self {
        self.sağda = sağda;
        self
    }

    pub fn ızgara(mut self, görünür: bool) -> Self {
        self.ızgara = görünür;
        self
    }

    pub fn eksen(mut self, görünür: bool) -> Self {
        self.eksen_görünür = görünür;
        self
    }

    pub fn eksen_rengi(mut self, renk: impl Into<String>) -> Self {
        self.eksen_rengi = renk.into();
        self
    }

    pub fn eksen_etiketi(mut self, etiket: impl Into<String>) -> Self {
        self.eksen_etiketi = etiket.into();
        self
    }

    pub fn birim(mut self, birim: impl Into<String>) -> Self {
        self.birim = birim.into();
        self
    }

    /// uPlot `scale.from` ve `scale.range` birleşiminin doğrusal karşılığıdır.
    /// `çıktı = kaynak * çarpan + kaydırma` dönüşümünü uygular.
    pub fn kaynak_dönüşümü(
        mut self,
        kaynak: impl Into<String>,
        çarpan: f64,
        kaydırma: f64,
    ) -> Self {
        if çarpan.is_finite() && çarpan.abs() > f64::EPSILON && kaydırma.is_finite() {
            self.kaynak = Some(kaynak.into());
            self.dönüşüm_çarpanı = çarpan;
            self.dönüşüm_kaydırması = kaydırma;
        }
        self
    }

    /// uPlot `distr: 4` ve `asinh` doğrusal eşik ayarını etkinleştirir.
    pub fn arcsinh(mut self, eşik: f64) -> Self {
        if eşik.is_finite() && eşik > 0.0 {
            self.dağılım = YÖlçekDağılımı::ArcSinh { eşik };
        }
        self
    }

    pub fn logaritmik(mut self, taban: f64) -> Self {
        if taban.is_finite() && taban > 1.0 {
            self.dağılım = YÖlçekDağılımı::Logaritmik { taban };
        }
        self
    }

    /// uPlot `rangeLog(..., fullMags: false)` karşılığıdır.
    pub fn logaritmik_kısmi(mut self, taban: f64) -> Self {
        if taban.is_finite() && taban > 1.0 {
            self.dağılım = YÖlçekDağılımı::Logaritmik { taban };
            self.log_tam_büyüklükler = false;
        }
        self
    }

    /// uPlot `Scale.dir: -1` karşılığıdır.
    pub fn ters_yön(mut self, ters: bool) -> Self {
        self.ters_yön = ters;
        self
    }

    /// Eksen değerini veri geometrisini değiştirmeden gösterim için dönüştürür.
    pub fn eksen_değer_çarpanı(mut self, çarpan: f64) -> Self {
        if çarpan.is_finite() && çarpan.abs() > f64::EPSILON {
            self.eksen_değer_çarpanı = çarpan;
        }
        self
    }

    pub fn etiket_biçimi(mut self, biçim: YÖlçekEtiketBiçimi) -> Self {
        self.etiket_biçimi = biçim;
        self
    }

    pub fn weibull(mut self) -> Self {
        self.dağılım = YÖlçekDağılımı::Weibull;
        self
    }

    pub fn güzel_ölçek(mut self, düzen: GüzelÖlçekDüzeni) -> Self {
        self.güzel_ölçek = Some(düzen);
        self
    }
}
