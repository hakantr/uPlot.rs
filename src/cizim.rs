#[cfg(any(feature = "gpui-svg", test))]
use std::fmt::Write as _;

#[cfg(test)]
use std::cell::Cell;

pub(crate) mod kirpma;

#[cfg(test)]
std::thread_local! {
    static SVG_SERİLEŞTİRME_ÇAĞRILARI: Cell<usize> = const { Cell::new(0) };
}

#[cfg(test)]
pub(crate) fn test_svg_serileştirme_sayacını_sıfırla() {
    SVG_SERİLEŞTİRME_ÇAĞRILARI.with(|sayı| sayı.set(0));
}

#[cfg(test)]
pub(crate) fn test_svg_serileştirme_çağrıları() -> usize {
    SVG_SERİLEŞTİRME_ÇAĞRILARI.with(Cell::get)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Nokta {
    pub x: f32,
    pub y: f32,
}

impl Nokta {
    pub fn yeni(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GradyanRenkDurağı {
    pub oran: f32,
    pub renk: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DoğrusalGradyan {
    pub başlangıç: Nokta,
    pub bitiş: Nokta,
    pub duraklar: Vec<GradyanRenkDurağı>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetinHizası {
    Başlangıç,
    Orta,
    Bitiş,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct KöşeYarıçapları {
    pub üst_sol: f32,
    pub üst_sağ: f32,
    pub alt_sağ: f32,
    pub alt_sol: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Komut {
    ArkaPlan {
        renk: String,
    },
    Çizgi {
        başlangıç: Nokta,
        bitiş: Nokta,
        renk: String,
        kalınlık: f32,
    },
    KesikliÇizgi {
        başlangıç: Nokta,
        bitiş: Nokta,
        renk: String,
        kalınlık: f32,
        kesik: f32,
    },
    Yol {
        parçalar: Vec<Vec<Nokta>>,
        renk: String,
        kalınlık: f32,
    },
    GradyanYol {
        parçalar: Vec<Vec<Nokta>>,
        gradyan: DoğrusalGradyan,
        kalınlık: f32,
    },
    KesikliYol {
        parçalar: Vec<Vec<Nokta>>,
        renk: String,
        kalınlık: f32,
        çizgi: f32,
        boşluk: f32,
    },
    Alan {
        çokgenler: Vec<Vec<Nokta>>,
        dolgu: String,
    },
    GradyanAlan {
        çokgenler: Vec<Vec<Nokta>>,
        gradyan: DoğrusalGradyan,
    },
    Daire {
        merkez: Nokta,
        yarıçap: f32,
        dolgu: String,
        çizgi: String,
        kalınlık: f32,
    },
    /// Aynı stile ve yarıçapa sahip çok sayıda daireyi tek çizim komutunda taşır.
    ///
    /// uPlot'un scatter `Path2D` yaklaşımının sahne karşılığıdır: arka uçlar
    /// her noktayı ayrı bir DOM/sahne öğesine dönüştürmek zorunda kalmaz.
    Daireler {
        merkezler: Vec<Nokta>,
        yarıçap: f32,
        dolgu: String,
        çizgi: String,
        kalınlık: f32,
        kesme_sınırları: Option<(Nokta, Nokta)>,
    },
    /// Aynı stile sahip değişken yarıçaplı daireleri tek Path2D/SVG yolunda taşır.
    DeğişkenDaireler {
        daireler: Vec<(Nokta, f32)>,
        dolgu: String,
        çizgi: String,
        kalınlık: f32,
        kesme_sınırları: Option<(Nokta, Nokta)>,
    },
    Dikdörtgen {
        konum: Nokta,
        genişlik: f32,
        yükseklik: f32,
        dolgu: String,
        çizgi: String,
        kalınlık: f32,
    },
    YuvarlatılmışDikdörtgen {
        konum: Nokta,
        genişlik: f32,
        yükseklik: f32,
        yarıçaplar: KöşeYarıçapları,
        dolgu: String,
        çizgi: String,
        kalınlık: f32,
    },
    Metin {
        konum: Nokta,
        içerik: String,
        renk: String,
        boyut: f32,
        hiza: MetinHizası,
    },
    DöndürülmüşMetin {
        konum: Nokta,
        içerik: String,
        renk: String,
        boyut: f32,
        hiza: MetinHizası,
        açı: f32,
    },
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum SahneKatmanı {
    #[default]
    Veri,
    Eksen,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Sahne {
    genişlik: u32,
    yükseklik: u32,
    komutlar: Vec<Komut>,
    geometri_kimlikleri: Vec<u64>,
    katmanlar: Vec<SahneKatmanı>,
    etkin_katman: SahneKatmanı,
}

impl Sahne {
    pub fn yeni(genişlik: u32, yükseklik: u32) -> Self {
        Self {
            genişlik,
            yükseklik,
            komutlar: Vec::new(),
            geometri_kimlikleri: Vec::new(),
            katmanlar: Vec::new(),
            etkin_katman: SahneKatmanı::Veri,
        }
    }

    pub fn ekle(&mut self, komut: Komut) {
        self.geometri_kimlikleri
            .push(komut_geometri_kimliği(&komut));
        self.katmanlar.push(self.etkin_katman);
        self.komutlar.push(komut);
    }

    pub(crate) fn katmanı_ayarla(&mut self, katman: SahneKatmanı) {
        self.etkin_katman = katman;
    }

    pub(crate) fn katmanı_süz(mut self, katman: SahneKatmanı) -> Self {
        let mut indeks = 0;
        self.komutlar.retain(|_| {
            let tutulacak = self.katmanlar.get(indeks).copied() == Some(katman);
            indeks += 1;
            tutulacak
        });
        let mut indeks = 0;
        self.geometri_kimlikleri.retain(|_| {
            let tutulacak = self.katmanlar.get(indeks).copied() == Some(katman);
            indeks += 1;
            tutulacak
        });
        self.katmanlar.retain(|değer| *değer == katman);
        self
    }

    pub fn komutlar(&self) -> &[Komut] {
        &self.komutlar
    }

    pub(crate) fn geometri_kimlikleri(&self) -> &[u64] {
        &self.geometri_kimlikleri
    }

    pub(crate) fn komut_aralığını_sona_taşı(&mut self, başlangıç: usize, bitiş: usize) {
        if başlangıç >= bitiş || bitiş > self.komutlar.len() {
            return;
        }
        let eksen_komutu_sayısı = bitiş - başlangıç;
        if let Some(komutlar) = self.komutlar.get_mut(başlangıç..) {
            komutlar.rotate_left(eksen_komutu_sayısı);
        }
        if let Some(kimlikler) = self.geometri_kimlikleri.get_mut(başlangıç..) {
            kimlikler.rotate_left(eksen_komutu_sayısı);
        }
        if let Some(katmanlar) = self.katmanlar.get_mut(başlangıç..) {
            katmanlar.rotate_left(eksen_komutu_sayısı);
        }
    }

    pub fn boyut(&self) -> (u32, u32) {
        (self.genişlik, self.yükseklik)
    }

    /// Bu sahnenin o anki retained komutlarını bağımsız bir SVG belgesine
    /// dönüştürür.
    ///
    /// Normal GPUI paint yolunda çağrılmaz; serileştirme maliyeti yalnız
    /// geliştirici açıkça SVG çıktısı istediğinde oluşur.
    #[cfg(feature = "gpui-svg")]
    pub fn svg(&self) -> String {
        let mut çıktı = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n",
            self.genişlik, self.yükseklik, self.genişlik, self.yükseklik
        );
        çıktı.push_str(&self.svg_içeriği(""));
        çıktı.push_str("</svg>\n");
        çıktı
    }

    #[cfg(test)]
    pub(crate) fn test_svg(&self) -> String {
        let mut çıktı = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n",
            self.genişlik, self.yükseklik, self.genişlik, self.yükseklik
        );
        çıktı.push_str(&self.svg_içeriği(""));
        çıktı.push_str("</svg>\n");
        çıktı
    }

    /// Bir üst SVG belgesine yerleştirilecek vektör gövdesini üretir.
    ///
    /// GPUI normal paint akışı bu yöntemi çağırmaz; yalnız açık dışa aktarım
    /// isteği retained sahneyi vektör kaydına dönüştürür.
    #[cfg(any(feature = "gpui-svg", test))]
    pub(crate) fn svg_içeriği(&self, kimlik_öneki: &str) -> String {
        #[cfg(test)]
        SVG_SERİLEŞTİRME_ÇAĞRILARI.with(|sayı| sayı.set(sayı.get() + 1));

        let mut çıktı = String::new();
        for (komut_indeksi, komut) in self.komutlar.iter().enumerate() {
            match komut {
                Komut::ArkaPlan { renk } => {
                    let _ = writeln!(
                        çıktı,
                        "  <rect width=\"{}\" height=\"{}\" fill=\"{}\"/>",
                        self.genişlik,
                        self.yükseklik,
                        kaçış(renk)
                    );
                }
                Komut::Çizgi {
                    başlangıç,
                    bitiş,
                    renk,
                    kalınlık,
                } => {
                    let _ = writeln!(
                        çıktı,
                        "  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"{}\"/>",
                        sayı(başlangıç.x),
                        sayı(başlangıç.y),
                        sayı(bitiş.x),
                        sayı(bitiş.y),
                        kaçış(renk),
                        sayı(*kalınlık)
                    );
                }
                Komut::KesikliÇizgi {
                    başlangıç,
                    bitiş,
                    renk,
                    kalınlık,
                    kesik,
                } => {
                    let _ = writeln!(
                        çıktı,
                        "  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"{}\" stroke-dasharray=\"{} {}\"/>",
                        sayı(başlangıç.x),
                        sayı(başlangıç.y),
                        sayı(bitiş.x),
                        sayı(bitiş.y),
                        kaçış(renk),
                        sayı(*kalınlık),
                        sayı(*kesik),
                        sayı(*kesik)
                    );
                }
                Komut::Yol {
                    parçalar,
                    renk,
                    kalınlık,
                } => {
                    let mut d = String::new();
                    for parça in parçalar {
                        for (indeks, nokta) in parça.iter().enumerate() {
                            let işlem = if indeks == 0 { 'M' } else { 'L' };
                            let _ = write!(d, "{işlem}{} {} ", sayı(nokta.x), sayı(nokta.y));
                        }
                    }
                    let _ = writeln!(
                        çıktı,
                        "  <path d=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"{}\" stroke-linejoin=\"round\"/>",
                        d.trim_end(),
                        kaçış(renk),
                        sayı(*kalınlık)
                    );
                }
                Komut::GradyanYol {
                    parçalar,
                    gradyan,
                    kalınlık,
                } => {
                    let kimlik = format!("{kimlik_öneki}uplot-gradyan-{komut_indeksi}");
                    gradyan_tanımını_yaz(&mut çıktı, &kimlik, gradyan);
                    let d = yol_verisi(parçalar, false);
                    let _ = writeln!(
                        çıktı,
                        "  <path d=\"{}\" fill=\"none\" stroke=\"url(#{})\" stroke-width=\"{}\" stroke-linejoin=\"round\"/>",
                        d,
                        kimlik,
                        sayı(*kalınlık)
                    );
                }
                Komut::KesikliYol {
                    parçalar,
                    renk,
                    kalınlık,
                    çizgi,
                    boşluk,
                } => {
                    let mut d = String::new();
                    for parça in parçalar {
                        for (indeks, nokta) in parça.iter().enumerate() {
                            let işlem = if indeks == 0 { 'M' } else { 'L' };
                            let _ = write!(d, "{işlem}{} {} ", sayı(nokta.x), sayı(nokta.y));
                        }
                    }
                    let _ = writeln!(
                        çıktı,
                        "  <path d=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"{}\" stroke-dasharray=\"{} {}\" stroke-linejoin=\"round\"/>",
                        d.trim_end(),
                        kaçış(renk),
                        sayı(*kalınlık),
                        sayı(*çizgi),
                        sayı(*boşluk)
                    );
                }
                Komut::Alan { çokgenler, dolgu } => {
                    let mut d = String::new();
                    for çokgen in çokgenler {
                        for (indeks, nokta) in çokgen.iter().enumerate() {
                            let işlem = if indeks == 0 { 'M' } else { 'L' };
                            let _ = write!(d, "{işlem}{} {} ", sayı(nokta.x), sayı(nokta.y));
                        }
                        if çokgen.len() >= 3 {
                            d.push_str("Z ");
                        }
                    }
                    let _ = writeln!(
                        çıktı,
                        "  <path d=\"{}\" fill=\"{}\" stroke=\"none\"/>",
                        d.trim_end(),
                        kaçış(dolgu)
                    );
                }
                Komut::GradyanAlan {
                    çokgenler, gradyan
                } => {
                    let kimlik = format!("{kimlik_öneki}uplot-gradyan-{komut_indeksi}");
                    gradyan_tanımını_yaz(&mut çıktı, &kimlik, gradyan);
                    let d = yol_verisi(çokgenler, true);
                    let _ = writeln!(
                        çıktı,
                        "  <path d=\"{}\" fill=\"url(#{})\" stroke=\"none\"/>",
                        d, kimlik
                    );
                }
                Komut::Daire {
                    merkez,
                    yarıçap,
                    dolgu,
                    çizgi,
                    kalınlık,
                } => {
                    let _ = writeln!(
                        çıktı,
                        "  <circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"{}\"/>",
                        sayı(merkez.x),
                        sayı(merkez.y),
                        sayı(*yarıçap),
                        kaçış(dolgu),
                        kaçış(çizgi),
                        sayı(*kalınlık)
                    );
                }
                Komut::Daireler {
                    merkezler,
                    yarıçap,
                    dolgu,
                    çizgi,
                    kalınlık,
                    kesme_sınırları,
                } => {
                    let kırpma_kimliği = kesme_sınırları.map(|(başlangıç, bitiş)| {
                        let kimlik =
                            format!("{kimlik_öneki}uplot-daire-kirpma-{komut_indeksi}");
                        let _ = writeln!(
                            çıktı,
                            "  <defs><clipPath id=\"{}\"><rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\"/></clipPath></defs>",
                            kimlik,
                            sayı(başlangıç.x),
                            sayı(başlangıç.y),
                            sayı((bitiş.x - başlangıç.x).max(0.0)),
                            sayı((bitiş.y - başlangıç.y).max(0.0)),
                        );
                        kimlik
                    });
                    let mut d = String::new();
                    let r = sayı(*yarıçap);
                    let çap = sayı(*yarıçap * 2.0);
                    for merkez in merkezler {
                        let _ = write!(
                            d,
                            "M{} {}a{} {} 0 1 0 {} 0a{} {} 0 1 0 -{} 0 ",
                            sayı(merkez.x - *yarıçap),
                            sayı(merkez.y),
                            r,
                            r,
                            çap,
                            r,
                            r,
                            çap,
                        );
                    }
                    let _ = writeln!(
                        çıktı,
                        "  <path d=\"{}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"{}\"{} />",
                        d.trim_end(),
                        kaçış(dolgu),
                        kaçış(çizgi),
                        sayı(*kalınlık),
                        kırpma_kimliği.map_or_else(String::new, |kimlik| {
                            format!(" clip-path=\"url(#{})\"", kaçış(&kimlik))
                        })
                    );
                }
                Komut::DeğişkenDaireler {
                    daireler,
                    dolgu,
                    çizgi,
                    kalınlık,
                    kesme_sınırları,
                } => {
                    let kırpma_kimliği = kesme_sınırları.map(|(başlangıç, bitiş)| {
                        let kimlik = format!(
                            "{kimlik_öneki}uplot-degisken-daire-kirpma-{komut_indeksi}"
                        );
                        let _ = writeln!(
                            çıktı,
                            "  <defs><clipPath id=\"{}\"><rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\"/></clipPath></defs>",
                            kimlik,
                            sayı(başlangıç.x),
                            sayı(başlangıç.y),
                            sayı((bitiş.x - başlangıç.x).max(0.0)),
                            sayı((bitiş.y - başlangıç.y).max(0.0)),
                        );
                        kimlik
                    });
                    let mut d = String::new();
                    for (merkez, yarıçap) in daireler {
                        let r = sayı(*yarıçap);
                        let çap = sayı(*yarıçap * 2.0);
                        let _ = write!(
                            d,
                            "M{} {}a{} {} 0 1 0 {} 0a{} {} 0 1 0 -{} 0 ",
                            sayı(merkez.x - *yarıçap),
                            sayı(merkez.y),
                            r,
                            r,
                            çap,
                            r,
                            r,
                            çap,
                        );
                    }
                    let _ = writeln!(
                        çıktı,
                        "  <path d=\"{}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"{}\"{} />",
                        d.trim_end(),
                        kaçış(dolgu),
                        kaçış(çizgi),
                        sayı(*kalınlık),
                        kırpma_kimliği.map_or_else(String::new, |kimlik| {
                            format!(" clip-path=\"url(#{})\"", kaçış(&kimlik))
                        })
                    );
                }
                Komut::Dikdörtgen {
                    konum,
                    genişlik,
                    yükseklik,
                    dolgu,
                    çizgi,
                    kalınlık,
                } => {
                    let _ = writeln!(
                        çıktı,
                        "  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"{}\"/>",
                        sayı(konum.x),
                        sayı(konum.y),
                        sayı(*genişlik),
                        sayı(*yükseklik),
                        kaçış(dolgu),
                        kaçış(çizgi),
                        sayı(*kalınlık)
                    );
                }
                Komut::YuvarlatılmışDikdörtgen {
                    konum,
                    genişlik,
                    yükseklik,
                    yarıçaplar,
                    dolgu,
                    çizgi,
                    kalınlık,
                } => {
                    let d = yuvarlatılmış_dikdörtgen_yolu(
                        *konum,
                        *genişlik,
                        *yükseklik,
                        *yarıçaplar,
                    );
                    let _ = writeln!(
                        çıktı,
                        "  <path d=\"{}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"{}\"/>",
                        d,
                        kaçış(dolgu),
                        kaçış(çizgi),
                        sayı(*kalınlık)
                    );
                }
                Komut::Metin {
                    konum,
                    içerik,
                    renk,
                    boyut,
                    hiza,
                } => {
                    svg_metnini_yaz(&mut çıktı, *konum, içerik, renk, *boyut, *hiza, None);
                }
                Komut::DöndürülmüşMetin {
                    konum,
                    içerik,
                    renk,
                    boyut,
                    hiza,
                    açı,
                } => {
                    svg_metnini_yaz(&mut çıktı, *konum, içerik, renk, *boyut, *hiza, Some(*açı));
                }
            }
        }
        çıktı
    }

    /// Golden testlerde kullanılacak okunabilir komut dökümü.
    pub fn döküm(&self) -> String {
        self.komutlar
            .iter()
            .map(|komut| format!("{komut:?}"))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

/// GPUI path önbelleğinin geometri değişimini sabit zamanda sınamasını sağlar.
///
/// Renk ve dolgu gibi tessellation'ı değiştirmeyen sunum alanları bilerek
/// kimliğe katılmaz. Kimlik komut oluşturulurken bir kez hesaplanır; sonraki
/// sahne geçişinde büyük nokta dizileri yeniden karşılaştırılmaz.
fn komut_geometri_kimliği(komut: &Komut) -> u64 {
    let mut özet = GeometriÖzeti::yeni();
    match komut {
        Komut::ArkaPlan { .. } => özet.tür(0),
        Komut::Çizgi {
            başlangıç,
            bitiş,
            kalınlık,
            ..
        } => {
            özet.tür(1);
            özet.nokta(*başlangıç);
            özet.nokta(*bitiş);
            özet.sayı(*kalınlık);
        }
        Komut::KesikliÇizgi {
            başlangıç,
            bitiş,
            kalınlık,
            kesik,
            ..
        } => {
            özet.tür(2);
            özet.nokta(*başlangıç);
            özet.nokta(*bitiş);
            özet.sayı(*kalınlık);
            özet.sayı(*kesik);
        }
        Komut::Yol {
            parçalar, kalınlık,
        ..
        }
        | Komut::GradyanYol {
            parçalar, kalınlık,
        ..
        } => {
            özet.tür(3);
            özet.parçalar(parçalar);
            özet.sayı(*kalınlık);
        }
        Komut::KesikliYol {
            parçalar,
            kalınlık,
            çizgi,
            boşluk,
            ..
        } => {
            özet.tür(4);
            özet.parçalar(parçalar);
            özet.sayı(*kalınlık);
            özet.sayı(*çizgi);
            özet.sayı(*boşluk);
        }
        Komut::Alan { çokgenler, .. } | Komut::GradyanAlan { çokgenler, .. } => {
            özet.tür(5);
            özet.parçalar(çokgenler);
        }
        Komut::Daire {
            merkez,
            yarıçap,
            kalınlık,
            ..
        } => {
            özet.tür(6);
            özet.nokta(*merkez);
            özet.sayı(*yarıçap);
            özet.sayı(*kalınlık);
        }
        Komut::Daireler {
            merkezler,
            yarıçap,
            kalınlık,
            kesme_sınırları,
            ..
        } => {
            özet.tür(7);
            özet.noktalar(merkezler);
            özet.sayı(*yarıçap);
            özet.sayı(*kalınlık);
            özet.kesme(*kesme_sınırları);
        }
        Komut::DeğişkenDaireler {
            daireler,
            kalınlık,
            kesme_sınırları,
            ..
        } => {
            özet.tür(8);
            özet.uzunluk(daireler.len());
            for (merkez, yarıçap) in daireler {
                özet.nokta(*merkez);
                özet.sayı(*yarıçap);
            }
            özet.sayı(*kalınlık);
            özet.kesme(*kesme_sınırları);
        }
        Komut::Dikdörtgen {
            konum,
            genişlik,
            yükseklik,
            kalınlık,
            ..
        } => {
            özet.tür(9);
            özet.nokta(*konum);
            özet.sayı(*genişlik);
            özet.sayı(*yükseklik);
            özet.sayı(*kalınlık);
        }
        Komut::YuvarlatılmışDikdörtgen {
            konum,
            genişlik,
            yükseklik,
            yarıçaplar,
            kalınlık,
            ..
        } => {
            özet.tür(10);
            özet.nokta(*konum);
            özet.sayı(*genişlik);
            özet.sayı(*yükseklik);
            özet.sayı(yarıçaplar.üst_sol);
            özet.sayı(yarıçaplar.üst_sağ);
            özet.sayı(yarıçaplar.alt_sağ);
            özet.sayı(yarıçaplar.alt_sol);
            özet.sayı(*kalınlık);
        }
        Komut::Metin {
            konum,
            içerik,
            boyut,
            hiza,
            ..
        } => {
            özet.tür(11);
            özet.nokta(*konum);
            özet.metin(içerik);
            özet.sayı(*boyut);
            özet.tür(*hiza as u8);
        }
        Komut::DöndürülmüşMetin {
            konum,
            içerik,
            boyut,
            hiza,
            açı,
            ..
        } => {
            özet.tür(12);
            özet.nokta(*konum);
            özet.metin(içerik);
            özet.sayı(*boyut);
            özet.tür(*hiza as u8);
            özet.sayı(*açı);
        }
    }
    özet.bitir()
}

struct GeometriÖzeti(u64);

impl GeometriÖzeti {
    const FNV_BAŞLANGIÇ: u64 = 0xcbf2_9ce4_8422_2325;
    const FNV_ASAL: u64 = 0x0000_0100_0000_01b3;

    fn yeni() -> Self {
        Self(Self::FNV_BAŞLANGIÇ)
    }

    fn byte(&mut self, değer: u8) {
        self.0 ^= u64::from(değer);
        self.0 = self.0.wrapping_mul(Self::FNV_ASAL);
    }

    fn tür(&mut self, değer: u8) {
        self.byte(değer);
    }

    fn uzunluk(&mut self, değer: usize) {
        for byte in (değer as u64).to_le_bytes() {
            self.byte(byte);
        }
    }

    fn sayı(&mut self, değer: f32) {
        for byte in değer.to_bits().to_le_bytes() {
            self.byte(byte);
        }
    }

    fn nokta(&mut self, nokta: Nokta) {
        self.sayı(nokta.x);
        self.sayı(nokta.y);
    }

    fn noktalar(&mut self, noktalar: &[Nokta]) {
        self.uzunluk(noktalar.len());
        for nokta in noktalar {
            self.nokta(*nokta);
        }
    }

    fn parçalar(&mut self, parçalar: &[Vec<Nokta>]) {
        self.uzunluk(parçalar.len());
        for parça in parçalar {
            self.noktalar(parça);
        }
    }

    fn kesme(&mut self, kesme: Option<(Nokta, Nokta)>) {
        self.tür(u8::from(kesme.is_some()));
        if let Some((başlangıç, bitiş)) = kesme {
            self.nokta(başlangıç);
            self.nokta(bitiş);
        }
    }

    fn metin(&mut self, metin: &str) {
        self.uzunluk(metin.len());
        for byte in metin.bytes() {
            self.byte(byte);
        }
    }

    fn bitir(self) -> u64 {
        self.0
    }
}

#[cfg(any(feature = "gpui-svg", test))]
fn yuvarlatılmış_dikdörtgen_yolu(
    konum: Nokta,
    genişlik: f32,
    yükseklik: f32,
    yarıçaplar: KöşeYarıçapları,
) -> String {
    let azami = (genişlik / 2.0).min(yükseklik / 2.0).max(0.0);
    let üst_sol = yarıçaplar.üst_sol.clamp(0.0, azami);
    let üst_sağ = yarıçaplar.üst_sağ.clamp(0.0, azami);
    let alt_sağ = yarıçaplar.alt_sağ.clamp(0.0, azami);
    let alt_sol = yarıçaplar.alt_sol.clamp(0.0, azami);
    let sol = konum.x;
    let sağ = konum.x + genişlik;
    let üst = konum.y;
    let alt = konum.y + yükseklik;
    format!(
        "M{} {} H{} Q{} {} {} {} V{} Q{} {} {} {} H{} Q{} {} {} {} V{} Q{} {} {} {} Z",
        sayı(sol + üst_sol),
        sayı(üst),
        sayı(sağ - üst_sağ),
        sayı(sağ),
        sayı(üst),
        sayı(sağ),
        sayı(üst + üst_sağ),
        sayı(alt - alt_sağ),
        sayı(sağ),
        sayı(alt),
        sayı(sağ - alt_sağ),
        sayı(alt),
        sayı(sol + alt_sol),
        sayı(sol),
        sayı(alt),
        sayı(sol),
        sayı(alt - alt_sol),
        sayı(üst + üst_sol),
        sayı(sol),
        sayı(üst),
        sayı(sol + üst_sol),
        sayı(üst),
    )
}

#[cfg(any(feature = "gpui-svg", test))]
fn yol_verisi(parçalar: &[Vec<Nokta>], kapat: bool) -> String {
    let mut d = String::new();
    for parça in parçalar {
        for (indeks, nokta) in parça.iter().enumerate() {
            let işlem = if indeks == 0 { 'M' } else { 'L' };
            let _ = write!(d, "{işlem}{} {} ", sayı(nokta.x), sayı(nokta.y));
        }
        if kapat && parça.len() >= 3 {
            d.push_str("Z ");
        }
    }
    d.trim_end().to_string()
}

#[cfg(any(feature = "gpui-svg", test))]
fn gradyan_tanımını_yaz(çıktı: &mut String, kimlik: &str, gradyan: &DoğrusalGradyan) {
    let _ = writeln!(
        çıktı,
        "  <defs><linearGradient id=\"{}\" gradientUnits=\"userSpaceOnUse\" x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\">",
        kaçış(kimlik),
        sayı(gradyan.başlangıç.x),
        sayı(gradyan.başlangıç.y),
        sayı(gradyan.bitiş.x),
        sayı(gradyan.bitiş.y)
    );
    for durak in &gradyan.duraklar {
        let _ = writeln!(
            çıktı,
            "    <stop offset=\"{}\" stop-color=\"{}\"/>",
            sayı(durak.oran.clamp(0.0, 1.0)),
            kaçış(&durak.renk)
        );
    }
    çıktı.push_str("  </linearGradient></defs>\n");
}

#[cfg(any(feature = "gpui-svg", test))]
fn svg_metnini_yaz(
    çıktı: &mut String,
    konum: Nokta,
    içerik: &str,
    renk: &str,
    boyut: f32,
    hiza: MetinHizası,
    açı: Option<f32>,
) {
    let çapa = match hiza {
        MetinHizası::Başlangıç => "start",
        MetinHizası::Orta => "middle",
        MetinHizası::Bitiş => "end",
    };
    let x = sayı(konum.x);
    let y = sayı(konum.y);
    let dönüşüm = açı.map_or_else(String::new, |açı| {
        format!(" transform=\"rotate({} {} {})\"", sayı(açı), x, y)
    });
    let normalleştirilmiş = içerik.replace("\r\n", "\n").replace('\r', "\n");
    let satırlar = normalleştirilmiş.split('\n').collect::<Vec<_>>();

    let _ = write!(
        çıktı,
        "  <text x=\"{}\" y=\"{}\" fill=\"{}\" font-family=\"Arial, sans-serif\" font-size=\"{}\" text-anchor=\"{}\"{}",
        x,
        y,
        kaçış(renk),
        sayı(boyut),
        çapa,
        dönüşüm,
    );
    if satırlar.len() == 1 {
        let _ = writeln!(
            çıktı,
            ">{}</text>",
            kaçış(satırlar.first().copied().unwrap_or_default())
        );
        return;
    }

    çıktı.push_str(" xml:space=\"preserve\">\n");
    for (indeks, satır) in satırlar.iter().enumerate() {
        let dikey_adım = if indeks == 0 { "0" } else { "1.2em" };
        let _ = writeln!(
            çıktı,
            "    <tspan x=\"{}\" dy=\"{}\">{}</tspan>",
            x,
            dikey_adım,
            kaçış(satır)
        );
    }
    çıktı.push_str("  </text>\n");
}

#[cfg(any(feature = "gpui-svg", test))]
fn sayı(değer: f32) -> String {
    let güvenli = if değer.is_finite() {
        f64::from(değer)
    } else {
        0.0
    };
    let yuvarlanmış = (güvenli * 100.0).round() / 100.0;
    let yuvarlanmış = if yuvarlanmış == 0.0 {
        0.0
    } else {
        yuvarlanmış
    };
    format!("{yuvarlanmış:.2}")
}

#[cfg(any(feature = "gpui-svg", test))]
fn kaçış(metin: &str) -> String {
    metin
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(test)]
mod svg_testleri {
    use super::*;

    #[test]
    fn çok_satırlı_ve_döndürülmüş_metin_tspan_ile_korunur() {
        let mut sahne = Sahne::yeni(320, 180);
        sahne.ekle(Komut::Metin {
            konum: Nokta::yeni(20.0, 30.0),
            içerik: "ilk & satır\r\nikinci <satır>\n".to_string(),
            renk: "#123456".to_string(),
            boyut: 12.0,
            hiza: MetinHizası::Başlangıç,
        });
        sahne.ekle(Komut::DöndürülmüşMetin {
            konum: Nokta::yeni(100.0, 120.0),
            içerik: "sol\nsağ".to_string(),
            renk: "#654321".to_string(),
            boyut: 14.0,
            hiza: MetinHizası::Orta,
            açı: -90.0,
        });

        let svg = sahne.test_svg();
        assert!(svg.contains("xml:space=\"preserve\""));
        assert!(svg.contains("<tspan x=\"20.00\" dy=\"0\">ilk &amp; satır</tspan>"));
        assert!(svg.contains("<tspan x=\"20.00\" dy=\"1.2em\">ikinci &lt;satır&gt;</tspan>"));
        assert!(svg.contains("<tspan x=\"20.00\" dy=\"1.2em\"></tspan>"));
        assert!(svg.contains("transform=\"rotate(-90.00 100.00 120.00)\""));
        assert!(svg.contains("<tspan x=\"100.00\" dy=\"0\">sol</tspan>"));
        assert!(svg.contains("<tspan x=\"100.00\" dy=\"1.2em\">sağ</tspan>"));
    }

    #[test]
    fn sonlu_olmayan_sayılar_svg_sözdizimine_sızmaz() {
        let mut sahne = Sahne::yeni(320, 180);
        sahne.ekle(Komut::Çizgi {
            başlangıç: Nokta::yeni(f32::NAN, f32::INFINITY),
            bitiş: Nokta::yeni(f32::NEG_INFINITY, f32::MAX),
            renk: "#123456".to_string(),
            kalınlık: f32::NAN,
        });
        sahne.ekle(Komut::DöndürülmüşMetin {
            konum: Nokta::yeni(f32::INFINITY, f32::NEG_INFINITY),
            içerik: "güvenli".to_string(),
            renk: "#654321".to_string(),
            boyut: f32::INFINITY,
            hiza: MetinHizası::Orta,
            açı: f32::NAN,
        });

        let svg = sahne.test_svg();
        assert!(!svg.contains("NaN"));
        assert!(!svg.contains("inf"));
        assert!(!svg.contains("INF"));
        assert!(svg.contains("x1=\"0.00\" y1=\"0.00\""));
        assert!(svg.contains("rotate(0.00 0.00 0.00)"));
    }
}
