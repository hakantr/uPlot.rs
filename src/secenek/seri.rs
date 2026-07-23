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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoktaFiltreKipi {
    Yok,
    /// uPlot `points.filter` örneğindeki gibi, normal nokta katmanı yoğunluk
    /// nedeniyle gizliyken yalnız null boşlukları arasındaki tekil değerleri
    /// görünür tutar.
    BoşlukArasındakiTekiller,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NoktaŞekli {
    #[default]
    Daire,
    Kare,
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
    /// Aynı X konumunda başka bir dönemi karşılaştıran serinin bilgi
    /// kutusunda kullanılacak saniye kaydırması.
    pub x_zaman_kaydırması: f64,
    pub azami_x_boşluğu: Option<f64>,
    pub boşlukları_birleştir: bool,
    pub çizim_türü: SeriÇizimTürü,
    /// Kaynak `Path2D` içinde her null koşusunu `moveTo/lineTo` ile kuran,
    /// yerleşik yol sadeleştirmesi kullanmayan özel doğrusal yol işaretidir.
    pub saf_doğrusal_yol: bool,
    pub çubuk_genişlik_oranı: f32,
    pub azami_çubuk_genişliği: f32,
    pub çubuk_hizası: i8,
    /// `uPlot.paths.bars({disp: {y1}})` karşılığı olarak, bu serideki
    /// değerleri çubuk alt ucu; belirtilen veri serisini üst uç yapar.
    pub yüzen_çubuk_üst_serisi: Option<usize>,
    /// `disp.fill.values` karşılığı nokta başına çubuk dolguları.
    pub çubuk_dolguları: Vec<String>,
    pub gösterim_değer_çarpanı: f64,
    /// Çizim verisi kümülatif/dönüştürülmüş olduğunda cursor ve lejantta
    /// gösterilecek ham kaynak değerleri. Uzunluk uyuşmazsa güvenle yoksayılır.
    pub lejant_değerleri: Option<Vec<Option<f64>>>,
    /// uPlot `series.pxAlign` karşılığıdır. `None`, grafik düzeyindeki
    /// `pxAlign` değerini devralır; `Some(0.0)` hizalamayı kapatır.
    pub piksel_hizası: Option<f32>,
    /// uPlot `series.points.show` karşılığıdır. `None`, çekirdeğin koşullu
    /// varsayılanını; `Some(true/false)` açık geliştirici kararını kullanır.
    pub noktaları_göster: Option<bool>,
    pub nokta_boşluğu: f32,
    pub nokta_boyutu: f32,
    pub nokta_kalınlığı: f32,
    pub nokta_dolgusu: Option<String>,
    pub nokta_şekli: NoktaŞekli,
    pub nokta_filtresi: NoktaFiltreKipi,
    /// Özel `points.filter` oluşturucularının göstereceği kaynak indeksleri.
    /// `None`, geçerli tüm noktaları kullanır.
    pub nokta_indeksleri: Option<Vec<usize>>,
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
            x_zaman_kaydırması: 0.0,
            azami_x_boşluğu: None,
            boşlukları_birleştir: false,
            çizim_türü: SeriÇizimTürü::Çizgi,
            saf_doğrusal_yol: false,
            çubuk_genişlik_oranı: 0.6,
            azami_çubuk_genişliği: f32::INFINITY,
            çubuk_hizası: 0,
            yüzen_çubuk_üst_serisi: None,
            çubuk_dolguları: Vec::new(),
            gösterim_değer_çarpanı: 1.0,
            lejant_değerleri: None,
            piksel_hizası: None,
            noktaları_göster: None,
            nokta_boşluğu: 10.0,
            nokta_boyutu: 5.0,
            nokta_kalınlığı: 1.0,
            nokta_dolgusu: None,
            nokta_şekli: NoktaŞekli::Daire,
            nokta_filtresi: NoktaFiltreKipi::Yok,
            nokta_indeksleri: None,
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

    pub fn x_zaman_kaydırması(mut self, saniye: f64) -> Self {
        if saniye.is_finite() {
            self.x_zaman_kaydırması = saniye;
        }
        self
    }

    pub fn yüzen_çubuk_üst_serisi(mut self, seri_indeksi: usize) -> Self {
        self.yüzen_çubuk_üst_serisi = Some(seri_indeksi);
        self
    }

    pub fn çubuk_dolguları<I, S>(mut self, dolgular: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.çubuk_dolguları = dolgular.into_iter().map(Into::into).collect();
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

    pub fn saf_doğrusal_yol(mut self, etkin: bool) -> Self {
        self.saf_doğrusal_yol = etkin;
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

    /// uPlot `series.value` gösterim dönüşümünün doğrusal karşılığıdır.
    /// Çizim geometrisi ham veriyi kullanmaya devam eder.
    pub fn gösterim_değer_çarpanı(mut self, çarpan: f64) -> Self {
        if çarpan.is_finite() && çarpan.abs() > f64::EPSILON {
            self.gösterim_değer_çarpanı = çarpan;
        }
        self
    }

    pub fn lejant_değerleri(mut self, değerler: Vec<Option<f64>>) -> Self {
        self.lejant_değerleri = Some(değerler);
        self
    }

    /// Seri yol ve nokta koordinatlarının hangi piksel adımına
    /// yuvarlanacağını belirler. `0`, uPlot'taki gibi yuvarlamayı kapatır.
    pub fn piksel_hizası(mut self, adım: f32) -> Self {
        if adım.is_finite() && adım >= 0.0 {
            self.piksel_hizası = Some(adım);
        }
        self
    }

    pub fn noktaları_göster(mut self, göster: bool) -> Self {
        self.noktaları_göster = Some(göster);
        self
    }

    pub fn nokta_boşluğu(mut self, boşluk: f32) -> Self {
        if boşluk.is_finite() && boşluk >= 0.0 {
            self.nokta_boşluğu = boşluk;
        }
        self
    }

    pub fn nokta_stili(
        mut self,
        boyut: f32,
        kalınlık: f32,
        dolgu: Option<impl Into<String>>,
    ) -> Self {
        if boyut.is_finite() && boyut > 0.0 {
            self.nokta_boyutu = boyut;
        }
        if kalınlık.is_finite() && kalınlık >= 0.0 {
            self.nokta_kalınlığı = kalınlık.min(self.nokta_boyutu);
        }
        self.nokta_dolgusu = dolgu.map(Into::into);
        self
    }

    pub fn nokta_filtresi(mut self, filtre: NoktaFiltreKipi) -> Self {
        self.nokta_filtresi = filtre;
        self
    }

    pub fn nokta_şekli(mut self, şekil: NoktaŞekli) -> Self {
        self.nokta_şekli = şekil;
        self
    }

    pub fn nokta_indeksleri(mut self, indeksler: Vec<usize>) -> Self {
        self.nokta_indeksleri = Some(indeksler);
        self
    }
}
use super::gradyan::ÖlçekGradyanı;
