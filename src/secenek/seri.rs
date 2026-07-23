#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeriÇizimTürü {
    Çizgi,
    Noktalar,
    BasamakÖnce,
    BasamakSonra,
    Eğri,
    CatmullRom,
    Çubuk,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SeriSeçenekleri {
    pub etiket: String,
    pub renk: String,
    pub çizgi_kalınlığı: f32,
    pub çizgi_kesik: Option<(f32, f32)>,
    pub çizgi_gradyanı: Option<ÖlçekGradyanı>,
    pub dolgu: Option<String>,
    pub dolgu_gradyanı: Option<ÖlçekGradyanı>,
    pub dolgu_tabanı: f64,
    pub göster: bool,
    pub ölçek: String,
    pub azami_x_boşluğu: Option<f64>,
    pub boşlukları_birleştir: bool,
    pub çizim_türü: SeriÇizimTürü,
    pub çubuk_genişlik_oranı: f32,
    pub azami_çubuk_genişliği: f32,
    pub çubuk_hizası: i8,
}

impl SeriSeçenekleri {
    pub fn yeni(etiket: impl Into<String>) -> Self {
        Self {
            etiket: etiket.into(),
            renk: "#000000".to_string(),
            çizgi_kalınlığı: 1.0,
            çizgi_kesik: None,
            çizgi_gradyanı: None,
            dolgu: None,
            dolgu_gradyanı: None,
            dolgu_tabanı: 0.0,
            göster: true,
            ölçek: "y".to_string(),
            azami_x_boşluğu: None,
            boşlukları_birleştir: false,
            çizim_türü: SeriÇizimTürü::Çizgi,
            çubuk_genişlik_oranı: 0.6,
            azami_çubuk_genişliği: f32::INFINITY,
            çubuk_hizası: 0,
        }
    }

    pub fn renk(mut self, renk: impl Into<String>) -> Self {
        self.renk = renk.into();
        self
    }

    pub fn çizgi_kalınlığı(mut self, kalınlık: f32) -> Self {
        if kalınlık.is_finite() {
            self.çizgi_kalınlığı = kalınlık.max(0.0);
        }
        self
    }

    pub fn çizgi_kesik(mut self, çizgi: f32, boşluk: f32) -> Self {
        if çizgi.is_finite() && boşluk.is_finite() && çizgi > 0.0 && boşluk > 0.0 {
            self.çizgi_kesik = Some((çizgi, boşluk));
        }
        self
    }

    pub fn çizgi_gradyanı(mut self, gradyan: ÖlçekGradyanı) -> Self {
        self.çizgi_gradyanı = Some(gradyan);
        self
    }

    /// uPlot `Series.fill` karşılığıdır. Doğrusal yol varsayılan olarak
    /// `fillTo = 0` tabanına kapatılır.
    pub fn dolgu(mut self, renk: impl Into<String>) -> Self {
        self.dolgu = Some(renk.into());
        self
    }

    pub fn dolgu_gradyanı(mut self, gradyan: ÖlçekGradyanı) -> Self {
        self.dolgu_gradyanı = Some(gradyan);
        self
    }

    pub fn dolgu_tabanı(mut self, değer: f64) -> Self {
        if değer.is_finite() {
            self.dolgu_tabanı = değer;
        }
        self
    }

    pub fn ölçek(mut self, anahtar: impl Into<String>) -> Self {
        self.ölçek = anahtar.into();
        self
    }

    /// Ardışık X değerleri arasındaki fark bu eşiği aştığında yol ve dolgu
    /// parçasını böler. uPlot `series.gaps` callback'inin sayısal karşılığıdır.
    pub fn azami_x_boşluğu(mut self, fark: f64) -> Self {
        if fark.is_finite() && fark > 0.0 {
            self.azami_x_boşluğu = Some(fark);
        }
        self
    }

    pub fn göster(mut self, göster: bool) -> Self {
        self.göster = göster;
        self
    }

    /// uPlot `spanGaps` karşılığıdır.
    pub fn boşlukları_birleştir(mut self, birleştir: bool) -> Self {
        self.boşlukları_birleştir = birleştir;
        self
    }

    /// Bu seriyi uPlot `paths.bars()` geometrisiyle çizer.
    pub fn çubuk(mut self, çubuk: bool) -> Self {
        self.çizim_türü = if çubuk {
            SeriÇizimTürü::Çubuk
        } else {
            SeriÇizimTürü::Çizgi
        };
        self
    }

    pub fn basamak_önce(mut self) -> Self {
        self.çizim_türü = SeriÇizimTürü::BasamakÖnce;
        self
    }

    pub fn basamak_sonra(mut self) -> Self {
        self.çizim_türü = SeriÇizimTürü::BasamakSonra;
        self
    }

    pub fn eğri(mut self) -> Self {
        self.çizim_türü = SeriÇizimTürü::Eğri;
        self
    }

    pub fn catmull_rom(mut self) -> Self {
        self.çizim_türü = SeriÇizimTürü::CatmullRom;
        self
    }

    pub fn yalnız_noktalar(mut self) -> Self {
        self.çizim_türü = SeriÇizimTürü::Noktalar;
        self
    }

    pub fn çubuk_boyutu(mut self, oran: f32, azami: f32) -> Self {
        if oran.is_finite() && (0.0..=1.0).contains(&oran) {
            self.çubuk_genişlik_oranı = oran;
        }
        if azami.is_finite() && azami > 0.0 {
            self.azami_çubuk_genişliği = azami;
        }
        self
    }

    pub fn çubuk_hizası(mut self, hiza: i8) -> Self {
        self.çubuk_hizası = hiza.clamp(-1, 1);
        self
    }
}
use super::gradyan::ÖlçekGradyanı;
