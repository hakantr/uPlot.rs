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
    GeçersizSeriİndeksi {
        indeks: usize,
        seri_sayısı: usize,
        ekleme: bool,
    },
    BilinmeyenKart {
        kimlik: String,
    },
    GeçersizTarih {
        yıl: i64,
        ay: u32,
        gün: u32,
    },
    BilinmeyenÖlçek {
        seri: usize,
        anahtar: String,
    },
    GeçersizVarlıkSatırı {
        varlık: &'static str,
        satır: usize,
    },
    GeçersizÇarpan {
        değer: f64,
    },
    GeçersizKaynakVeri {
        varlık: &'static str,
        açıklama: String,
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
                    "uPlot hizalı verisi en az 1 nokta ister; bulunan: {uzunluk}"
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
            Self::GeçersizSeriİndeksi {
                indeks,
                seri_sayısı,
                ekleme,
            } => write!(
                f,
                "{} için geçersiz Y-serisi indeksi: {indeks}; seri sayısı: {seri_sayısı}",
                if *ekleme { "seri ekleme" } else { "seri silme" }
            ),
            Self::BilinmeyenKart { kimlik } => write!(f, "bilinmeyen kart kimliği: {kimlik}"),
            Self::GeçersizTarih { yıl, ay, gün } => {
                write!(f, "geçersiz UTC tarihi: {yıl:04}-{ay:02}-{gün:02}")
            }
            Self::BilinmeyenÖlçek { seri, anahtar } => {
                write!(
                    f,
                    "{seri}. seri bilinmeyen Y ölçeğini kullanıyor: {anahtar}"
                )
            }
            Self::GeçersizVarlıkSatırı { varlık, satır } => {
                write!(f, "{varlık} varlığının {satır}. satırı geçersiz")
            }
            Self::GeçersizÇarpan { değer } => {
                write!(f, "çarpan sonlu ve pozitif olmalı; bulunan: {değer}")
            }
            Self::GeçersizKaynakVeri {
                varlık, açıklama
            } => {
                write!(f, "{varlık} kaynak verisi çözümlenemedi: {açıklama}")
            }
        }
    }
}

impl std::error::Error for UplotHatası {}
