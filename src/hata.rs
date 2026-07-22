use std::fmt::{Display, Formatter};

/// Kütüphanenin doğrulama ve çizim sırasında döndürdüğü tipli hatalar.
#[derive(Debug, Clone, PartialEq)]
pub enum UplotHatası {
    GeçersizBoyut {
        genişlik: u32,
        yükseklik: u32,
    },
    YetersizVeri {
        uzunluk: usize,
    },
    SeriUzunluğu {
        seri: usize,
        beklenen: usize,
        bulunan: usize,
    },
    SırasızX {
        indeks: usize,
    },
    SonluOlmayanX {
        indeks: usize,
    },
    SonluOlmayanY {
        seri: usize,
        indeks: usize,
    },
    GeçersizAralık {
        en_az: f64,
        en_çok: f64,
    },
    SeriSeçeneğiEksik {
        beklenen: usize,
        bulunan: usize,
    },
}

impl Display for UplotHatası {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GeçersizBoyut {
                genişlik,
                yükseklik,
            } => {
                write!(f, "geçersiz çizelge boyutu: {genişlik}x{yükseklik}")
            }
            Self::YetersizVeri { uzunluk } => {
                write!(
                    f,
                    "uPlot hizalı verisi en az 2 nokta ister; bulunan: {uzunluk}"
                )
            }
            Self::SeriUzunluğu {
                seri,
                beklenen,
                bulunan,
            } => write!(f, "{seri}. seri uzunluğu {bulunan}; x uzunluğu {beklenen}"),
            Self::SırasızX { indeks } => {
                write!(
                    f,
                    "x değerleri benzersiz ve artan olmalı; hata indeksi: {indeks}"
                )
            }
            Self::SonluOlmayanX { indeks } => {
                write!(f, "x değeri sonlu değil; indeks: {indeks}")
            }
            Self::SonluOlmayanY { seri, indeks } => {
                write!(f, "{seri}. serinin {indeks}. değeri sonlu değil")
            }
            Self::GeçersizAralık { en_az, en_çok } => {
                write!(f, "geçersiz ölçek aralığı: [{en_az}, {en_çok}]")
            }
            Self::SeriSeçeneğiEksik { beklenen, bulunan } => write!(
                f,
                "veri için {beklenen} seri seçeneği gerekir; bulunan: {bulunan}"
            ),
        }
    }
}

impl std::error::Error for UplotHatası {}
