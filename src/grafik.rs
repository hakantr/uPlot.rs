mod bant;
mod isi_haritasi;
mod seri_geometrisi;
mod timeline;

use std::{
    cell::RefCell,
    collections::BTreeMap,
    sync::atomic::{AtomicU64, Ordering},
};
use web_time::Instant;

use seri_geometrisi::seri_yol_noktaları;

use crate::cizim::kirpma::{
    nokta_dikdörtgende, sahipli_yolu_dikdörtgene_kırp, yolu_dikdörtgene_kırp,
    çokgeni_dikdörtgene_kırp,
};
use crate::cizim::{
    DoğrusalGradyan, GradyanRenkDurağı, Komut, KöşeYarıçapları, MetinHizası, Nokta, Sahne,
    SahneKatmanı,
};
use crate::etkilesim::{
    EtkileşimDenetleyicisi, x_aralığını_dönüştür, x_aralığını_geri_dönüştür, y_aralığını_dönüştür,
    y_aralığını_geri_dönüştür, y_değerini_geri_dönüştür,
};
use crate::{
    Aralık, GradyanEkseni, GradyanKonumu, GrafikSeçenekleri, HizalıVeri, NullİmleçDüzeni,
    SeriBandı, TekerlekEkseni, UplotHatası, XÖlçekDağılımı, YÖlçekDağılımı, YÖlçekEtiketBiçimi,
    ÖlçekGradyanı,
};

/// Bir işaretçi seçiminin çekirdekte çözümlenen sonucu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeçimEylemi {
    /// Kart ayarları seçimi devre dışı bıraktı veya görünüm değişmedi.
    Değişmedi,
    /// Seçilen X aralığı görünür aralık olarak uygulandı.
    Yakınlaştırıldı,
    /// `cursor-bind` bağı yakınlaştırmayı durdurup açıklama UI'si istedi.
    Açıklamaİstendi,
}

/// uPlot `addSeries` / `delSeries` kancalarının kaynak sırasını, X serisini
/// de sayan resmî `seriesIdx` değeriyle taşır.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeriYaşamDöngüsüOlayı {
    Eklendi {
        seri_indeksi: usize,
        başlangıç: bool,
    },
    Silindi {
        seri_indeksi: usize,
    },
    VeriAyarlandı {
        seri_sayısı: usize,
    },
}

static SON_GRAFİK_KİMLİĞİ: AtomicU64 = AtomicU64::new(1);

/// Null bir hizalı örneğin çevresinde imleç indeksinin hangi yönde aranacağını belirler.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NullAtlamaYönü {
    /// X ölçeği uzaklığı en küçük olan dolu örnek; eşitlikte soldaki.
    EnYakın,
    /// İmlecin solundaki veya üzerindeki son dolu örnek.
    Önceki,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct İmleçSeriÖrneği {
    pub indeks: usize,
    pub x: f64,
    pub değer: f64,
    pub hizalama_eksiğinden_atlandı: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct İmleçÇözümü {
    /// Cursor çizgisinin veri X'i. `cursor.move` kipinde örneğe yapışabilir.
    pub imleç_x: f64,
    /// Fareye en yakın ortak hizalı X.
    pub ortak_x: f64,
    /// Her görünür seri için bağımsız hover örneği.
    pub seriler: Vec<Option<İmleçSeriÖrneği>>,
}

/// GPUI retained veri yüzeyinden görünür pencereye yapılan fiziksel kırpma.
///
/// Değerler tam çizim alanının normalize koordinatlarındadır. Yakınlaştırma
/// yalnız bu pencereyi GPUI tarafında büyütür; veri noktaları yeniden
/// örneklenmez veya yeniden konumlandırılmaz.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OransalGörünüm {
    pub sol: f32,
    pub sağ: f32,
    pub üst: f32,
    pub alt: f32,
}

impl Default for OransalGörünüm {
    fn default() -> Self {
        Self {
            sol: 0.0,
            sağ: 1.0,
            üst: 0.0,
            alt: 1.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZoomRangerSürüklemeEkseni {
    Yok,
    X,
    Y,
    XY,
}

/// Overview grafiğindeki taşınabilir ve uçlardan boyutlandırılabilir X/Y seçimi.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ZoomRangerDurumu {
    tam_x: Aralık,
    seçim_x: Aralık,
    tam_y: Aralık,
    seçim_y: Aralık,
    ayarlar: crate::ZoomRangerSeçenekleri,
}

impl ZoomRangerDurumu {
    pub fn yeni(tam: Aralık, seçim: Aralık) -> Result<Self, UplotHatası> {
        Self::xy(
            tam,
            seçim,
            Aralık::yeni(0.0, 1.0)?,
            Aralık::yeni(0.0, 1.0)?,
            crate::ZoomRangerSeçenekleri::default().etkin(true),
        )
    }

    pub fn xy(
        tam_x: Aralık,
        seçim_x: Aralık,
        tam_y: Aralık,
        seçim_y: Aralık,
        ayarlar: crate::ZoomRangerSeçenekleri,
    ) -> Result<Self, UplotHatası> {
        let mut durum = Self {
            tam_x,
            seçim_x: tam_x,
            tam_y,
            seçim_y: tam_y,
            ayarlar,
        };
        durum.ana_görünümle_xy_senkronla(seçim_x, seçim_y)?;
        Ok(durum)
    }

    pub fn seçim_aralığı(self) -> Aralık {
        self.seçim_x
    }

    pub fn tam_aralık(self) -> Aralık {
        self.tam_x
    }

    pub fn y_seçim_aralığı(self) -> Aralık {
        self.seçim_y
    }

    pub fn y_tam_aralık(self) -> Aralık {
        self.tam_y
    }

    pub fn sürükleme_ayarlarını_ayarla(&mut self, ayarlar: crate::ZoomRangerSeçenekleri) {
        self.ayarlar = ayarlar;
    }

    pub fn seçim_oranları(self) -> (f64, f64) {
        let genişlik = self.tam_x.en_çok - self.tam_x.en_az;
        (
            (self.seçim_x.en_az - self.tam_x.en_az) / genişlik,
            (self.seçim_x.en_çok - self.tam_x.en_az) / genişlik,
        )
    }

    pub fn y_seçim_oranları(self) -> (f64, f64) {
        let yükseklik = self.tam_y.en_çok - self.tam_y.en_az;
        (
            (self.seçim_y.en_az - self.tam_y.en_az) / yükseklik,
            (self.seçim_y.en_çok - self.tam_y.en_az) / yükseklik,
        )
    }

    pub fn pencereyi_taşı(&mut self, fark: f64) -> bool {
        if !fark.is_finite() {
            return false;
        }
        let genişlik = self.seçim_x.en_çok - self.seçim_x.en_az;
        let en_az =
            (self.seçim_x.en_az + fark).clamp(self.tam_x.en_az, self.tam_x.en_çok - genişlik);
        self.x_değiştir(Aralık {
            en_az,
            en_çok: en_az + genişlik,
        })
    }

    pub fn y_pencereyi_taşı(&mut self, fark: f64) -> bool {
        if !fark.is_finite() {
            return false;
        }
        let yükseklik = self.seçim_y.en_çok - self.seçim_y.en_az;
        let en_az =
            (self.seçim_y.en_az + fark).clamp(self.tam_y.en_az, self.tam_y.en_çok - yükseklik);
        self.y_değiştir(Aralık {
            en_az,
            en_çok: en_az + yükseklik,
        })
    }

    pub fn sol_tutamağı_ayarla(&mut self, değer: f64) -> bool {
        değer.is_finite()
            && self.x_değiştir(Aralık {
                en_az: değer.clamp(self.tam_x.en_az, self.seçim_x.en_çok),
                en_çok: self.seçim_x.en_çok,
            })
    }

    pub fn sağ_tutamağı_ayarla(&mut self, değer: f64) -> bool {
        değer.is_finite()
            && self.x_değiştir(Aralık {
                en_az: self.seçim_x.en_az,
                en_çok: değer.clamp(self.seçim_x.en_az, self.tam_x.en_çok),
            })
    }

    pub fn alt_tutamağı_ayarla(&mut self, değer: f64) -> bool {
        değer.is_finite()
            && self.y_değiştir(Aralık {
                en_az: değer.clamp(self.tam_y.en_az, self.seçim_y.en_çok),
                en_çok: self.seçim_y.en_çok,
            })
    }

    pub fn üst_tutamağı_ayarla(&mut self, değer: f64) -> bool {
        değer.is_finite()
            && self.y_değiştir(Aralık {
                en_az: self.seçim_y.en_az,
                en_çok: değer.clamp(self.seçim_y.en_az, self.tam_y.en_çok),
            })
    }

    pub fn ana_görünümle_senkronla(&mut self, aralık: Aralık) -> Result<bool, UplotHatası> {
        let en_az = aralık.en_az.clamp(self.tam_x.en_az, self.tam_x.en_çok);
        let en_çok = aralık.en_çok.clamp(self.tam_x.en_az, self.tam_x.en_çok);
        Ok(self.x_değiştir(Aralık::yeni(en_az.min(en_çok), en_az.max(en_çok))?))
    }

    pub fn ana_görünümle_xy_senkronla(
        &mut self,
        x: Aralık,
        y: Aralık,
    ) -> Result<bool, UplotHatası> {
        let x_değişti = self.ana_görünümle_senkronla(x)?;
        let en_az = y.en_az.clamp(self.tam_y.en_az, self.tam_y.en_çok);
        let en_çok = y.en_çok.clamp(self.tam_y.en_az, self.tam_y.en_çok);
        let y_değişti = self.y_değiştir(Aralık::yeni(en_az.min(en_çok), en_az.max(en_çok))?);
        Ok(x_değişti || y_değişti)
    }

    pub fn uyarlanabilir_sürükleme_ekseni(
        self,
        yatay_fark_px: f64,
        dikey_fark_px: f64,
    ) -> ZoomRangerSürüklemeEkseni {
        if !self.ayarlar.etkin || !yatay_fark_px.is_finite() || !dikey_fark_px.is_finite() {
            return ZoomRangerSürüklemeEkseni::Yok;
        }
        let x = yatay_fark_px.abs();
        let y = dikey_fark_px.abs();
        if x.max(y) <= f64::EPSILON || x.hypot(y) < self.ayarlar.en_az_sürükleme_px {
            return ZoomRangerSürüklemeEkseni::Yok;
        }
        if self.ayarlar.x && self.ayarlar.y {
            let uni = self.ayarlar.tek_eksen_eşiği_px;
            if uni == 0.0 || (uni.is_finite() && (x - y).abs() < uni) {
                return ZoomRangerSürüklemeEkseni::XY;
            }
        }
        if self.ayarlar.x && (!self.ayarlar.y || x >= y) {
            ZoomRangerSürüklemeEkseni::X
        } else if self.ayarlar.y {
            ZoomRangerSürüklemeEkseni::Y
        } else {
            ZoomRangerSürüklemeEkseni::Yok
        }
    }

    fn x_değiştir(&mut self, seçim: Aralık) -> bool {
        if self.seçim_x == seçim {
            return false;
        }
        self.seçim_x = seçim;
        true
    }

    fn y_değiştir(&mut self, seçim: Aralık) -> bool {
        if self.seçim_y == seçim {
            return false;
        }
        self.seçim_y = seçim;
        true
    }
}

/// İşaretçi konumunda sürüklenebilen eksenin çekirdek karşılığı.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EksenHedefi {
    X,
    Y(String),
}

#[derive(Debug, Clone)]
struct EksenSürüklemeBaşlangıcı {
    hedef: EksenHedefi,
    konum: Nokta,
    aralık: Aralık,
    boyut: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DağılımVuruşu {
    pub seri: usize,
    pub indeks: usize,
    pub merkez: Nokta,
    pub boyut: f32,
    pub x: f64,
    pub y: f64,
    pub değer: Option<f64>,
    pub etiket: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TimelineVuruşu {
    pub seri: usize,
    pub indeks: usize,
    pub başlangıç: f64,
    pub bitiş: f64,
    pub değer: String,
}

/// `annotations.html` DOM işaretinin platformdan bağımsız vuruş ve hover
/// geometrisi. Yüzey adaptörleri yalnız bu hafif katmanı yeniden boyar.
#[derive(Debug, Clone, PartialEq)]
pub struct AçıklamaVuruşu {
    pub indeks: usize,
    pub başlangıç_x: f32,
    pub bitiş_x: f32,
    pub üst: f32,
    pub alt: f32,
    pub etiket_konumu: Option<Nokta>,
    pub etiket_genişliği: f32,
    pub etiket_yüksekliği: f32,
    pub etiket_üzerinde: bool,
    pub etiket: String,
    pub açıklama: String,
    pub çizgi: String,
    pub kalınlık: f32,
}

#[derive(Debug, Clone, PartialEq)]
struct ÇubukVuruşAnahtarı {
    genişlik: u32,
    yükseklik: u32,
    x_aralığı: Aralık,
    y_aralığı: Aralık,
    görünür_seriler: Vec<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct ÇubukVuruşKaydı {
    seri: usize,
    indeks: usize,
    konum: Nokta,
    genişlik: f32,
    yükseklik: f32,
    değer: f64,
}

#[derive(Debug, Clone)]
struct ÇubukVuruşDizini {
    anahtar: ÇubukVuruşAnahtarı,
    sütun_sayısı: usize,
    satır_sayısı: usize,
    hücreler: Vec<Vec<usize>>,
    kayıtlar: Vec<ÇubukVuruşKaydı>,
}

impl ÇubukVuruşDizini {
    fn yeni(anahtar: ÇubukVuruşAnahtarı, kayıtlar: Vec<ÇubukVuruşKaydı>) -> Self {
        let kenar = (kayıtlar.len() as f64).sqrt().ceil().clamp(1.0, 32.0) as usize;
        let sütun_sayısı = kenar;
        let satır_sayısı = kenar;
        let mut hücreler = vec![Vec::new(); sütun_sayısı * satır_sayısı];
        let yüzey_genişliği = anahtar.genişlik.max(1) as f32;
        let yüzey_yüksekliği = anahtar.yükseklik.max(1) as f32;
        for (kayıt_indeksi, kayıt) in kayıtlar.iter().enumerate() {
            let sütun = |x: f32| {
                ((x / yüzey_genişliği * sütun_sayısı as f32).floor() as isize)
                    .clamp(0, sütun_sayısı.saturating_sub(1) as isize) as usize
            };
            let satır = |y: f32| {
                ((y / yüzey_yüksekliği * satır_sayısı as f32).floor() as isize)
                    .clamp(0, satır_sayısı.saturating_sub(1) as isize) as usize
            };
            let ilk_sütun = sütun(kayıt.konum.x);
            let son_sütun = sütun(kayıt.konum.x + kayıt.genişlik);
            let ilk_satır = satır(kayıt.konum.y);
            let son_satır = satır(kayıt.konum.y + kayıt.yükseklik);
            for satır in ilk_satır..=son_satır {
                for sütun in ilk_sütun..=son_sütun {
                    if let Some(hücre) = hücreler.get_mut(satır * sütun_sayısı + sütun) {
                        hücre.push(kayıt_indeksi);
                    }
                }
            }
        }
        Self {
            anahtar,
            sütun_sayısı,
            satır_sayısı,
            hücreler,
            kayıtlar,
        }
    }

    fn vuruş(&self, x: f32, y: f32) -> Option<ÇubukVuruşKaydı> {
        if x < 0.0
            || y < 0.0
            || x > self.anahtar.genişlik as f32
            || y > self.anahtar.yükseklik as f32
        {
            return None;
        }
        let sütun = ((x / self.anahtar.genişlik.max(1) as f32 * self.sütun_sayısı as f32).floor()
            as usize)
            .min(self.sütun_sayısı.saturating_sub(1));
        let satır = ((y / self.anahtar.yükseklik.max(1) as f32 * self.satır_sayısı as f32).floor()
            as usize)
            .min(self.satır_sayısı.saturating_sub(1));
        self.hücreler
            .get(satır * self.sütun_sayısı + sütun)?
            .iter()
            .filter_map(|indeks| self.kayıtlar.get(*indeks).copied())
            .find(|kayıt| {
                x >= kayıt.konum.x
                    && x <= kayıt.konum.x + kayıt.genişlik
                    && y >= kayıt.konum.y
                    && y <= kayıt.konum.y + kayıt.yükseklik
            })
    }
}

#[derive(Debug, Clone, PartialEq)]
struct DağılımVuruşAnahtarı {
    genişlik: u32,
    yükseklik: u32,
    x_aralığı: Aralık,
    y_aralıkları: Vec<(String, Aralık)>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct DağılımVuruşKaydı {
    seri: usize,
    indeks: usize,
    merkez: Nokta,
    boyut: f32,
}

/// Bubble hover için küçük ve yeniden kullanılabilir bir uzamsal dizin.
///
/// Kaynak uPlot demosu bir quadtree kullanır. Buradaki eş boyutlu bölütleme
/// aynı bbox ekleme/nokta sorgulama semantiğini korur ve her pointer olayında
/// bütün bubble dizisini tekrar taramayı engeller.
#[derive(Debug, Clone)]
struct DağılımVuruşDizini {
    anahtar: DağılımVuruşAnahtarı,
    sütun_sayısı: usize,
    satır_sayısı: usize,
    hücreler: Vec<Vec<usize>>,
    kayıtlar: Vec<DağılımVuruşKaydı>,
}

impl DağılımVuruşDizini {
    fn yeni(anahtar: DağılımVuruşAnahtarı, kayıtlar: Vec<DağılımVuruşKaydı>) -> Self {
        let kenar = (kayıtlar.len() as f64).sqrt().ceil().clamp(1.0, 32.0) as usize;
        let sütun_sayısı = kenar;
        let satır_sayısı = kenar;
        let mut hücreler = vec![Vec::new(); sütun_sayısı * satır_sayısı];
        let yüzey_genişliği = anahtar.genişlik.max(1) as f32;
        let yüzey_yüksekliği = anahtar.yükseklik.max(1) as f32;
        let sütun = |x: f32| {
            ((x / yüzey_genişliği * sütun_sayısı as f32).floor() as isize)
                .clamp(0, sütun_sayısı.saturating_sub(1) as isize) as usize
        };
        let satır = |y: f32| {
            ((y / yüzey_yüksekliği * satır_sayısı as f32).floor() as isize)
                .clamp(0, satır_sayısı.saturating_sub(1) as isize) as usize
        };
        for (kayıt_indeksi, kayıt) in kayıtlar.iter().enumerate() {
            // Kaynak bbox'a strokeWidth ekler; 0.5 px yarıçap payı aynı
            // sınırdaki hover davranışını korur.
            let yarıçap = kayıt.boyut / 2.0 + 0.5;
            let ilk_sütun = sütun(kayıt.merkez.x - yarıçap);
            let son_sütun = sütun(kayıt.merkez.x + yarıçap);
            let ilk_satır = satır(kayıt.merkez.y - yarıçap);
            let son_satır = satır(kayıt.merkez.y + yarıçap);
            for satır in ilk_satır..=son_satır {
                for sütun in ilk_sütun..=son_sütun {
                    if let Some(hücre) = hücreler.get_mut(satır * sütun_sayısı + sütun) {
                        hücre.push(kayıt_indeksi);
                    }
                }
            }
        }
        Self {
            anahtar,
            sütun_sayısı,
            satır_sayısı,
            hücreler,
            kayıtlar,
        }
    }

    fn adaylar(&self, x: f32, y: f32) -> impl Iterator<Item = DağılımVuruşKaydı> + '_ {
        let sütun = ((x / self.anahtar.genişlik.max(1) as f32 * self.sütun_sayısı as f32).floor()
            as isize)
            .clamp(0, self.sütun_sayısı.saturating_sub(1) as isize) as usize;
        let satır = ((y / self.anahtar.yükseklik.max(1) as f32 * self.satır_sayısı as f32).floor()
            as isize)
            .clamp(0, self.satır_sayısı.saturating_sub(1) as isize) as usize;
        self.hücreler
            .get(satır * self.sütun_sayısı + sütun)
            .into_iter()
            .flatten()
            .filter_map(|indeks| self.kayıtlar.get(*indeks).copied())
    }
}

/// Doğrulanmış seçenek ve veriyi taşıyan çizelge örneği.
pub struct Grafik {
    kimlik: u64,
    seçenekler: GrafikSeçenekleri,
    veri: HizalıVeri,
    /// Raster yüzeyin fiziksel/logical piksel oranı. Platform adaptörü bunu
    /// sağlar; SVG gibi vektör tüketicileri varsayılan 1× değeri kullanır.
    cihaz_piksel_oranı: f32,
    etkileşim: EtkileşimDenetleyicisi,
    odak_serisi: Option<usize>,
    elle_x_aralığı: Option<Aralık>,
    elle_y_aralıkları: BTreeMap<String, Aralık>,
    eksen_sürükleme: Option<EksenSürüklemeBaşlangıcı>,
    ölçüm_datumları: [Option<(f64, f64)>; 2],
    açıklama_stil_indeksleri: Vec<Option<usize>>,
    çubuk_vuruş_dizini: RefCell<Option<ÇubukVuruşDizini>>,
    dağılım_vuruş_dizini: RefCell<Option<DağılımVuruşDizini>>,
    otomatik_çubuk_metinleri: Option<OtomatikÇubukMetinÖnbelleği>,
    /// `seriesMediansPlugin.setData` karşılığı; medyanlar veri değişirken
    /// hesaplanır, her `drawSeries` çağrısında yeniden sıralanmaz.
    çizim_kancası_medyanları: Vec<Option<f64>>,
    seri_yaşam_döngüsü_olayları: Vec<SeriYaşamDöngüsüOlayı>,
}

#[derive(Debug)]
struct OtomatikÇubukMetinÖnbelleği {
    gösterimler: Vec<Vec<Option<String>>>,
    azami_10px_genişlik: f32,
    azami_10px_yükseklik: f32,
}

fn otomatik_çubuk_metin_önbelleği(
    seçenekler: &GrafikSeçenekleri,
    veri: &HizalıVeri,
) -> Option<OtomatikÇubukMetinÖnbelleği> {
    seçenekler
        .çubuk_düzeni
        .is_some_and(|düzen| düzen.değer_etiketi_otomatik)
        .then(|| {
            let gösterimler = veri
                .seriler()
                .iter()
                .map(|seri| {
                    seri.iter()
                        .map(|değer| değer.map(kompakt_sayı))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            let azami_10px_genişlik = gösterimler
                .iter()
                .flat_map(|seri| seri.iter().flatten())
                .map(|metin| {
                    metin
                        .chars()
                        .map(|karakter| match karakter {
                            '-' => 3.33,
                            '.' | ',' => 2.78,
                            '1' => 5.0,
                            'K' | 'M' | 'B' | 'T' => 6.67,
                            _ => 5.56,
                        })
                        .sum::<f32>()
                })
                .fold(1.0_f32, f32::max);
            OtomatikÇubukMetinÖnbelleği {
                gösterimler,
                azami_10px_genişlik,
                // Arial 10px `actualBoundingBoxAscent - actualBoundingBoxDescent`
                // ölçüsünün platformlar arası kararlı yaklaşık karşılığı.
                azami_10px_yükseklik: 8.0,
            }
        })
}

fn çizim_kancası_medyanları(
    seçenekler: &GrafikSeçenekleri,
    veri: &HizalıVeri,
) -> Vec<Option<f64>> {
    if !seçenekler
        .çizim_kancaları
        .as_ref()
        .is_some_and(|düzen| düzen.seri_medyanları)
    {
        return Vec::new();
    }
    veri.seriler()
        .iter()
        .map(|değerler| {
            let mut sıralı = değerler.iter().copied().flatten().collect::<Vec<_>>();
            sıralı.sort_by(f64::total_cmp);
            medyan(&sıralı)
        })
        .collect()
}

fn oran_aralığı(
    mevcut: Aralık, başlangıç: f64, bitiş: f64
) -> Result<Aralık, UplotHatası> {
    let en_az_oran = başlangıç.min(bitiş).clamp(0.0, 1.0);
    let en_çok_oran = başlangıç.max(bitiş).clamp(0.0, 1.0);
    let uzunluk = mevcut.en_çok - mevcut.en_az;
    Aralık::yeni(
        mevcut.en_az + en_az_oran * uzunluk,
        mevcut.en_az + en_çok_oran * uzunluk,
    )
}

impl Grafik {
    pub fn yeni(seçenekler: GrafikSeçenekleri, veri: HizalıVeri) -> Result<Self, UplotHatası> {
        if seçenekler.seriler.len() != veri.seriler().len() {
            return Err(UplotHatası::SeriSeçeneğiEksik {
                beklenen: veri.seriler().len(),
                bulunan: seçenekler.seriler.len(),
            });
        }
        for (seri, ayarlar) in seçenekler.seriler.iter().enumerate() {
            if ayarlar.ölçek != seçenekler.birincil_y_ölçeği
                && !seçenekler
                    .y_ölçekleri
                    .iter()
                    .any(|ölçek| ölçek.anahtar == ayarlar.ölçek)
            {
                return Err(UplotHatası::BilinmeyenÖlçek {
                    seri,
                    anahtar: ayarlar.ölçek.clone(),
                });
            }
            if let Some(üst_seri) = ayarlar.yüzen_çubuk_üst_serisi
                && üst_seri >= veri.seriler().len()
            {
                return Err(UplotHatası::GeçersizSeriİndeksi {
                    indeks: üst_seri,
                    seri_sayısı: veri.seriler().len(),
                    ekleme: false,
                });
            }
        }
        if seçenekler
            .ısı_haritası_düzeni
            .as_ref()
            .is_some_and(|düzen| !düzen.geçerli_mi())
        {
            return Err(UplotHatası::GeçersizKaynakVeri {
                varlık: "IsıHaritasıDüzeni",
                açıklama: "hücre konumu, boyutu veya rengi geçersiz".to_string(),
            });
        }
        if seçenekler
            .timeline_düzeni
            .as_ref()
            .is_some_and(|düzen| !düzen.geçerli_mi(veri.seriler().len()))
        {
            return Err(UplotHatası::GeçersizKaynakVeri {
                varlık: "TimelineDüzeni",
                açıklama: "şerit, hücre sınırı, renk veya boyut geçersiz".to_string(),
            });
        }
        if seçenekler
            .dağılım_düzeni
            .as_ref()
            .is_some_and(|düzen| !düzen.geçerli_mi())
        {
            return Err(UplotHatası::GeçersizKaynakVeri {
                varlık: "DağılımDüzeni",
                açıklama: "seri, nokta koordinatı veya nokta boyutu geçersiz".to_string(),
            });
        }
        if seçenekler.rüzgar_yönü_düzeni.as_ref().is_some_and(|düzen| {
            düzen.hız_serisi >= veri.seriler().len()
                || düzen.yön_serisi >= veri.seriler().len()
                || düzen.ölçek.is_empty()
                || !düzen.uzunluk.is_finite()
                || düzen.uzunluk <= 0.0
                || !düzen.kalınlık.is_finite()
                || düzen.kalınlık <= 0.0
        }) {
            return Err(UplotHatası::GeçersizKaynakVeri {
                varlık: "RüzgarYönüDüzeni",
                açıklama: "seri indeksi, ölçek veya vektör stili geçersiz".to_string(),
            });
        }
        let mut tam = seçenekler
            .x_aralığı
            .or_else(|| tam_x_aralığı(&veri).ok())
            .or(seçenekler.boş_x_aralığı)
            .unwrap_or(Aralık {
                en_az: 0.0,
                en_çok: 1.0,
            });
        if seçenekler.x_aralığı.is_none()
            && let XÖlçekDağılımı::Logaritmik { taban } = seçenekler.x_dağılımı
            && let Some((en_az, en_çok)) =
                sonlu_sınırlar(veri.x().iter().copied().filter(|değer| *değer > 0.0))
            && let Some(log_aralığı) = logaritmik_aralık_sınırlardan(en_az, en_çok, taban, true)
        {
            tam = log_aralığı;
        }
        if (seçenekler
            .çubuk_düzeni
            .is_some_and(|düzen| düzen.x_kenar_paylı)
            || seçenekler.kutu_bıyık_düzeni.is_some())
            && veri.uzunluk() > 1
        {
            tam = Aralık::yeni(tam.en_az - 0.5, tam.en_çok + 0.5)?;
        }
        let birincil_ölçek = seçenekler
            .y_ölçekleri
            .iter()
            .find(|ölçek| ölçek.anahtar == seçenekler.birincil_y_ölçeği);
        let mut tam_y = birincil_ölçek
            .and_then(|ölçek| ölçek.aralık)
            .or(seçenekler.y_aralığı)
            .or_else(|| {
                let sonlu_veri_var = veri
                    .seriler()
                    .iter()
                    .zip(seçenekler.seriler.iter())
                    .filter(|(_, ayarlar)| {
                        ayarlar.göster
                            && ayarlar.otomatik_ölçeğe_katıl
                            && ayarlar.ölçek == seçenekler.birincil_y_ölçeği
                    })
                    .flat_map(|(seri, _)| seri.iter().flatten())
                    .any(|değer| değer.is_finite());
                (!sonlu_veri_var)
                    .then_some(seçenekler.boş_y_aralığı)
                    .flatten()
            })
            .unwrap_or_else(|| {
                let değerler = || {
                    veri.seriler()
                        .iter()
                        .zip(seçenekler.seriler.iter())
                        .filter(|(_, ayarlar)| {
                            ayarlar.göster
                                && ayarlar.otomatik_ölçeğe_katıl
                                && ayarlar.ölçek == seçenekler.birincil_y_ölçeği
                        })
                        .flat_map(|(seri, _)| seri.iter())
                };
                match birincil_ölçek.map(|ölçek| ölçek.dağılım) {
                    Some(YÖlçekDağılımı::Logaritmik { taban }) => logaritmik_otomatik_aralık(
                        değerler(),
                        taban,
                        birincil_ölçek.is_none_or(|ölçek| ölçek.log_tam_büyüklükler),
                    )
                    .unwrap_or_else(|| Aralık::otomatik(değerler())),
                    Some(YÖlçekDağılımı::ArcSinh { .. }) => {
                        arcsinh_otomatik_aralık(değerler().flatten().copied())
                            .unwrap_or_else(|| Aralık::otomatik(değerler()))
                    }
                    _ => birincil_ölçek
                        .and_then(|ölçek| ölçek.sayısal_aralık)
                        .and_then(|ayarlar| {
                            sonlu_sınırlar(değerler().flatten().copied()).and_then(
                                |(en_az, en_çok)| {
                                    Aralık::uplot_yapılandırılmış(en_az, en_çok, ayarlar).ok()
                                },
                            )
                        })
                        .unwrap_or_else(|| Aralık::otomatik(değerler())),
                }
            });
        if let Some(düzen) = birincil_ölçek.and_then(|ölçek| ölçek.güzel_ölçek) {
            let ham_aralık = sonlu_aralık(
                veri.seriler()
                    .iter()
                    .zip(seçenekler.seriler.iter())
                    .filter(|(_, ayarlar)| {
                        ayarlar.göster
                            && ayarlar.otomatik_ölçeğe_katıl
                            && ayarlar.ölçek == seçenekler.birincil_y_ölçeği
                    })
                    .flat_map(|(seri, _)| seri.iter().flatten().copied()),
            );
            let çizim_yüksekliği = seçenekler.yükseklik.saturating_sub(96).max(1) as f32;
            if let Some((aralık, _)) = ham_aralık.and_then(|aralık| {
                güzel_ölçek(aralık, çizim_yüksekliği, düzen.en_az_etiket_boşluğu)
            }) {
                tam_y = aralık;
            }
        }
        if let Some(düzen) = &seçenekler.kutu_bıyık_düzeni {
            let mut değerler = veri
                .seriler()
                .iter()
                .flat_map(|seri| seri.iter().copied().flatten())
                .collect::<Vec<_>>();
            değerler.extend(
                düzen
                    .ayrık_değerler
                    .iter()
                    .flat_map(|ayrıklar| ayrıklar.iter().copied()),
            );
            if let Some(ham) = sonlu_aralık(değerler.into_iter()) {
                tam_y = Aralık::uplot_sayısal(ham.en_az, ham.en_çok, 0.1, true)?;
            }
        }
        let etkileşim = EtkileşimDenetleyicisi::yeni(tam, tam_y, seçenekler.etkileşimler);
        let açıklama_stil_indeksleri =
            seçenekler
                .açıklama_düzeni
                .as_ref()
                .map_or_else(Vec::new, |düzen| {
                    düzen
                        .işaretler
                        .iter()
                        .map(|işaret| düzen.stiller.iter().position(|stil| stil.tür == işaret.tür))
                        .collect()
                });
        let otomatik_çubuk_metinleri = otomatik_çubuk_metin_önbelleği(&seçenekler, &veri);
        let çizim_kancası_medyanları = çizim_kancası_medyanları(&seçenekler, &veri);
        let seri_yaşam_döngüsü_olayları = if seçenekler.seri_yaşam_döngüsünü_izle {
            (0..=seçenekler.seriler.len())
                .map(|seri_indeksi| SeriYaşamDöngüsüOlayı::Eklendi {
                    seri_indeksi,
                    başlangıç: true,
                })
                .collect()
        } else {
            Vec::new()
        };
        Ok(Self {
            kimlik: SON_GRAFİK_KİMLİĞİ.fetch_add(1, Ordering::Relaxed),
            seçenekler,
            veri,
            cihaz_piksel_oranı: 1.0,
            etkileşim,
            odak_serisi: None,
            elle_x_aralığı: None,
            elle_y_aralıkları: BTreeMap::new(),
            eksen_sürükleme: None,
            ölçüm_datumları: [None, None],
            açıklama_stil_indeksleri,
            çubuk_vuruş_dizini: RefCell::new(None),
            dağılım_vuruş_dizini: RefCell::new(None),
            otomatik_çubuk_metinleri,
            çizim_kancası_medyanları,
            seri_yaşam_döngüsü_olayları,
        })
    }

    pub fn çiz(&self) -> Sahne {
        self.çiz_boyutta_aralıklarla(
            self.seçenekler.genişlik,
            self.seçenekler.yükseklik,
            self.elle_x_aralığı.or_else(|| {
                self.etkileşim
                    .yakınlaştırılmış()
                    .then(|| self.etkileşim.görünür_x())
            }),
            self.elle_y_aralıkları
                .get(&self.seçenekler.birincil_y_ölçeği)
                .copied()
                .or_else(|| self.etkileşim.görünür_y()),
            false,
        )
    }

    /// GPUI'nin retained veri katmanı için tam ölçek geometrisini üretir.
    ///
    /// Bu sahne yalnız veri, seri veya boyut değiştiğinde yenilenir. Zoom ve
    /// pan sırasında aynı geometri GPUI dönüşüm matrisiyle yeniden kullanılır.
    pub(crate) fn gpui_tam_sahneyi_çiz(&self) -> Sahne {
        self.çiz_boyutta_aralıklarla(
            self.seçenekler.genişlik,
            self.seçenekler.yükseklik,
            Some(self.etkileşim.tam_x()),
            Some(self.etkileşim.tam_y()),
            false,
        )
        .katmanı_süz(SahneKatmanı::Veri)
    }

    /// Güncel görünümün yalnız hafif eksen/grid katmanını üretir.
    pub(crate) fn gpui_eksen_sahnesini_çiz(&self) -> Sahne {
        self.çiz_boyutta_aralıklarla(
            self.seçenekler.genişlik,
            self.seçenekler.yükseklik,
            Some(self.görünür_x_aralığı()),
            Some(self.gpui_görünür_y_aralığı()),
            true,
        )
        .katmanı_süz(SahneKatmanı::Eksen)
    }

    /// GPUI zoomunda görünür X dilimini taramadan kullanılan Y penceresi.
    ///
    /// Otomatik Y aralığı tam retained yüzeyde sabit kalır. Yalnız geliştirici
    /// veya kullanıcı açıkça bir Y görünümü tanımladığında bu pencere değişir.
    fn gpui_görünür_y_aralığı(&self) -> Aralık {
        self.elle_y_aralıkları
            .get(&self.seçenekler.birincil_y_ölçeği)
            .copied()
            .or_else(|| self.etkileşim.görünür_y())
            .unwrap_or_else(|| self.etkileşim.tam_y())
    }

    /// Geçerli görünümün tam retained sahnedeki fiziksel kaynak penceresini
    /// döndürür. Logaritmik, ters ve dikey ölçekler burada bir kez çözülür.
    /// Tam X/Y ölçekleri içinde geçerli görünümün fiziksel oranlarını verir.
    ///
    /// Bu değer farklı piksel boyutlarına, veri aralıklarına ve ölçek
    /// dağılımlarına sahip grup üyeleri arasında doğrudan paylaşılabilir.
    pub fn oransal_görünüm(&self) -> OransalGörünüm {
        let tam_x = self.etkileşim.tam_x();
        let görünür_x = self.görünür_x_aralığı();
        let tam_y = self.etkileşim.tam_y();
        let görünür_y = self.gpui_görünür_y_aralığı();

        let x0 = self.x_konumu(tam_x, görünür_x.en_az, 0.0, 1.0);
        let x1 = self.x_konumu(tam_x, görünür_x.en_çok, 0.0, 1.0);
        let y0 = self.y_konumu(
            &self.seçenekler.birincil_y_ölçeği,
            tam_y,
            görünür_y.en_az,
            0.0,
            1.0,
        );
        let y1 = self.y_konumu(
            &self.seçenekler.birincil_y_ölçeği,
            tam_y,
            görünür_y.en_çok,
            0.0,
            1.0,
        );

        let (x_sol, x_sağ) = (x0.min(x1), x0.max(x1));
        // Y veri konumu çizimde alttan yukarı çevrilir.
        let (y_üst, y_alt) = ((1.0 - y0).min(1.0 - y1), (1.0 - y0).max(1.0 - y1));
        let mut pencere = if self.seçenekler.x_dikey {
            OransalGörünüm {
                sol: y_üst,
                sağ: y_alt,
                üst: 1.0 - x_sağ,
                alt: 1.0 - x_sol,
            }
        } else {
            OransalGörünüm {
                sol: x_sol,
                sağ: x_sağ,
                üst: y_üst,
                alt: y_alt,
            }
        };
        let sınırla = |başlangıç: f32, bitiş: f32| {
            let başlangıç = başlangıç.clamp(0.0, 1.0 - f32::EPSILON);
            let bitiş = bitiş.clamp(başlangıç + f32::EPSILON, 1.0);
            (başlangıç, bitiş)
        };
        (pencere.sol, pencere.sağ) = sınırla(pencere.sol, pencere.sağ);
        (pencere.üst, pencere.alt) = sınırla(pencere.üst, pencere.alt);
        pencere
    }

    /// Başka bir grafik tarafından yayımlanan fiziksel oran penceresini bu
    /// grafiğin kendi tam ölçeklerine uygular.
    pub fn oransal_görünümü_ayarla(
        &mut self,
        görünüm: OransalGörünüm,
        geçmişe_ekle: bool,
    ) -> Result<bool, UplotHatası> {
        let değerler = [görünüm.sol, görünüm.sağ, görünüm.üst, görünüm.alt];
        if değerler.iter().any(|değer| !değer.is_finite())
            || görünüm.sol < 0.0
            || görünüm.sağ > 1.0
            || görünüm.üst < 0.0
            || görünüm.alt > 1.0
            || görünüm.sol >= görünüm.sağ
            || görünüm.üst >= görünüm.alt
        {
            return Err(UplotHatası::GeçersizAralık {
                en_az: f64::from(görünüm.sol),
                en_çok: f64::from(görünüm.sağ),
            });
        }

        let (x_sol, x_sağ, y_üst, y_alt) = if self.seçenekler.x_dikey {
            (
                1.0 - görünüm.alt,
                1.0 - görünüm.üst,
                görünüm.sol,
                görünüm.sağ,
            )
        } else {
            (görünüm.sol, görünüm.sağ, görünüm.üst, görünüm.alt)
        };
        let tam_x = self.etkileşim.tam_x();
        let x0 = self.x_değeri_orandan(tam_x, f64::from(x_sol));
        let x1 = self.x_değeri_orandan(tam_x, f64::from(x_sağ));
        let x = Aralık::yeni(x0.min(x1), x0.max(x1))?;

        let tam_y = self.etkileşim.tam_y();
        let y0 = self.birincil_y_değeri_konum_oranından(tam_y, f64::from(1.0 - y_alt));
        let y1 = self.birincil_y_değeri_konum_oranından(tam_y, f64::from(1.0 - y_üst));
        let y = Aralık::yeni(y0.min(y1), y0.max(y1))?;

        self.elle_x_aralığı = None;
        self.elle_y_aralıkları.clear();
        Ok(self.etkileşim.görünür_aralıkları_ayarla(x, y, geçmişe_ekle))
    }

    /// Raster ölçeğini platform adaptöründen alır.
    ///
    /// Bu değer bir geliştirici seçeneği değildir: GPUI kendi pencere ölçeğini
    /// iletir; çekirdeğin uPlot `pxAlign` adımı fiziksel piksel ızgarasına
    /// dönüştürülür. SVG kaydı 1× varsayılanıyla aynı vektör semantiğini korur.
    pub(crate) fn cihaz_piksel_oranını_ayarla(&mut self, oran: f32) -> bool {
        let oran = if oran.is_finite() && oran > 0.0 {
            oran
        } else {
            1.0
        };
        if self.cihaz_piksel_oranı.to_bits() == oran.to_bits() {
            return false;
        }
        self.cihaz_piksel_oranı = oran;
        self.çubuk_vuruş_dizini = RefCell::new(None);
        self.dağılım_vuruş_dizini = RefCell::new(None);
        true
    }

    fn doğrulanmış_durumu_uygula(&mut self, yeni: Self, seçenekleri_değiştir: bool) {
        if seçenekleri_değiştir {
            self.seçenekler = yeni.seçenekler;
            self.açıklama_stil_indeksleri = yeni.açıklama_stil_indeksleri;
        }
        self.veri = yeni.veri;
        self.etkileşim = yeni.etkileşim;
        self.odak_serisi = None;
        self.elle_x_aralığı = None;
        self.elle_y_aralıkları.clear();
        self.eksen_sürükleme = None;
        self.ölçüm_datumları = [None, None];
        self.çubuk_vuruş_dizini = RefCell::new(None);
        self.dağılım_vuruş_dizini = RefCell::new(None);
        self.otomatik_çubuk_metinleri = yeni.otomatik_çubuk_metinleri;
        self.çizim_kancası_medyanları = yeni.çizim_kancası_medyanları;
    }

    pub fn ölçüm_datumunu_ayarla(
        &mut self,
        datum: usize,
        yatay_oran: f64,
        dikey_oran: f64,
    ) -> bool {
        if !self.seçenekler.ölçüm_datumları
            || !(1..=2).contains(&datum)
            || !yatay_oran.is_finite()
            || !dikey_oran.is_finite()
            || !(0.0..=1.0).contains(&yatay_oran)
            || !(0.0..=1.0).contains(&dikey_oran)
        {
            return false;
        }
        let x_aralığı = self.görünür_x_aralığı();
        let x = x_aralığı.en_az + yatay_oran * (x_aralığı.en_çok - x_aralığı.en_az);
        let y = self.y_değeri_orandan(self.görünür_y_aralığı(), 1.0 - dikey_oran);
        self.ölçüm_datumları
            .get_mut(datum - 1)
            .is_some_and(|hedef| {
                *hedef = Some((x, y));
                true
            })
    }

    pub fn ölçüm_datumlarını_temizle(&mut self) -> bool {
        let değişti = self.ölçüm_datumları.iter().any(Option::is_some);
        self.ölçüm_datumları = [None, None];
        değişti
    }

    pub const fn ölçüm_datumları(&self) -> [Option<(f64, f64)>; 2] {
        self.ölçüm_datumları
    }

    pub const fn ölçüm_datumları_etkin(&self) -> bool {
        self.seçenekler.ölçüm_datumları
    }

    pub fn görünür_x_aralığı(&self) -> Aralık {
        self.elle_x_aralığı
            .unwrap_or_else(|| self.etkileşim.görünür_x())
    }

    /// Grafiğin doğrulanmış, immutable sütun deposunu döndürür.
    ///
    /// `HizalıVeri::clone()` O(1) olduğu için aynı frame'de güncellenen
    /// ilişkili yüzeyler bu depoyu kopyalamadan paylaşabilir.
    pub const fn veri(&self) -> &HizalıVeri {
        &self.veri
    }

    /// Birincil görünür X aralığından türetilen ikinci zaman eksenini döndürür.
    /// İkinci eksen bağımsız görünüm durumu taşımaz; zoom/pan sonrasında da
    /// kaynak ölçeği aynı sabit farkla izler.
    pub fn ikincil_x_aralığı(&self) -> Option<Aralık> {
        let kaydırma = self.seçenekler.ikincil_x_eksen.as_ref()?.zaman_kaydırması;
        let görünür = self.görünür_x_aralığı();
        Aralık::yeni(görünür.en_az + kaydırma, görünür.en_çok + kaydırma).ok()
    }

    pub fn zoom_ranger_durumu(&self) -> Result<ZoomRangerDurumu, UplotHatası> {
        ZoomRangerDurumu::xy(
            self.etkileşim.tam_x(),
            self.görünür_x_aralığı(),
            self.etkileşim.tam_y(),
            self.görünür_y_aralığı(),
            self.seçenekler.etkileşimler.zoom_ranger,
        )
    }

    pub fn zoom_ranger_uygula(&mut self, ranger: ZoomRangerDurumu) -> bool {
        let x = self.etkileşim.görünür_x_ayarla(ranger.seçim_aralığı());
        let y = self.etkileşim.görünür_y_ayarla(ranger.y_seçim_aralığı());
        x || y
    }

    pub fn boyut(&self) -> (u32, u32) {
        (self.seçenekler.genişlik, self.seçenekler.yükseklik)
    }

    pub fn duyarlı_boyut_mu(&self) -> bool {
        self.seçenekler.duyarlı_boyut
    }

    /// `update-cursor-select-resize` kaynağının adaptör katmanında kurduğu
    /// kalıcı cursor/select/hover durumunun başlangıç oranlarını döndürür.
    ///
    /// Bu durum ana veri sahnesine çizilmez; GPUI masaüstü ve web katmanları uPlot'un
    /// DOM overlay mimarisi gibi ayrı ve hafif bir etkileşim yüzeyi kullanır.
    pub fn boyut_senkron_düzeni(&self) -> Option<crate::BoyutSenkronDüzeni> {
        self.seçenekler.boyut_senkron_düzeni
    }

    /// uPlot `setSize({width, height})` karşılığıdır. Veri ve etkileşim
    /// görünümü korunurken yalnız hedef sahne boyutu değiştirilir.
    pub fn boyutu_ayarla(&mut self, genişlik: u32, yükseklik: u32) -> Result<bool, UplotHatası> {
        if genişlik < 160 || yükseklik < 120 {
            return Err(UplotHatası::GeçersizBoyut {
                genişlik,
                yükseklik,
            });
        }
        if self.boyut() == (genişlik, yükseklik) {
            return Ok(false);
        }
        self.seçenekler.genişlik = genişlik;
        self.seçenekler.yükseklik = yükseklik;
        Ok(true)
    }

    pub fn dağılım_vuruşu_boyutta(
        &self,
        genişlik_px: u32,
        yükseklik_px: u32,
        x: f32,
        y: f32,
    ) -> Option<DağılımVuruşu> {
        let düzen = self.seçenekler.dağılım_düzeni.as_ref()?;
        if !düzen.vuruş_etkin || !x.is_finite() || !y.is_finite() {
            return None;
        }
        let (sol, sağ, üst, alt) = self.çizim_alanı_boyutta(genişlik_px, yükseklik_px);
        if !(sol..=sağ).contains(&x) || !(üst..=alt).contains(&y) {
            return None;
        }
        let x_aralığı = self.görünür_x_aralığı();
        let görünür_y = self.etkileşim.görünür_y();
        let genişlik = sağ - sol;
        let yükseklik = alt - üst;
        let y_aralıkları = düzen
            .seriler
            .iter()
            .map(|seri| {
                (
                    seri.ölçek.clone(),
                    self.görünür_ölçek_aralığı(&seri.ölçek, x_aralığı, görünür_y),
                )
            })
            .collect::<Vec<_>>();
        let anahtar = DağılımVuruşAnahtarı {
            genişlik: genişlik_px,
            yükseklik: yükseklik_px,
            x_aralığı,
            y_aralıkları: y_aralıkları.clone(),
        };
        let güncel = self
            .dağılım_vuruş_dizini
            .borrow()
            .as_ref()
            .is_some_and(|dizin| dizin.anahtar == anahtar);
        if !güncel {
            let mut kayıtlar = Vec::new();
            for (seri, (seri_düzeni, (_, y_aralığı))) in
                düzen.seriler.iter().zip(y_aralıkları.iter()).enumerate()
            {
                for (indeks, nokta) in seri_düzeni.noktalar.iter().enumerate() {
                    let merkez = Nokta::yeni(
                        self.x_konumu(x_aralığı, nokta.x, sol, genişlik),
                        alt - self.y_konumu(
                            &seri_düzeni.ölçek,
                            *y_aralığı,
                            nokta.y,
                            0.0,
                            yükseklik,
                        ),
                    );
                    let yarıçap = nokta.boyut / 2.0 + 0.5;
                    if merkez.x + yarıçap < sol
                        || merkez.x - yarıçap > sağ
                        || merkez.y + yarıçap < üst
                        || merkez.y - yarıçap > alt
                    {
                        continue;
                    }
                    kayıtlar.push(DağılımVuruşKaydı {
                        seri,
                        indeks,
                        merkez,
                        boyut: nokta.boyut,
                    });
                }
            }
            *self.dağılım_vuruş_dizini.borrow_mut() =
                Some(DağılımVuruşDizini::yeni(anahtar, kayıtlar));
        }
        let mut sonuç = None::<(f32, f32, DağılımVuruşu)>;
        let dizin = self.dağılım_vuruş_dizini.borrow();
        for kayıt in dizin
            .as_ref()
            .into_iter()
            .flat_map(|dizin| dizin.adaylar(x, y))
        {
            let Some(nokta) = düzen
                .seriler
                .get(kayıt.seri)
                .and_then(|seri| seri.noktalar.get(kayıt.indeks))
            else {
                continue;
            };
            let yarıçap = kayıt.boyut / 2.0 + 0.5;
            let dx = kayıt.merkez.x - x;
            let dy = kayıt.merkez.y - y;
            let uzaklık_kare = dx * dx + dy * dy;
            if uzaklık_kare > yarıçap * yarıçap {
                continue;
            }
            let alan = kayıt.boyut * kayıt.boyut;
            let aday = DağılımVuruşu {
                seri: kayıt.seri,
                indeks: kayıt.indeks,
                merkez: kayıt.merkez,
                boyut: kayıt.boyut,
                x: nokta.x,
                y: nokta.y,
                değer: nokta.değer,
                etiket: nokta.etiket.clone(),
            };
            if sonuç
                .as_ref()
                .is_none_or(|(önceki_alan, önceki_uzaklık, _)| {
                    alan < *önceki_alan
                        || ((alan - *önceki_alan).abs() <= f32::EPSILON
                            && uzaklık_kare <= *önceki_uzaklık)
                })
            {
                sonuç = Some((alan, uzaklık_kare, aday));
            }
        }
        sonuç.map(|(_, _, vuruş)| vuruş)
    }

    pub fn açıklama_vuruşu_boyutta(
        &self,
        genişlik_px: u32,
        yükseklik_px: u32,
        x: f32,
        y: f32,
    ) -> Option<AçıklamaVuruşu> {
        if !x.is_finite() || !y.is_finite() {
            return None;
        }
        let düzen = self.seçenekler.açıklama_düzeni.as_ref()?;
        let x_aralığı = self.görünür_x_aralığı();
        let (sol, sağ, üst, alt) = self.çizim_alanı_boyutta(genişlik_px, yükseklik_px);
        if y < üst || y > alt {
            return None;
        }
        let çizim_genişliği = sağ - sol;
        düzen
            .işaretler
            .iter()
            .zip(self.açıklama_stil_indeksleri.iter())
            .enumerate()
            .rev()
            .find_map(|(indeks, (işaret, stil_indeksi))| {
                if !işaret.başlangıç.is_finite()
                    || !işaret.bitiş.is_finite()
                    || işaret.bitiş < işaret.başlangıç
                {
                    return None;
                }
                let görünür = (işaret.başlangıç >= x_aralığı.en_az
                    && işaret.başlangıç <= x_aralığı.en_çok)
                    || (işaret.bitiş >= x_aralığı.en_az && işaret.bitiş <= x_aralığı.en_çok)
                    || (işaret.başlangıç <= x_aralığı.en_az && işaret.bitiş >= x_aralığı.en_çok);
                if !görünür {
                    return None;
                }
                let stil = stil_indeksi.and_then(|indeks| düzen.stiller.get(indeks))?;
                let başlangıç_x = self
                    .x_konumu(x_aralığı, işaret.başlangıç, sol, çizim_genişliği)
                    .round();
                let bitiş_x = self
                    .x_konumu(x_aralığı, işaret.bitiş, sol, çizim_genişliği)
                    .round();
                let etiket_genişliği = (işaret.etiket.chars().count() as f32 * 7.0 + 8.0).max(12.0);
                let etiket_yüksekliği = 18.0;
                let etiket_konumu = (başlangıç_x >= sol && başlangıç_x <= sağ).then(|| {
                    Nokta::yeni(
                        başlangıç_x - etiket_genişliği / 2.0,
                        match stil.hiza {
                            crate::AçıklamaHizası::Üst => üst,
                            crate::AçıklamaHizası::Alt => alt - etiket_yüksekliği,
                        },
                    )
                });
                let etiket_üzerinde = etiket_konumu.is_some_and(|konum| {
                    x >= konum.x
                        && x <= konum.x + etiket_genişliği
                        && y >= konum.y
                        && y <= konum.y + etiket_yüksekliği
                });
                let işaret_üzerinde = if işaret.bitiş > işaret.başlangıç {
                    x >= başlangıç_x.clamp(sol, sağ) && x <= bitiş_x.clamp(sol, sağ)
                } else {
                    (x - başlangıç_x).abs() <= stil.kalınlık.max(4.0) / 2.0
                };
                (işaret_üzerinde || etiket_üzerinde).then(|| AçıklamaVuruşu {
                    indeks,
                    başlangıç_x,
                    bitiş_x,
                    üst,
                    alt,
                    etiket_konumu,
                    etiket_genişliği,
                    etiket_yüksekliği,
                    etiket_üzerinde,
                    etiket: işaret.etiket.clone(),
                    açıklama: işaret.açıklama.clone(),
                    çizgi: stil.çizgi.clone(),
                    kalınlık: stil.kalınlık,
                })
            })
    }

    pub fn açıklama_vurgu_sahnesi_boyutta(
        &self,
        genişlik_px: u32,
        yükseklik_px: u32,
        vuruş: &AçıklamaVuruşu,
    ) -> Sahne {
        let mut sahne = Sahne::yeni(genişlik_px, yükseklik_px);
        let (sol, sağ, _, _) = self.çizim_alanı_boyutta(genişlik_px, yükseklik_px);
        if vuruş.başlangıç_x >= sol && vuruş.başlangıç_x <= sağ {
            sahne.ekle(Komut::Çizgi {
                başlangıç: Nokta::yeni(vuruş.başlangıç_x, vuruş.üst),
                bitiş: Nokta::yeni(vuruş.başlangıç_x, vuruş.alt),
                renk: vuruş.çizgi.clone(),
                kalınlık: vuruş.kalınlık,
            });
        }
        if let Some(konum) = vuruş.etiket_konumu {
            for (başlangıç, bitiş) in [
                (
                    konum,
                    Nokta::yeni(konum.x + vuruş.etiket_genişliği, konum.y),
                ),
                (
                    Nokta::yeni(konum.x + vuruş.etiket_genişliği, konum.y),
                    Nokta::yeni(
                        konum.x + vuruş.etiket_genişliği,
                        konum.y + vuruş.etiket_yüksekliği,
                    ),
                ),
                (
                    Nokta::yeni(
                        konum.x + vuruş.etiket_genişliği,
                        konum.y + vuruş.etiket_yüksekliği,
                    ),
                    Nokta::yeni(konum.x, konum.y + vuruş.etiket_yüksekliği),
                ),
                (
                    Nokta::yeni(konum.x, konum.y + vuruş.etiket_yüksekliği),
                    konum,
                ),
            ] {
                sahne.ekle(Komut::Çizgi {
                    başlangıç,
                    bitiş,
                    renk: vuruş.çizgi.clone(),
                    kalınlık: vuruş.kalınlık,
                });
            }
        }
        sahne
    }

    /// uPlot `setData(data)` karşılığı olarak hizalı veriyi doğrular, uygular
    /// ve otomatik ölçeklerle etkileşim görünümünü tam aralığa sıfırlar.
    pub fn veriyi_ayarla(&mut self, veri: HizalıVeri) -> Result<(), UplotHatası> {
        if self.seçenekler.seriler.len() != veri.seriler().len() {
            return Err(UplotHatası::SeriSeçeneğiEksik {
                beklenen: veri.seriler().len(),
                bulunan: self.seçenekler.seriler.len(),
            });
        }
        let birincil_sabit_y = self
            .seçenekler
            .y_ölçekleri
            .iter()
            .find(|ölçek| ölçek.anahtar == self.seçenekler.birincil_y_ölçeği)
            .and_then(|ölçek| ölçek.aralık)
            .or(self.seçenekler.y_aralığı);
        if let Some(tam_y) = birincil_sabit_y {
            // Canlı sabit-Y grafiklerinde seçenek ağacını, stil dizilerini ve
            // eklenti yapılandırmasını her kare yeniden kurmaya gerek yoktur.
            // uPlot `setData(data)` gibi aynı Grafik örneğinde yalnız veri,
            // tam ölçekler ve veriye bağlı dizinler yenilenir.
            let mut tam_x = self
                .seçenekler
                .x_aralığı
                .or_else(|| tam_x_aralığı(&veri).ok())
                .unwrap_or(Aralık {
                    en_az: 0.0,
                    en_çok: 1.0,
                });
            if (self
                .seçenekler
                .çubuk_düzeni
                .is_some_and(|düzen| düzen.x_kenar_paylı)
                || self.seçenekler.kutu_bıyık_düzeni.is_some())
                && veri.uzunluk() > 1
            {
                tam_x = Aralık::yeni(tam_x.en_az - 0.5, tam_x.en_çok + 0.5)?;
            }
            let etkileşim_ayarları = self.etkileşim.ayarlar();
            self.otomatik_çubuk_metinleri =
                otomatik_çubuk_metin_önbelleği(&self.seçenekler, &veri);
            self.çizim_kancası_medyanları = çizim_kancası_medyanları(&self.seçenekler, &veri);
            self.veri = veri;
            self.etkileşim = EtkileşimDenetleyicisi::yeni(tam_x, tam_y, etkileşim_ayarları);
            self.odak_serisi = None;
            self.elle_x_aralığı = None;
            self.elle_y_aralıkları.clear();
            self.eksen_sürükleme = None;
            self.ölçüm_datumları = [None, None];
            *self.çubuk_vuruş_dizini.borrow_mut() = None;
            *self.dağılım_vuruş_dizini.borrow_mut() = None;
            return Ok(());
        }
        let mut seçenekler = self.seçenekler.clone();
        seçenekler.etkileşimler = self.etkileşim.ayarlar();
        let yeni = Self::yeni(seçenekler, veri)?;
        self.doğrulanmış_durumu_uygula(yeni, false);
        Ok(())
    }

    /// Timeline eklentisinin `setData()` + path/quadtree yenilemesini aynı
    /// Grafik örneğinde atomik olarak uygular.
    pub fn timeline_verisini_ayarla(
        &mut self,
        veri: HizalıVeri,
        düzen: crate::TimelineDüzeni,
    ) -> Result<(), UplotHatası> {
        if !düzen.geçerli_mi(veri.seriler().len()) {
            return Err(UplotHatası::GeçersizKaynakVeri {
                varlık: "TimelineDüzeni",
                açıklama: "şerit, hücre sınırı, renk veya boyut geçersiz".to_string(),
            });
        }
        self.veriyi_ayarla(veri)?;
        self.seçenekler.timeline_düzeni = Some(düzen);
        Ok(())
    }

    /// `setData(data)` ile birlikte sabit birincil Y aralığını aynı grafik
    /// örneğinde yeniler. Kaynak eklentinin veriyi yeniden yığdıktan sonra
    /// yaptığı ölçek sıfırlamasını, seçenek ağacını yeniden kurmadan uygular.
    pub fn veriyi_y_aralığında_ayarla(
        &mut self,
        veri: HizalıVeri,
        aralık: Aralık,
    ) -> Result<(), UplotHatası> {
        let önceki = self.seçenekler.y_aralığı;
        self.seçenekler.y_aralığı = Some(aralık);
        if let Err(hata) = self.veriyi_ayarla(veri) {
            self.seçenekler.y_aralığı = önceki;
            return Err(hata);
        }
        Ok(())
    }

    /// `setData()` ile birlikte dinamik Y range/values/fillTo sunumunu aynı
    /// grafik örneğinde atomik olarak değiştirir. Kaynak callback'leri kip
    /// durumundan okuyan grafiklerde seçenek ve etkileşim ağacı korunur.
    pub fn veriyi_y_sunumunda_ayarla(
        &mut self,
        veri: HizalıVeri,
        aralık: Aralık,
        özel_etiketler: Vec<(f64, String)>,
        dolgu_tabanları: Vec<f64>,
    ) -> Result<(), UplotHatası> {
        if dolgu_tabanları.len() != self.seçenekler.seriler.len()
            || dolgu_tabanları.iter().any(|taban| !taban.is_finite())
            || özel_etiketler.iter().any(|(değer, _)| !değer.is_finite())
        {
            return Err(UplotHatası::GeçersizKaynakVeri {
                varlık: "Y sunumu",
                açıklama: "seri tabanları veya özel etiketler geçersiz".to_string(),
            });
        }
        let önceki_aralık = self.seçenekler.y_aralığı;
        let önceki_etiketler = std::mem::replace(
            &mut self.seçenekler.birincil_y_özel_etiketler,
            özel_etiketler,
        );
        let önceki_tabanlar = self
            .seçenekler
            .seriler
            .iter()
            .map(|seri| seri.dolgu_tabanı)
            .collect::<Vec<_>>();
        self.seçenekler.y_aralığı = Some(aralık);
        for (seri, taban) in self.seçenekler.seriler.iter_mut().zip(dolgu_tabanları) {
            seri.dolgu_tabanı = taban;
        }
        if let Err(hata) = self.veriyi_ayarla(veri) {
            self.seçenekler.y_aralığı = önceki_aralık;
            self.seçenekler.birincil_y_özel_etiketler = önceki_etiketler;
            for (seri, taban) in self.seçenekler.seriler.iter_mut().zip(önceki_tabanlar) {
                seri.dolgu_tabanı = taban;
            }
            return Err(hata);
        }
        Ok(())
    }

    /// uPlot `setData(data)` için seçenek/stil ağacını yeniden kurmayan canlı yol.
    ///
    /// Seri sayısı değişmeyen akışlarda yalnız veri, tam ölçekler ve veriye
    /// bağlı vuruş dizinleri yenilenir. Kullanıcı yakınlaştırılmışsa görünür
    /// aralık ve geçmiş korunur; tam görünümdeyse yeni tam ölçek izlenir.
    /// Böylece native/GPUI Web yüzeyleri 100 ms gibi sık tiklerde aynı grafik ve
    /// etkileşim katmanlarını korur.
    pub fn canlı_veriyi_ayarla(&mut self, veri: HizalıVeri) -> Result<(), UplotHatası> {
        if self.seçenekler.seriler.len() != veri.seriler().len() {
            return Err(UplotHatası::SeriSeçeneğiEksik {
                beklenen: veri.seriler().len(),
                bulunan: self.seçenekler.seriler.len(),
            });
        }
        let mut tam_x = self
            .seçenekler
            .x_aralığı
            .or_else(|| tam_x_aralığı(&veri).ok())
            .or(self.seçenekler.boş_x_aralığı)
            .unwrap_or(Aralık {
                en_az: 0.0,
                en_çok: 1.0,
            });
        if (self
            .seçenekler
            .çubuk_düzeni
            .is_some_and(|düzen| düzen.x_kenar_paylı)
            || self.seçenekler.kutu_bıyık_düzeni.is_some())
            && veri.uzunluk() > 1
        {
            tam_x = Aralık::yeni(tam_x.en_az - 0.5, tam_x.en_çok + 0.5)?;
        }
        self.otomatik_çubuk_metinleri = otomatik_çubuk_metin_önbelleği(&self.seçenekler, &veri);
        self.çizim_kancası_medyanları = çizim_kancası_medyanları(&self.seçenekler, &veri);
        self.veri = veri;
        let tam_y = self
            .seçenekler
            .y_ölçekleri
            .iter()
            .find(|ölçek| ölçek.anahtar == self.seçenekler.birincil_y_ölçeği)
            .and_then(|ölçek| ölçek.aralık)
            .or(self.seçenekler.y_aralığı)
            .unwrap_or_else(|| self.y_aralığı(tam_x));
        self.etkileşim.canlı_tam_x_ayarla(tam_x);
        self.etkileşim.canlı_tam_y_ayarla(tam_y);
        self.odak_serisi = None;
        self.eksen_sürükleme = None;
        self.ölçüm_datumları = [None, None];
        *self.çubuk_vuruş_dizini.borrow_mut() = None;
        *self.dağılım_vuruş_dizini.borrow_mut() = None;
        Ok(())
    }

    /// `axis-autosize.html` içindeki `setData()` ile X `values` callback
    /// durumunu aynı grafik örneğinde atomik olarak günceller.
    ///
    /// Seri görünürlüğü, kullanıcı görünümü ve etkileşim geçmişi korunur;
    /// yalnız veri, otomatik tam Y aralığı ve X ekseni etiket çarpanı değişir.
    pub fn canlı_veriyi_x_etiket_çarpanında_ayarla(
        &mut self,
        veri: HizalıVeri,
        çarpan: f64,
    ) -> Result<(), UplotHatası> {
        if !çarpan.is_finite() || çarpan <= 0.0 {
            return Err(UplotHatası::GeçersizÇarpan { değer: çarpan });
        }
        let önceki = self.seçenekler.x_eksen_değer_çarpanı;
        self.seçenekler.x_eksen_değer_çarpanı = çarpan;
        if let Err(hata) = self.canlı_veriyi_ayarla(veri) {
            self.seçenekler.x_eksen_değer_çarpanı = önceki;
            return Err(hata);
        }
        Ok(())
    }

    /// Uzak veri sağlayıcıdan gelen yeni veriyi uygular ve istenen X aralığını korur.
    ///
    /// `zoom-fetch` kaynağındaki `setData(data, false)` + `setScale("x", range)`
    /// çiftinin atomik, yeniden kullanılabilir karşılığıdır.
    pub fn veriyi_x_aralığında_ayarla(
        &mut self,
        veri: HizalıVeri,
        aralık: Aralık,
    ) -> Result<(), UplotHatası> {
        self.veriyi_ayarla(veri)?;
        self.etkileşim.görünür_x_ayarla(aralık);
        Ok(())
    }

    /// Canlı bir grafikte uPlot `setData(data, false)` ve ardından gelen
    /// `setScale("x", range)` çiftini mevcut Grafik örneğini ve kullanıcı
    /// etkileşim durumunu koruyarak uygular.
    pub fn canlı_veriyi_x_aralığında_ayarla(
        &mut self,
        veri: HizalıVeri,
        aralık: Aralık,
    ) -> Result<bool, UplotHatası> {
        let mut seçenekler = self.seçenekler.clone();
        seçenekler.etkileşimler = self.etkileşim.ayarlar();
        let doğrulanmış = Self::yeni(seçenekler, veri)?;
        self.veri = doğrulanmış.veri;
        self.çizim_kancası_medyanları = doğrulanmış.çizim_kancası_medyanları;
        self.çubuk_vuruş_dizini = RefCell::new(None);
        self.dağılım_vuruş_dizini = RefCell::new(None);
        self.seçenekler.x_aralığı = Some(aralık);
        Ok(self.etkileşim.canlı_tam_x_ayarla(aralık))
    }

    /// Veri değişmeden kayan canlı X penceresini ilerletir. Kullanıcı
    /// görünümü yakınlaştırılmışsa tam aralık güncellenir, görünür aralık
    /// dondurulur ve pahalı yeniden boya gerektirilmez.
    pub fn canlı_x_aralığını_ayarla(&mut self, aralık: Aralık) -> bool {
        self.seçenekler.x_aralığı = Some(aralık);
        self.etkileşim.canlı_tam_x_ayarla(aralık)
    }

    /// Otomatik Y ölçeğinin yeni tam aralığını veri ve seri seçeneklerini
    /// yeniden kurmadan uygular. Kullanıcı Y görünümünü elle yakınlaştırdıysa
    /// o görünüm korunur; tam görünümdeyse yeni aralık hemen görünür olur.
    pub fn canlı_y_aralığını_ayarla(&mut self, aralık: Aralık) -> bool {
        self.seçenekler.y_aralığı = Some(aralık);
        self.etkileşim.canlı_tam_y_ayarla(aralık)
    }

    /// Birden çok adlandırılmış Y ölçeğinin sabit aralığını aynı Grafik
    /// örneğinde atomik olarak değiştirir. uPlot `redraw(true)` öncesinde
    /// dinamik `scale.range` sonuçlarının yenilenmesine karşılık gelir.
    pub fn y_ölçek_aralıklarını_ayarla(
        &mut self,
        aralıklar: &[(&str, Aralık)],
    ) -> Result<bool, UplotHatası> {
        for (indeks, (anahtar, _)) in aralıklar.iter().enumerate() {
            if !self
                .seçenekler
                .y_ölçekleri
                .iter()
                .any(|ölçek| ölçek.anahtar == *anahtar)
            {
                return Err(UplotHatası::BilinmeyenÖlçek {
                    seri: indeks,
                    anahtar: (*anahtar).to_string(),
                });
            }
        }

        let mut değişti = false;
        let mut birincil = None;
        for (anahtar, aralık) in aralıklar {
            let Some(ölçek) = self
                .seçenekler
                .y_ölçekleri
                .iter_mut()
                .find(|ölçek| ölçek.anahtar == *anahtar)
            else {
                continue;
            };
            değişti |= ölçek.aralık != Some(*aralık);
            ölçek.aralık = Some(*aralık);
            if *anahtar == self.seçenekler.birincil_y_ölçeği {
                birincil = Some(*aralık);
            }
        }
        if let Some(aralık) = birincil {
            değişti |= self.etkileşim.canlı_tam_y_ayarla(aralık);
        }
        Ok(değişti)
    }

    /// Görünür X ölçeğinde iki normalize edilmiş seçim ucunu veri aralığına dönüştürür.
    pub fn x_aralığı_oranlardan(
        &self,
        başlangıç_oranı: f64,
        bitiş_oranı: f64,
    ) -> Result<Aralık, UplotHatası> {
        let görünür = self.görünür_x_aralığı();
        let başlangıç = self.x_değeri_orandan(görünür, başlangıç_oranı.clamp(0.0, 1.0));
        let bitiş = self.x_değeri_orandan(görünür, bitiş_oranı.clamp(0.0, 1.0));
        Aralık::yeni(başlangıç.min(bitiş), başlangıç.max(bitiş))
    }

    /// Y-serisi seçeneğini ve hizalı değerlerini tek, doğrulanmış işlemle ekler.
    /// İndeks yalnız Y serilerini sayar; uPlot'un X dahil `seriesIdx = 2`
    /// değeri burada `indeks = 1` olur.
    pub fn seri_ekle(
        &mut self,
        indeks: usize,
        seçenek: crate::SeriSeçenekleri,
        değerler: Vec<Option<f64>>,
    ) -> Result<(), UplotHatası> {
        let seri_sayısı = self.veri.seriler().len();
        if indeks > seri_sayısı {
            return Err(UplotHatası::GeçersizSeriİndeksi {
                indeks,
                seri_sayısı,
                ekleme: true,
            });
        }
        let veri = self.veri.seri_ekle(indeks, değerler)?;
        let mut seçenekler = self.seçenekler.clone();
        seçenekler.etkileşimler = self.etkileşim.ayarlar();
        seçenekler.seriler.insert(indeks, seçenek);
        let yeni = Self::yeni(seçenekler, veri)?;
        self.doğrulanmış_durumu_uygula(yeni, true);
        if self.seçenekler.seri_yaşam_döngüsünü_izle {
            self.seri_yaşam_döngüsü_olayları
                .push(SeriYaşamDöngüsüOlayı::Eklendi {
                    seri_indeksi: indeks + 1,
                    başlangıç: false,
                });
            self.seri_yaşam_döngüsü_olayları
                .push(SeriYaşamDöngüsüOlayı::VeriAyarlandı {
                    seri_sayısı: self.seçenekler.seriler.len() + 1,
                });
        }
        Ok(())
    }

    /// Y-serisi seçeneğini ve hizalı değerlerini aynı anda siler.
    pub fn seri_sil(&mut self, indeks: usize) -> Result<(), UplotHatası> {
        let seri_sayısı = self.veri.seriler().len();
        if indeks >= seri_sayısı {
            return Err(UplotHatası::GeçersizSeriİndeksi {
                indeks,
                seri_sayısı,
                ekleme: false,
            });
        }
        let veri = self.veri.seri_sil(indeks)?;
        let mut seçenekler = self.seçenekler.clone();
        seçenekler.etkileşimler = self.etkileşim.ayarlar();
        seçenekler.seriler.remove(indeks);
        let yeni = Self::yeni(seçenekler, veri)?;
        self.doğrulanmış_durumu_uygula(yeni, true);
        if self.seçenekler.seri_yaşam_döngüsünü_izle {
            self.seri_yaşam_döngüsü_olayları
                .push(SeriYaşamDöngüsüOlayı::Silindi {
                    seri_indeksi: indeks + 1,
                });
            self.seri_yaşam_döngüsü_olayları
                .push(SeriYaşamDöngüsüOlayı::VeriAyarlandı {
                    seri_sayısı: self.seçenekler.seriler.len() + 1,
                });
        }
        Ok(())
    }

    /// Bir Y serisini veri ve diğer seri ayarlarını yeniden kurmadan gösterir
    /// veya gizler. Çubuk grupları görünür seri sayısına göre yeniden yerleşir.
    pub fn seri_görünürlüğünü_ayarla(
        &mut self,
        indeks: usize,
        görünür: bool,
    ) -> Result<bool, UplotHatası> {
        let seri_sayısı = self.seçenekler.seriler.len();
        if let Some(düzen) = self.seçenekler.timeline_düzeni.as_mut() {
            let Some(seri_görünür) = düzen.seri_görünürlükleri.get_mut(indeks) else {
                return Err(UplotHatası::GeçersizSeriİndeksi {
                    indeks,
                    seri_sayısı,
                    ekleme: false,
                });
            };
            if *seri_görünür == görünür {
                return Ok(false);
            }
            *seri_görünür = görünür;
            return Ok(true);
        }
        let Some(seri) = self.seçenekler.seriler.get_mut(indeks) else {
            return Err(UplotHatası::GeçersizSeriİndeksi {
                indeks,
                seri_sayısı,
                ekleme: false,
            });
        };
        if seri.göster == görünür {
            return Ok(false);
        }
        let ölçek = seri.ölçek.clone();
        seri.göster = görünür;
        // uPlot `setSeries({show})`, otomatik serinin ölçeğini yeniden
        // kuyruğa alır. Eksen sürüklemesinden kalan elle aralık bu değişim
        // sonrasında eski görünür seri kümesine bağlı kalmamalıdır.
        self.elle_y_aralıkları.remove(&ölçek);
        if !görünür && self.odak_serisi == Some(indeks) {
            self.odak_serisi = None;
        }
        Ok(true)
    }

    pub fn seri_görünür_mü(&self, indeks: usize) -> bool {
        self.seçenekler
            .timeline_düzeni
            .as_ref()
            .and_then(|düzen| düzen.seri_görünürlükleri.get(indeks))
            .copied()
            .or_else(|| self.seçenekler.seriler.get(indeks).map(|seri| seri.göster))
            .unwrap_or(false)
    }

    /// Çalışan grafik örneğini yeniden kurmadan seri bantlarını değiştirir.
    ///
    /// uPlot eklentilerinin `delBand()` / `addBand()` akışına karşılık gelir;
    /// özellikle `setSeries` sonrasında görünür serileri yeniden yığan
    /// uygulamalar bunu `veriyi_ayarla()` öncesinde kullanabilir.
    pub fn bantları_ayarla(&mut self, bantlar: Vec<SeriBandı>) -> bool {
        if self.seçenekler.bantlar == bantlar {
            return false;
        }
        self.seçenekler.bantlar = bantlar;
        true
    }

    /// Bir çizgi veya çubuk serisinin temel çizgi/dolgu renklerini çalışma
    /// anında değiştirir. `dolgu = None`, yalnız çizgi rengini koruyan seriler
    /// için dolguyu temizler.
    pub fn seri_renklerini_ayarla(
        &mut self,
        indeks: usize,
        çizgi: impl Into<String>,
        dolgu: Option<String>,
    ) -> Result<bool, UplotHatası> {
        let seri_sayısı = self.seçenekler.seriler.len();
        let Some(seri) = self.seçenekler.seriler.get_mut(indeks) else {
            return Err(UplotHatası::GeçersizSeriİndeksi {
                indeks,
                seri_sayısı,
                ekleme: false,
            });
        };
        let çizgi = çizgi.into();
        if seri.renk == çizgi && seri.dolgu == dolgu {
            return Ok(false);
        }
        seri.renk = çizgi;
        seri.dolgu = dolgu;
        Ok(true)
    }

    /// Her veri noktası için ayrı çubuk dolgu ve vuruş renklerini çalışma
    /// anında değiştirir. Eksik girişler serinin temel renklerine geri düşer.
    pub fn seri_çubuk_renklerini_ayarla(
        &mut self,
        indeks: usize,
        dolgular: Vec<String>,
        çizgiler: Vec<String>,
    ) -> Result<bool, UplotHatası> {
        let seri_sayısı = self.seçenekler.seriler.len();
        let Some(seri) = self.seçenekler.seriler.get_mut(indeks) else {
            return Err(UplotHatası::GeçersizSeriİndeksi {
                indeks,
                seri_sayısı,
                ekleme: false,
            });
        };
        if seri.çubuk_dolguları == dolgular && seri.çubuk_çizgileri == çizgiler {
            return Ok(false);
        }
        seri.çubuk_dolguları = dolgular;
        seri.çubuk_çizgileri = çizgiler;
        Ok(true)
    }

    /// Bütün Y serilerinin uPlot `spanGaps` değerini birlikte değiştirir.
    pub fn boşlukları_birleştir_ayarla(&mut self, birleştir: bool) -> bool {
        let mut değişti = false;
        for seri in &mut self.seçenekler.seriler {
            if seri.boşlukları_birleştir != birleştir {
                seri.boşlukları_birleştir = birleştir;
                değişti = true;
            }
        }
        değişti
    }

    /// Başlık ve eksen payları çıkarıldıktan sonraki gerçek çizim alanını
    /// `(sol, sağ, üst, alt)` olarak döndürür. Yüzey adaptörleri sabit sayı
    /// çoğaltmak yerine bu çekirdek geometrisini kullanır.
    pub fn çizim_alanı_boyutta(
        &self,
        genişlik_px: u32,
        yükseklik_px: u32,
    ) -> (f32, f32, f32, f32) {
        let genişlik_px = if self.seçenekler.kompakt_yüzey {
            genişlik_px.max(2)
        } else {
            genişlik_px.max(160)
        } as f32;
        let yükseklik_px = if self.seçenekler.kompakt_yüzey {
            yükseklik_px.max(2)
        } else {
            yükseklik_px.max(120)
        } as f32;
        let gizli_eksen_payı = if self.seçenekler.kompakt_yüzey {
            0.0
        } else {
            8.0
        };
        if let Some(düzen) = self.seçenekler.çubuk_düzeni {
            return match düzen.yön {
                crate::ÇubukYönü::Dikey => (64.0, genişlik_px - 24.0, 48.0, yükseklik_px - 72.0),
                crate::ÇubukYönü::Yatay => {
                    (150.0, genişlik_px - 32.0, 48.0, yükseklik_px - 48.0)
                }
            };
        }
        if self.seçenekler.kutu_bıyık_düzeni.is_some() {
            return (64.0, genişlik_px - 24.0, 48.0, yükseklik_px - 130.0);
        }
        if self.seçenekler.mum_düzeni.is_some() {
            return (72.0, genişlik_px - 72.0, 48.0, yükseklik_px - 48.0);
        }
        if self.seçenekler.x_dikey {
            let sol_pay = if !self.seçenekler.x_eksen_görünür {
                gizli_eksen_payı
            } else if self.seçenekler.x_eksen_karşıda {
                24.0
            } else {
                64.0
            };
            let sağ_pay = if !self.seçenekler.x_eksen_görünür {
                gizli_eksen_payı
            } else if self.seçenekler.x_eksen_karşıda {
                64.0
            } else {
                24.0
            };
            let üst_pay = if !self.seçenekler.birincil_y_eksen_görünür {
                if self.seçenekler.başlık.is_empty() {
                    gizli_eksen_payı
                } else {
                    48.0
                }
            } else if self.seçenekler.birincil_y_karşıda {
                48.0
            } else {
                68.0
            };
            let alt_pay = if !self.seçenekler.birincil_y_eksen_görünür {
                gizli_eksen_payı
            } else if self.seçenekler.birincil_y_karşıda {
                48.0
            } else {
                24.0
            };
            return (
                sol_pay,
                genişlik_px - sağ_pay,
                üst_pay,
                yükseklik_px - alt_pay,
            );
        }
        let sağ_eksen_genişliği = self
            .seçenekler
            .y_ölçekleri
            .iter()
            .filter(|ölçek| {
                ölçek.anahtar != self.seçenekler.birincil_y_ölçeği
                    && ölçek.eksen_görünür
                    && ölçek.sağda
            })
            .map(|ölçek| ölçek.eksen_genişliği)
            .sum::<f32>();
        let sol_eksen_genişliği = self
            .seçenekler
            .y_ölçekleri
            .iter()
            .filter(|ölçek| {
                ölçek.anahtar != self.seçenekler.birincil_y_ölçeği
                    && ölçek.eksen_görünür
                    && !ölçek.sağda
            })
            .map(|ölçek| ölçek.eksen_genişliği)
            .sum::<f32>();
        let mut sağ_pay: f32 = if !self.seçenekler.birincil_y_eksen_görünür
            && sağ_eksen_genişliği <= f32::EPSILON
        {
            gizli_eksen_payı
        } else if self.seçenekler.birincil_y_sağda {
            72.0 + sağ_eksen_genişliği
        } else if sağ_eksen_genişliği > 0.0 {
            24.0 + sağ_eksen_genişliği
        } else {
            24.0
        };
        let mut sol_pay: f32 = if !self.seçenekler.birincil_y_eksen_görünür
            && sol_eksen_genişliği <= f32::EPSILON
        {
            gizli_eksen_payı
        } else if self.seçenekler.birincil_y_sağda {
            24.0 + sol_eksen_genişliği
        } else {
            64.0 + sol_eksen_genişliği
        };
        if let Some(genişlik) = self.seçenekler.birincil_y_eksen_genişliği {
            if self.seçenekler.birincil_y_sağda {
                sağ_pay = genişlik + sağ_eksen_genişliği;
            } else {
                sol_pay = genişlik + sol_eksen_genişliği;
            }
        }
        if self.seçenekler.timeline_düzeni.is_some() {
            // Kaynak timelinePlugin axes[1].size=70 ve gap=15 kullanır.
            sol_pay = sol_pay.max(85.0);
        }
        let alt_pay = if !self.seçenekler.x_eksen_görünür {
            gizli_eksen_payı
        } else if self.seçenekler.x_eksen_karşıda {
            24.0
        } else if self.seçenekler.ikincil_x_eksen.is_some() {
            68.0
        } else if self.seçenekler.x_eksen_etiketi.is_empty() {
            48.0
        } else {
            68.0
        };
        let üst_pay = if !self.seçenekler.x_eksen_görünür {
            if self.seçenekler.başlık.is_empty() {
                gizli_eksen_payı
            } else {
                48.0
            }
        } else if self.seçenekler.x_eksen_karşıda {
            if self.seçenekler.x_eksen_etiketi.is_empty() {
                68.0
            } else {
                88.0
            }
        } else {
            48.0
        };
        if self.seçenekler.otomatik_y_eksen_genişliği && !self.seçenekler.birincil_y_sağda {
            let aralık = self.görünür_y_aralığı();
            let çizim_yüksekliği = (yükseklik_px - üst_pay - alt_pay).max(1.0);
            let artım = uygun_artım(aralık, çizim_yüksekliği, 30.0);
            let ölçek = self.ölçek_seçeneği(&self.seçenekler.birincil_y_ölçeği);
            let birim = ölçek.map_or("", |ölçek| ölçek.birim.as_str());
            let dağılım = ölçek.map(|ölçek| ölçek.dağılım);
            let biçim = ölçek.map_or(YÖlçekEtiketBiçimi::Otomatik, |ölçek| ölçek.etiket_biçimi);
            let çarpan = ölçek.map_or(1.0, |ölçek| ölçek.eksen_değer_çarpanı);
            let etiketler = self
                .y_eksen_bölmeleri(&self.seçenekler.birincil_y_ölçeği, aralık, çizim_yüksekliği)
                .into_iter()
                .map(|değer| {
                    ölçek_eksen_değerini_yaz(değer * çarpan, artım, birim, dağılım, biçim)
                })
                .collect::<Vec<_>>();
            if let Some(hesap) = self.seçenekler.otomatik_y_eksen_genişliği_hesabı {
                let en_uzun = etiketler
                    .iter()
                    .map(|etiket| etiket.chars().count())
                    .max()
                    .unwrap_or(1);
                sol_pay = hesap.taban + en_uzun as f32 * hesap.karakter_başına;
            } else {
                let en_uzun = etiketler
                    .iter()
                    .map(|etiket| yaklaşık_metin_genişliği(etiket, 12.0))
                    .fold(0.0_f32, f32::max);
                // Kaynak callback: ticks.size + axis.gap + measureText().
                sol_pay = (15.0 + en_uzun).ceil();
            }
        }
        if self.seçenekler.otomatik_x_sağ_pay {
            // Kaynak `autoPadRight` en fazla üç yerleşim çevrimi yapar ve
            // son gerçek split etiketinin yarı genişliği taşarsa payı artırır.
            sağ_pay = 8.0;
            let x_aralığı = self.görünür_x_aralığı();
            for _ in 0..3 {
                let çizim_genişliği = (genişlik_px - sol_pay - sağ_pay).max(1.0);
                let x_artımı = uygun_artım(
                    x_aralığı,
                    çizim_genişliği,
                    self.seçenekler.x_eksen_asgari_etiket_boşluğu,
                );
                let Some(son_bölme) = eksen_bölmeleri_artımla(x_aralığı, x_artımı).last().copied()
                else {
                    break;
                };
                let son_etiket = eksen_değerini_yaz(
                    son_bölme * self.seçenekler.x_eksen_değer_çarpanı,
                    x_artımı * self.seçenekler.x_eksen_değer_çarpanı,
                );
                let yarı_genişlik = yaklaşık_metin_genişliği(&son_etiket, 12.0) / 2.0;
                let bölme_oranı =
                    ((son_bölme - x_aralığı.en_az) / (x_aralığı.en_çok - x_aralığı.en_az)) as f32;
                let sağ_etiket_kenarı = sol_pay + bölme_oranı * çizim_genişliği + yarı_genişlik;
                let sağ_grafik_kenarı = genişlik_px - sağ_pay;
                let yeni = if sağ_etiket_kenarı >= genişlik_px {
                    (sağ_etiket_kenarı - sağ_grafik_kenarı).max(8.0)
                } else {
                    8.0
                };
                if (yeni - sağ_pay).abs() < 0.5 {
                    sağ_pay = yeni;
                    break;
                }
                sağ_pay = yeni;
            }
        }
        (
            sol_pay,
            genişlik_px - sağ_pay,
            üst_pay,
            yükseklik_px - alt_pay,
        )
    }

    pub fn yakınlaştırılmış(&self) -> bool {
        self.etkileşim.yakınlaştırılmış()
            || self.elle_x_aralığı.is_some()
            || !self.elle_y_aralıkları.is_empty()
    }

    pub fn geri_var(&self) -> bool {
        self.etkileşim.geri_var()
    }

    pub fn etkileşim_seçenekleri(&self) -> crate::EtkileşimSeçenekleri {
        self.etkileşim.ayarlar()
    }

    /// Grafik örneğinin `setData`, `addSeries` ve `delSeries` boyunca
    /// değişmeyen kimliği.
    pub const fn kimlik(&self) -> u64 {
        self.kimlik
    }

    pub fn seri_yaşam_döngüsü_olayları(&self) -> &[SeriYaşamDöngüsüOlayı] {
        &self.seri_yaşam_döngüsü_olayları
    }

    pub fn seri_yaşam_döngüsü_olaylarını_al(&mut self) -> Vec<SeriYaşamDöngüsüOlayı> {
        std::mem::take(&mut self.seri_yaşam_döngüsü_olayları)
    }

    pub fn seri_seçenekleri(&self) -> &[crate::SeriSeçenekleri] {
        &self.seçenekler.seriler
    }

    /// Standart, içi boş kırılım noktalarının görünürlük tercihi.
    pub const fn kırılım_noktaları_görünür(&self) -> bool {
        self.seçenekler.kırılım_noktaları_görünür
    }

    /// Standart, içi boş kırılım noktalarının görünürlüğünü veri ve ölçekleri
    /// yeniden kurmadan değiştirir.
    pub fn kırılım_noktalarını_göster(&mut self, görünür: bool) -> bool {
        if self.seçenekler.kırılım_noktaları_görünür == görünür {
            return false;
        }
        self.seçenekler.kırılım_noktaları_görünür = görünür;
        true
    }

    pub fn hizalı_veri(&self) -> &HizalıVeri {
        &self.veri
    }

    pub fn eksen_göstergeleri_etkin(&self) -> bool {
        self.seçenekler.eksen_göstergeleri
    }

    pub fn çubuk_grafiği(&self) -> bool {
        self.seçenekler.çubuk_düzeni.is_some()
    }

    fn çubuk_vuruş_anahtarı(
        &self,
        genişlik: u32,
        yükseklik: u32,
        x_aralığı: Aralık,
        y_aralığı: Aralık,
    ) -> ÇubukVuruşAnahtarı {
        ÇubukVuruşAnahtarı {
            genişlik,
            yükseklik,
            x_aralığı,
            y_aralığı,
            görünür_seriler: self
                .seçenekler
                .seriler
                .iter()
                .map(|seri| seri.göster)
                .collect(),
        }
    }

    pub fn kutu_bıyık_grafiği(&self) -> bool {
        self.seçenekler.kutu_bıyık_düzeni.is_some()
    }

    /// Kutu-bıyık hover bilgisindeki kaynak framework/kategori etiketini döndürür.
    pub fn kutu_bıyık_kategorisi(&self, indeks: usize) -> Option<&str> {
        self.kutu_bıyık_grafiği()
            .then(|| self.seçenekler.kategoriler.get(indeks))
            .flatten()
            .map(String::as_str)
    }

    pub fn mum_grafiği(&self) -> bool {
        self.seçenekler.mum_düzeni.is_some()
    }

    /// Mum hover tooltip'indeki kaynak UTC tarih etiketini döndürür.
    pub fn mum_tarih_etiketi(&self, indeks: usize) -> Option<String> {
        let zaman = self
            .seçenekler
            .mum_düzeni
            .as_ref()?
            .zamanlar
            .get(indeks)
            .copied()?;
        let (yıl, ay, gün, ..) = crate::zaman::utc_alanları(zaman)?;
        Some(format!("{yıl:04}-{ay:02}-{gün:02}"))
    }

    pub fn kutu_bıyık_vuruşu(
        &self,
        genişlik_px: u32,
        yükseklik_px: u32,
        x: f32,
        y: f32,
    ) -> Option<(usize, Nokta, f32, f32, [f64; 5])> {
        if (!self.kutu_bıyık_grafiği() && !self.mum_grafiği()) || !x.is_finite() || !y.is_finite()
        {
            return None;
        }
        let (sol, sağ, üst, alt) = self.çizim_alanı_boyutta(genişlik_px, yükseklik_px);
        if x < sol || x > sağ || y < üst || y > alt {
            return None;
        }
        let aralık = self.görünür_x_aralığı();
        let açıklık = aralık.en_çok - aralık.en_az;
        if açıklık <= f64::EPSILON {
            return None;
        }
        let sütun_genişliği = (sağ - sol) / açıklık as f32;
        let hedef = aralık.en_az + f64::from((x - sol) / (sağ - sol)) * açıklık;
        let x_değerleri = self.veri.x();
        let sağ_indeks = x_değerleri.partition_point(|değer| *değer < hedef);
        let (indeks, x_değeri) = [sağ_indeks.checked_sub(1), Some(sağ_indeks)]
            .into_iter()
            .flatten()
            .filter_map(|indeks| {
                x_değerleri
                    .get(indeks)
                    .copied()
                    .map(|değer| (indeks, değer))
            })
            .filter(|(_, değer)| (*değer - hedef).abs() <= 0.5)
            .min_by(|(_, sol), (_, sağ)| (*sol - hedef).abs().total_cmp(&(*sağ - hedef).abs()))?;
        let değer = |seri: usize| {
            self.veri
                .seriler()
                .get(seri)
                .and_then(|değerler| değerler.get(indeks))
                .copied()
                .flatten()
        };
        let değerler = [
            değer(0)?,
            değer(1)?,
            değer(2)?,
            değer(3).unwrap_or(f64::NAN),
            değer(4).unwrap_or(f64::NAN),
        ];
        let merkez = sol + ((x_değeri - aralık.en_az) / açıklık) as f32 * (sağ - sol);
        let sütun_sol = (merkez - sütun_genişliği / 2.0).clamp(sol, sağ);
        let sütun_sağ = (merkez + sütun_genişliği / 2.0).clamp(sol, sağ);
        Some((
            indeks,
            Nokta::yeni(sütun_sol, üst),
            sütun_sağ - sütun_sol,
            alt - üst,
            değerler,
        ))
    }

    /// Çizim koordinatındaki noktayı kaynak çubuk dikdörtgenlerinden biriyle
    /// eşleştirir. Yüzey adaptörleri yerleşim veya hit-test kodu tekrarlamaz.
    pub fn çubuk_vuruşu(
        &self,
        genişlik_px: u32,
        yükseklik_px: u32,
        x: f32,
        y: f32,
    ) -> Option<(usize, usize, Nokta, f32, f32, f64)> {
        if !self.çubuk_grafiği() || !x.is_finite() || !y.is_finite() {
            return None;
        }
        if self.veri.seriler().is_empty() {
            return None;
        }
        let beklenen = self.çubuk_vuruş_anahtarı(
            genişlik_px,
            yükseklik_px,
            self.görünür_x_aralığı(),
            self.görünür_y_aralığı(),
        );
        let güncel = self
            .çubuk_vuruş_dizini
            .borrow()
            .as_ref()
            .is_some_and(|dizin| dizin.anahtar == beklenen);
        if !güncel {
            // İlk vuruşta geometri bir kez ana çizim yolundan kurulur. Sonraki
            // pointer hareketleri yalnız uzamsal hücreyi tarar; sahne yeniden
            // üretilmez ve seri renkleri üzerinden komut eşleştirilmez.
            drop(self.çiz_görünür_boyutta(genişlik_px, yükseklik_px));
        }
        self.çubuk_vuruş_dizini.borrow().as_ref().and_then(|dizin| {
            dizin.vuruş(x, y).map(|kayıt| {
                (
                    kayıt.seri,
                    kayıt.indeks,
                    kayıt.konum,
                    kayıt.genişlik,
                    kayıt.yükseklik,
                    kayıt.değer,
                )
            })
        })
    }

    /// Yüzey imlecinin normalize edilmiş konumunu kartın çekirdek ayarına göre
    /// uyarlar. Izgara kapalıysa oranlar kesintisiz biçimde geri döner.
    pub fn imleç_oranlarını_uyarla(
        &self,
        yatay_oran: f64,
        dikey_oran: f64,
        çizim_genişliği: f64,
        çizim_yüksekliği: f64,
    ) -> Option<(f64, f64)> {
        if !yatay_oran.is_finite()
            || !dikey_oran.is_finite()
            || !çizim_genişliği.is_finite()
            || !çizim_yüksekliği.is_finite()
            || çizim_genişliği <= 0.0
            || çizim_yüksekliği <= 0.0
        {
            return None;
        }
        let yatay = yatay_oran.clamp(0.0, 1.0);
        let dikey = dikey_oran.clamp(0.0, 1.0);
        let Some(adım) = self.seçenekler.imleç_ızgara_adımı.map(f64::from) else {
            return Some((yatay, dikey));
        };
        let x = ((yatay * çizim_genişliği / adım).round() * adım) / çizim_genişliği;
        let y = ((dikey * çizim_yüksekliği / adım).round() * adım) / çizim_yüksekliği;
        // Resmî `cursor.move` callback'i sonucu ikinci kez kırpmaz. Çizim
        // alanının son 10 px hücresi sınırda bittiğinde yuvarlanan imleç çok
        // az dışarı taşabilir; seçim uçları da aynı dönüştürülmüş koordinatı
        // kullanır.
        Some((x, y))
    }

    pub fn tekerlek_etkileşimi_ayarla(&mut self, etkin: bool) {
        self.etkileşim.tekerlek_etkileşimi_ayarla(etkin);
    }

    /// GPUI yüzeyinde tekerlek yakınlaştırmasının grafik odağı alınmadan da
    /// çalışıp çalışmayacağını değiştirir.
    pub fn tekerlek_odaksız_etkileşimi_ayarla(&mut self, etkin: bool) {
        self.etkileşim.tekerlek_odaksız_etkileşimi_ayarla(etkin);
    }

    /// Verilen yüzey koordinatında sürüklenebilir bir eksen olup olmadığını
    /// belirler. Platform bağlayıcıları eksen yerleşimini tekrar hesaplamaz.
    pub fn eksen_vuruşu_boyutta(
        &self,
        genişlik_px: u32,
        yükseklik_px: u32,
        x: f32,
        y: f32,
    ) -> Option<EksenHedefi> {
        if !self.etkileşim.ayarlar().eksen_sürükleme || !x.is_finite() || !y.is_finite() {
            return None;
        }
        let (sol, sağ, üst, alt) = self.çizim_alanı_boyutta(genişlik_px, yükseklik_px);
        if self.seçenekler.x_dikey {
            if self.seçenekler.x_eksen_görünür
                && (üst..=alt).contains(&y)
                && if self.seçenekler.x_eksen_karşıda {
                    x > sağ && x <= genişlik_px as f32
                } else {
                    x >= 0.0 && x < sol
                }
            {
                return Some(EksenHedefi::X);
            }
            if (sol..=sağ).contains(&x) {
                let birincil_bölge = if self.seçenekler.birincil_y_karşıda {
                    y > alt && y <= yükseklik_px as f32
                } else {
                    y >= 0.0 && y < üst
                };
                if self.seçenekler.birincil_y_eksen_görünür && birincil_bölge {
                    return Some(EksenHedefi::Y(self.seçenekler.birincil_y_ölçeği.clone()));
                }
            }
            return None;
        }

        if self.seçenekler.x_eksen_görünür
            && (sol..=sağ).contains(&x)
            && if self.seçenekler.x_eksen_karşıda {
                y >= 0.0 && y < üst
            } else {
                y > alt && y <= yükseklik_px as f32
            }
        {
            return Some(EksenHedefi::X);
        }
        if !(üst..=alt).contains(&y) {
            return None;
        }
        let (sağda, uzaklık) = if x >= 0.0 && x < sol {
            (false, sol - x)
        } else if x > sağ && x <= genişlik_px as f32 {
            (true, x - sağ)
        } else {
            return None;
        };
        let mut eksenler = Vec::new();
        if self.seçenekler.birincil_y_eksen_görünür && self.seçenekler.birincil_y_sağda == sağda
        {
            eksenler.push((
                self.seçenekler.birincil_y_ölçeği.clone(),
                self.seçenekler.birincil_y_eksen_genişliği.unwrap_or(56.0),
            ));
        }
        eksenler.extend(
            self.seçenekler
                .y_ölçekleri
                .iter()
                .filter(|ölçek| {
                    ölçek.anahtar != self.seçenekler.birincil_y_ölçeği
                        && ölçek.eksen_görünür
                        && ölçek.sağda == sağda
                })
                .map(|ölçek| (ölçek.anahtar.clone(), ölçek.eksen_genişliği)),
        );
        if eksenler.is_empty() {
            return None;
        }
        let mut sınır = 0.0;
        for (anahtar, genişlik) in &eksenler {
            sınır += genişlik.max(0.0);
            if uzaklık <= sınır {
                return Some(EksenHedefi::Y(anahtar.clone()));
            }
        }
        eksenler
            .last()
            .map(|(anahtar, _)| EksenHedefi::Y(anahtar.clone()))
    }

    /// Resmî `y-scale-drag` kancasındaki eksen sürüklemesini çekirdekte başlatır.
    pub fn eksen_sürüklemeyi_başlat(
        &mut self,
        genişlik_px: u32,
        yükseklik_px: u32,
        x: f32,
        y: f32,
    ) -> bool {
        let Some(hedef) = self.eksen_vuruşu_boyutta(genişlik_px, yükseklik_px, x, y) else {
            return false;
        };
        let (sol, sağ, üst, alt) = self.çizim_alanı_boyutta(genişlik_px, yükseklik_px);
        let boyut = match hedef {
            EksenHedefi::X if self.seçenekler.x_dikey => f64::from(alt - üst),
            EksenHedefi::X => f64::from(sağ - sol),
            EksenHedefi::Y(_) if self.seçenekler.x_dikey => f64::from(sağ - sol),
            EksenHedefi::Y(_) => f64::from(alt - üst),
        };
        if !boyut.is_finite() || boyut <= f64::EPSILON {
            return false;
        }
        let aralık = match &hedef {
            EksenHedefi::X => self.görünür_x_aralığı(),
            EksenHedefi::Y(anahtar) => self.görünür_ölçek_aralığı(
                anahtar,
                self.görünür_x_aralığı(),
                self.etkileşim.görünür_y(),
            ),
        };
        self.eksen_sürükleme = Some(EksenSürüklemeBaşlangıcı {
            hedef,
            konum: Nokta::yeni(x, y),
            aralık,
            boyut,
        });
        true
    }

    /// Aktif ekseni taşır; Shift basılıyken kaynak demodaki gibi ölçeği
    /// iki uçtan büyütür veya daraltır.
    pub fn eksen_sürükle(&mut self, x: f32, y: f32, shift: bool) -> Result<bool, UplotHatası> {
        if !x.is_finite() || !y.is_finite() {
            return Ok(false);
        }
        let Some(başlangıç) = self.eksen_sürükleme.clone() else {
            return Ok(false);
        };
        let fark = match başlangıç.hedef {
            EksenHedefi::X if self.seçenekler.x_dikey => başlangıç.konum.y - y,
            EksenHedefi::X => başlangıç.konum.x - x,
            EksenHedefi::Y(_) if self.seçenekler.x_dikey => x - başlangıç.konum.x,
            EksenHedefi::Y(_) => y - başlangıç.konum.y,
        };
        let açıklık = başlangıç.aralık.en_çok - başlangıç.aralık.en_az;
        let kaydırma = f64::from(fark) * açıklık / başlangıç.boyut;
        if !kaydırma.is_finite() {
            return Ok(false);
        }
        let ham_en_az = if shift {
            başlangıç.aralık.en_az - kaydırma
        } else {
            başlangıç.aralık.en_az + kaydırma
        };
        let ham_en_çok = başlangıç.aralık.en_çok + kaydırma;
        if !ham_en_az.is_finite()
            || !ham_en_çok.is_finite()
            || (ham_en_çok - ham_en_az).abs() < 1e-16
        {
            return Ok(false);
        }
        let aralık = Aralık::yeni(ham_en_az.min(ham_en_çok), ham_en_az.max(ham_en_çok))?;
        match &başlangıç.hedef {
            EksenHedefi::X => {
                let değişti = self.elle_x_aralığı != Some(aralık);
                self.elle_x_aralığı = Some(aralık);
                Ok(değişti)
            }
            EksenHedefi::Y(anahtar) => {
                let değişti = self.elle_y_aralıkları.get(anahtar) != Some(&aralık);
                self.elle_y_aralıkları.insert(anahtar.clone(), aralık);
                Ok(değişti)
            }
        }
    }

    pub fn eksen_sürüklemeyi_bitir(&mut self) {
        self.eksen_sürükleme = None;
    }

    pub fn eksen_sürükleniyor(&self) -> bool {
        self.eksen_sürükleme.is_some()
    }

    /// ArcSinh ölçeğinin doğrusal merkez eşiğini çalışma anında değiştirir.
    /// Platform yüzeyleri yalnız bu çekirdek API'sini çağırır.
    pub fn y_arcsinh_eşiği_ayarla(&mut self, anahtar: &str, eşik: f64) -> bool {
        if !eşik.is_finite() || eşik <= 0.0 {
            return false;
        }
        let Some(ölçek) = self
            .seçenekler
            .y_ölçekleri
            .iter_mut()
            .find(|ölçek| ölçek.anahtar == anahtar)
        else {
            return false;
        };
        if !matches!(ölçek.dağılım, YÖlçekDağılımı::ArcSinh { .. }) {
            return false;
        }
        ölçek.dağılım = YÖlçekDağılımı::ArcSinh { eşik };
        true
    }

    pub fn tekerlek(
        &mut self,
        yatay_odak_oranı: f64,
        dikey_odak_oranı: f64,
        delta: f64,
        hassas: bool,
    ) -> Result<bool, UplotHatası> {
        self.tekerlek_eksende(
            yatay_odak_oranı,
            dikey_odak_oranı,
            delta,
            hassas,
            TekerlekEkseni::İkisi,
        )
    }

    /// Tekerlek yakınlaştırmasını yalnız X, yalnız Y veya iki eksende uygular.
    /// Yüzey adaptörleri Shift'i X'e, Ctrl'ü Y'ye bağlayabilir.
    pub fn tekerlek_eksende(
        &mut self,
        yatay_odak_oranı: f64,
        dikey_odak_oranı: f64,
        delta: f64,
        hassas: bool,
        eksen: TekerlekEkseni,
    ) -> Result<bool, UplotHatası> {
        self.tekerlek_eksende_zamanda(
            yatay_odak_oranı,
            dikey_odak_oranı,
            delta,
            hassas,
            eksen,
            Instant::now(),
        )
    }

    pub(crate) fn tekerlek_eksende_zamanda(
        &mut self,
        yatay_odak_oranı: f64,
        dikey_odak_oranı: f64,
        delta: f64,
        hassas: bool,
        eksen: TekerlekEkseni,
        şimdi: Instant,
    ) -> Result<bool, UplotHatası> {
        self.elle_x_aralığını_etkileşime_aktar();
        self.elle_y_aralıklarını_etkileşime_aktar();
        let görünür_y = self.görünür_y_aralığı();
        let y_dağılımı = self.birincil_y_dağılımı();
        let (x_oranı, y_oranı) =
            self.fiziksel_oranları_mantıksala(yatay_odak_oranı, dikey_odak_oranı);
        let değişti = self.etkileşim.tekerlek(
            x_oranı,
            y_oranı,
            görünür_y,
            delta,
            hassas,
            eksen,
            self.seçenekler.x_dağılımı,
            y_dağılımı,
            şimdi,
        )?;
        if değişti && matches!(eksen, TekerlekEkseni::İkisi | TekerlekEkseni::X) {
            self.x_aralığını_veriye_yapıştır();
        }
        Ok(değişti)
    }

    pub fn seçim_yakınlaştır(
        &mut self,
        başlangıç_oranı: f64,
        bitiş_oranı: f64,
    ) -> Result<bool, UplotHatası> {
        self.elle_x_aralığını_etkileşime_aktar();
        let (başlangıç_oranı, bitiş_oranı) = if self.seçenekler.x_ters_yön {
            (1.0 - başlangıç_oranı, 1.0 - bitiş_oranı)
        } else {
            (başlangıç_oranı, bitiş_oranı)
        };
        let değişti = self.etkileşim.seçim_yakınlaştır(
            başlangıç_oranı,
            bitiş_oranı,
            self.seçenekler.x_dağılımı,
        )?;
        if değişti {
            self.x_aralığını_veriye_yapıştır();
        }
        Ok(değişti)
    }

    /// Fiziksel bir seçim dikdörtgenini, ölçek yönü ve yöneliminden bağımsız
    /// biçimde hem X hem Y görünümüne uygular.
    pub fn fiziksel_seçim_yakınlaştır(
        &mut self,
        yatay_başlangıç: f64,
        dikey_başlangıç: f64,
        yatay_bitiş: f64,
        dikey_bitiş: f64,
    ) -> Result<bool, UplotHatası> {
        self.fiziksel_seçim_yakınlaştır_eksenlerde(
            yatay_başlangıç,
            dikey_başlangıç,
            yatay_bitiş,
            dikey_bitiş,
            true,
            true,
        )
    }

    /// Fiziksel sürükleme farklarını uPlot'un `cursor.drag` eşiğine göre
    /// uygulanacak ekran eksenlerine dönüştürür.
    ///
    /// `drag.x` ve `drag.y` birlikte açık, `drag.uni` ise `null` olduğunda
    /// uPlot eksenlerden yalnız biri eşiği geçse bile iki ekseni birlikte
    /// seçer. Bu yöntem GPUI masaüstü ve web yüzeylerinin aynı kararı vermesini sağlar.
    pub fn fiziksel_seçim_eksenleri(
        &self,
        yatay_fark: f64,
        dikey_fark: f64,
        eşik: f64,
    ) -> (bool, bool) {
        if ![yatay_fark, dikey_fark, eşik]
            .into_iter()
            .all(f64::is_finite)
            || eşik < 0.0
            || !self.etkileşim.ayarlar().seçim_yakınlaştır
        {
            return (false, false);
        }
        let yatay = yatay_fark.abs() >= eşik;
        let dikey = dikey_fark.abs() >= eşik;
        if self.etkileşim.ayarlar().seçim_xy_yakınlaştır {
            let ikisi = yatay || dikey;
            (ikisi, ikisi)
        } else if self.x_dikey_mi() {
            (false, dikey)
        } else {
            (yatay, false)
        }
    }

    /// Fiziksel seçimin yalnız hareket eden ekran eksenlerini uygular.
    /// Böylece uPlot'un `drag.x`/`drag.y` davranışındaki gibi yatay veya dikey
    /// tek eksenli bir sürükleme, öteki ölçeği sıfır genişliğe indirmez.
    #[allow(clippy::too_many_arguments)]
    pub fn fiziksel_seçim_yakınlaştır_eksenlerde(
        &mut self,
        yatay_başlangıç: f64,
        dikey_başlangıç: f64,
        yatay_bitiş: f64,
        dikey_bitiş: f64,
        yatay_etkin: bool,
        dikey_etkin: bool,
    ) -> Result<bool, UplotHatası> {
        if !self.etkileşim.ayarlar().seçim_yakınlaştır {
            return Ok(false);
        }
        if !yatay_etkin && !dikey_etkin {
            return Ok(false);
        }
        if ![yatay_başlangıç, dikey_başlangıç, yatay_bitiş, dikey_bitiş]
            .into_iter()
            .all(f64::is_finite)
        {
            return Err(UplotHatası::GeçersizAralık {
                en_az: yatay_başlangıç,
                en_çok: yatay_bitiş,
            });
        }
        self.elle_x_aralığını_etkileşime_aktar();
        self.elle_y_aralıklarını_etkileşime_aktar();
        let (x0, y0) = self.fiziksel_oranları_mantıksala(yatay_başlangıç, dikey_başlangıç);
        let (x1, y1) = self.fiziksel_oranları_mantıksala(yatay_bitiş, dikey_bitiş);
        let mevcut_x = self.görünür_x_aralığı();
        let mevcut_y = self.görünür_y_aralığı();
        let x_dikey = self.x_dikey_mi();
        let x_etkin = if x_dikey { dikey_etkin } else { yatay_etkin };
        let y_etkin = if x_dikey { yatay_etkin } else { dikey_etkin };
        let x = if x_etkin {
            let dağılım = self.seçenekler.x_dağılımı;
            let dönüştürülmüş = x_aralığını_dönüştür(mevcut_x, dağılım).unwrap_or(mevcut_x);
            let aralık = oran_aralığı(dönüştürülmüş, x0, x1)?;
            x_aralığını_geri_dönüştür(aralık, dağılım).unwrap_or(aralık)
        } else {
            mevcut_x
        };
        let y = if y_etkin {
            let dağılım = self.birincil_y_dağılımı();
            let dönüştürülmüş = y_aralığını_dönüştür(mevcut_y, dağılım).unwrap_or(mevcut_y);
            let aralık = oran_aralığı(dönüştürülmüş, 1.0 - y0, 1.0 - y1)?;
            y_aralığını_geri_dönüştür(aralık, dağılım).unwrap_or(aralık)
        } else {
            mevcut_y
        };
        let değişti = self.etkileşim.görünür_aralıkları_ayarla(x, y, true);
        if değişti {
            self.x_aralığını_veriye_yapıştır();
        }
        Ok(değişti)
    }

    /// Bir senkron grubun kaynak grafiğindeki görünümü hedef grafiğe taşır.
    pub fn görünür_aralıkları_ayarla(
        &mut self,
        x: Aralık,
        y: Aralık,
        geçmişe_ekle: bool,
    ) -> bool {
        self.elle_x_aralığı = None;
        self.elle_y_aralıkları.clear();
        self.etkileşim.görünür_aralıkları_ayarla(x, y, geçmişe_ekle)
    }

    /// Senkron gruplarında yalnız ortak X ölçeğini taşır. Hedefin Y görünümü
    /// elle ayarlıysa korunur; otomatikse yeni X penceresindeki veriden tekrar
    /// hesaplanır.
    pub fn görünür_x_aralığını_ayarla(&mut self, x: Aralık, geçmişe_ekle: bool) -> bool {
        self.elle_x_aralığı = None;
        self.etkileşim.görünür_x_aralığını_ayarla(x, geçmişe_ekle)
    }

    /// Seçim bırakma davranışını kart ayarlarına göre çekirdekte çözümler.
    ///
    /// `açıklama_tuşu` açıkken cursor bind sözleşmesi normal seçim ölçeklemesini
    /// geçici olarak durdurur; yüzeyin metin istemesi için ayrı sonuç döner.
    pub fn seçimi_bitir(
        &mut self,
        başlangıç_oranı: f64,
        bitiş_oranı: f64,
        açıklama_tuşu: bool,
    ) -> Result<SeçimEylemi, UplotHatası> {
        let ayarlar = self.etkileşim.ayarlar();
        if !ayarlar.seçim_yakınlaştır {
            return Ok(SeçimEylemi::Değişmedi);
        }
        if açıklama_tuşu && ayarlar.imleç_bağları.ctrl_seçim_ölçeğini_durdur {
            if !başlangıç_oranı.is_finite() || !bitiş_oranı.is_finite() {
                return Err(UplotHatası::GeçersizAralık {
                    en_az: başlangıç_oranı,
                    en_çok: bitiş_oranı,
                });
            }
            return Ok(if ayarlar.imleç_bağları.açıklama_metni_iste {
                SeçimEylemi::Açıklamaİstendi
            } else {
                SeçimEylemi::Değişmedi
            });
        }
        self.seçim_yakınlaştır(başlangıç_oranı, bitiş_oranı)
            .map(|değişti| {
                if değişti {
                    SeçimEylemi::Yakınlaştırıldı
                } else {
                    SeçimEylemi::Değişmedi
                }
            })
    }

    pub fn tam_görünüm(&mut self) -> bool {
        self.eksen_sürükleme = None;
        let elle_x_değişti = self.elle_x_aralığı.take().is_some();
        let elle_değişti = !self.elle_y_aralıkları.is_empty();
        self.elle_y_aralıkları.clear();
        self.etkileşim.tam_görünüm() || elle_x_değişti || elle_değişti
    }

    pub fn önceki_görünüm(&mut self) -> bool {
        self.etkileşim.geri()
    }

    pub fn taşımayı_başlat(&mut self) -> bool {
        self.elle_x_aralığını_etkileşime_aktar();
        self.elle_y_aralıklarını_etkileşime_aktar();
        let görünür_y = self.görünür_y_aralığı();
        self.etkileşim.taşımayı_başlat(görünür_y)
    }

    pub fn taşı(
        &mut self,
        yatay_fark_oranı: f64,
        dikey_fark_oranı: f64,
    ) -> Result<bool, UplotHatası> {
        let (x_farkı, y_farkı) =
            self.fiziksel_farkları_mantıksala(yatay_fark_oranı, dikey_fark_oranı);
        let değişti = self.etkileşim.taşı(
            x_farkı,
            y_farkı,
            self.seçenekler.x_dağılımı,
            self.birincil_y_dağılımı(),
        )?;
        if değişti {
            self.x_aralığını_veriye_yapıştır();
        }
        Ok(değişti)
    }

    pub fn taşımayı_bitir(&mut self) {
        self.etkileşim.taşımayı_bitir();
    }

    pub fn dokunmayı_başlat(&mut self) -> bool {
        self.elle_x_aralığını_etkileşime_aktar();
        self.elle_y_aralıklarını_etkileşime_aktar();
        let görünür_y = self.görünür_y_aralığı();
        self.etkileşim.dokunmayı_başlat(görünür_y)
    }

    pub fn dokunma_yakınlaştır(
        &mut self,
        yatay_odak_oranı: f64,
        dikey_odak_oranı: f64,
        çarpan: f64,
    ) -> Result<bool, UplotHatası> {
        let (x_oranı, y_oranı) =
            self.fiziksel_oranları_mantıksala(yatay_odak_oranı, dikey_odak_oranı);
        let y_dağılımı = self.birincil_y_dağılımı();
        let değişti = self.etkileşim.dokunma_yakınlaştır(
            x_oranı,
            y_oranı,
            çarpan,
            self.seçenekler.x_dağılımı,
            y_dağılımı,
        )?;
        if değişti {
            self.x_aralığını_veriye_yapıştır();
        }
        Ok(değişti)
    }

    /// Yüzeydeki fiziksel oranları uPlot ölçek yönü ve yönelimine göre
    /// çekirdeğin mantıksal X/Y oranlarına dönüştürür.
    pub fn fiziksel_oranları_mantıksala(&self, yatay: f64, dikey: f64) -> (f64, f64) {
        let yatay = yatay.clamp(0.0, 1.0);
        let dikey = dikey.clamp(0.0, 1.0);
        let y_ters = self
            .ölçek_seçeneği(&self.seçenekler.birincil_y_ölçeği)
            .is_some_and(|ölçek| ölçek.ters_yön);
        if self.seçenekler.x_dikey {
            let x = if self.seçenekler.x_ters_yön {
                dikey
            } else {
                1.0 - dikey
            };
            let y = if y_ters { yatay } else { 1.0 - yatay };
            (x, y)
        } else {
            let x = if self.seçenekler.x_ters_yön {
                1.0 - yatay
            } else {
                yatay
            };
            let y = if y_ters { 1.0 - dikey } else { dikey };
            (x, y)
        }
    }

    fn fiziksel_farkları_mantıksala(&self, yatay: f64, dikey: f64) -> (f64, f64) {
        let y_ters = self
            .ölçek_seçeneği(&self.seçenekler.birincil_y_ölçeği)
            .is_some_and(|ölçek| ölçek.ters_yön);
        if self.seçenekler.x_dikey {
            let x = if self.seçenekler.x_ters_yön {
                -dikey
            } else {
                dikey
            };
            let y = if y_ters { -yatay } else { yatay };
            (x, y)
        } else {
            let x = if self.seçenekler.x_ters_yön {
                -yatay
            } else {
                yatay
            };
            let y = if y_ters { -dikey } else { dikey };
            (x, y)
        }
    }

    pub fn x_dikey_mi(&self) -> bool {
        self.seçenekler.x_dikey
    }

    fn x_aralığını_veriye_yapıştır(&mut self) {
        if !self.seçenekler.x_aralığı_veriye_yapışık {
            return;
        }
        let görünür = self.etkileşim.görünür_x();
        let en_yakın = |hedef: f64| {
            self.veri
                .x()
                .iter()
                .copied()
                .min_by(|sol, sağ| (sol - hedef).abs().total_cmp(&(sağ - hedef).abs()))
        };
        let (Some(en_az), Some(en_çok)) = (en_yakın(görünür.en_az), en_yakın(görünür.en_çok))
        else {
            return;
        };
        if en_az >= en_çok {
            return;
        }
        if let Ok(aralık) = Aralık::yeni(en_az, en_çok) {
            self.etkileşim.görünür_x_ayarla(aralık);
        }
    }

    pub fn dokunmayı_bitir(&mut self) {
        self.etkileşim.dokunmayı_bitir();
    }

    fn elle_x_aralığını_etkileşime_aktar(&mut self) {
        if let Some(aralık) = self.elle_x_aralığı.take() {
            self.etkileşim.görünür_x_ayarla(aralık);
        }
    }

    fn elle_y_aralıklarını_etkileşime_aktar(&mut self) {
        if let Some(aralık) = self
            .elle_y_aralıkları
            .remove(&self.seçenekler.birincil_y_ölçeği)
        {
            self.etkileşim.görünür_y_ayarla(aralık);
        }
        self.elle_y_aralıkları.clear();
    }

    /// Geçerli X görünümündeki veriden hesaplanan Y aralığını döndürür.
    pub fn görünür_y_aralığı(&self) -> Aralık {
        self.elle_y_aralıkları
            .get(&self.seçenekler.birincil_y_ölçeği)
            .copied()
            .or_else(|| self.etkileşim.görünür_y())
            .unwrap_or_else(|| {
                if self.kutu_bıyık_grafiği() {
                    self.kutu_bıyık_y_aralığı()
                } else {
                    let x_aralığı = self.görünür_x_aralığı();
                    let çizim_yüksekliği =
                        self.seçenekler.yükseklik.saturating_sub(96).max(1) as f32;
                    self.güzel_ölçek_aralığı(
                        &self.seçenekler.birincil_y_ölçeği,
                        x_aralığı,
                        çizim_yüksekliği,
                    )
                    .map_or_else(|| self.y_aralığı(x_aralığı), |(aralık, _)| aralık)
                }
            })
    }

    fn birincil_y_dağılımı(&self) -> YÖlçekDağılımı {
        self.ölçek_seçeneği(&self.seçenekler.birincil_y_ölçeği)
            .map_or(YÖlçekDağılımı::Doğrusal, |ölçek| ölçek.dağılım)
    }

    fn y_değeri_orandan(&self, aralık: Aralık, oran: f64) -> f64 {
        let dağılım = self.birincil_y_dağılımı();
        let dönüştürülmüş = y_aralığını_dönüştür(aralık, dağılım).unwrap_or(aralık);
        let değer = dönüştürülmüş.en_az
            + oran.clamp(0.0, 1.0) * (dönüştürülmüş.en_çok - dönüştürülmüş.en_az);
        y_değerini_geri_dönüştür(değer, dağılım).unwrap_or(değer)
    }

    /// [`Self::y_konumu`] tarafından üretilen, alttan yukarı ölçülen fiziksel
    /// oranı birincil Y ölçeğinin veri değerine geri dönüştürür.
    fn birincil_y_değeri_konum_oranından(&self, aralık: Aralık, oran: f64) -> f64 {
        let ters = self
            .ölçek_seçeneği(&self.seçenekler.birincil_y_ölçeği)
            .is_some_and(|ölçek| ölçek.ters_yön);
        self.y_değeri_orandan(aralık, if ters { 1.0 - oran } else { oran })
    }

    /// Adlandırılmış bir Y ölçeğinin geçerli görünür aralığını döndürür.
    ///
    /// `scale.from` ile türetilen ölçekler, X autoscale ve birincil Y
    /// etkileşimleri sonrasında da kaynak ölçekle aynı görünüm oranını korur.
    pub fn görünür_y_ölçek_aralığı(&self, anahtar: &str) -> Option<Aralık> {
        self.ölçek_seçeneği(anahtar)?;
        Some(self.görünür_ölçek_aralığı(
            anahtar,
            self.görünür_x_aralığı(),
            self.etkileşim.görünür_y(),
        ))
    }

    pub fn seri_görünür_y_aralığı(&self, seri_indeksi: usize) -> Option<Aralık> {
        let seri = self.seçenekler.seriler.get(seri_indeksi)?;
        Some(self.görünür_ölçek_aralığı(
            &seri.ölçek,
            self.görünür_x_aralığı(),
            self.etkileşim.görünür_y(),
        ))
    }

    pub fn seri_y_konum_oranı(&self, seri_indeksi: usize, değer: f64) -> Option<f64> {
        if !değer.is_finite() {
            return None;
        }
        let seri = self.seçenekler.seriler.get(seri_indeksi)?;
        let aralık = self.seri_görünür_y_aralığı(seri_indeksi)?;
        Some(f64::from(self.y_konumu(
            &seri.ölçek,
            aralık,
            değer,
            0.0,
            1.0,
        )))
    }

    pub fn x_konum_oranı(&self, değer: f64) -> Option<f64> {
        if !değer.is_finite() {
            return None;
        }
        let aralık = self.görünür_x_aralığı();
        Some(f64::from(self.x_konumu(aralık, değer, 0.0, 1.0)))
    }

    /// Timeline eklentisinin kaynak quadtree hover davranışı gibi, geçerli
    /// X konumunu kapsayan her şeridin tam hücresini döndürür.
    pub fn timeline_vuruşları(&self, yatay_oran: f64) -> Vec<TimelineVuruşu> {
        self.timeline_vuruşları_pikselde(yatay_oran, f64::INFINITY)
    }

    /// Timeline vuruşunu gerçek çizim genişliğinde çözer. `azami_genişlik`
    /// kullanan matrix hücrelerinde veri aralığı yerine ekranda boyanan
    /// dikdörtgen sınırını kullanır.
    pub fn timeline_vuruşları_pikselde(
        &self,
        yatay_oran: f64,
        çizim_genişliği: f64,
    ) -> Vec<TimelineVuruşu> {
        if !yatay_oran.is_finite() {
            return Vec::new();
        }
        let Some(düzen) = self.seçenekler.timeline_düzeni.as_ref() else {
            return Vec::new();
        };
        let aralık = self.görünür_x_aralığı();
        let hedef = self.x_değeri_orandan(aralık, yatay_oran.clamp(0.0, 1.0));
        let aralık_genişliği = aralık.en_çok - aralık.en_az;
        düzen
            .hücreler
            .iter()
            .filter_map(|hücre| {
                if !düzen
                    .seri_görünürlükleri
                    .get(hücre.seri_indeksi)
                    .copied()
                    .unwrap_or(false)
                {
                    return None;
                }
                let bitiş = if hücre.sağ_kenara_uzat {
                    aralık.en_çok
                } else {
                    hücre.bitiş
                };
                let (mut başlangıç, mut bitiş) = (hücre.başlangıç, bitiş);
                if let Some(azami) = hücre.azami_genişlik
                    && çizim_genişliği.is_finite()
                    && çizim_genişliği > 0.0
                    && aralık_genişliği > 0.0
                {
                    let veri_genişliği = bitiş - başlangıç;
                    let piksel_genişliği = veri_genişliği / aralık_genişliği * çizim_genişliği;
                    if piksel_genişliği > f64::from(azami) {
                        let merkez = (başlangıç + bitiş) / 2.0;
                        let yarı = f64::from(azami) / çizim_genişliği * aralık_genişliği / 2.0;
                        başlangıç = merkez - yarı;
                        bitiş = merkez + yarı;
                    }
                }
                (hedef >= başlangıç && hedef <= bitiş).then(|| TimelineVuruşu {
                    seri: hücre.seri_indeksi,
                    indeks: hücre.veri_indeksi,
                    başlangıç,
                    bitiş,
                    değer: hücre.değer.clone(),
                })
            })
            .collect()
    }

    pub fn timeline_seri_sayısı(&self) -> usize {
        self.seçenekler
            .timeline_düzeni
            .as_ref()
            .map_or(0, |düzen| düzen.seri_etiketleri.len())
    }

    /// Geçerli görünümde, normalize edilmiş yatay konuma en yakın seri noktasını bulur.
    pub fn en_yakın_nokta(&self, yatay_oran: f64, seri_indeksi: usize) -> Option<(f64, f64)> {
        if !yatay_oran.is_finite() {
            return None;
        }
        let seri = self.veri.seriler().get(seri_indeksi)?;
        let aralık = self.görünür_x_aralığı();
        let hedef = self.x_değeri_orandan(aralık, yatay_oran.clamp(0.0, 1.0));
        let indeks =
            en_yakın_dolu_x_indeksi(self.veri.x(), seri, aralık, hedef, NullAtlamaYönü::EnYakın)?;
        Some((
            self.veri.x().get(indeks).copied()?,
            seri.get(indeks).copied().flatten()?,
        ))
    }

    /// Hizalı seride null değerleri atlayıp kaynak X ölçeği uzaklığına göre bir indeks seçer.
    pub fn en_yakın_null_olmayan_indeks(
        &self,
        yatay_oran: f64,
        seri_indeksi: usize,
        yön: NullAtlamaYönü,
    ) -> Option<usize> {
        if !yatay_oran.is_finite() {
            return None;
        }
        let seri = self.veri.seriler().get(seri_indeksi)?;
        let aralık = self.görünür_x_aralığı();
        let hedef = self.x_değeri_orandan(aralık, yatay_oran.clamp(0.0, 1.0));
        en_yakın_dolu_x_indeksi(self.veri.x(), seri, aralık, hedef, yön)
    }

    /// En yakın ortak X indeksini ve o indeksteki tüm seri değerlerini döndürür.
    /// uPlot'un hizalı cursor/legend davranışının çekirdek karşılığıdır.
    pub fn en_yakın_noktalar(&self, yatay_oran: f64) -> Option<(f64, Vec<Option<f64>>)> {
        if !yatay_oran.is_finite() {
            return None;
        }
        let aralık = self.görünür_x_aralığı();
        let hedef = self.x_değeri_orandan(aralık, yatay_oran.clamp(0.0, 1.0));
        let indeks = en_yakın_x_indeksi(self.veri.x(), aralık, hedef)?;
        let x = self.veri.x().get(indeks).copied()?;
        let değerler = self
            .veri
            .seriler()
            .iter()
            .zip(self.seçenekler.seriler.iter())
            .map(|(seri, seçenek)| {
                if !seçenek.göster {
                    return None;
                }
                seçenek
                    .lejant_değerleri
                    .as_ref()
                    .filter(|değerler| değerler.len() == seri.len())
                    .unwrap_or(seri)
                    .get(indeks)
                    .copied()
                    .flatten()
                    .map(|değer| değer * seçenek.gösterim_değer_çarpanı)
            })
            .collect();
        Some((x, değerler))
    }

    /// Cursor politikasını çizim alanının gerçek CSS piksel genişliğinde
    /// değerlendirir. Böylece proximity eşikleri zoom ve resize sonrasında da
    /// uPlot ile aynı ekran uzaklığını ifade eder.
    pub fn imleç_çözümü(
        &self,
        yatay_oran: f64,
        çizim_genişliği: f64,
    ) -> Option<İmleçÇözümü> {
        if !yatay_oran.is_finite() || !çizim_genişliği.is_finite() || çizim_genişliği <= 0.0 {
            return None;
        }
        let aralık = self.görünür_x_aralığı();
        let hedef = self.x_değeri_orandan(aralık, yatay_oran.clamp(0.0, 1.0));
        let ortak_indeks = en_yakın_x_indeksi(self.veri.x(), aralık, hedef)?;
        let ortak_x = *self.veri.x().get(ortak_indeks)?;
        let düzen = self.seçenekler.null_imleç_düzeni;
        let x_aralığı = (aralık.en_çok - aralık.en_az).abs().max(f64::EPSILON);
        let piksel_uzaklığı = |indeks: usize| {
            self.veri.x().get(indeks).map_or(f64::INFINITY, |x| {
                (x - hedef).abs() / x_aralığı * çizim_genişliği
            })
        };

        let seriler = self
            .veri
            .seriler()
            .iter()
            .enumerate()
            .map(|(seri_indeksi, seri)| {
                if !self
                    .seçenekler
                    .seriler
                    .get(seri_indeksi)
                    .is_some_and(|seçenek| seçenek.göster)
                {
                    return None;
                }
                let ortak_dolu = seri.get(ortak_indeks).copied().flatten().is_some();
                let hizalama_eksiği = self.veri.hizalama_eksiği_mi(seri_indeksi, ortak_indeks);
                let aday = match düzen {
                    NullİmleçDüzeni::Ortak => Some(ortak_indeks),
                    NullİmleçDüzeni::EnYakınX if ortak_dolu => Some(ortak_indeks),
                    NullİmleçDüzeni::EnYakınX => en_yakın_dolu_x_indeksi(
                        self.veri.x(),
                        seri,
                        aralık,
                        hedef,
                        NullAtlamaYönü::EnYakın,
                    ),
                    NullİmleçDüzeni::PikselYakınlığı { piksel } => {
                        let aday = en_yakın_dolu_x_indeksi(
                            self.veri.x(),
                            seri,
                            aralık,
                            hedef,
                            NullAtlamaYönü::EnYakın,
                        );
                        aday.filter(|indeks| piksel_uzaklığı(*indeks) <= piksel.max(0.0))
                    }
                    NullİmleçDüzeni::YalnızNullsaPiksel { piksel: _ } if ortak_dolu => {
                        Some(ortak_indeks)
                    }
                    NullİmleçDüzeni::YalnızNullsaPiksel { piksel } => {
                        let aday = en_yakın_dolu_x_indeksi(
                            self.veri.x(),
                            seri,
                            aralık,
                            hedef,
                            NullAtlamaYönü::EnYakın,
                        );
                        if hizalama_eksiği {
                            aday
                        } else {
                            aday.filter(|indeks| piksel_uzaklığı(*indeks) <= piksel.max(0.0))
                        }
                    }
                    NullİmleçDüzeni::ÖncekiSeri | NullİmleçDüzeni::ÖncekiİmleçVeSeri => {
                        en_yakın_dolu_x_indeksi(
                            self.veri.x(),
                            seri,
                            aralık,
                            hedef,
                            NullAtlamaYönü::Önceki,
                        )
                    }
                }?;
                let x = *self.veri.x().get(aday)?;
                let değer = seri.get(aday).copied().flatten()?;
                Some(İmleçSeriÖrneği {
                    indeks: aday,
                    x,
                    değer,
                    hizalama_eksiğinden_atlandı: hizalama_eksiği && aday != ortak_indeks,
                })
            })
            .collect::<Vec<_>>();

        let imleç_x = if düzen == NullİmleçDüzeni::ÖncekiİmleçVeSeri {
            seriler
                .iter()
                .flatten()
                .next()
                .map_or(hedef, |örnek| örnek.x)
        } else {
            hedef
        };
        Some(İmleçÇözümü {
            imleç_x,
            ortak_x,
            seriler,
        })
    }

    pub fn imleç_y_görünür(&self) -> bool {
        self.seçenekler.imleç_y_görünür
    }

    pub fn imleç_noktaları_görünür(&self) -> bool {
        self.seçenekler.imleç_noktaları_görünür
    }

    /// Dolu imleç noktalarının görünürlüğünü veri ve ölçekleri yeniden
    /// kurmadan değiştirir.
    pub fn imleç_noktalarını_göster(&mut self, görünür: bool) -> bool {
        if self.seçenekler.imleç_noktaları_görünür == görünür {
            return false;
        }
        self.seçenekler.imleç_noktaları_görünür = görünür;
        true
    }

    pub fn ham_seri_değeri(&self, seri: usize, indeks: usize) -> Option<f64> {
        self.veri
            .seriler()
            .get(seri)
            .and_then(|değerler| değerler.get(indeks))
            .copied()
            .flatten()
    }

    pub fn en_yakın_tooltip(
        &self,
        yatay_oran: f64,
        seri: usize,
    ) -> Option<crate::EnYakınTooltipBilgisi> {
        let düzen = self.seçenekler.en_yakın_tooltip.as_ref()?;
        let aralık = self.görünür_x_aralığı();
        let hedef = self.x_değeri_orandan(aralık, yatay_oran.clamp(0.0, 1.0));
        let indeks = en_yakın_x_indeksi(self.veri.x(), aralık, hedef)?;
        let zaman = self.veri.x().get(indeks).copied()?;
        let değer = self.ham_seri_değeri(seri, indeks)?;
        let başlangıç = self
            .ham_seri_değeri(seri, 0)
            .filter(|değer| değer.abs() > f64::EPSILON)?;
        let commit = düzen.commitler.get(indeks)?.clone();
        let önceki_commit = indeks
            .checked_sub(1)
            .and_then(|önceki| düzen.commitler.get(önceki))
            .cloned();
        let interpolasyon = düzen.interpolasyonlar.contains(&indeks);
        let kenarlık_rengi = if interpolasyon {
            düzen.interpolasyon_rengi.clone()
        } else {
            self.seçenekler
                .seriler
                .get(seri)
                .map_or_else(|| "#000000".to_string(), |seri| seri.renk.clone())
        };
        let başlangıç_parametresi = önceki_commit.as_deref().unwrap_or("null");
        let karşılaştırma_url = format!(
            "https://perf.rust-lang.org/compare.html?start={başlangıç_parametresi}&end={commit}&stat={}",
            düzen.stat
        );
        let yüzde = (değer - başlangıç) / başlangıç * 100.0;
        let tarih = crate::zaman::tooltip_tarihi(zaman).unwrap_or_else(|| zaman.to_string());
        let kısa_commit = commit.get(..10).unwrap_or(commit.as_str()).to_string();
        Some(crate::EnYakınTooltipBilgisi {
            zaman,
            commit,
            önceki_commit,
            seri,
            değer,
            başlangıçtan_yüzde: yüzde,
            interpolasyon,
            kenarlık_rengi,
            karşılaştırma_url,
            metin: format!(
                "{tarih} - {kısa_commit}\n{} ({yüzde:.2}% since start)",
                tooltip_sayısını_biçimlendir(değer)
            ),
        })
    }

    pub fn en_yakın_tooltip_etkin(&self) -> bool {
        self.seçenekler.en_yakın_tooltip.is_some()
    }

    pub fn tooltip_bilgileri(
        &self,
        yatay_oran: f64,
        dikey_oran: f64,
    ) -> Vec<crate::TooltipBilgisi> {
        let Some(düzen) = self.seçenekler.tooltip else {
            return Vec::new();
        };
        if !yatay_oran.is_finite() || !dikey_oran.is_finite() {
            return Vec::new();
        }
        let yatay_oran = yatay_oran.clamp(0.0, 1.0);
        let dikey_oran = dikey_oran.clamp(0.0, 1.0);
        let mut bilgiler = Vec::with_capacity(self.seçenekler.seriler.len().saturating_add(1));
        if düzen.imleç_değeri {
            let x = self.x_değeri_orandan(self.görünür_x_aralığı(), yatay_oran);
            let y = self.y_değeri_orandan(self.görünür_y_aralığı(), 1.0 - dikey_oran);
            bilgiler.push(crate::TooltipBilgisi {
                seri: None,
                metin: format!("({x:.2}, {y:.2})"),
                yatay_oran,
                dikey_oran,
                arka_plan_rengi: "#0000ff1a".to_string(),
                metin_rengi: "#111111".to_string(),
            });
        }
        if !düzen.seri_değerleri {
            return bilgiler;
        }
        let Some((x, değerler)) = self.en_yakın_noktalar(yatay_oran) else {
            return bilgiler;
        };
        let Some(x_oranı) = self.x_konum_oranı(x) else {
            return bilgiler;
        };
        for (seri, değer) in değerler.into_iter().enumerate() {
            let Some(seçenek) = self.seçenekler.seriler.get(seri) else {
                continue;
            };
            if !seçenek.göster {
                continue;
            }
            let Some(y) = değer else { continue };
            let Some(y_oranı) = self.seri_y_konum_oranı(seri, y) else {
                continue;
            };
            bilgiler.push(crate::TooltipBilgisi {
                seri: Some(seri),
                metin: format!("({x}, {y})"),
                yatay_oran: x_oranı,
                dikey_oran: 1.0 - y_oranı,
                arka_plan_rengi: "#0000001a".to_string(),
                metin_rengi: seçenek.renk.clone(),
            });
        }
        bilgiler
    }

    pub fn tooltip_düzeni(&self) -> Option<crate::TooltipDüzeni> {
        self.seçenekler.tooltip
    }

    pub fn lejant_canlı(&self) -> bool {
        self.seçenekler.lejant_canlı
    }

    /// Canlı akış ve özel formatter'ların kullanabildiği son hizalı veri
    /// satırını döndürür.
    pub fn son_değerler(&self) -> Option<(f64, Vec<Option<f64>>)> {
        let indeks = self.veri.uzunluk().checked_sub(1)?;
        let x = self.veri.x().get(indeks).copied()?;
        let değerler = self
            .veri
            .seriler()
            .iter()
            .zip(self.seçenekler.seriler.iter())
            .map(|(seri, seçenek)| {
                seçenek
                    .lejant_değerleri
                    .as_ref()
                    .filter(|değerler| değerler.len() == seri.len())
                    .unwrap_or(seri)
                    .get(indeks)
                    .copied()
                    .flatten()
                    .map(|değer| değer * seçenek.gösterim_değer_çarpanı)
            })
            .collect();
        Some((x, değerler))
    }

    /// İmleç grafik dışında olduğunda yalnız özel `series.value`
    /// callback'i bulunan serilerin son değerlerini döndürür. X serisi
    /// uPlot varsayılanı gibi boş kalır.
    pub fn boşta_lejant_değerleri(&self) -> Option<Vec<Option<f64>>> {
        let indeks = self.veri.uzunluk().checked_sub(1)?;
        let mut özel_değer_var = false;
        let değerler = self
            .veri
            .seriler()
            .iter()
            .zip(self.seçenekler.seriler.iter())
            .map(|(seri, seçenek)| {
                if !seçenek.boşta_son_değeri_göster {
                    return None;
                }
                özel_değer_var = true;
                seçenek
                    .lejant_değerleri
                    .as_ref()
                    .filter(|değerler| değerler.len() == seri.len())
                    .unwrap_or(seri)
                    .get(indeks)
                    .copied()
                    .flatten()
                    .map(|değer| değer * seçenek.gösterim_değer_çarpanı)
            })
            .collect::<Vec<_>>();
        özel_değer_var.then_some(değerler)
    }

    /// Kaynak `series.values` tarih eşlemesini yüzey adaptörlerine taşır.
    pub fn seri_zamanı(&self, seri_indeksi: usize, x: f64) -> Option<f64> {
        x.is_finite().then(|| {
            self.seçenekler
                .seriler
                .get(seri_indeksi)
                .map_or(x, |seri| x + seri.x_zaman_kaydırması)
        })
    }

    /// İmleç noktasının rengini seri gradyanının geçerli ölçek duraklarına göre çözer.
    pub fn seri_imleç_rengi(
        &self,
        seri_indeksi: usize,
        x_değeri: f64,
        y_değeri: f64,
    ) -> Option<String> {
        let seri = self.seçenekler.seriler.get(seri_indeksi)?;
        let gradyan = seri.çizgi_gradyanı.as_ref()?;
        let x_aralığı = self.görünür_x_aralığı();
        let y_aralığı =
            self.görünür_ölçek_aralığı(&seri.ölçek, x_aralığı, self.etkileşim.görünür_y());
        let duraklar =
            self.gradyan_değerlerini_çöz(gradyan, &seri.ölçek, x_aralığı, y_aralığı)?;
        let değer = match gradyan.eksen {
            GradyanEkseni::X => x_değeri,
            GradyanEkseni::Y => y_değeri,
        };
        duraklar
            .iter()
            .rev()
            .find(|(durak, _)| değer >= *durak)
            .or_else(|| duraklar.first())
            .map(|(_, renk)| renk.clone())
    }

    /// `cursor.focus` eşdeğeri: en yakın X örneğindeki Y mesafesine göre
    /// odaklanan seriyi çekirdekte seçer. `true`, sahnenin yeniden çizilmesi
    /// gerektiğini bildirir.
    pub fn imleç_odağını_güncelle(
        &mut self,
        yatay_oran: f64,
        dikey_oran: f64,
        çizim_boyutu: f64,
    ) -> bool {
        let Some(düzen) = self.seçenekler.odak else {
            return false;
        };
        if düzen.yakınlık < 0.0 || !çizim_boyutu.is_finite() || çizim_boyutu <= 0.0 {
            return self.odağı_ayarla(None);
        }
        let x_oranı = if self.seçenekler.x_dikey {
            1.0 - dikey_oran
        } else {
            yatay_oran
        };
        let Some((_, değerler)) = self.en_yakın_noktalar(x_oranı) else {
            return self.odağı_ayarla(None);
        };
        let x_aralığı = self.görünür_x_aralığı();
        let fare_y = if self.seçenekler.x_dikey {
            yatay_oran.clamp(0.0, 1.0) * çizim_boyutu
        } else {
            dikey_oran.clamp(0.0, 1.0) * çizim_boyutu
        };
        let fare_değeri =
            self.y_değeri_orandan(self.görünür_y_aralığı(), 1.0 - dikey_oran.clamp(0.0, 1.0));
        let mut en_yakın = None;
        let mut en_kısa = f64::INFINITY;
        for (indeks, değer) in değerler.into_iter().enumerate() {
            let Some(değer) = değer else { continue };
            let Some(seri) = self.seçenekler.seriler.get(indeks) else {
                continue;
            };
            if !seri.göster {
                continue;
            }
            let aralık = self.görünür_ölçek_aralığı(&seri.ölçek, x_aralığı, None);
            if düzen.yön_eğilimi != 0 {
                let aynı_işaret = değer.is_sign_negative() == fare_değeri.is_sign_negative();
                let uygun = if fare_değeri.is_sign_negative() {
                    if düzen.yön_eğilimi == 1 {
                        değer <= fare_değeri
                    } else {
                        değer >= fare_değeri
                    }
                } else if düzen.yön_eğilimi == 1 {
                    değer >= fare_değeri
                } else {
                    değer <= fare_değeri
                };
                if !aynı_işaret || !uygun {
                    continue;
                }
            }
            let ölçek_konumu =
                f64::from(self.y_konumu(&seri.ölçek, aralık, değer, 0.0, çizim_boyutu as f32));
            let konum = if self.seçenekler.x_dikey {
                ölçek_konumu
            } else {
                çizim_boyutu - ölçek_konumu
            };
            let mesafe = (konum - fare_y).abs();
            if mesafe < en_kısa {
                en_kısa = mesafe;
                en_yakın = Some(indeks);
            }
        }
        self.odağı_ayarla(
            (en_kısa <= f64::from(düzen.yakınlık))
                .then_some(en_yakın)
                .flatten(),
        )
    }

    pub fn imleç_odağını_temizle(&mut self) -> bool {
        self.odağı_ayarla(None)
    }

    pub fn imleç_odağını_seriye_ayarla(&mut self, seri: Option<usize>) -> bool {
        let geçerli = seri.filter(|indeks| *indeks < self.seçenekler.seriler.len());
        self.odağı_ayarla(geçerli)
    }

    pub const fn odak_serisi(&self) -> Option<usize> {
        self.odak_serisi
    }

    /// Güncel `cursor.focus` durumundan sonra serinin yalnız boya sunumunu
    /// döndürür. Yüzey adaptörleri retained geometriyi yeniden kurmadan
    /// stroke/fill/width özelliklerini yerinde güncelleyebilir.
    pub fn seri_odak_sunumu(&self, seri_indeksi: usize) -> Option<(String, Option<String>, f32)> {
        let seri = self.seçenekler.seriler.get(seri_indeksi)?;
        Some(odaklı_seri_stili(
            seri,
            self.seçenekler.odak,
            self.odak_serisi,
            seri_indeksi,
        ))
    }

    fn odağı_ayarla(&mut self, seri: Option<usize>) -> bool {
        if self.odak_serisi == seri {
            return false;
        }
        self.odak_serisi = seri;
        true
    }

    /// Grafiği belirli bir görünür X aralığında çizer.
    pub fn çiz_aralıkta(&self, görünür_x: Option<Aralık>) -> Sahne {
        self.çiz_boyutta(
            self.seçenekler.genişlik,
            self.seçenekler.yükseklik,
            görünür_x,
        )
    }

    /// Etkileşim denetleyicisindeki güncel görünümü hedef yüzey boyutunda çizer.
    pub fn çiz_görünür_boyutta(&self, genişlik_px: u32, yükseklik_px: u32) -> Sahne {
        let görünür = self
            .elle_x_aralığı
            .or_else(|| self.yakınlaştırılmış().then(|| self.görünür_x_aralığı()));
        self.çiz_boyutta_aralıklarla(
            genişlik_px,
            yükseklik_px,
            görünür,
            self.elle_y_aralıkları
                .get(&self.seçenekler.birincil_y_ölçeği)
                .copied()
                .or_else(|| self.etkileşim.görünür_y()),
            false,
        )
    }

    /// Resize demosundaki gibi hedef yüzey boyutuna göre yeniden yerleşim yapar.
    pub fn çiz_boyutta(
        &self,
        genişlik_px: u32,
        yükseklik_px: u32,
        görünür_x: Option<Aralık>,
    ) -> Sahne {
        self.çiz_boyutta_aralıklarla(genişlik_px, yükseklik_px, görünür_x, None, false)
    }

    fn çiz_boyutta_aralıklarla(
        &self,
        genişlik_px: u32,
        yükseklik_px: u32,
        görünür_x: Option<Aralık>,
        görünür_y: Option<Aralık>,
        yalnız_eksen: bool,
    ) -> Sahne {
        let çizim_başlangıcı = self
            .seçenekler
            .çizim_kancaları
            .as_ref()
            .is_some_and(|düzen| düzen.çizim_süresi_metni)
            .then(Instant::now);
        let genişlik_px = if self.seçenekler.kompakt_yüzey {
            genişlik_px.max(2)
        } else {
            genişlik_px.max(160)
        };
        let yükseklik_px = if self.seçenekler.kompakt_yüzey {
            yükseklik_px.max(2)
        } else {
            yükseklik_px.max(120)
        };
        let mut sahne = Sahne::yeni(genişlik_px, yükseklik_px);
        sahne.ekle(Komut::ArkaPlan {
            renk: self.seçenekler.arka_plan_rengi.clone(),
        });

        let (sol, sağ, üst, alt) = self.çizim_alanı_boyutta(genişlik_px, yükseklik_px);
        let genişlik = sağ - sol;
        let yükseklik = alt - üst;
        if let Some(renk) = &self.seçenekler.çizim_alanı_arka_plan_rengi {
            sahne.ekle(Komut::Dikdörtgen {
                konum: Nokta::yeni(sol, üst),
                genişlik,
                yükseklik,
                dolgu: renk.clone(),
                çizgi: "#00000000".to_string(),
                kalınlık: 0.0,
            });
        }

        if let Some(duraklar) = self
            .seçenekler
            .çizim_kancaları
            .as_ref()
            .and_then(|düzen| düzen.gradyan_durakları.as_ref())
        {
            let payda = duraklar.len().saturating_sub(1).max(1) as f32;
            sahne.ekle(Komut::GradyanAlan {
                çokgenler: vec![vec![
                    Nokta::yeni(sol, üst),
                    Nokta::yeni(sağ, üst),
                    Nokta::yeni(sağ, alt),
                    Nokta::yeni(sol, alt),
                ]],
                // Resmî eklenti gradyanı `bbox.top` yerine global y=0'dan
                // `bbox.height` değerine kurup yalnız bbox dikdörtgenini doldurur.
                gradyan: DoğrusalGradyan {
                    başlangıç: Nokta::yeni(0.0, 0.0),
                    bitiş: Nokta::yeni(0.0, yükseklik),
                    duraklar: duraklar
                        .iter()
                        .enumerate()
                        .map(|(indeks, renk)| GradyanRenkDurağı {
                            oran: indeks as f32 / payda,
                            renk: renk.clone(),
                        })
                        .collect(),
                },
            });
        }

        if !self.seçenekler.başlık.is_empty() {
            sahne.ekle(Komut::Metin {
                konum: Nokta::yeni(genişlik_px as f32 / 2.0, 26.0),
                içerik: self.seçenekler.başlık.clone(),
                renk: self.seçenekler.başlık_rengi.clone(),
                boyut: 18.0,
                hiza: MetinHizası::Orta,
            });
        }

        // uPlot'un ölçek `range()` sonucu `[null, null]` olan boş veri
        // yüzeyi yalnız başlığı taşır. Özel X/Y aralığı tanımlanan boş
        // yüzeyler ise normal eksen ve ızgara çiziminden geçer.
        if self.veri.x().is_empty()
            && self.seçenekler.x_aralığı.is_none()
            && self.seçenekler.boş_x_aralığı.is_none()
            && self.seçenekler.y_aralığı.is_none()
            && self.seçenekler.boş_y_aralığı.is_none()
        {
            return sahne;
        }

        if yalnız_eksen
            && (self.seçenekler.çubuk_düzeni.is_some()
                || self.seçenekler.kutu_bıyık_düzeni.is_some()
                || self.seçenekler.mum_düzeni.is_some())
        {
            return sahne;
        }
        if let Some(düzen) = self.seçenekler.çubuk_düzeni {
            self.çubukları_çiz(
                &mut sahne,
                genişlik_px,
                yükseklik_px,
                düzen,
                görünür_x,
                görünür_y,
            );
            return sahne;
        }
        if let Some(düzen) = &self.seçenekler.kutu_bıyık_düzeni {
            self.kutu_bıyıkları_çiz(
                &mut sahne,
                genişlik_px,
                yükseklik_px,
                düzen,
                görünür_x,
                görünür_y,
            );
            return sahne;
        }
        if let Some(düzen) = &self.seçenekler.mum_düzeni {
            self.mumları_çiz(
                &mut sahne,
                genişlik_px,
                yükseklik_px,
                düzen,
                görünür_x,
                görünür_y,
            );
            return sahne;
        }

        let tam_x_aralığı = self
            .seçenekler
            .x_aralığı
            .or_else(|| tam_x_aralığı(&self.veri).ok())
            .unwrap_or(Aralık {
                en_az: 0.0,
                en_çok: 1.0,
            });
        let x_aralığı = görünür_x
            .and_then(|aralık| {
                if self.elle_x_aralığı == Some(aralık) {
                    return Some(aralık);
                }
                Aralık::yeni(
                    aralık.en_az.max(tam_x_aralığı.en_az),
                    aralık.en_çok.min(tam_x_aralığı.en_çok),
                )
                .ok()
            })
            .unwrap_or(tam_x_aralığı);
        let (y_aralığı, güzel_y_artımı) = görünür_y.map_or_else(
            || {
                self.güzel_ölçek_aralığı(
                    &self.seçenekler.birincil_y_ölçeği,
                    x_aralığı,
                    yükseklik,
                )
                .map_or_else(
                    || (self.y_aralığı(x_aralığı), None),
                    |(aralık, artım)| (aralık, Some(artım)),
                )
            },
            |aralık| (aralık, None),
        );
        let birincil_ölçek = self.ölçek_seçeneği(&self.seçenekler.birincil_y_ölçeği);
        let birincil_birim = birincil_ölçek.map_or("", |ölçek| ölçek.birim.as_str());
        let birincil_dağılım = birincil_ölçek.map(|ölçek| ölçek.dağılım);
        let birincil_biçim =
            birincil_ölçek.map_or(YÖlçekEtiketBiçimi::Otomatik, |ölçek| ölçek.etiket_biçimi);
        let birincil_çarpan = birincil_ölçek.map_or(1.0, |ölçek| ölçek.eksen_değer_çarpanı);
        let birincil_etiket_boşluğu =
            birincil_ölçek.map_or(30.0, |ölçek| ölçek.eksen_en_az_etiket_boşluğu);

        let eksen_komutları_başlangıcı = sahne.komutlar().len();
        sahne.katmanı_ayarla(SahneKatmanı::Eksen);
        let y_boyutu = if self.seçenekler.x_dikey {
            genişlik
        } else {
            yükseklik
        };
        let y_artımı = güzel_y_artımı.unwrap_or_else(|| uygun_artım(y_aralığı, y_boyutu, 30.0));
        let y_bölmeleri = self
            .seçenekler
            .birincil_y_sabit_bölmeler
            .clone()
            .unwrap_or_else(|| {
                güzel_y_artımı.map_or_else(
                    || {
                        self.y_eksen_bölmeleri(
                            &self.seçenekler.birincil_y_ölçeği,
                            y_aralığı,
                            y_boyutu,
                        )
                    },
                    |artım| eksen_bölmeleri_artımla(y_aralığı, artım),
                )
            })
            .into_iter()
            .filter(|değer| *değer >= y_aralığı.en_az && *değer <= y_aralığı.en_çok)
            .collect::<Vec<_>>();
        if self.seçenekler.eksen_göstergeleri
            && self.seçenekler.birincil_y_eksen_görünür
            && !self.seçenekler.x_dikey
        {
            let x = if self.seçenekler.birincil_y_karşıda {
                sağ
            } else {
                sol
            };
            sahne.ekle(Komut::Çizgi {
                başlangıç: Nokta::yeni(x, üst),
                bitiş: Nokta::yeni(x, alt),
                renk: self.seçenekler.birincil_y_eksen_rengi.clone(),
                kalınlık: 1.0,
            });
        }
        for y_değeri in y_bölmeleri {
            if self.seçenekler.x_dikey {
                let x = piksele_hizala(
                    sol + self.y_konumu(
                        &self.seçenekler.birincil_y_ölçeği,
                        y_aralığı,
                        y_değeri,
                        0.0,
                        genişlik,
                    ),
                    self.seçenekler.piksel_hizası,
                    self.cihaz_piksel_oranı,
                );
                if self.seçenekler.birincil_y_ızgara_görünür {
                    self.birincil_y_ızgara_çizgisini_ekle(
                        &mut sahne,
                        Nokta::yeni(x, üst),
                        Nokta::yeni(x, alt),
                    );
                }
                if self.seçenekler.birincil_y_eksen_görünür
                    && log_etiketi_göster(
                        y_değeri,
                        y_aralığı,
                        genişlik,
                        birincil_dağılım,
                        birincil_biçim,
                        birincil_etiket_boşluğu,
                    )
                {
                    sahne.ekle(Komut::Metin {
                        konum: Nokta::yeni(
                            x,
                            if self.seçenekler.birincil_y_karşıda {
                                alt + 20.0
                            } else {
                                üst - 8.0
                            },
                        ),
                        içerik: self.birincil_y_etiketi(
                            y_değeri,
                            y_artımı,
                            birincil_birim,
                            birincil_dağılım,
                            birincil_biçim,
                            birincil_çarpan,
                        ),
                        renk: self.seçenekler.birincil_y_eksen_rengi.clone(),
                        boyut: 11.0,
                        hiza: MetinHizası::Orta,
                    });
                }
                continue;
            }
            let y = piksele_hizala(
                üst + yükseklik
                    - self.y_konumu(
                        &self.seçenekler.birincil_y_ölçeği,
                        y_aralığı,
                        y_değeri,
                        0.0,
                        yükseklik,
                    ),
                self.seçenekler.piksel_hizası,
                self.cihaz_piksel_oranı,
            );
            if self.seçenekler.birincil_y_ızgara_görünür {
                self.birincil_y_ızgara_çizgisini_ekle(
                    &mut sahne,
                    Nokta::yeni(sol, y),
                    Nokta::yeni(sağ, y),
                );
            }
            if self.seçenekler.eksen_göstergeleri && self.seçenekler.birincil_y_eksen_görünür
            {
                let çentik = self.seçenekler.birincil_y_eksen_çentik_uzunluğu;
                let (başlangıç_x, bitiş_x) = if self.seçenekler.birincil_y_karşıda {
                    (sağ, sağ + çentik)
                } else {
                    (sol - çentik, sol)
                };
                sahne.ekle(Komut::Çizgi {
                    başlangıç: Nokta::yeni(başlangıç_x, y),
                    bitiş: Nokta::yeni(bitiş_x, y),
                    renk: self
                        .seçenekler
                        .birincil_y_eksen_çentik_rengi
                        .clone()
                        .unwrap_or_else(|| self.seçenekler.birincil_y_eksen_rengi.clone()),
                    kalınlık: 1.0,
                });
            }
            if self.seçenekler.birincil_y_eksen_görünür
                && log_etiketi_göster(
                    y_değeri,
                    y_aralığı,
                    yükseklik,
                    birincil_dağılım,
                    birincil_biçim,
                    birincil_etiket_boşluğu,
                )
            {
                sahne.ekle(Komut::Metin {
                    konum: Nokta::yeni(
                        if self.seçenekler.birincil_y_karşıda {
                            sağ + 8.0
                        } else {
                            sol - 8.0
                        },
                        y + 4.0,
                    ),
                    içerik: self.birincil_y_etiketi(
                        y_değeri,
                        y_artımı,
                        birincil_birim,
                        birincil_dağılım,
                        birincil_biçim,
                        birincil_çarpan,
                    ),
                    renk: self.seçenekler.birincil_y_eksen_rengi.clone(),
                    boyut: 11.0,
                    hiza: if self.seçenekler.birincil_y_karşıda {
                        MetinHizası::Başlangıç
                    } else {
                        MetinHizası::Bitiş
                    },
                });
            }
        }

        if !self.seçenekler.y_eksen_etiketi.is_empty() {
            if self.seçenekler.x_dikey {
                sahne.ekle(Komut::Metin {
                    konum: Nokta::yeni(
                        (sol + sağ) / 2.0,
                        if self.seçenekler.birincil_y_karşıda {
                            alt + 40.0
                        } else {
                            üst - 12.0
                        },
                    ),
                    içerik: self.seçenekler.y_eksen_etiketi.clone(),
                    renk: self.seçenekler.birincil_y_eksen_rengi.clone(),
                    boyut: 12.0,
                    hiza: MetinHizası::Orta,
                });
            } else {
                let eksen_etiketi_x = if self.seçenekler.birincil_y_karşıda {
                    (sağ + genişlik_px as f32) / 2.0
                } else {
                    sol / 2.0
                };
                sahne.ekle(Komut::DöndürülmüşMetin {
                    konum: Nokta::yeni(eksen_etiketi_x, (üst + alt) / 2.0),
                    içerik: self.seçenekler.y_eksen_etiketi.clone(),
                    renk: self.seçenekler.birincil_y_eksen_rengi.clone(),
                    boyut: 12.0,
                    hiza: MetinHizası::Orta,
                    açı: -90.0,
                });
            }
        }

        let birincil_eksen_dilimi = self.seçenekler.birincil_y_eksen_genişliği.unwrap_or(56.0);
        let mut sol_ikincil_ofset =
            if self.seçenekler.birincil_y_eksen_görünür && !self.seçenekler.birincil_y_sağda {
                birincil_eksen_dilimi
            } else {
                0.0
            };
        let mut sağ_ikincil_ofset =
            if self.seçenekler.birincil_y_eksen_görünür && self.seçenekler.birincil_y_sağda {
                birincil_eksen_dilimi
            } else {
                0.0
            };
        for ölçek in self.seçenekler.y_ölçekleri.iter().filter(|ölçek| {
            ölçek.anahtar != self.seçenekler.birincil_y_ölçeği
                && (ölçek.sağda || ölçek.eksen_görünür)
        }) {
            let eksen_x = if ölçek.sağda {
                let x = sağ + 8.0 + sağ_ikincil_ofset;
                sağ_ikincil_ofset += ölçek.eksen_genişliği;
                x
            } else {
                let x = sol - 8.0 - sol_ikincil_ofset;
                sol_ikincil_ofset += ölçek.eksen_genişliği;
                x
            };
            let eksen_sınırı_x = if ölçek.sağda {
                eksen_x - 8.0
            } else {
                eksen_x + 8.0
            };
            if self.seçenekler.eksen_göstergeleri && ölçek.eksen_görünür {
                sahne.ekle(Komut::Çizgi {
                    başlangıç: Nokta::yeni(eksen_sınırı_x, üst),
                    bitiş: Nokta::yeni(eksen_sınırı_x, alt),
                    renk: ölçek.eksen_rengi.clone(),
                    kalınlık: 1.0,
                });
            }
            let aralık = self.görünür_ölçek_aralığı(&ölçek.anahtar, x_aralığı, görünür_y);
            let artım = uygun_artım(aralık, yükseklik, 30.0);
            for değer in self.y_eksen_bölmeleri(&ölçek.anahtar, aralık, yükseklik) {
                let y = piksele_hizala(
                    alt - self.y_konumu(&ölçek.anahtar, aralık, değer, 0.0, yükseklik),
                    self.seçenekler.piksel_hizası,
                    self.cihaz_piksel_oranı,
                );
                if ölçek.ızgara {
                    sahne.ekle(Komut::Çizgi {
                        başlangıç: Nokta::yeni(sol, y),
                        bitiş: Nokta::yeni(sağ, y),
                        renk: self.seçenekler.ızgara_rengi.clone(),
                        kalınlık: 1.0,
                    });
                }
                if self.seçenekler.eksen_göstergeleri && ölçek.eksen_görünür {
                    let (başlangıç_x, bitiş_x) = if ölçek.sağda {
                        (eksen_sınırı_x, eksen_sınırı_x + 5.0)
                    } else {
                        (eksen_sınırı_x - 5.0, eksen_sınırı_x)
                    };
                    sahne.ekle(Komut::Çizgi {
                        başlangıç: Nokta::yeni(başlangıç_x, y),
                        bitiş: Nokta::yeni(bitiş_x, y),
                        renk: ölçek.eksen_rengi.clone(),
                        kalınlık: 1.0,
                    });
                }
                if log_etiketi_göster(
                    değer,
                    aralık,
                    yükseklik,
                    Some(ölçek.dağılım),
                    ölçek.etiket_biçimi,
                    ölçek.eksen_en_az_etiket_boşluğu,
                ) {
                    sahne.ekle(Komut::Metin {
                        konum: Nokta::yeni(eksen_x, y + 4.0),
                        içerik: ölçek_eksen_değerini_yaz(
                            değer * ölçek.eksen_değer_çarpanı,
                            artım,
                            &ölçek.birim,
                            Some(ölçek.dağılım),
                            ölçek.etiket_biçimi,
                        ),
                        renk: ölçek.eksen_rengi.clone(),
                        boyut: 11.0,
                        hiza: if ölçek.sağda {
                            MetinHizası::Başlangıç
                        } else {
                            MetinHizası::Bitiş
                        },
                    });
                }
            }
            if !ölçek.eksen_etiketi.is_empty() {
                sahne.ekle(Komut::Metin {
                    konum: Nokta::yeni(if ölçek.sağda { sağ } else { sol }, üst - 12.0),
                    içerik: ölçek.eksen_etiketi.clone(),
                    renk: ölçek.eksen_rengi.clone(),
                    boyut: 12.0,
                    hiza: if ölçek.sağda {
                        MetinHizası::Bitiş
                    } else {
                        MetinHizası::Başlangıç
                    },
                });
            }
        }

        let x_boyutu = if self.seçenekler.x_dikey {
            yükseklik
        } else {
            genişlik
        };
        let x_etiket_boşluğu = self.seçenekler.x_eksen_asgari_etiket_boşluğu;
        let (x_bölmeleri, x_artımı) = match self.seçenekler.x_dağılımı {
            XÖlçekDağılımı::Logaritmik { taban } => (
                logaritmik_bölmeler(x_aralığı, taban),
                uygun_artım(x_aralığı, x_boyutu, x_etiket_boşluğu),
            ),
            XÖlçekDağılımı::Doğrusal if self.seçenekler.x_zaman => zaman_bölmeleri(
                x_aralığı,
                x_boyutu,
                x_etiket_boşluğu,
                self.seçenekler.x_zaman_milisaniye,
                self.seçenekler.x_zaman_dilimi,
                self.seçenekler.x_zaman_sabit_artımı,
            ),
            XÖlçekDağılımı::Doğrusal => (
                eksen_bölmeleri(x_aralığı, x_boyutu, x_etiket_boşluğu),
                uygun_artım(x_aralığı, x_boyutu, x_etiket_boşluğu),
            ),
        };
        let mut x_zaman_etiket_durumu = crate::zaman::ZamanEtiketDurumu::default();
        let mut ikincil_x_zaman_etiket_durumu = crate::zaman::ZamanEtiketDurumu::default();
        if self.seçenekler.eksen_göstergeleri
            && self.seçenekler.x_eksen_görünür
            && !self.seçenekler.x_dikey
        {
            let y = if self.seçenekler.x_eksen_karşıda {
                üst
            } else {
                alt
            };
            sahne.ekle(Komut::Çizgi {
                başlangıç: Nokta::yeni(sol, y),
                bitiş: Nokta::yeni(sağ, y),
                renk: self.seçenekler.x_eksen_rengi.clone(),
                kalınlık: 1.0,
            });
        }
        for x_değeri in x_bölmeleri {
            let (etiket_konumu, etiket_hizası) = if self.seçenekler.x_dikey {
                let y = piksele_hizala(
                    alt - self.x_konumu(x_aralığı, x_değeri, 0.0, yükseklik),
                    self.seçenekler.piksel_hizası,
                    self.cihaz_piksel_oranı,
                );
                if self.seçenekler.x_ızgara_görünür {
                    sahne.ekle(Komut::Çizgi {
                        başlangıç: Nokta::yeni(sol, y),
                        bitiş: Nokta::yeni(sağ, y),
                        renk: self.seçenekler.ızgara_rengi.clone(),
                        kalınlık: 1.0,
                    });
                }
                (
                    Nokta::yeni(
                        if self.seçenekler.x_eksen_karşıda {
                            sağ + 8.0
                        } else {
                            sol - 8.0
                        },
                        y + 4.0,
                    ),
                    if self.seçenekler.x_eksen_karşıda {
                        MetinHizası::Başlangıç
                    } else {
                        MetinHizası::Bitiş
                    },
                )
            } else {
                let x = piksele_hizala(
                    self.x_konumu(x_aralığı, x_değeri, sol, genişlik),
                    self.seçenekler.piksel_hizası,
                    self.cihaz_piksel_oranı,
                );
                if self.seçenekler.x_ızgara_görünür {
                    sahne.ekle(Komut::Çizgi {
                        başlangıç: Nokta::yeni(x, üst),
                        bitiş: Nokta::yeni(x, alt),
                        renk: self.seçenekler.ızgara_rengi.clone(),
                        kalınlık: 1.0,
                    });
                }
                if self.seçenekler.eksen_göstergeleri && self.seçenekler.x_eksen_görünür {
                    let çentik = self.seçenekler.x_eksen_çentik_uzunluğu;
                    let (başlangıç_y, bitiş_y) = if self.seçenekler.x_eksen_karşıda {
                        (üst - çentik, üst)
                    } else {
                        (alt, alt + çentik)
                    };
                    sahne.ekle(Komut::Çizgi {
                        başlangıç: Nokta::yeni(x, başlangıç_y),
                        bitiş: Nokta::yeni(x, bitiş_y),
                        renk: self
                            .seçenekler
                            .x_eksen_çentik_rengi
                            .clone()
                            .unwrap_or_else(|| self.seçenekler.x_eksen_rengi.clone()),
                        kalınlık: 1.0,
                    });
                }
                (
                    Nokta::yeni(
                        x,
                        if self.seçenekler.x_eksen_karşıda {
                            üst - 8.0
                        } else {
                            alt + 20.0
                        },
                    ),
                    MetinHizası::Orta,
                )
            };
            if self.seçenekler.x_eksen_görünür {
                sahne.ekle(Komut::Metin {
                    konum: etiket_konumu,
                    içerik: if self.seçenekler.x_zaman {
                        let birim = if self.seçenekler.x_zaman_milisaniye {
                            1_000.0
                        } else {
                            1.0
                        };
                        crate::zaman::yerel_eksen_etiketi(
                            x_değeri / birim,
                            x_artımı / birim,
                            &self.seçenekler.x_tarih_adları,
                            self.seçenekler.x_zaman_dilimi,
                            &mut x_zaman_etiket_durumu,
                        )
                        .unwrap_or_else(|| eksen_değerini_yaz(x_değeri, x_artımı))
                    } else {
                        let değer = x_değeri * self.seçenekler.x_eksen_değer_çarpanı;
                        let artım = x_artımı * self.seçenekler.x_eksen_değer_çarpanı;
                        match self.seçenekler.x_eksen_etiket_biçimi {
                            YÖlçekEtiketBiçimi::Otomatik => eksen_değerini_yaz(değer, artım),
                            biçim => ölçek_eksen_değerini_yaz(değer, artım, "", None, biçim),
                        }
                    },
                    renk: self.seçenekler.x_eksen_rengi.clone(),
                    boyut: 11.0,
                    hiza: etiket_hizası,
                });
            }
            if let Some(ikincil) = self
                .seçenekler
                .ikincil_x_eksen
                .as_ref()
                .filter(|_| self.seçenekler.x_zaman && !self.seçenekler.x_dikey)
            {
                let x = piksele_hizala(
                    self.x_konumu(x_aralığı, x_değeri, sol, genişlik),
                    self.seçenekler.piksel_hizası,
                    self.cihaz_piksel_oranı,
                );
                let birim = if self.seçenekler.x_zaman_milisaniye {
                    1_000.0
                } else {
                    1.0
                };
                let kaydırılmış = (x_değeri + ikincil.zaman_kaydırması) / birim;
                let etiket = crate::zaman::yerel_eksen_etiketi(
                    kaydırılmış,
                    x_artımı / birim,
                    &self.seçenekler.x_tarih_adları,
                    self.seçenekler.x_zaman_dilimi,
                    &mut ikincil_x_zaman_etiket_durumu,
                )
                .unwrap_or_else(|| eksen_değerini_yaz(kaydırılmış, x_artımı / birim));
                sahne.ekle(Komut::Metin {
                    konum: Nokta::yeni(x, alt + 38.0),
                    içerik: etiket,
                    renk: ikincil.renk.clone(),
                    boyut: 11.0,
                    hiza: MetinHizası::Orta,
                });
            }
        }

        if self.seçenekler.x_eksen_görünür && !self.seçenekler.x_eksen_etiketi.is_empty() {
            sahne.ekle(Komut::Metin {
                konum: if self.seçenekler.x_dikey {
                    Nokta::yeni(
                        if self.seçenekler.x_eksen_karşıda {
                            sağ
                        } else {
                            sol
                        },
                        üst - 12.0,
                    )
                } else {
                    Nokta::yeni(
                        (sol + sağ) / 2.0,
                        if self.seçenekler.x_eksen_karşıda {
                            üst - 28.0
                        } else {
                            alt + 42.0
                        },
                    )
                },
                içerik: self.seçenekler.x_eksen_etiketi.clone(),
                renk: self.seçenekler.x_eksen_rengi.clone(),
                boyut: 12.0,
                hiza: if self.seçenekler.x_dikey && self.seçenekler.x_eksen_karşıda {
                    MetinHizası::Bitiş
                } else if self.seçenekler.x_dikey {
                    MetinHizası::Başlangıç
                } else {
                    MetinHizası::Orta
                },
            });
        }
        let eksen_komutları_bitişi = sahne.komutlar().len();
        sahne.katmanı_ayarla(SahneKatmanı::Veri);
        if yalnız_eksen {
            return sahne;
        }

        if let Some(düzen) = &self.seçenekler.ısı_haritası_düzeni {
            self.ısı_haritasını_çiz(
                &mut sahne,
                düzen,
                x_aralığı,
                y_aralığı,
                sol,
                sağ,
                üst,
                alt,
            );
        }
        if let Some(düzen) = &self.seçenekler.timeline_düzeni {
            self.timeline_çiz(&mut sahne, düzen, x_aralığı, sol, sağ, üst, alt);
        }
        if let Some(düzen) = &self.seçenekler.dağılım_düzeni {
            for seri in &düzen.seriler {
                let seri_y_aralığı =
                    self.görünür_ölçek_aralığı(&seri.ölçek, x_aralığı, görünür_y);
                let ortak_yarıçap = seri.noktalar.first().map(|nokta| nokta.boyut / 2.0);
                let sabit_boyut = ortak_yarıçap.is_some_and(|yarıçap| {
                    seri.noktalar
                        .iter()
                        .all(|nokta| (nokta.boyut / 2.0 - yarıçap).abs() <= f32::EPSILON)
                });
                let mut toplu_merkezler = Vec::new();
                let mut değişken_daireler = Vec::new();
                for nokta in &seri.noktalar {
                    let merkez = Nokta::yeni(
                        self.x_konumu(x_aralığı, nokta.x, sol, genişlik),
                        alt - self.y_konumu(&seri.ölçek, seri_y_aralığı, nokta.y, 0.0, yükseklik),
                    );
                    let yarıçap = nokta.boyut / 2.0;
                    // Sabit scatter yolu kaynak gibi yalnız merkezi görünür
                    // değerleri alır. Bubble yolu ise en büyük yarıçap payıyla
                    // plot alanına giren daireleri tutup tek maskede kırpar.
                    let görünür = if sabit_boyut && !düzen.vuruş_etkin {
                        (x_aralığı.en_az..=x_aralığı.en_çok).contains(&nokta.x)
                            && (seri_y_aralığı.en_az..=seri_y_aralığı.en_çok).contains(&nokta.y)
                    } else {
                        merkez.x + yarıçap >= sol
                            && merkez.x - yarıçap <= sağ
                            && merkez.y + yarıçap >= üst
                            && merkez.y - yarıçap <= alt
                    };
                    if !görünür {
                        continue;
                    }
                    if sabit_boyut {
                        toplu_merkezler.push(merkez);
                    } else {
                        değişken_daireler.push((merkez, yarıçap));
                    }
                }
                if let Some(yarıçap) = ortak_yarıçap
                    && !toplu_merkezler.is_empty()
                {
                    sahne.ekle(Komut::Daireler {
                        merkezler: toplu_merkezler,
                        yarıçap,
                        dolgu: seri.dolgu.clone(),
                        çizgi: "#00000000".to_string(),
                        kalınlık: 0.0,
                        kesme_sınırları: Some((Nokta::yeni(sol, üst), Nokta::yeni(sağ, alt))),
                    });
                }
                if !değişken_daireler.is_empty() {
                    sahne.ekle(Komut::DeğişkenDaireler {
                        daireler: değişken_daireler,
                        dolgu: seri.dolgu.clone(),
                        çizgi: seri.renk.clone(),
                        kalınlık: 1.0,
                        kesme_sınırları: Some((Nokta::yeni(sol, üst), Nokta::yeni(sağ, alt))),
                    });
                }
            }
        }

        for (seri_indeksi, değerler) in self.veri.seriler().iter().enumerate() {
            let Some(seri) = self.seçenekler.seriler.get(seri_indeksi) else {
                continue;
            };
            if !seri.göster {
                continue;
            }
            let (seri_rengi, seri_dolgusu, seri_kalınlığı) =
                odaklı_seri_stili(seri, self.seçenekler.odak, self.odak_serisi, seri_indeksi);
            let seri_y_aralığı =
                self.görünür_ölçek_aralığı(&seri.ölçek, x_aralığı, görünür_y);
            self.seri_bantlarını_çiz(
                &mut sahne,
                seri_indeksi,
                x_aralığı,
                seri_y_aralığı,
                sol,
                sağ,
                üst,
                alt,
            );
            let bant_dolgusu = self
                .seçenekler
                .bantlar
                .iter()
                .any(|bant| bant.üst_seri == seri_indeksi);
            if seri.çizim_türü == crate::SeriÇizimTürü::Çubuk {
                if !bant_dolgusu {
                    self.karma_çubuk_serisini_çiz(
                        &mut sahne,
                        seri,
                        değerler,
                        x_aralığı,
                        seri_y_aralığı,
                        sol,
                        sağ,
                        üst,
                        alt,
                    );
                }
                continue;
            }
            let mut ham_parçalar = Vec::<Vec<Nokta>>::new();
            let mut parça = Vec::<Nokta>::new();
            let mut son_gerçek_boş_x = None::<f64>;
            let mut görünür_noktalar = Vec::<(usize, Nokta, f64, f64)>::new();
            let piksel_hizası = seri.piksel_hizası.unwrap_or(self.seçenekler.piksel_hizası);
            let x_piksel_uzunluğu = if self.seçenekler.x_dikey {
                yükseklik
            } else {
                genişlik
            };
            let çizilecek_indeksler = if seri.saf_doğrusal_yol
                || seri.çizim_türü == crate::SeriÇizimTürü::Noktalar
            {
                görünür_x_indeksleri(self.veri.x(), x_aralığı).collect()
            } else {
                çizilecek_indeksler(self.veri.x(), değerler, x_aralığı, x_piksel_uzunluğu)
            };
            let ilk_görünür = self
                .veri
                .x()
                .partition_point(|değer| *değer < x_aralığı.en_az);
            let görünür_bitiş = self
                .veri
                .x()
                .partition_point(|değer| *değer <= x_aralığı.en_çok);
            let görünür_indeks_sayısı = görünür_bitiş.saturating_sub(ilk_görünür);
            // uPlot, yoğun çizgi yollarını cihaz-pikseli hassasiyetinde kurar; nokta
            // işaretlerinin gizlenmesi yol koordinatlarını tam CSS pikseline
            // kuantize etmez. Bir CSS pikselinden sık örneklerde vektör yolu ham
            // koordinatlarla korumak sinüs gibi düzgün eğrilerde merdivenlenmeyi önler.
            let yoğun_çizgi = görünür_indeks_sayısı as f32 > x_piksel_uzunluğu.max(1.0);
            let nokta_piksel_açıklığı = ilk_görünür
                .checked_add(görünür_indeks_sayısı.saturating_sub(1))
                .and_then(|son| self.veri.x().get(ilk_görünür).zip(self.veri.x().get(son)))
                .map_or(0.0, |(ilk, son)| {
                    (self.x_konumu(x_aralığı, *son, 0.0, x_piksel_uzunluğu)
                        - self.x_konumu(x_aralığı, *ilk, 0.0, x_piksel_uzunluğu))
                    .abs()
                });
            for indeks in çizilecek_indeksler {
                let Some(değer) = değerler.get(indeks) else {
                    continue;
                };
                let Some(x_değeri) = self.veri.x().get(indeks) else {
                    continue;
                };
                match değer {
                    Some(y_değeri) => {
                        let komşu_x_boşluğu = seri.azami_x_boşluğu.is_some_and(|azami| {
                            indeks.checked_sub(1).is_some_and(|önceki_indeks| {
                                değerler.get(önceki_indeks).is_some_and(Option::is_some)
                                    && self
                                        .veri
                                        .x()
                                        .get(önceki_indeks)
                                        .is_some_and(|önceki| *x_değeri - *önceki > azami)
                            })
                        });
                        if komşu_x_boşluğu && !parça.is_empty() {
                            ham_parçalar.push(std::mem::take(&mut parça));
                        }
                        let (ham_x, ham_y) = if self.seçenekler.x_dikey {
                            (
                                sol + self.y_konumu(
                                    &seri.ölçek,
                                    seri_y_aralığı,
                                    *y_değeri,
                                    0.0,
                                    genişlik,
                                ),
                                alt - self.x_konumu(x_aralığı, *x_değeri, 0.0, yükseklik),
                            )
                        } else {
                            (
                                self.x_konumu(x_aralığı, *x_değeri, sol, genişlik),
                                alt - self.y_konumu(
                                    &seri.ölçek,
                                    seri_y_aralığı,
                                    *y_değeri,
                                    0.0,
                                    yükseklik,
                                ),
                            )
                        };
                        let yol_noktası = if yoğun_çizgi {
                            Nokta::yeni(ham_x, ham_y)
                        } else {
                            Nokta::yeni(
                                piksele_hizala(ham_x, piksel_hizası, self.cihaz_piksel_oranı),
                                piksele_hizala(ham_y, piksel_hizası, self.cihaz_piksel_oranı),
                            )
                        };
                        if seri.çizim_türü == crate::SeriÇizimTürü::BasamakÖnce
                            && seri.basamak_boşluk_hizası == -1
                            && parça.is_empty()
                            && let Some(boş_x) = son_gerçek_boş_x.take()
                        {
                            let yarım_vuruş = seri_kalınlığı.max(0.0) / 2.0;
                            let sınır = if self.seçenekler.x_dikey {
                                let yön = if self.seçenekler.x_ters_yön {
                                    1.0
                                } else {
                                    -1.0
                                };
                                Nokta::yeni(
                                    yol_noktası.x,
                                    alt - self.x_konumu(x_aralığı, boş_x, 0.0, yükseklik)
                                        - yön * yarım_vuruş,
                                )
                            } else {
                                let yön = if self.seçenekler.x_ters_yön {
                                    -1.0
                                } else {
                                    1.0
                                };
                                Nokta::yeni(
                                    self.x_konumu(x_aralığı, boş_x, sol, genişlik)
                                        - yön * yarım_vuruş,
                                    yol_noktası.y,
                                )
                            };
                            parça.push(sınır);
                        } else {
                            son_gerçek_boş_x = None;
                        }
                        parça.push(yol_noktası);
                        let işaret_noktası = Nokta::yeni(
                            piksele_hizala(ham_x, piksel_hizası, self.cihaz_piksel_oranı),
                            piksele_hizala(ham_y, piksel_hizası, self.cihaz_piksel_oranı),
                        );
                        if nokta_dikdörtgende(işaret_noktası, sol, sağ, üst, alt) {
                            görünür_noktalar.push((indeks, işaret_noktası, *x_değeri, *y_değeri));
                        }
                    }
                    _ if self.veri.hizalama_eksiği_mi(seri_indeksi, indeks) => {}
                    _ if seri.boşlukları_birleştir => {
                        son_gerçek_boş_x = None;
                    }
                    _ if !parça.is_empty() => {
                        if seri.çizim_türü == crate::SeriÇizimTürü::BasamakSonra
                            && seri.basamak_boşluk_hizası == 1
                            && let Some(önceki) = parça.last().copied()
                        {
                            let yarım_vuruş = seri_kalınlığı.max(0.0) / 2.0;
                            let sınır = if self.seçenekler.x_dikey {
                                let yön = if self.seçenekler.x_ters_yön {
                                    1.0
                                } else {
                                    -1.0
                                };
                                Nokta::yeni(
                                    önceki.x,
                                    alt - self.x_konumu(x_aralığı, *x_değeri, 0.0, yükseklik)
                                        + yön * yarım_vuruş,
                                )
                            } else {
                                let yön = if self.seçenekler.x_ters_yön {
                                    -1.0
                                } else {
                                    1.0
                                };
                                Nokta::yeni(
                                    self.x_konumu(x_aralığı, *x_değeri, sol, genişlik)
                                        + yön * yarım_vuruş,
                                    önceki.y,
                                )
                            };
                            parça.push(sınır);
                        }
                        son_gerçek_boş_x = Some(*x_değeri);
                        ham_parçalar.push(std::mem::take(&mut parça));
                    }
                    _ => {
                        son_gerçek_boş_x = Some(*x_değeri);
                    }
                }
            }
            if !parça.is_empty() {
                ham_parçalar.push(parça);
            }
            let mut ham_parçalar = ham_parçalar
                .into_iter()
                .map(|parça| seri_yol_noktaları(parça, seri.çizim_türü))
                .collect::<Vec<_>>();
            let dolgu_üretilecek = !bant_dolgusu
                && seri.çizim_türü != crate::SeriÇizimTürü::Noktalar
                && (seri_dolgusu.is_some() || seri.dolgu_gradyanı.is_some());
            let parçalar = if seri.saf_doğrusal_yol && !dolgu_üretilecek {
                // Resmî sparse naive pathBuilder görünür X dilimindeki tüm
                // non-null değerleri Path2D'ye ekler ve Y kırpmasını çizim
                // yüzeyine bırakır. Bu kasıtlı maliyet karşılaştırmasını
                // geometriyi önceden optimize ederek değiştirmeyiz.
                std::mem::take(&mut ham_parçalar)
            } else if dolgu_üretilecek {
                yolu_dikdörtgene_kırp(&ham_parçalar, sol, sağ, üst, alt)
            } else {
                sahipli_yolu_dikdörtgene_kırp(
                    std::mem::take(&mut ham_parçalar),
                    sol,
                    sağ,
                    üst,
                    alt,
                )
            };
            if dolgu_üretilecek {
                let taban = if self.seçenekler.x_dikey {
                    sol + self.y_konumu(
                        &seri.ölçek,
                        seri_y_aralığı,
                        seri.dolgu_tabanı,
                        0.0,
                        genişlik,
                    )
                } else {
                    alt - self.y_konumu(
                        &seri.ölçek,
                        seri_y_aralığı,
                        seri.dolgu_tabanı,
                        0.0,
                        yükseklik,
                    )
                };
                let taban = if self.seçenekler.x_dikey {
                    taban.clamp(sol, sağ)
                } else {
                    taban.clamp(üst, alt)
                };
                let çokgenler = ham_parçalar
                    .iter()
                    .filter_map(|parça| {
                        let ilk = parça.first()?;
                        let son = parça.last()?;
                        let mut çokgen = parça.clone();
                        if self.seçenekler.x_dikey {
                            çokgen.push(Nokta::yeni(taban, son.y));
                            çokgen.push(Nokta::yeni(taban, ilk.y));
                        } else {
                            çokgen.push(Nokta::yeni(son.x, taban));
                            çokgen.push(Nokta::yeni(ilk.x, taban));
                        }
                        let kırpılmış = çokgeni_dikdörtgene_kırp(&çokgen, sol, sağ, üst, alt);
                        (kırpılmış.len() >= 3).then_some(kırpılmış)
                    })
                    .collect();
                if let Some(gradyan) = seri.dolgu_gradyanı.as_ref().and_then(|düzen| {
                    self.ölçek_gradyanını_çöz(
                        düzen,
                        &seri.ölçek,
                        x_aralığı,
                        seri_y_aralığı,
                        sol,
                        üst,
                        genişlik,
                        yükseklik,
                    )
                }) {
                    sahne.ekle(Komut::GradyanAlan {
                        çokgenler, gradyan
                    });
                } else if let Some(dolgu) = &seri_dolgusu {
                    sahne.ekle(Komut::Alan {
                        çokgenler,
                        dolgu: dolgu.clone(),
                    });
                }
            }
            if seri.çizim_türü == crate::SeriÇizimTürü::Noktalar || seri_kalınlığı <= 0.0
            {
                // `paths: null` ve sıfır vuruş kalınlığı normal seri yolunu
                // üretmez; koşullu noktalar ve özel çekirdek yolları ayrıdır.
            } else if let Some(gradyan) = seri.çizgi_gradyanı.as_ref().and_then(|düzen| {
                self.ölçek_gradyanını_çöz(
                    düzen,
                    &seri.ölçek,
                    x_aralığı,
                    seri_y_aralığı,
                    sol,
                    üst,
                    genişlik,
                    yükseklik,
                )
            }) {
                sahne.ekle(Komut::GradyanYol {
                    parçalar,
                    gradyan,
                    kalınlık: seri_kalınlığı,
                });
            } else if let Some((çizgi, boşluk)) = seri.çizgi_kesik {
                sahne.ekle(Komut::KesikliYol {
                    parçalar,
                    renk: seri_rengi.clone(),
                    kalınlık: seri_kalınlığı,
                    çizgi,
                    boşluk,
                });
            } else {
                sahne.ekle(Komut::Yol {
                    parçalar,
                    renk: seri_rengi.clone(),
                    kalınlık: seri_kalınlığı,
                });
            }

            // uPlot'un varsayılanı, görünür indeks sayısını çizim genişliğinin
            // `points.space` kapasitesiyle karşılaştırır.
            let kanca = self.seçenekler.çizim_kancaları.as_ref();
            if let Some((uçlar, düzen)) =
                kanca.and_then(|düzen| düzen.yıldız_uçları.map(|uçlar| (uçlar, düzen)))
            {
                for (_, nokta, _, _) in &görünür_noktalar {
                    sahne.ekle(Komut::Alan {
                        çokgenler: vec![yıldız_çokgeni(
                            *nokta,
                            uçlar,
                            düzen.yıldız_dış_yarıçapı,
                            düzen.yıldız_iç_yarıçapı,
                        )],
                        dolgu: seri_rengi.clone(),
                    });
                }
            } else {
                let noktalar_gizli = !self.seçenekler.kırılım_noktaları_görünür;
                let noktalar_görünür = !noktalar_gizli
                    && seri.noktaları_göster.unwrap_or_else(|| {
                        seri.nokta_boşluğu <= 0.0
                            || görünür_indeks_sayısı.saturating_sub(1) as f32
                                <= nokta_piksel_açıklığı / seri.nokta_boşluğu.max(f32::EPSILON)
                    });
                let filtreli_indeksler = (!noktalar_gizli
                    && !noktalar_görünür
                    && seri.nokta_filtresi == crate::NoktaFiltreKipi::BoşlukArasındakiTekiller)
                    .then(|| {
                        boşluk_tekil_indeksleri(
                            self.veri.x(),
                            değerler,
                            ilk_görünür,
                            görünür_bitiş,
                            x_aralığı,
                            sol,
                            genişlik,
                            piksel_hizası,
                            self.cihaz_piksel_oranı,
                        )
                    });
                let mut toplu_kareler = Vec::new();
                let mut toplu_daireler = Vec::<(String, String, Vec<Nokta>)>::new();
                for (indeks, nokta, x_değeri, y_değeri) in &görünür_noktalar {
                    if seri
                        .nokta_indeksleri
                        .as_ref()
                        .is_some_and(|indeksler| indeksler.binary_search(indeks).is_err())
                    {
                        continue;
                    }
                    let filtreli_tekil = filtreli_indeksler
                        .as_ref()
                        .is_some_and(|indeksler| indeksler.binary_search(indeks).is_ok());
                    if !noktalar_görünür && !filtreli_tekil {
                        continue;
                    }
                    let nokta_rengi = self
                        .seri_imleç_rengi(seri_indeksi, *x_değeri, *y_değeri)
                        .unwrap_or_else(|| seri_rengi.clone());
                    let dolgu = seri
                        .nokta_dolgusu
                        .clone()
                        .unwrap_or_else(|| "#ffffff".to_string());
                    match seri.nokta_şekli {
                        crate::NoktaŞekli::Daire => {
                            if let Some((_, _, merkezler)) =
                                toplu_daireler
                                    .iter_mut()
                                    .find(|(grup_dolgusu, grup_çizgisi, _)| {
                                        grup_dolgusu == &dolgu && grup_çizgisi == &nokta_rengi
                                    })
                            {
                                merkezler.push(*nokta);
                            } else {
                                toplu_daireler.push((dolgu, nokta_rengi, vec![*nokta]));
                            }
                        }
                        crate::NoktaŞekli::Kare => {
                            let yarı = seri.nokta_boyutu / 2.0;
                            toplu_kareler.push(vec![
                                Nokta::yeni(nokta.x - yarı, nokta.y - yarı),
                                Nokta::yeni(nokta.x + yarı, nokta.y - yarı),
                                Nokta::yeni(nokta.x + yarı, nokta.y + yarı),
                                Nokta::yeni(nokta.x - yarı, nokta.y + yarı),
                            ]);
                        }
                    }
                }
                for (dolgu, çizgi, merkezler) in toplu_daireler {
                    let yarıçap = ((seri.nokta_boyutu - seri.nokta_kalınlığı) / 2.0).max(0.0);
                    if yarıçap > 0.0 {
                        sahne.ekle(Komut::Daireler {
                            merkezler,
                            yarıçap,
                            dolgu,
                            çizgi,
                            kalınlık: seri.nokta_kalınlığı,
                            kesme_sınırları: None,
                        });
                    }
                }
                if !toplu_kareler.is_empty() {
                    sahne.ekle(Komut::Alan {
                        çokgenler: toplu_kareler,
                        dolgu: seri
                            .nokta_dolgusu
                            .clone()
                            .unwrap_or_else(|| seri_rengi.clone()),
                    });
                }
            }

            if let Some(düzen) = kanca.filter(|düzen| düzen.seri_uç_trendleri)
                && let (Some((_, başlangıç, _, _)), Some((_, bitiş, _, _))) =
                    (görünür_noktalar.first(), görünür_noktalar.last())
                && başlangıç != bitiş
            {
                // Kaynak drawSeries kancası tek sayılı stroke genişliğini
                // yarım piksel öteleyerek kesik çizgiyi raster ızgarasına oturtur.
                let ofset = (seri_kalınlığı % 2.0) / 2.0;
                sahne.ekle(Komut::KesikliÇizgi {
                    başlangıç: Nokta::yeni(başlangıç.x + ofset, başlangıç.y + ofset),
                    bitiş: Nokta::yeni(bitiş.x + ofset, bitiş.y + ofset),
                    renk: seri_rengi.clone(),
                    kalınlık: seri_kalınlığı,
                    kesik: düzen.trend_kesik,
                });
            }

            if let Some(düzen) = kanca.filter(|düzen| düzen.seri_medyanları)
                && let Some(medyan) = self
                    .çizim_kancası_medyanları
                    .get(seri_indeksi)
                    .copied()
                    .flatten()
            {
                let y = alt - self.y_konumu(&seri.ölçek, seri_y_aralığı, medyan, 0.0, yükseklik);
                let dış_kalınlık = düzen.medyan_kalınlığı + düzen.medyan_bulanıklığı.max(0.0) * 2.0;
                if düzen.medyan_bulanıklığı > 0.0 {
                    sahne.ekle(Komut::Çizgi {
                        başlangıç: Nokta::yeni(sol, y),
                        bitiş: Nokta::yeni(sağ, y),
                        renk: renk_alfa(&seri_rengi, 0x14),
                        kalınlık: dış_kalınlık,
                    });
                }
                sahne.ekle(Komut::Çizgi {
                    başlangıç: Nokta::yeni(sol, y),
                    bitiş: Nokta::yeni(sağ, y),
                    renk: renk_alfa(&seri_rengi, 0x33),
                    kalınlık: düzen.medyan_kalınlığı,
                });
            }
        }

        // Kaynak `drawAxes` kancası gibi interpolasyon kılavuzları seri
        // yollarından sonra, görünür X dilimiyle sınırlı tek bir path'te boyanır.
        if let Some(düzen) = self.seçenekler.en_yakın_tooltip.as_ref() {
            let mut kılavuzlar = Vec::new();
            for indeks in &düzen.interpolasyonlar {
                let Some(x_değeri) = self.veri.x().get(*indeks).copied() else {
                    continue;
                };
                if x_değeri < x_aralığı.en_az || x_değeri > x_aralığı.en_çok {
                    continue;
                }
                let x = self.x_konumu(x_aralığı, x_değeri, sol, genişlik);
                kılavuzlar.push(vec![Nokta::yeni(x, üst), Nokta::yeni(x, alt)]);
            }
            if !kılavuzlar.is_empty() {
                sahne.ekle(Komut::Yol {
                    parçalar: kılavuzlar,
                    renk: renk_alfa(&düzen.interpolasyon_rengi, 0x7a),
                    kalınlık: 1.0,
                });
            }
        }

        if let Some(düzen) = self.seçenekler.açıklama_düzeni.as_ref() {
            for (işaret, stil_indeksi) in düzen
                .işaretler
                .iter()
                .zip(self.açıklama_stil_indeksleri.iter())
            {
                if !işaret.başlangıç.is_finite()
                    || !işaret.bitiş.is_finite()
                    || işaret.bitiş < işaret.başlangıç
                {
                    continue;
                }
                let görünür = (işaret.başlangıç >= x_aralığı.en_az
                    && işaret.başlangıç <= x_aralığı.en_çok)
                    || (işaret.bitiş >= x_aralığı.en_az && işaret.bitiş <= x_aralığı.en_çok)
                    || (işaret.başlangıç <= x_aralığı.en_az && işaret.bitiş >= x_aralığı.en_çok);
                if !görünür {
                    continue;
                }
                let Some(stil) = stil_indeksi.and_then(|indeks| düzen.stiller.get(indeks)) else {
                    continue;
                };
                let başlangıç_x = self
                    .x_konumu(x_aralığı, işaret.başlangıç, sol, genişlik)
                    .round();
                let bitiş_x = self
                    .x_konumu(x_aralığı, işaret.bitiş, sol, genişlik)
                    .round();
                let kırpılmış_sol = başlangıç_x.clamp(sol, sağ);
                let kırpılmış_sağ = bitiş_x.clamp(sol, sağ);

                if işaret.bitiş > işaret.başlangıç && kırpılmış_sağ > kırpılmış_sol {
                    sahne.ekle(Komut::Dikdörtgen {
                        konum: Nokta::yeni(kırpılmış_sol, üst),
                        genişlik: kırpılmış_sağ - kırpılmış_sol,
                        yükseklik,
                        dolgu: stil.dolgu.clone(),
                        çizgi: "#00000000".to_string(),
                        kalınlık: 0.0,
                    });
                }
                if başlangıç_x >= sol && başlangıç_x <= sağ {
                    sahne.ekle(Komut::KesikliÇizgi {
                        başlangıç: Nokta::yeni(başlangıç_x, üst),
                        bitiş: Nokta::yeni(başlangıç_x, alt),
                        renk: stil.çizgi.clone(),
                        kalınlık: stil.kalınlık,
                        kesik: stil.kesik,
                    });
                }
                if işaret.bitiş > işaret.başlangıç && bitiş_x >= sol && bitiş_x <= sağ {
                    sahne.ekle(Komut::KesikliÇizgi {
                        başlangıç: Nokta::yeni(bitiş_x, üst),
                        bitiş: Nokta::yeni(bitiş_x, alt),
                        renk: stil.çizgi.clone(),
                        kalınlık: stil.kalınlık,
                        kesik: stil.kesik,
                    });
                }

                // Kaynak etiket `from` çizgisinde ortalanır; aralığın ortasına
                // taşınmaz. Overlay kırpması nedeniyle `from` görünür değilse
                // etiket de görünmez.
                if başlangıç_x < sol || başlangıç_x > sağ {
                    continue;
                }
                let etiket_genişliği = (işaret.etiket.chars().count() as f32 * 7.0 + 8.0).max(12.0);
                let etiket_yüksekliği = 18.0;
                let etiket_üst = match stil.hiza {
                    crate::AçıklamaHizası::Üst => üst,
                    crate::AçıklamaHizası::Alt => alt - etiket_yüksekliği,
                };
                let etiket_sol = başlangıç_x - etiket_genişliği / 2.0;
                sahne.ekle(Komut::Dikdörtgen {
                    konum: Nokta::yeni(etiket_sol, etiket_üst),
                    genişlik: etiket_genişliği,
                    yükseklik: etiket_yüksekliği,
                    dolgu: "#ffffff".to_string(),
                    çizgi: "#00000000".to_string(),
                    kalınlık: 0.0,
                });
                for (başlangıç, bitiş) in [
                    (
                        Nokta::yeni(etiket_sol, etiket_üst),
                        Nokta::yeni(etiket_sol + etiket_genişliği, etiket_üst),
                    ),
                    (
                        Nokta::yeni(etiket_sol + etiket_genişliği, etiket_üst),
                        Nokta::yeni(
                            etiket_sol + etiket_genişliği,
                            etiket_üst + etiket_yüksekliği,
                        ),
                    ),
                    (
                        Nokta::yeni(
                            etiket_sol + etiket_genişliği,
                            etiket_üst + etiket_yüksekliği,
                        ),
                        Nokta::yeni(etiket_sol, etiket_üst + etiket_yüksekliği),
                    ),
                    (
                        Nokta::yeni(etiket_sol, etiket_üst + etiket_yüksekliği),
                        Nokta::yeni(etiket_sol, etiket_üst),
                    ),
                ] {
                    sahne.ekle(Komut::KesikliÇizgi {
                        başlangıç,
                        bitiş,
                        renk: stil.çizgi.clone(),
                        kalınlık: stil.kalınlık,
                        kesik: stil.kesik,
                    });
                }
                sahne.ekle(Komut::Metin {
                    konum: Nokta::yeni(başlangıç_x, etiket_üst + 13.0),
                    içerik: işaret.etiket.clone(),
                    renk: "#111111".to_string(),
                    boyut: 12.0,
                    hiza: MetinHizası::Orta,
                });
            }
        }

        if self.seçenekler.ölçüm_datumları {
            let datum_noktası = |(x, y): (f64, f64)| {
                Nokta::yeni(
                    self.x_konumu(x_aralığı, x, sol, genişlik),
                    alt - self.y_konumu(
                        &self.seçenekler.birincil_y_ölçeği,
                        y_aralığı,
                        y,
                        0.0,
                        yükseklik,
                    ),
                )
            };
            for (datum, renk) in self.ölçüm_datumları.into_iter().zip(["blue", "orange"]) {
                let Some(değer) = datum else {
                    continue;
                };
                let merkez = datum_noktası(değer);
                sahne.ekle(Komut::Daire {
                    merkez,
                    yarıçap: 10.0,
                    dolgu: "#00000000".to_string(),
                    çizgi: renk.to_string(),
                    kalınlık: 2.0,
                });
                sahne.ekle(Komut::Çizgi {
                    başlangıç: Nokta::yeni(merkez.x - 15.0, merkez.y),
                    bitiş: Nokta::yeni(merkez.x + 15.0, merkez.y),
                    renk: renk.to_string(),
                    kalınlık: 2.0,
                });
                sahne.ekle(Komut::Çizgi {
                    başlangıç: Nokta::yeni(merkez.x, merkez.y - 15.0),
                    bitiş: Nokta::yeni(merkez.x, merkez.y + 15.0),
                    renk: renk.to_string(),
                    kalınlık: 2.0,
                });
            }
            if let (Some((x1, y1)), Some((x2, y2))) =
                (self.ölçüm_datumları[0], self.ölçüm_datumları[1])
            {
                let orta = datum_noktası(((x1 + x2) / 2.0, (y1 + y2) / 2.0));
                sahne.ekle(Komut::Metin {
                    // Canvas `textBaseline = "middle"` karşılığı: SVG/GPUI
                    // baseline konumunu 12 px yazının optik merkezine taşır.
                    konum: Nokta::yeni(orta.x, orta.y + 4.0),
                    içerik: format!(
                        "dx: {}, dy: {}",
                        üç_anlamlı_basamak(x2 - x1),
                        üç_anlamlı_basamak(y2 - y1)
                    ),
                    renk: "black".to_string(),
                    boyut: 12.0,
                    hiza: MetinHizası::Orta,
                });
            }
        }

        if let Some(düzen) = self.seçenekler.rüzgar_yönü_düzeni.as_ref() {
            self.rüzgar_yönlerini_çiz(
                &mut sahne,
                düzen,
                x_aralığı,
                görünür_y,
                sol,
                sağ,
                üst,
                alt,
            );
        }

        let birincil_aralık = self.görünür_ölçek_aralığı(
            &self.seçenekler.birincil_y_ölçeği,
            x_aralığı,
            görünür_y,
        );
        for katman in &self.seçenekler.nokta_katmanları {
            for (x_değeri, y_değeri) in katman.noktalar.iter().copied() {
                if x_değeri < x_aralığı.en_az || x_değeri > x_aralığı.en_çok {
                    continue;
                }
                let x = self.x_konumu(x_aralığı, x_değeri, sol, genişlik);
                let y = alt
                    - self.y_konumu(
                        &self.seçenekler.birincil_y_ölçeği,
                        birincil_aralık,
                        y_değeri,
                        0.0,
                        yükseklik,
                    );
                if nokta_dikdörtgende(Nokta::yeni(x, y), sol, sağ, üst, alt) {
                    sahne.ekle(Komut::Dikdörtgen {
                        konum: Nokta::yeni(x, y),
                        genişlik: katman.boyut,
                        yükseklik: katman.boyut,
                        dolgu: katman.renk.clone(),
                        çizgi: katman.renk.clone(),
                        kalınlık: 0.0,
                    });
                }
            }
        }

        if let Some(düzen) = self
            .seçenekler
            .çizim_kancaları
            .as_ref()
            .filter(|düzen| düzen.çizim_süresi_metni)
        {
            sahne.ekle(Komut::Metin {
                konum: Nokta::yeni(sol + 10.0, üst + 22.0),
                içerik: format!(
                    "Time to Draw: {}ms",
                    çizim_başlangıcı
                        .map(|başlangıç| başlangıç.elapsed().as_millis())
                        .unwrap_or_default()
                ),
                renk: düzen.çizim_süresi_metni_rengi.clone(),
                boyut: düzen.çizim_süresi_yazı_boyutu,
                hiza: MetinHizası::Başlangıç,
            });
        }

        if self.seçenekler.çizim_sırası == crate::ÇizimSırası::SerilerEksenler {
            sahne.komut_aralığını_sona_taşı(
                eksen_komutları_başlangıcı,
                eksen_komutları_bitişi,
            );
        }

        sahne
    }

    fn birincil_y_ızgara_çizgisini_ekle(
        &self,
        sahne: &mut Sahne,
        başlangıç: Nokta,
        bitiş: Nokta,
    ) {
        if let Some(kesik) = self.seçenekler.birincil_y_ızgara_kesik {
            sahne.ekle(Komut::KesikliÇizgi {
                başlangıç,
                bitiş,
                renk: self.seçenekler.ızgara_rengi.clone(),
                kalınlık: 1.0,
                kesik,
            });
        } else {
            sahne.ekle(Komut::Çizgi {
                başlangıç,
                bitiş,
                renk: self.seçenekler.ızgara_rengi.clone(),
                kalınlık: 1.0,
            });
        }
    }

    fn birincil_y_etiketi(
        &self,
        değer: f64,
        artım: f64,
        birim: &str,
        dağılım: Option<YÖlçekDağılımı>,
        biçim: YÖlçekEtiketBiçimi,
        çarpan: f64,
    ) -> String {
        self.seçenekler
            .birincil_y_özel_etiketler
            .iter()
            .find(|(aday, _)| (*aday - değer).abs() <= f64::EPSILON)
            .map(|(_, etiket)| etiket.clone())
            .unwrap_or_else(|| {
                ölçek_eksen_değerini_yaz(değer * çarpan, artım, birim, dağılım, biçim)
            })
    }

    #[allow(clippy::too_many_arguments)]
    fn rüzgar_yönlerini_çiz(
        &self,
        sahne: &mut Sahne,
        düzen: &crate::RüzgarYönüDüzeni,
        x_aralığı: Aralık,
        görünür_y: Option<Aralık>,
        sol: f32,
        sağ: f32,
        üst: f32,
        alt: f32,
    ) {
        let Some(hızlar) = self.veri.seriler().get(düzen.hız_serisi) else {
            return;
        };
        let Some(yönler) = self.veri.seriler().get(düzen.yön_serisi) else {
            return;
        };
        let Some(yön_serisi) = self.seçenekler.seriler.get(düzen.yön_serisi) else {
            return;
        };
        if !yön_serisi.göster {
            return;
        }
        let uzunluk = self.veri.x().len().min(hızlar.len()).min(yönler.len());
        if uzunluk == 0 {
            return;
        }
        let genişlik = sağ - sol;
        let yükseklik = alt - üst;
        let y_aralığı = self.görünür_ölçek_aralığı(&düzen.ölçek, x_aralığı, görünür_y);
        let Some(xs) = self.veri.x().get(..uzunluk) else {
            return;
        };
        let görünür_başlangıç = xs.partition_point(|x| *x < x_aralığı.en_az);
        let görünür_bitiş = xs.partition_point(|x| *x <= x_aralığı.en_çok);
        if görünür_başlangıç >= görünür_bitiş {
            return;
        }

        // Resmî özel path kurucusu `getOuterIdxs()` ile görünümün iki
        // yanındaki ilk veri noktasını da alır ve null yönleri dışa doğru
        // geçer. Böylece yakınlaştırma sınırındaki vektörler kopmaz.
        let mut başlangıç = görünür_başlangıç.saturating_sub(1);
        let mut bitiş = görünür_bitiş.min(uzunluk.saturating_sub(1));
        while başlangıç > 0 && yönler.get(başlangıç).is_some_and(Option::is_none) {
            başlangıç -= 1;
        }
        while bitiş < uzunluk.saturating_sub(1) && yönler.get(bitiş).is_some_and(Option::is_none)
        {
            bitiş += 1;
        }

        let mut parçalar = Vec::with_capacity(bitiş.saturating_sub(başlangıç) + 1);
        for indeks in başlangıç..=bitiş {
            let Some(hız) = hızlar.get(indeks).copied().flatten() else {
                continue;
            };
            // JavaScript'te `null - 90` sayısal olarak çalışır. Kaynak
            // demoda null yön ve hızlar hizalıdır; bu dönüşüm uyumu korur.
            let yön = yönler.get(indeks).copied().flatten().unwrap_or(0.0);
            if !hız.is_finite() || !yön.is_finite() {
                continue;
            }
            let Some(x_değeri) = xs.get(indeks).copied() else {
                continue;
            };
            let (x, y) = if self.seçenekler.x_dikey {
                (
                    sol + self.y_konumu(&düzen.ölçek, y_aralığı, hız, 0.0, genişlik),
                    alt - self.x_konumu(x_aralığı, x_değeri, 0.0, yükseklik),
                )
            } else {
                (
                    self.x_konumu(x_aralığı, x_değeri, sol, genişlik),
                    alt - self.y_konumu(&düzen.ölçek, y_aralığı, hız, 0.0, yükseklik),
                )
            };
            let açı = (yön - 90.0).to_radians();
            let dx = düzen.uzunluk * açı.cos() as f32;
            let dy = düzen.uzunluk * açı.sin() as f32;
            parçalar.push(vec![Nokta::yeni(x, y), Nokta::yeni(x + dx, y + dy)]);
        }
        if !parçalar.is_empty() {
            // uPlot tek beginPath/stroke kullanır; SVG ve GPUI de bütün
            // vektörleri tek bir yol komutuyla boyar.
            sahne.ekle(Komut::Yol {
                parçalar,
                renk: düzen.renk.clone(),
                kalınlık: düzen.kalınlık,
            });
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn karma_çubuk_serisini_çiz(
        &self,
        sahne: &mut Sahne,
        seri: &crate::SeriSeçenekleri,
        değerler: &[Option<f64>],
        x_aralığı: Aralık,
        y_aralığı: Aralık,
        sol: f32,
        sağ: f32,
        üst: f32,
        alt: f32,
    ) {
        let genişlik = sağ - sol;
        let yükseklik = alt - üst;
        let mut önceki_x = None;
        let mut en_küçük_fark = f64::INFINITY;
        for (x, değer) in self.veri.x().iter().zip(değerler.iter()) {
            if değer.is_none() || *x < x_aralığı.en_az || *x > x_aralığı.en_çok {
                continue;
            }
            if let Some(önceki) = önceki_x {
                let fark = *x - önceki;
                if fark > 0.0 {
                    en_küçük_fark = en_küçük_fark.min(fark);
                }
            }
            önceki_x = Some(*x);
        }
        let varsayılan_fark =
            (x_aralığı.en_çok - x_aralığı.en_az) / değerler.len().saturating_sub(1).max(1) as f64;
        let veri_farkı = if en_küçük_fark.is_finite() {
            en_küçük_fark
        } else {
            varsayılan_fark
        };
        let çubuk_genişliği = (veri_farkı / (x_aralığı.en_çok - x_aralığı.en_az)
            * f64::from(genişlik)
            * f64::from(seri.çubuk_genişlik_oranı)) as f32;
        let çubuk_genişliği =
            (çubuk_genişliği.min(seri.azami_çubuk_genişliği) - seri.çubuk_boşluğu_piksel).max(0.0);
        let üst_değerler = seri
            .yüzen_çubuk_üst_serisi
            .and_then(|indeks| self.veri.seriler().get(indeks));
        let varsayılan_dolgu = seri.dolgu.as_ref().unwrap_or(&seri.renk);
        let mut gradyan_çokgenleri = Vec::new();
        let mut toplu_çokgenler = Vec::new();
        for (indeks, (x_değeri, değer)) in self.veri.x().iter().zip(değerler.iter()).enumerate()
        {
            let Some(alt_değer) = değer else {
                continue;
            };
            let üst_değer = if let Some(üst_değerler) = üst_değerler {
                let Some(üst_değer) = üst_değerler.get(indeks).copied().flatten() else {
                    continue;
                };
                üst_değer
            } else {
                *alt_değer
            };
            let taban_değer = if üst_değerler.is_some() {
                *alt_değer
            } else {
                seri.dolgu_tabanı
            };
            if *x_değeri < x_aralığı.en_az || *x_değeri > x_aralığı.en_çok {
                continue;
            }
            let merkez = self.x_konumu(x_aralığı, *x_değeri, sol, genişlik);
            let y0 = (alt - self.y_konumu(&seri.ölçek, y_aralığı, taban_değer, 0.0, yükseklik))
                .clamp(üst, alt);
            let y1 = (alt - self.y_konumu(&seri.ölçek, y_aralığı, üst_değer, 0.0, yükseklik))
                .clamp(üst, alt);
            let (ham_x0, ham_x1) = match seri.çubuk_hizası {
                1 => (merkez, merkez + çubuk_genişliği),
                -1 => (merkez - çubuk_genişliği, merkez),
                _ => (
                    merkez - çubuk_genişliği / 2.0,
                    merkez + çubuk_genişliği / 2.0,
                ),
            };
            let x0 = ham_x0.clamp(sol, sağ);
            let x1 = ham_x1.clamp(sol, sağ);
            let çubuk_üst = y1.min(y0);
            let çubuk_alt = y1.max(y0);
            if x1 <= x0 || çubuk_alt <= çubuk_üst {
                continue;
            }
            if seri.dolgu_gradyanı.is_some() {
                gradyan_çokgenleri.push(vec![
                    Nokta::yeni(x0, çubuk_üst),
                    Nokta::yeni(x1, çubuk_üst),
                    Nokta::yeni(x1, çubuk_alt),
                    Nokta::yeni(x0, çubuk_alt),
                ]);
            } else if seri.toplu_çubuk_yolu
                && seri.çubuk_dolguları.is_empty()
                && seri.çubuk_çizgileri.is_empty()
                && seri.çubuk_uç_yarıçap_oranı <= 0.0
            {
                toplu_çokgenler.push(vec![
                    Nokta::yeni(x0, çubuk_üst),
                    Nokta::yeni(x1, çubuk_üst),
                    Nokta::yeni(x1, çubuk_alt),
                    Nokta::yeni(x0, çubuk_alt),
                ]);
            } else {
                let dolgu = seri.çubuk_dolguları.get(indeks).unwrap_or(varsayılan_dolgu);
                let çizgi = seri.çubuk_çizgileri.get(indeks).unwrap_or(&seri.renk);
                sahne.ekle(çubuk_komutu(
                    Nokta::yeni(x0, çubuk_üst),
                    x1 - x0,
                    çubuk_alt - çubuk_üst,
                    dolgu.clone(),
                    çizgi.clone(),
                    seri.çizgi_kalınlığı,
                    seri.çubuk_uç_yarıçap_oranı,
                    crate::ÇubukYönü::Dikey,
                    üst_değer < taban_değer,
                ));
            }
        }
        if !toplu_çokgenler.is_empty() {
            sahne.ekle(Komut::Alan {
                çokgenler: toplu_çokgenler.clone(),
                dolgu: varsayılan_dolgu.clone(),
            });
            if seri.çizgi_kalınlığı > 0.0 {
                let parçalar = toplu_çokgenler
                    .into_iter()
                    .map(|mut çokgen| {
                        if let Some(ilk) = çokgen.first().copied() {
                            çokgen.push(ilk);
                        }
                        çokgen
                    })
                    .collect();
                sahne.ekle(Komut::Yol {
                    parçalar,
                    renk: seri.renk.clone(),
                    kalınlık: seri.çizgi_kalınlığı,
                });
            }
        }
        if !gradyan_çokgenleri.is_empty()
            && let Some(gradyan) = seri.dolgu_gradyanı.as_ref().and_then(|düzen| {
                self.ölçek_gradyanını_çöz(
                    düzen,
                    &seri.ölçek,
                    x_aralığı,
                    y_aralığı,
                    sol,
                    üst,
                    genişlik,
                    yükseklik,
                )
            })
        {
            sahne.ekle(Komut::GradyanAlan {
                çokgenler: gradyan_çokgenleri,
                gradyan,
            });
        }
    }

    fn çubukları_çiz(
        &self,
        sahne: &mut Sahne,
        genişlik_px: u32,
        yükseklik_px: u32,
        düzen: crate::ÇubukDüzeni,
        görünür_x: Option<Aralık>,
        görünür_y: Option<Aralık>,
    ) {
        let grup_sayısı = self.veri.uzunluk();
        let karışık_çizim = self
            .seçenekler
            .seriler
            .iter()
            .any(|seri| seri.çizim_türü == crate::SeriÇizimTürü::Çubuk);
        let çubuk_serileri = (0..self.veri.seriler().len())
            .filter(|indeks| {
                self.seçenekler.seriler.get(*indeks).is_some_and(|seri| {
                    !karışık_çizim || seri.çizim_türü == crate::SeriÇizimTürü::Çubuk
                })
            })
            .collect::<Vec<_>>();
        let çizgi_serileri = if karışık_çizim {
            self.seçenekler
                .seriler
                .iter()
                .enumerate()
                .filter_map(|(indeks, seri)| {
                    (seri.göster && seri.çizim_türü != crate::SeriÇizimTürü::Çubuk)
                        .then_some(indeks)
                })
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };
        let seri_sayısı = çubuk_serileri.len().max(1);
        if grup_sayısı == 0 {
            return;
        }
        let kategoriler = (0..grup_sayısı)
            .map(|indeks| {
                self.seçenekler
                    .kategoriler
                    .get(indeks)
                    .cloned()
                    .or_else(|| self.veri.x().get(indeks).map(|değer| format!("{değer:.0}")))
                    .unwrap_or_default()
            })
            .collect::<Vec<_>>();
        let (en_az, en_çok) = (0..grup_sayısı).fold((0.0_f64, 0.0_f64), |sonuç, indeks| {
            let değerler = self
                .veri
                .seriler()
                .iter()
                .filter_map(|seri| seri.get(indeks).copied().flatten());
            let (alt, üst) = if düzen.yığılmış {
                değerler.fold((0.0_f64, 0.0_f64), |(alt, üst), değer| {
                    if değer < 0.0 {
                        (alt + değer, üst)
                    } else {
                        (alt, üst + değer)
                    }
                })
            } else {
                değerler.fold((0.0_f64, 0.0_f64), |(alt, üst), değer| {
                    (alt.min(değer), üst.max(değer))
                })
            };
            (sonuç.0.min(alt), sonuç.1.max(üst))
        });
        let ham_açıklık = (en_çok - en_az).max(1.0);
        let veri_aralığı = Aralık {
            en_az: if en_az < 0.0 {
                en_az - ham_açıklık * 0.05
            } else {
                0.0
            },
            en_çok: if en_çok > 0.0 {
                en_çok + ham_açıklık * 0.05
            } else {
                0.0
            },
        };
        let aralık = görünür_y
            .or_else(|| {
                self.seçenekler
                    .y_ölçekleri
                    .iter()
                    .find(|ölçek| ölçek.anahtar == self.seçenekler.birincil_y_ölçeği)
                    .and_then(|ölçek| ölçek.aralık)
            })
            .or(self.seçenekler.y_aralığı)
            .unwrap_or(veri_aralığı);
        let tam_x = self
            .seçenekler
            .x_aralığı
            .or_else(|| tam_x_aralığı(&self.veri).ok())
            .and_then(|aralık| {
                if grup_sayısı > 1 && düzen.x_kenar_paylı {
                    Aralık::yeni(aralık.en_az - 0.5, aralık.en_çok + 0.5).ok()
                } else {
                    Some(aralık)
                }
            })
            .unwrap_or(Aralık {
                en_az: -0.5,
                en_çok: 0.5,
            });
        let x_aralığı = görünür_x.unwrap_or(tam_x);
        let x_açıklığı = (x_aralığı.en_çok - x_aralığı.en_az).max(f64::EPSILON);
        let vuruş_anahtarı =
            self.çubuk_vuruş_anahtarı(genişlik_px, yükseklik_px, x_aralığı, aralık);
        let mut vuruş_kayıtları = Vec::<ÇubukVuruşKaydı>::new();
        let birincil_y_birimi = self
            .ölçek_seçeneği(&self.seçenekler.birincil_y_ölçeği)
            .map_or("", |ölçek| ölçek.birim.as_str());

        match düzen.yön {
            crate::ÇubukYönü::Dikey => {
                let (sol, sağ, üst, alt) = (
                    64.0,
                    genişlik_px as f32 - 24.0,
                    48.0,
                    yükseklik_px as f32 - 72.0,
                );
                let çizim_g = sağ - sol;
                let çizim_y = alt - üst;
                for değer in eksen_bölmeleri(aralık, çizim_y, 30.0) {
                    let y = alt - aralık.konum(değer, 0.0, çizim_y);
                    sahne.ekle(Komut::Çizgi {
                        başlangıç: Nokta::yeni(sol, y),
                        bitiş: Nokta::yeni(sağ, y),
                        renk: "#e5e7eb".to_string(),
                        kalınlık: 1.0,
                    });
                    sahne.ekle(Komut::Metin {
                        konum: Nokta::yeni(sol - 8.0, y + 4.0),
                        içerik: format!(
                            "{}{}",
                            eksen_değerini_yaz(değer, uygun_artım(aralık, çizim_y, 30.0)),
                            birincil_y_birimi
                        ),
                        renk: "#4b5563".to_string(),
                        boyut: 11.0,
                        hiza: MetinHizası::Bitiş,
                    });
                }
                let grup_adımı = if grup_sayısı > 1 {
                    çizim_g / x_açıklığı as f32
                } else {
                    çizim_g
                };
                let sayısal_x_adımı = self
                    .seçenekler
                    .kategoriler
                    .is_empty()
                    .then(|| uygun_artım(x_aralığı, çizim_g, 40.0));
                let ilk_görünür = self
                    .veri
                    .x()
                    .partition_point(|değer| *değer < x_aralığı.en_az);
                let görünür_bitiş = self
                    .veri
                    .x()
                    .partition_point(|değer| *değer <= x_aralığı.en_çok);
                let görünür_indeks_sayısı = görünür_bitiş.saturating_sub(ilk_görünür);
                let nokta_piksel_açıklığı = ilk_görünür
                    .checked_add(görünür_indeks_sayısı.saturating_sub(1))
                    .and_then(|son| self.veri.x().get(ilk_görünür).zip(self.veri.x().get(son)))
                    .map_or(0.0, |(ilk, son)| {
                        (((son - ilk) / x_açıklığı) as f32 * çizim_g).abs()
                    });
                let grup_genişliği = if düzen.grupları_kenarlara_yay {
                    space_between_grup_boyutu(
                        self.veri.x(),
                        x_aralığı,
                        çizim_g,
                        düzen.genişlik_oranı,
                    )
                } else {
                    grup_adımı * düzen.genişlik_oranı - düzen.ek_boşluk
                }
                .max(1.0);
                let mut tam_boşluk = (grup_adımı - grup_genişliği).max(0.0) + düzen.ek_boşluk;
                if tam_boşluk < 1.0 {
                    tam_boşluk = 0.0;
                }
                let otomatik_yazı_boyutu = düzen.değer_etiketi_otomatik.then(|| {
                    let (azami_metin_genişliği, azami_metin_yüksekliği) = self
                        .otomatik_çubuk_metinleri
                        .as_ref()
                        .map_or((1.0, 10.0), |önbellek| {
                            (önbellek.azami_10px_genişlik, önbellek.azami_10px_yükseklik)
                        });
                    let kullanılabilir_yükseklik = self
                        .veri
                        .seriler()
                        .iter()
                        .flat_map(|seri| {
                            seri.get(ilk_görünür..görünür_bitiş)
                                .into_iter()
                                .flatten()
                                .copied()
                                .flatten()
                        })
                        .map(|değer| {
                            let uç_y = alt - aralık.konum(değer, 0.0, çizim_y);
                            if değer < 0.0 {
                                alt - uç_y
                            } else {
                                uç_y - üst
                            }
                        })
                        .fold(f32::INFINITY, f32::min);
                    let ölçek = (grup_genişliği * 0.8 / azami_metin_genişliği)
                        .min(kullanılabilir_yükseklik / azami_metin_yüksekliği);
                    (ölçek * 10.0).min(25.0)
                });
                for indeks in 0..grup_sayısı {
                    let Some(x_değeri) = self.veri.x().get(indeks).copied() else {
                        continue;
                    };
                    let oran = if düzen.ters {
                        (x_aralığı.en_çok - x_değeri) / x_açıklığı
                    } else {
                        (x_değeri - x_aralığı.en_az) / x_açıklığı
                    };
                    let merkez = sol + oran as f32 * çizim_g;
                    if merkez + grup_genişliği / 2.0 < sol || merkez - grup_genişliği / 2.0 > sağ
                    {
                        continue;
                    }
                    let x_etiketi = sayısal_x_adımı.map_or_else(
                        || kategoriler.get(indeks).cloned(),
                        |adım| {
                            let en_yakın = (x_değeri / adım).round() * adım;
                            ((x_değeri - en_yakın).abs() <= adım.abs() * 1e-9).then(|| {
                                if adım.abs() >= 1.0 && x_değeri.fract().abs() <= f64::EPSILON {
                                    format!("{x_değeri:.0}")
                                } else {
                                    eksen_değerini_yaz(x_değeri, adım)
                                }
                            })
                        },
                    );
                    if let Some(içerik) = x_etiketi {
                        sahne.ekle(Komut::Metin {
                            konum: Nokta::yeni(merkez, alt + 22.0),
                            içerik,
                            renk: "#4b5563".to_string(),
                            boyut: 11.0,
                            hiza: MetinHizası::Orta,
                        });
                    }
                    let mut birikim = 0.0_f64;
                    for (çubuk_sırası, seri_indeksi) in çubuk_serileri.iter().copied().enumerate()
                    {
                        let Some(değerler) = self.veri.seriler().get(seri_indeksi) else {
                            continue;
                        };
                        let Some(değer) = değerler.get(indeks).copied().flatten() else {
                            continue;
                        };
                        let seri = self.seçenekler.seriler.get(seri_indeksi);
                        let seri_aralığı = seri.map_or(aralık, |seri| {
                            if seri.ölçek == self.seçenekler.birincil_y_ölçeği {
                                aralık
                            } else {
                                self.görünür_ölçek_aralığı(
                                    &seri.ölçek,
                                    x_aralığı,
                                    görünür_y,
                                )
                            }
                        });
                        let istenen_vuruş = seri.map_or(0.0, |seri| seri.çizgi_kalınlığı);
                        // `bars.js`, canvas genişliğini piksel ızgarasına yuvarlayan
                        // `pxRound` sonucu üzerinden ince vuruşu düşürür. Vektör
                        // sahnesinde de aynı sınır kararını CSS pikseline hizalanmış
                        // bar genişliğiyle vermek 800/1000 px kaynak eşiğini korur.
                        let eşik_genişliği = piksele_hizala(
                            grup_genişliği,
                            self.seçenekler.piksel_hizası,
                            self.cihaz_piksel_oranı,
                        );
                        let vuruş = if istenen_vuruş >= eşik_genişliği / 2.0 {
                            0.0
                        } else {
                            istenen_vuruş
                        };
                        let iç_vuruş = tam_boşluk > 0.0 && vuruş > 0.0;
                        let ham_genişlik = if düzen.grupları_kenarlara_yay {
                            grup_genişliği
                        } else {
                            grup_adımı - tam_boşluk
                        } - if iç_vuruş { vuruş } else { 0.0 };
                        let kaynak_genişliği = ham_genişlik.max(1.0);
                        let yön = if düzen.ters { -1_i8 } else { 1_i8 };
                        let hizalama = düzen.hizalama;
                        let kaydırma = if hizalama == 0 {
                            kaynak_genişliği / 2.0
                        } else if hizalama == yön {
                            0.0
                        } else {
                            kaynak_genişliği
                        } - f32::from(hizalama * yön)
                            * (if hizalama == 0 {
                                düzen.ek_boşluk / 2.0
                            } else {
                                0.0
                            } + if iç_vuruş { vuruş / 2.0 } else { 0.0 });
                        let kaynak_x = merkez - kaydırma;
                        let (x, genişlik) = if düzen.yığılmış {
                            (kaynak_x, kaynak_genişliği)
                        } else {
                            let genişlik = kaynak_genişliği / seri_sayısı as f32;
                            (kaynak_x + çubuk_sırası as f32 * genişlik, genişlik)
                        };
                        let taban = if düzen.yığılmış { birikim } else { 0.0 };
                        birikim += if düzen.yığılmış { değer } else { 0.0 };
                        let tepe = if düzen.yığılmış {
                            birikim
                        } else {
                            değer
                        };
                        if !seri.is_some_and(|seri| seri.göster) {
                            continue;
                        }
                        let y0 = alt - seri_aralığı.konum(taban, 0.0, çizim_y);
                        let y1 = alt - seri_aralığı.konum(tepe, 0.0, çizim_y);
                        let nokta_komutu = seri
                            .filter(|seri| {
                                self.seçenekler.kırılım_noktaları_görünür
                                    && seri.noktaları_göster.unwrap_or_else(|| {
                                        seri.nokta_boşluğu <= 0.0
                                            || görünür_indeks_sayısı.saturating_sub(1) as f32
                                                <= nokta_piksel_açıklığı
                                                    / seri.nokta_boşluğu.max(f32::EPSILON)
                                    })
                            })
                            .map(|seri| {
                                let dolgu = seri
                                    .nokta_dolgusu
                                    .clone()
                                    .unwrap_or_else(|| "#ffffff".to_string());
                                Komut::Daire {
                                    merkez: Nokta::yeni(merkez, y1.clamp(üst, alt)),
                                    yarıçap: ((seri.nokta_boyutu - seri.nokta_kalınlığı) / 2.0)
                                        .max(0.0),
                                    dolgu,
                                    çizgi: seri.renk.clone(),
                                    kalınlık: seri.nokta_kalınlığı,
                                }
                            });
                        if (tepe - taban).abs() <= f64::EPSILON {
                            if let Some(komut) = nokta_komutu {
                                sahne.ekle(komut);
                            }
                            continue;
                        }
                        let çubuk_sol = x.clamp(sol, sağ);
                        let çubuk_sağ = (x + genişlik).clamp(sol, sağ);
                        let vuruş_rengi =
                            seri.map_or_else(|| "#6b7280".to_string(), |seri| seri.renk.clone());
                        let normal_dolgu = seri
                            .and_then(|seri| {
                                seri.çubuk_dolguları
                                    .get(indeks)
                                    .cloned()
                                    .or_else(|| seri.dolgu.clone())
                            })
                            .unwrap_or_else(|| vuruş_rengi.clone());
                        let dolgu = if vuruş <= 0.0 && istenen_vuruş > 0.0 {
                            vuruş_rengi.clone()
                        } else {
                            normal_dolgu
                        };
                        let çubuk_üst = y1.min(y0).clamp(üst, alt);
                        let çubuk_alt = y1.max(y0).clamp(üst, alt);
                        let çubuk_genişliği = (çubuk_sağ - çubuk_sol).max(0.0);
                        let çubuk_yüksekliği = (çubuk_alt - çubuk_üst).max(0.0);
                        let nokta_çizgisi = seri
                            .and_then(|seri| seri.çubuk_çizgileri.get(indeks))
                            .cloned()
                            .unwrap_or(vuruş_rengi);
                        let çubuk_konumu = Nokta::yeni(çubuk_sol, çubuk_üst);
                        sahne.ekle(çubuk_komutu(
                            çubuk_konumu,
                            çubuk_genişliği,
                            çubuk_yüksekliği,
                            dolgu,
                            nokta_çizgisi,
                            vuruş,
                            düzen.uç_yarıçap_oranı,
                            düzen.yön,
                            değer < 0.0,
                        ));
                        if çubuk_genişliği > 0.0 && çubuk_yüksekliği > 0.0 {
                            vuruş_kayıtları.push(ÇubukVuruşKaydı {
                                seri: seri_indeksi,
                                indeks,
                                konum: çubuk_konumu,
                                genişlik: çubuk_genişliği,
                                yükseklik: çubuk_yüksekliği,
                                değer: if düzen.yığılmış { tepe } else { değer },
                            });
                        }
                        if let Some(komut) = nokta_komutu {
                            sahne.ekle(komut);
                        }
                        if let Some(yazı_boyutu) = otomatik_yazı_boyutu {
                            let (alan_y, alan_yüksekliği) = if değer < 0.0 {
                                (y1, alt - y1)
                            } else {
                                (üst, y1 - üst)
                            };
                            sahne.ekle(Komut::Dikdörtgen {
                                konum: Nokta::yeni(çubuk_sol, alan_y),
                                genişlik: (çubuk_sağ - çubuk_sol).max(0.0),
                                yükseklik: alan_yüksekliği.max(0.0),
                                dolgu: "#00ff0022".to_string(),
                                çizgi: "none".to_string(),
                                kalınlık: 0.0,
                            });
                            if yazı_boyutu >= 10.0 {
                                let etiket_y = if değer < 0.0 {
                                    y1 + yazı_boyutu
                                } else {
                                    y1 - yazı_boyutu * 0.4
                                };
                                let etiket_x = x + genişlik / 2.0;
                                if etiket_x >= sol
                                    && etiket_x <= sağ
                                    && etiket_y - yazı_boyutu >= üst
                                    && etiket_y <= alt
                                {
                                    sahne.ekle(Komut::Metin {
                                        konum: Nokta::yeni(etiket_x, etiket_y),
                                        içerik: self
                                            .otomatik_çubuk_metinleri
                                            .as_ref()
                                            .and_then(|önbellek| {
                                                önbellek.gösterimler.get(seri_indeksi)
                                            })
                                            .and_then(|seri| seri.get(indeks))
                                            .and_then(Clone::clone)
                                            .unwrap_or_else(|| kompakt_sayı(değer)),
                                        renk: "#111111".to_string(),
                                        boyut: yazı_boyutu,
                                        hiza: MetinHizası::Orta,
                                    });
                                }
                            }
                        } else if düzen.değer_etiketleri {
                            let etiket_y = if değer < 0.0 { y1 + 11.0 } else { y1 - 2.0 };
                            let etiket_x = x + genişlik / 2.0;
                            if etiket_x >= sol
                                && etiket_x <= sağ
                                && etiket_y - 10.0 >= üst
                                && etiket_y <= alt
                            {
                                sahne.ekle(Komut::Metin {
                                    konum: Nokta::yeni(etiket_x, etiket_y),
                                    içerik: format!("{tepe}"),
                                    renk: "#111111".to_string(),
                                    boyut: 10.0,
                                    hiza: MetinHizası::Orta,
                                });
                            }
                        }
                    }
                }
            }
            crate::ÇubukYönü::Yatay => {
                let (sol, sağ, üst, alt) = (
                    150.0,
                    genişlik_px as f32 - 32.0,
                    48.0,
                    yükseklik_px as f32 - 48.0,
                );
                let çizim_g = sağ - sol;
                let çizim_y = alt - üst;
                for değer in eksen_bölmeleri(aralık, çizim_g, 50.0) {
                    let x = aralık.konum(değer, sol, çizim_g);
                    sahne.ekle(Komut::Çizgi {
                        başlangıç: Nokta::yeni(x, üst),
                        bitiş: Nokta::yeni(x, alt),
                        renk: "#e5e7eb".to_string(),
                        kalınlık: 1.0,
                    });
                    sahne.ekle(Komut::Metin {
                        konum: Nokta::yeni(x, alt + 20.0),
                        içerik: format!(
                            "{}{}",
                            eksen_değerini_yaz(değer, uygun_artım(aralık, çizim_g, 50.0)),
                            birincil_y_birimi
                        ),
                        renk: "#4b5563".to_string(),
                        boyut: 11.0,
                        hiza: MetinHizası::Orta,
                    });
                }
                let grup_adımı = çizim_y / x_açıklığı as f32;
                let grup_yüksekliği = if düzen.grupları_kenarlara_yay {
                    space_between_grup_boyutu(
                        self.veri.x(),
                        x_aralığı,
                        çizim_y,
                        düzen.genişlik_oranı,
                    )
                } else {
                    grup_adımı * düzen.genişlik_oranı
                };
                let otomatik_yazı_boyutu = düzen
                    .değer_etiketi_otomatik
                    .then_some((grup_yüksekliği * 0.8).min(25.0));
                for indeks in 0..grup_sayısı {
                    let Some(x_değeri) = self.veri.x().get(indeks).copied() else {
                        continue;
                    };
                    let oran = if düzen.ters {
                        (x_aralığı.en_çok - x_değeri) / x_açıklığı
                    } else {
                        (x_değeri - x_aralığı.en_az) / x_açıklığı
                    };
                    let merkez = üst + oran as f32 * çizim_y;
                    if merkez + grup_yüksekliği / 2.0 < üst || merkez - grup_yüksekliği / 2.0 > alt
                    {
                        continue;
                    }
                    sahne.ekle(Komut::Metin {
                        konum: Nokta::yeni(sol - 10.0, merkez + 4.0),
                        içerik: kategoriler.get(indeks).cloned().unwrap_or_default(),
                        renk: "#4b5563".to_string(),
                        boyut: 11.0,
                        hiza: MetinHizası::Bitiş,
                    });
                    let mut birikim = 0.0_f64;
                    for (çubuk_sırası, seri_indeksi) in çubuk_serileri.iter().copied().enumerate()
                    {
                        let Some(değerler) = self.veri.seriler().get(seri_indeksi) else {
                            continue;
                        };
                        let Some(değer) = değerler.get(indeks).copied().flatten() else {
                            continue;
                        };
                        let seri = self.seçenekler.seriler.get(seri_indeksi);
                        let seri_aralığı = seri.map_or(aralık, |seri| {
                            if seri.ölçek == self.seçenekler.birincil_y_ölçeği {
                                aralık
                            } else {
                                self.görünür_ölçek_aralığı(
                                    &seri.ölçek,
                                    x_aralığı,
                                    görünür_y,
                                )
                            }
                        });
                        let (y, yükseklik) = if düzen.yığılmış {
                            (merkez - grup_yüksekliği / 2.0, grup_yüksekliği)
                        } else {
                            let yükseklik = grup_yüksekliği / seri_sayısı as f32;
                            (
                                merkez - grup_yüksekliği / 2.0 + çubuk_sırası as f32 * yükseklik,
                                yükseklik,
                            )
                        };
                        let taban = if düzen.yığılmış { birikim } else { 0.0 };
                        birikim += if düzen.yığılmış { değer } else { 0.0 };
                        let uç = if düzen.yığılmış {
                            birikim
                        } else {
                            değer
                        };
                        if !seri.is_some_and(|seri| seri.göster) {
                            continue;
                        }
                        let x0 = seri_aralığı.konum(taban, sol, çizim_g);
                        let x1 = seri_aralığı.konum(uç, sol, çizim_g);
                        let çubuk_üst = y.clamp(üst, alt);
                        let çubuk_alt = (y + yükseklik).clamp(üst, alt);
                        let vuruş_rengi = seri
                            .map(|seri| seri.renk.clone())
                            .unwrap_or_else(|| "#6b7280".to_string());
                        let dolgu = seri
                            .and_then(|seri| {
                                seri.çubuk_dolguları
                                    .get(indeks)
                                    .cloned()
                                    .or_else(|| seri.dolgu.clone())
                            })
                            .unwrap_or_else(|| vuruş_rengi.clone());
                        let çizgi = seri
                            .and_then(|seri| seri.çubuk_çizgileri.get(indeks))
                            .cloned()
                            .unwrap_or(vuruş_rengi);
                        let kalınlık = seri.map_or(0.0, |seri| seri.çizgi_kalınlığı);
                        let çubuk_sol = x0.min(x1).clamp(sol, sağ);
                        let çubuk_sağ = x0.max(x1).clamp(sol, sağ);
                        let çubuk_konumu = Nokta::yeni(çubuk_sol, çubuk_üst);
                        let çubuk_genişliği = (çubuk_sağ - çubuk_sol).max(0.0);
                        let çubuk_yüksekliği = (çubuk_alt - çubuk_üst).max(0.0);
                        sahne.ekle(çubuk_komutu(
                            çubuk_konumu,
                            çubuk_genişliği,
                            çubuk_yüksekliği,
                            dolgu,
                            çizgi,
                            kalınlık,
                            düzen.uç_yarıçap_oranı,
                            düzen.yön,
                            değer < 0.0,
                        ));
                        if çubuk_genişliği > 0.0 && çubuk_yüksekliği > 0.0 {
                            vuruş_kayıtları.push(ÇubukVuruşKaydı {
                                seri: seri_indeksi,
                                indeks,
                                konum: çubuk_konumu,
                                genişlik: çubuk_genişliği,
                                yükseklik: çubuk_yüksekliği,
                                değer: if düzen.yığılmış { uç } else { değer },
                            });
                        }
                        if let Some(yazı_boyutu) = otomatik_yazı_boyutu {
                            let (alan_x, alan_genişliği) = if değer < 0.0 {
                                (sol, x1 - sol)
                            } else {
                                (x1, sağ - x1)
                            };
                            sahne.ekle(Komut::Dikdörtgen {
                                konum: Nokta::yeni(alan_x, çubuk_üst),
                                genişlik: alan_genişliği.max(0.0),
                                yükseklik: (çubuk_alt - çubuk_üst).max(0.0),
                                dolgu: "#00ff0022".to_string(),
                                çizgi: "none".to_string(),
                                kalınlık: 0.0,
                            });
                            if yazı_boyutu >= 10.0 {
                                let metin = self
                                    .otomatik_çubuk_metinleri
                                    .as_ref()
                                    .and_then(|önbellek| önbellek.gösterimler.get(seri_indeksi))
                                    .and_then(|seri| seri.get(indeks))
                                    .and_then(Clone::clone)
                                    .unwrap_or_else(|| kompakt_sayı(değer));
                                let yarım_metin = metin.chars().count() as f32 * yazı_boyutu * 0.3;
                                let etiket_x = if değer < 0.0 {
                                    x1 - yarım_metin - yazı_boyutu * 0.4
                                } else {
                                    x1 + yarım_metin + yazı_boyutu * 0.4
                                };
                                let etiket_y = y + yükseklik / 2.0 + 4.0;
                                if etiket_x - yarım_metin >= sol
                                    && etiket_x + yarım_metin <= sağ
                                    && etiket_y - yazı_boyutu >= üst
                                    && etiket_y <= alt
                                {
                                    sahne.ekle(Komut::Metin {
                                        konum: Nokta::yeni(etiket_x, etiket_y),
                                        içerik: metin,
                                        renk: "#111111".to_string(),
                                        boyut: yazı_boyutu,
                                        hiza: MetinHizası::Orta,
                                    });
                                }
                            }
                        } else if düzen.değer_etiketleri {
                            let (etiket_x, hiza) = if değer < 0.0 {
                                (x1 - 3.0, MetinHizası::Bitiş)
                            } else {
                                (x1 + 3.0, MetinHizası::Başlangıç)
                            };
                            let içerik = format!("{uç}");
                            let metin_genişliği = içerik.chars().count() as f32 * 5.5;
                            let etiket_y = y + yükseklik / 2.0 + 4.0;
                            let yatay_sığıyor = if değer < 0.0 {
                                etiket_x - metin_genişliği >= sol
                            } else {
                                etiket_x + metin_genişliği <= sağ
                            };
                            if yatay_sığıyor && etiket_y - 10.0 >= üst && etiket_y <= alt {
                                sahne.ekle(Komut::Metin {
                                    konum: Nokta::yeni(etiket_x, etiket_y),
                                    içerik,
                                    renk: "#111111".to_string(),
                                    boyut: 10.0,
                                    hiza,
                                });
                            }
                        }
                    }
                }
            }
        }
        *self.çubuk_vuruş_dizini.borrow_mut() =
            Some(ÇubukVuruşDizini::yeni(vuruş_anahtarı, vuruş_kayıtları));
        for seri_indeksi in çizgi_serileri {
            self.gruplu_çizgi_serisini_çiz(
                sahne,
                genişlik_px,
                yükseklik_px,
                düzen,
                seri_indeksi,
                x_aralığı,
                aralık,
                görünür_y,
            );
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn gruplu_çizgi_serisini_çiz(
        &self,
        sahne: &mut Sahne,
        genişlik_px: u32,
        yükseklik_px: u32,
        düzen: crate::ÇubukDüzeni,
        seri_indeksi: usize,
        x_aralığı: Aralık,
        birincil_aralık: Aralık,
        görünür_y: Option<Aralık>,
    ) {
        let Some(seri) = self.seçenekler.seriler.get(seri_indeksi) else {
            return;
        };
        if !seri.göster {
            return;
        }
        let Some(değerler) = self.veri.seriler().get(seri_indeksi) else {
            return;
        };
        let seri_aralığı = if seri.ölçek == self.seçenekler.birincil_y_ölçeği {
            birincil_aralık
        } else {
            self.görünür_ölçek_aralığı(&seri.ölçek, x_aralığı, görünür_y)
        };
        let x_açıklığı = (x_aralığı.en_çok - x_aralığı.en_az).max(f64::EPSILON);
        let (sol, sağ, üst, alt) = match düzen.yön {
            crate::ÇubukYönü::Dikey => (
                64.0,
                genişlik_px as f32 - 24.0,
                48.0,
                yükseklik_px as f32 - 72.0,
            ),
            crate::ÇubukYönü::Yatay => (
                150.0,
                genişlik_px as f32 - 32.0,
                48.0,
                yükseklik_px as f32 - 48.0,
            ),
        };
        let mut noktalar = Vec::new();
        for (indeks, x_değeri) in self.veri.x().iter().copied().enumerate() {
            if x_değeri < x_aralığı.en_az || x_değeri > x_aralığı.en_çok {
                continue;
            }
            let Some(değer) = değerler.get(indeks).copied().flatten() else {
                continue;
            };
            let oran = if düzen.ters {
                (x_aralığı.en_çok - x_değeri) / x_açıklığı
            } else {
                (x_değeri - x_aralığı.en_az) / x_açıklığı
            };
            let nokta = match düzen.yön {
                crate::ÇubukYönü::Dikey => Nokta::yeni(
                    sol + oran as f32 * (sağ - sol),
                    alt - seri_aralığı.konum(değer, 0.0, alt - üst),
                ),
                crate::ÇubukYönü::Yatay => Nokta::yeni(
                    seri_aralığı.konum(değer, sol, sağ - sol),
                    üst + oran as f32 * (alt - üst),
                ),
            };
            noktalar.push(nokta);
        }
        if noktalar.len() < 2 {
            return;
        }
        sahne.ekle(Komut::Yol {
            parçalar: vec![noktalar.clone()],
            renk: seri.renk.clone(),
            kalınlık: seri.çizgi_kalınlığı.max(1.0),
        });
        for merkez in noktalar {
            sahne.ekle(Komut::Daire {
                merkez,
                yarıçap: 3.0,
                dolgu: "#ffffff".to_string(),
                çizgi: seri.renk.clone(),
                kalınlık: 1.5,
            });
        }
    }

    fn kutu_bıyık_y_aralığı(&self) -> Aralık {
        let mut değerler = self
            .veri
            .seriler()
            .iter()
            .flat_map(|seri| seri.iter().copied().flatten())
            .collect::<Vec<_>>();
        if let Some(düzen) = &self.seçenekler.kutu_bıyık_düzeni {
            değerler.extend(
                düzen
                    .ayrık_değerler
                    .iter()
                    .flat_map(|ayrıklar| ayrıklar.iter().copied()),
            );
        }
        sonlu_aralık(değerler.into_iter())
            .and_then(|ham| Aralık::uplot_sayısal(ham.en_az, ham.en_çok, 0.1, true).ok())
            .unwrap_or(Aralık {
                en_az: 0.0,
                en_çok: 1.0,
            })
    }

    fn kutu_bıyıkları_çiz(
        &self,
        sahne: &mut Sahne,
        genişlik_px: u32,
        yükseklik_px: u32,
        düzen: &crate::KutuBıyıkDüzeni,
        görünür_x: Option<Aralık>,
        görünür_y: Option<Aralık>,
    ) {
        let (sol, sağ, üst, alt) = self.çizim_alanı_boyutta(genişlik_px, yükseklik_px);
        let çizim_g = sağ - sol;
        let çizim_y = alt - üst;
        let grup_sayısı = self.veri.uzunluk();
        if grup_sayısı == 0 || çizim_g <= 0.0 || çizim_y <= 0.0 {
            return;
        }
        let tam_x = tam_x_aralığı(&self.veri)
            .ok()
            .and_then(|aralık| {
                if grup_sayısı > 1 {
                    Aralık::yeni(aralık.en_az - 0.5, aralık.en_çok + 0.5).ok()
                } else {
                    Some(aralık)
                }
            })
            .unwrap_or(Aralık {
                en_az: -0.5,
                en_çok: 0.5,
            });
        let x_aralığı = görünür_x.unwrap_or(tam_x);
        let y_aralığı = görünür_y.unwrap_or_else(|| self.kutu_bıyık_y_aralığı());
        let x_açıklığı = (x_aralığı.en_çok - x_aralığı.en_az).max(f64::EPSILON);
        let sütun_genişliği = çizim_g / x_açıklığı as f32;
        let gövde_genişliği = (düzen.gövde_genişlik_oranı * (sütun_genişliği - 2.0)).max(1.0);
        let artım = uygun_artım(y_aralığı, çizim_y, 30.0);

        for değer in eksen_bölmeleri(y_aralığı, çizim_y, 30.0) {
            let y = alt - y_aralığı.konum(değer, 0.0, çizim_y);
            sahne.ekle(Komut::Çizgi {
                başlangıç: Nokta::yeni(sol, y),
                bitiş: Nokta::yeni(sağ, y),
                renk: "#e5e7eb".to_string(),
                kalınlık: 1.0,
            });
            sahne.ekle(Komut::Metin {
                konum: Nokta::yeni(sol - 8.0, y + 4.0),
                içerik: eksen_değerini_yaz(değer, artım),
                renk: "#4b5563".to_string(),
                boyut: 11.0,
                hiza: MetinHizası::Bitiş,
            });
        }

        for indeks in 0..grup_sayısı {
            let Some(x_değeri) = self.veri.x().get(indeks).copied() else {
                continue;
            };
            let merkez = sol + ((x_değeri - x_aralığı.en_az) / x_açıklığı) as f32 * çizim_g;
            if merkez + gövde_genişliği / 2.0 < sol || merkez - gövde_genişliği / 2.0 > sağ {
                continue;
            }
            let değer = |seri: usize| {
                self.veri
                    .seriler()
                    .get(seri)
                    .and_then(|değerler| değerler.get(indeks))
                    .copied()
                    .flatten()
            };
            let (Some(medyan), Some(q1), Some(q3)) = (değer(0), değer(1), değer(2)) else {
                continue;
            };
            let en_az = değer(3);
            let en_çok = değer(4);
            let y_konumu = |değer| alt - y_aralığı.konum(değer, 0.0, çizim_y);
            let medyan_y = y_konumu(medyan).clamp(üst, alt);
            let q1_y = y_konumu(q1).clamp(üst, alt);
            let q3_y = y_konumu(q3).clamp(üst, alt);
            let gövde_sol = (merkez - gövde_genişliği / 2.0).clamp(sol, sağ);
            let gövde_sağ = (merkez + gövde_genişliği / 2.0).clamp(sol, sağ);
            let gövde_üst = q1_y.min(q3_y);
            let gövde_alt = q1_y.max(q3_y);

            if let (Some(en_az), Some(en_çok)) = (en_az, en_çok) {
                let min_y = y_konumu(en_az).clamp(üst, alt);
                let max_y = y_konumu(en_çok).clamp(üst, alt);
                sahne.ekle(Komut::KesikliÇizgi {
                    başlangıç: Nokta::yeni(merkez.clamp(sol, sağ), max_y.min(min_y)),
                    bitiş: Nokta::yeni(merkez.clamp(sol, sağ), max_y.max(min_y)),
                    renk: "#000000".to_string(),
                    kalınlık: 2.0,
                    kesik: 4.0,
                });
                for y in [min_y, max_y] {
                    sahne.ekle(Komut::Çizgi {
                        başlangıç: Nokta::yeni(gövde_sol, y),
                        bitiş: Nokta::yeni(gövde_sağ, y),
                        renk: "#000000".to_string(),
                        kalınlık: 2.0,
                    });
                }
            }
            sahne.ekle(Komut::Dikdörtgen {
                konum: Nokta::yeni(gövde_sol, gövde_üst),
                genişlik: (gövde_sağ - gövde_sol).max(0.0),
                yükseklik: (gövde_alt - gövde_üst).max(0.0),
                dolgu: "#eeeeee".to_string(),
                çizgi: "#000000".to_string(),
                kalınlık: 1.0,
            });
            sahne.ekle(Komut::Dikdörtgen {
                konum: Nokta::yeni(gövde_sol, medyan_y - 1.0),
                genişlik: (gövde_sağ - gövde_sol).max(0.0),
                yükseklik: 2.0,
                dolgu: "#000000".to_string(),
                çizgi: "#000000".to_string(),
                kalınlık: 0.0,
            });
            if let Some(ayrıklar) = düzen.ayrık_değerler.get(indeks) {
                for ayrık in ayrıklar {
                    let y = y_konumu(*ayrık);
                    if y >= üst && y <= alt {
                        sahne.ekle(Komut::Dikdörtgen {
                            konum: Nokta::yeni(merkez - 4.0, y - 4.0),
                            genişlik: 8.0,
                            yükseklik: 8.0,
                            dolgu: "#000000".to_string(),
                            çizgi: "#000000".to_string(),
                            kalınlık: 0.0,
                        });
                    }
                }
            }
            let etiket = self
                .seçenekler
                .kategoriler
                .get(indeks)
                .cloned()
                .unwrap_or_default();
            sahne.ekle(Komut::DöndürülmüşMetin {
                konum: Nokta::yeni(merkez, alt + 8.0),
                içerik: etiket,
                renk: "#4b5563".to_string(),
                boyut: 10.0,
                hiza: MetinHizası::Bitiş,
                açı: -90.0,
            });
        }
    }

    fn mumları_çiz(
        &self,
        sahne: &mut Sahne,
        genişlik_px: u32,
        yükseklik_px: u32,
        düzen: &crate::MumDüzeni,
        görünür_x: Option<Aralık>,
        görünür_y: Option<Aralık>,
    ) {
        let (sol, sağ, üst, alt) = self.çizim_alanı_boyutta(genişlik_px, yükseklik_px);
        let çizim_g = sağ - sol;
        let çizim_y = alt - üst;
        // Kaynak `distr: 2` çizicisi ilk ve son ordinal indeksi doğrudan
        // çizim alanının iki kenarına eşler; ekstra yarım sütun payı eklemez.
        let tam_x = tam_x_aralığı(&self.veri).unwrap_or(Aralık {
            en_az: -0.5,
            en_çok: 0.5,
        });
        let x_aralığı = görünür_x.unwrap_or(tam_x);
        let y_aralığı = görünür_y.unwrap_or_else(|| self.y_aralığı(x_aralığı));
        let x_açıklığı = (x_aralığı.en_çok - x_aralığı.en_az).max(f64::EPSILON);
        let sütun_genişliği = çizim_g / x_açıklığı as f32;
        let gövde_genişliği = düzen
            .azami_gövde_genişliği
            .min((sütun_genişliği - 2.0).max(0.0));
        for değer in eksen_bölmeleri(y_aralığı, çizim_y, 30.0) {
            let y = alt - y_aralığı.konum(değer, 0.0, çizim_y);
            sahne.ekle(Komut::Çizgi {
                başlangıç: Nokta::yeni(sol, y),
                bitiş: Nokta::yeni(sağ, y),
                renk: "#e5e7eb".to_string(),
                kalınlık: 1.0,
            });
            sahne.ekle(Komut::Metin {
                konum: Nokta::yeni(sol - 8.0, y + 4.0),
                içerik: usd_biçimle(değer, 0),
                renk: "#4b5563".to_string(),
                boyut: 11.0,
                hiza: MetinHizası::Bitiş,
            });
        }
        let değer = |seri: usize, indeks: usize| {
            self.veri
                .seriler()
                .get(seri)
                .and_then(|değerler| değerler.get(indeks))
                .copied()
                .flatten()
        };
        let etiket_adımı = ((60.0 / sütun_genişliği.max(1.0)).ceil() as usize).max(1);
        let ilk_görünür = self
            .veri
            .x()
            .partition_point(|değer| *değer < x_aralığı.en_az - 1.0);
        let görünür_bitiş = self
            .veri
            .x()
            .partition_point(|değer| *değer <= x_aralığı.en_çok + 1.0);
        for indeks in ilk_görünür..görünür_bitiş {
            let Some(x_değeri) = self.veri.x().get(indeks).copied() else {
                continue;
            };
            let merkez = sol + ((x_değeri - x_aralığı.en_az) / x_açıklığı) as f32 * çizim_g;
            if merkez + gövde_genişliği / 2.0 < sol || merkez - gövde_genişliği / 2.0 > sağ {
                continue;
            }
            let (Some(açılış), Some(yüksek), Some(düşük), Some(kapanış), Some(hacim)) = (
                değer(0, indeks),
                değer(1, indeks),
                değer(2, indeks),
                değer(3, indeks),
                değer(4, indeks),
            ) else {
                continue;
            };
            let y_konumu = |değer| alt - y_aralığı.konum(değer, 0.0, çizim_y);
            let piksel = self.seçenekler.piksel_hizası;
            let oran = self.cihaz_piksel_oranı;
            let yüksek_y = piksele_hizala(y_konumu(yüksek).clamp(üst, alt), piksel, oran);
            let düşük_y = piksele_hizala(y_konumu(düşük).clamp(üst, alt), piksel, oran);
            let açılış_y = piksele_hizala(y_konumu(açılış).clamp(üst, alt), piksel, oran);
            let kapanış_y = piksele_hizala(y_konumu(kapanış).clamp(üst, alt), piksel, oran);
            let merkez = piksele_hizala(merkez, piksel, oran);
            let gövde_genişliği = piksele_hizala(gövde_genişliği, piksel, oran).max(0.0);
            let renk = if açılış > kapanış {
                &düzen.düşüş_rengi
            } else {
                &düzen.yükseliş_rengi
            };
            let fitil_y = yüksek_y.min(düşük_y);
            let fitil_yüksekliği =
                piksele_hizala((düşük_y - yüksek_y).abs(), piksel, oran).max(0.0);
            sahne.ekle(Komut::Dikdörtgen {
                konum: Nokta::yeni(piksele_hizala(merkez - 1.0, piksel, oran), fitil_y),
                genişlik: 2.0,
                yükseklik: fitil_yüksekliği,
                dolgu: "#000000".to_string(),
                çizgi: "#000000".to_string(),
                kalınlık: 0.0,
            });
            let gövde_x = piksele_hizala(merkez - gövde_genişliği / 2.0, piksel, oran);
            let gövde_y = açılış_y.min(kapanış_y);
            let gövde_yüksekliği =
                piksele_hizala((kapanış_y - açılış_y).abs(), piksel, oran).max(0.0);
            sahne.ekle(Komut::Dikdörtgen {
                konum: Nokta::yeni(gövde_x, gövde_y),
                genişlik: gövde_genişliği,
                yükseklik: gövde_yüksekliği,
                dolgu: "#000000".to_string(),
                çizgi: "none".to_string(),
                kalınlık: 0.0,
            });
            if gövde_genişliği > 2.0 && gövde_yüksekliği > 2.0 {
                sahne.ekle(Komut::Dikdörtgen {
                    konum: Nokta::yeni(gövde_x + 1.0, gövde_y + 1.0),
                    genişlik: gövde_genişliği - 2.0,
                    yükseklik: gövde_yüksekliği - 2.0,
                    dolgu: renk.clone(),
                    çizgi: "none".to_string(),
                    kalınlık: 0.0,
                });
            }
            let hacim_y = piksele_hizala(
                alt - (hacim / 2_000.0).clamp(0.0, 1.0) as f32 * çizim_y,
                piksel,
                oran,
            );
            sahne.ekle(Komut::Dikdörtgen {
                konum: Nokta::yeni(gövde_x, hacim_y),
                genişlik: gövde_genişliği,
                yükseklik: piksele_hizala(alt - hacim_y, piksel, oran).max(0.0),
                dolgu: renk.clone(),
                çizgi: "none".to_string(),
                kalınlık: 0.0,
            });
            if indeks.is_multiple_of(etiket_adımı)
                && let Some(zaman) = düzen.zamanlar.get(indeks)
                && let Some((yıl, ay, gün, ..)) = crate::zaman::utc_alanları(*zaman)
            {
                sahne.ekle(Komut::Metin {
                    konum: Nokta::yeni(merkez, alt + 18.0),
                    içerik: format!("{yıl:04}-{ay:02}-{gün:02}"),
                    renk: "#4b5563".to_string(),
                    boyut: 9.0,
                    hiza: MetinHizası::Orta,
                });
            }
        }
        for değer in [0.0, 500.0, 1_000.0, 1_500.0, 2_000.0] {
            let y = alt - (değer / 2_000.0) as f32 * çizim_y;
            sahne.ekle(Komut::Metin {
                konum: Nokta::yeni(sağ + 8.0, y + 4.0),
                içerik: format!("{değer:.0}"),
                renk: "#4b5563".to_string(),
                boyut: 10.0,
                hiza: MetinHizası::Başlangıç,
            });
        }
    }

    fn güzel_ölçek_aralığı(
        &self,
        anahtar: &str,
        x_aralığı: Aralık,
        çizim_yüksekliği: f32,
    ) -> Option<(Aralık, f64)> {
        let ölçek = self.ölçek_seçeneği(anahtar)?;
        let düzen = ölçek.güzel_ölçek?;
        if ölçek.aralık.is_some()
            || (anahtar == self.seçenekler.birincil_y_ölçeği && self.seçenekler.y_aralığı.is_some())
        {
            return None;
        }
        let ham_aralık = sonlu_aralık(
            self.veri
                .x()
                .iter()
                .enumerate()
                .filter(|(_, x)| **x >= x_aralığı.en_az && **x <= x_aralığı.en_çok)
                .flat_map(|(indeks, _)| {
                    self.veri
                        .seriler()
                        .iter()
                        .zip(self.seçenekler.seriler.iter())
                        .filter(move |(_, ayarlar)| {
                            ayarlar.göster
                                && ayarlar.otomatik_ölçeğe_katıl
                                && ayarlar.ölçek == anahtar
                        })
                        .filter_map(move |(seri, _)| seri.get(indeks).copied().flatten())
                }),
        )?;
        güzel_ölçek(ham_aralık, çizim_yüksekliği, düzen.en_az_etiket_boşluğu)
    }

    fn y_aralığı(&self, x_aralığı: Aralık) -> Aralık {
        self.y_aralığı_ölçek(&self.seçenekler.birincil_y_ölçeği, x_aralığı)
    }

    fn y_aralığı_ölçek(&self, anahtar: &str, x_aralığı: Aralık) -> Aralık {
        if let Some(ölçek) = self.ölçek_seçeneği(anahtar)
            && let Some(kaynak) = ölçek.kaynak.as_deref()
            && kaynak != anahtar
        {
            let kaynak_aralığı = self.ham_y_aralığı_ölçek(kaynak, x_aralığı);
            let dönüştür = |değer: f64| değer * ölçek.dönüşüm_çarpanı + ölçek.dönüşüm_kaydırması;
            let ilk = dönüştür(kaynak_aralığı.en_az);
            let son = dönüştür(kaynak_aralığı.en_çok);
            if let Ok(aralık) = Aralık::yeni(ilk.min(son), ilk.max(son)) {
                return aralık;
            }
        }
        self.ham_y_aralığı_ölçek(anahtar, x_aralığı)
    }

    fn ham_y_aralığı_ölçek(&self, anahtar: &str, x_aralığı: Aralık) -> Aralık {
        self.ölçek_seçeneği(anahtar)
            .and_then(|ölçek| ölçek.aralık)
            .or_else(|| {
                (anahtar == self.seçenekler.birincil_y_ölçeği)
                    .then_some(self.seçenekler.y_aralığı)
                    .flatten()
            })
            .or_else(|| {
                if anahtar != self.seçenekler.birincil_y_ölçeği {
                    return None;
                }
                let sonlu_veri_var = self
                    .veri
                    .seriler()
                    .iter()
                    .zip(self.seçenekler.seriler.iter())
                    .filter(|(_, ayarlar)| {
                        ayarlar.göster && ayarlar.otomatik_ölçeğe_katıl && ayarlar.ölçek == anahtar
                    })
                    .flat_map(|(seri, _)| seri.iter().flatten())
                    .any(|değer| değer.is_finite());
                (!sonlu_veri_var)
                    .then_some(self.seçenekler.boş_y_aralığı)
                    .flatten()
            })
            .unwrap_or_else(|| {
                let nominal_yükseklik = self.seçenekler.yükseklik.saturating_sub(96).max(1) as f32;
                if let Some((aralık, _)) =
                    self.güzel_ölçek_aralığı(anahtar, x_aralığı, nominal_yükseklik)
                {
                    return aralık;
                }
                let görünür = || {
                    self.veri
                        .x()
                        .iter()
                        .enumerate()
                        .filter(|(_, x)| **x >= x_aralığı.en_az && **x <= x_aralığı.en_çok)
                        .flat_map(|(indeks, _)| {
                            self.veri
                                .seriler()
                                .iter()
                                .zip(self.seçenekler.seriler.iter())
                                .filter(move |(_, ayarlar)| {
                                    ayarlar.göster
                                        && ayarlar.otomatik_ölçeğe_katıl
                                        && ayarlar.ölçek == anahtar
                                })
                                .filter_map(move |(seri, _)| seri.get(indeks))
                        })
                };
                match self.ölçek_seçeneği(anahtar).map(|ölçek| ölçek.dağılım) {
                    Some(YÖlçekDağılımı::Logaritmik { taban }) => {
                        let tam = self
                            .ölçek_seçeneği(anahtar)
                            .is_none_or(|ölçek| ölçek.log_tam_büyüklükler);
                        logaritmik_otomatik_aralık(görünür(), taban, tam)
                            .unwrap_or_else(|| Aralık::otomatik(görünür()))
                    }
                    Some(YÖlçekDağılımı::ArcSinh { .. }) => {
                        arcsinh_otomatik_aralık(görünür().flatten().copied())
                            .unwrap_or_else(|| Aralık::otomatik(görünür()))
                    }
                    _ if anahtar == self.seçenekler.birincil_y_ölçeği
                        && self.seçenekler.kütle_spektrumu_y_aralığı =>
                    {
                        sonlu_sınırlar(görünür().flatten().copied())
                            .and_then(|(en_az, en_çok)| {
                                if en_az == en_çok {
                                    let üst = if en_az == 0.0 { 100.0 } else { 2.0 * en_az };
                                    Aralık::yeni(0.0_f64.min(üst), 0.0_f64.max(üst)).ok()
                                } else {
                                    Aralık::yeni(en_az, en_çok).ok()
                                }
                            })
                            .unwrap_or_else(|| Aralık::otomatik(görünür()))
                    }
                    _ => self
                        .ölçek_seçeneği(anahtar)
                        .and_then(|ölçek| ölçek.sayısal_aralık)
                        .and_then(|ayarlar| {
                            sonlu_sınırlar(görünür().flatten().copied()).and_then(
                                |(en_az, en_çok)| {
                                    Aralık::uplot_yapılandırılmış(en_az, en_çok, ayarlar).ok()
                                },
                            )
                        })
                        .unwrap_or_else(|| Aralık::otomatik(görünür())),
                }
            })
    }

    fn görünür_ölçek_aralığı(
        &self,
        anahtar: &str,
        x_aralığı: Aralık,
        görünür_birincil: Option<Aralık>,
    ) -> Aralık {
        if let Some(aralık) = self.elle_y_aralıkları.get(anahtar) {
            return *aralık;
        }
        if anahtar != self.seçenekler.birincil_y_ölçeği
            && self
                .elle_y_aralıkları
                .contains_key(&self.seçenekler.birincil_y_ölçeği)
        {
            return self.y_aralığı_ölçek(anahtar, x_aralığı);
        }
        if anahtar == self.seçenekler.birincil_y_ölçeği {
            return görünür_birincil.unwrap_or_else(|| {
                self.y_aralığı_ölçek(&self.seçenekler.birincil_y_ölçeği, x_aralığı)
            });
        }
        let Some(görünür_birincil) = görünür_birincil else {
            return self.y_aralığı_ölçek(anahtar, x_aralığı);
        };
        let Some(tam_x) = self.tam_x_aralığı() else {
            return self.y_aralığı_ölçek(anahtar, x_aralığı);
        };
        let tam_birincil = self.y_aralığı_ölçek(&self.seçenekler.birincil_y_ölçeği, tam_x);
        let tam_ikincil = self.y_aralığı_ölçek(anahtar, tam_x);
        let birincil_uzunluk = tam_birincil.en_çok - tam_birincil.en_az;
        if birincil_uzunluk <= f64::EPSILON {
            return tam_ikincil;
        }
        let ikincil_uzunluk = tam_ikincil.en_çok - tam_ikincil.en_az;
        let en_az_oran = (görünür_birincil.en_az - tam_birincil.en_az) / birincil_uzunluk;
        let en_çok_oran = (görünür_birincil.en_çok - tam_birincil.en_az) / birincil_uzunluk;
        Aralık::yeni(
            tam_ikincil.en_az + en_az_oran * ikincil_uzunluk,
            tam_ikincil.en_az + en_çok_oran * ikincil_uzunluk,
        )
        .unwrap_or(tam_ikincil)
    }

    fn gradyan_değerlerini_çöz(
        &self,
        gradyan: &ÖlçekGradyanı,
        ölçek: &str,
        x_aralığı: Aralık,
        ölçek_aralığı: Aralık,
    ) -> Option<Vec<(f64, String)>> {
        let göreli = gradyan
            .duraklar
            .iter()
            .any(|durak| matches!(durak.konum, GradyanKonumu::GörünürVeriOranı(_)));
        let (veri_en_az, veri_en_çok) = if göreli {
            self.görünür_veri_aralığı(ölçek, x_aralığı)
                .filter(|(en_az, en_çok)| en_çok - en_az > f64::EPSILON)
                .unwrap_or((ölçek_aralığı.en_az, ölçek_aralığı.en_çok))
        } else {
            (ölçek_aralığı.en_az, ölçek_aralığı.en_çok)
        };
        let veri_aralığı = veri_en_çok - veri_en_az;
        gradyan
            .duraklar
            .iter()
            .map(|durak| {
                let değer = match durak.konum {
                    GradyanKonumu::Değer(değer) => değer,
                    GradyanKonumu::NegatifSonsuz => f64::NEG_INFINITY,
                    GradyanKonumu::PozitifSonsuz => f64::INFINITY,
                    GradyanKonumu::GörünürVeriOranı(oran) => veri_en_az + veri_aralığı * oran,
                };
                (!değer.is_nan()).then(|| (değer, durak.renk.clone()))
            })
            .collect()
    }

    fn görünür_veri_aralığı(
        &self, ölçek: &str, x_aralığı: Aralık
    ) -> Option<(f64, f64)> {
        let mut en_az = f64::INFINITY;
        let mut en_çok = f64::NEG_INFINITY;
        for (seri, _) in self
            .veri
            .seriler()
            .iter()
            .zip(self.seçenekler.seriler.iter())
            .filter(|(_, seçenek)| seçenek.göster && seçenek.ölçek == ölçek)
        {
            for (x, değer) in self.veri.x().iter().zip(seri.iter()) {
                if *x < x_aralığı.en_az || *x > x_aralığı.en_çok {
                    continue;
                }
                let Some(değer) = değer else { continue };
                en_az = en_az.min(*değer);
                en_çok = en_çok.max(*değer);
            }
        }
        (en_az.is_finite() && en_çok.is_finite()).then_some((en_az, en_çok))
    }

    #[allow(clippy::too_many_arguments)]
    fn ölçek_gradyanını_çöz(
        &self,
        gradyan: &ÖlçekGradyanı,
        ölçek: &str,
        x_aralığı: Aralık,
        y_aralığı: Aralık,
        sol: f32,
        üst: f32,
        genişlik: f32,
        yükseklik: f32,
    ) -> Option<DoğrusalGradyan> {
        let ölçek_aralığı = match gradyan.eksen {
            GradyanEkseni::X => x_aralığı,
            GradyanEkseni::Y => y_aralığı,
        };
        let değerler = self.gradyan_değerlerini_çöz(gradyan, ölçek, x_aralığı, ölçek_aralığı)?;
        let mut en_az_indeks = None;
        let mut en_çok_indeks = None;
        for (indeks, (değer, _)) in değerler.iter().enumerate() {
            if *değer <= ölçek_aralığı.en_az || en_az_indeks.is_none() {
                en_az_indeks = Some(indeks);
            }
            en_çok_indeks = Some(indeks);
            if *değer >= ölçek_aralığı.en_çok {
                break;
            }
        }
        let (en_az_indeks, en_çok_indeks) = (en_az_indeks?, en_çok_indeks?);
        let en_az_durak = değerler.get(en_az_indeks)?;
        let en_çok_durak = değerler.get(en_çok_indeks)?;
        let en_az_değer = if en_az_durak.0.is_infinite() {
            ölçek_aralığı.en_az
        } else {
            en_az_durak.0
        };
        let en_çok_değer = if en_çok_durak.0.is_infinite() {
            ölçek_aralığı.en_çok
        } else {
            en_çok_durak.0
        };
        let alt = üst + yükseklik;
        let konum = |değer: f64| match gradyan.eksen {
            GradyanEkseni::X => self.x_konumu(x_aralığı, değer, sol, genişlik),
            GradyanEkseni::Y => alt - self.y_konumu(ölçek, y_aralığı, değer, 0.0, yükseklik),
        };
        let en_az_konum = konum(en_az_değer);
        let en_çok_konum = konum(en_çok_değer);
        let başlangıç = match gradyan.eksen {
            GradyanEkseni::X => Nokta::yeni(en_az_konum, üst),
            GradyanEkseni::Y => Nokta::yeni(sol, en_az_konum),
        };
        let bitiş = match gradyan.eksen {
            GradyanEkseni::X => Nokta::yeni(en_çok_konum, üst),
            GradyanEkseni::Y => Nokta::yeni(sol, en_çok_konum),
        };
        if en_az_indeks == en_çok_indeks || (en_az_konum - en_çok_konum).abs() <= f32::EPSILON {
            return Some(DoğrusalGradyan {
                başlangıç,
                bitiş,
                duraklar: vec![
                    GradyanRenkDurağı {
                        oran: 0.0,
                        renk: en_az_durak.1.clone(),
                    },
                    GradyanRenkDurağı {
                        oran: 1.0,
                        renk: en_az_durak.1.clone(),
                    },
                ],
            });
        }
        let seçilenler = değerler.get(en_az_indeks..=en_çok_indeks)?;
        let fark = en_az_konum - en_çok_konum;
        let mut duraklar = Vec::new();
        let mut önceki_renk = None::<String>;
        for (yerel_indeks, (değer, renk)) in seçilenler.iter().enumerate() {
            let durak_konumu = if yerel_indeks == 0 {
                en_az_konum
            } else if yerel_indeks + 1 == seçilenler.len() {
                en_çok_konum
            } else {
                konum(*değer)
            };
            let oran = ((en_az_konum - durak_konumu) / fark).clamp(0.0, 1.0);
            if gradyan.ayrık
                && yerel_indeks > 0
                && let Some(önceki_renk) = önceki_renk.as_ref()
            {
                duraklar.push(GradyanRenkDurağı {
                    oran,
                    renk: önceki_renk.clone(),
                });
            }
            duraklar.push(GradyanRenkDurağı {
                oran,
                renk: renk.clone(),
            });
            önceki_renk = Some(renk.clone());
        }
        Some(DoğrusalGradyan {
            başlangıç,
            bitiş,
            duraklar,
        })
    }

    fn tam_x_aralığı(&self) -> Option<Aralık> {
        self.seçenekler
            .x_aralığı
            .or_else(|| tam_x_aralığı(&self.veri).ok())
    }

    fn ölçek_seçeneği(&self, anahtar: &str) -> Option<&crate::YÖlçekSeçenekleri> {
        self.seçenekler
            .y_ölçekleri
            .iter()
            .find(|ölçek| ölçek.anahtar == anahtar)
    }

    fn y_konumu(
        &self,
        anahtar: &str,
        aralık: Aralık,
        değer: f64,
        başlangıç: f32,
        uzunluk: f32,
    ) -> f32 {
        let ölçek = self.ölçek_seçeneği(anahtar);
        let konum = match ölçek.map(|ölçek| ölçek.dağılım) {
            Some(YÖlçekDağılımı::Logaritmik { taban })
                if taban.is_finite() && taban > 1.0 && aralık.en_az > 0.0 =>
            {
                let dönüştür = |sayı: f64| sayı.log(taban);
                let değer = if değer > 0.0 {
                    değer
                } else {
                    aralık.en_az / taban
                };
                dönüştürülmüş_konum(aralık, değer, başlangıç, uzunluk, dönüştür)
            }
            Some(YÖlçekDağılımı::Weibull)
                if aralık.en_az > 0.0 && aralık.en_çok < 1.0 && değer > 0.0 && değer < 1.0 =>
            {
                let dönüştür = |sayı: f64| (-(-sayı).ln_1p()).ln();
                dönüştürülmüş_konum(aralık, değer, başlangıç, uzunluk, dönüştür)
            }
            Some(YÖlçekDağılımı::Özel(dönüşüm)) => {
                let dönüştürülmüş = (dönüşüm.ileri)(aralık.en_az)
                    .zip((dönüşüm.ileri)(aralık.en_çok))
                    .zip((dönüşüm.ileri)(değer));
                dönüştürülmüş.map_or_else(
                    || aralık.konum(değer, başlangıç, uzunluk),
                    |((en_az, en_çok), değer)| {
                        let oran = (değer - en_az) / (en_çok - en_az);
                        başlangıç + oran as f32 * uzunluk
                    },
                )
            }
            Some(YÖlçekDağılımı::ArcSinh { eşik }) if eşik.is_finite() && eşik > 0.0 => {
                let dönüştür = |sayı: f64| (sayı / eşik).asinh();
                let en_az = dönüştür(aralık.en_az);
                let en_çok = dönüştür(aralık.en_çok);
                let değer = dönüştür(değer);
                let oran = (değer - en_az) / (en_çok - en_az);
                başlangıç + oran as f32 * uzunluk
            }
            _ => aralık.konum(değer, başlangıç, uzunluk),
        };
        if ölçek.is_some_and(|ölçek| ölçek.ters_yön) {
            başlangıç + uzunluk - (konum - başlangıç)
        } else {
            konum
        }
    }

    fn y_eksen_bölmeleri(&self, anahtar: &str, aralık: Aralık, boyut: f32) -> Vec<f64> {
        let ölçek = self.ölçek_seçeneği(anahtar);
        match ölçek.map(|ölçek| ölçek.dağılım) {
            Some(YÖlçekDağılımı::ArcSinh { eşik }) if eşik.is_finite() && eşik > 0.0 => {
                arcsinh_bölmeleri(aralık, eşik)
            }
            Some(YÖlçekDağılımı::Logaritmik { taban }) if aralık.en_az > 0.0 => {
                logaritmik_bölmeler(aralık, taban)
            }
            Some(YÖlçekDağılımı::Weibull) => [
                0.00001, 0.0001, 0.001, 0.01, 0.1, 0.2, 0.3, 0.5, 0.6, 0.7, 0.8, 0.9, 0.95, 0.99,
                0.999, 0.9999, 0.99999, 0.999999,
            ]
            .into_iter()
            .filter(|değer| (*değer >= aralık.en_az) && (*değer <= aralık.en_çok))
            .collect(),
            Some(YÖlçekDağılımı::Özel(_)) => eksen_bölmeleri(
                aralık,
                boyut,
                ölçek.map_or(30.0, |ölçek| ölçek.eksen_en_az_etiket_boşluğu),
            ),
            _ => eksen_bölmeleri(
                aralık,
                boyut,
                ölçek.map_or(30.0, |ölçek| ölçek.eksen_en_az_etiket_boşluğu),
            ),
        }
    }

    fn x_konumu(&self, aralık: Aralık, değer: f64, başlangıç: f32, uzunluk: f32) -> f32 {
        let konum = match self.seçenekler.x_dağılımı {
            XÖlçekDağılımı::Logaritmik { taban }
                if aralık.en_az > 0.0 && değer > 0.0 && taban > 1.0 =>
            {
                dönüştürülmüş_konum(aralık, değer, başlangıç, uzunluk, |sayı| {
                    sayı.log(taban)
                })
            }
            _ => aralık.konum(değer, başlangıç, uzunluk),
        };
        if self.seçenekler.x_ters_yön {
            başlangıç + uzunluk - (konum - başlangıç)
        } else {
            konum
        }
    }

    fn x_değeri_orandan(&self, aralık: Aralık, oran: f64) -> f64 {
        let oran = if self.seçenekler.x_ters_yön {
            1.0 - oran
        } else {
            oran
        };
        match self.seçenekler.x_dağılımı {
            XÖlçekDağılımı::Logaritmik { taban } if aralık.en_az > 0.0 && taban > 1.0 => {
                let en_az = aralık.en_az.log(taban);
                let en_çok = aralık.en_çok.log(taban);
                taban.powf(en_az + oran * (en_çok - en_az))
            }
            _ => aralık.en_az + oran * (aralık.en_çok - aralık.en_az),
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn çubuk_komutu(
    konum: Nokta,
    genişlik: f32,
    yükseklik: f32,
    dolgu: String,
    çizgi: String,
    kalınlık: f32,
    uç_yarıçap_oranı: f32,
    yön: crate::ÇubukYönü,
    negatif: bool,
) -> Komut {
    if uç_yarıçap_oranı <= 0.0 {
        return Komut::Dikdörtgen {
            konum,
            genişlik,
            yükseklik,
            dolgu,
            çizgi,
            kalınlık,
        };
    }
    let yarıçap = match yön {
        crate::ÇubukYönü::Dikey => genişlik * uç_yarıçap_oranı,
        crate::ÇubukYönü::Yatay => yükseklik * uç_yarıçap_oranı,
    }
    .min(genişlik / 2.0)
    .min(yükseklik / 2.0)
    .max(0.0);
    let yarıçaplar = match (yön, negatif) {
        (crate::ÇubukYönü::Dikey, false) => KöşeYarıçapları {
            üst_sol: yarıçap,
            üst_sağ: yarıçap,
            ..KöşeYarıçapları::default()
        },
        (crate::ÇubukYönü::Dikey, true) => KöşeYarıçapları {
            alt_sağ: yarıçap,
            alt_sol: yarıçap,
            ..KöşeYarıçapları::default()
        },
        (crate::ÇubukYönü::Yatay, false) => KöşeYarıçapları {
            üst_sağ: yarıçap,
            alt_sağ: yarıçap,
            ..KöşeYarıçapları::default()
        },
        (crate::ÇubukYönü::Yatay, true) => KöşeYarıçapları {
            üst_sol: yarıçap,
            alt_sol: yarıçap,
            ..KöşeYarıçapları::default()
        },
    };
    Komut::YuvarlatılmışDikdörtgen {
        konum,
        genişlik,
        yükseklik,
        yarıçaplar,
        dolgu,
        çizgi,
        kalınlık,
    }
}

fn dönüştürülmüş_konum(
    aralık: Aralık,
    değer: f64,
    başlangıç: f32,
    uzunluk: f32,
    dönüştür: impl Fn(f64) -> f64,
) -> f32 {
    let en_az = dönüştür(aralık.en_az);
    let en_çok = dönüştür(aralık.en_çok);
    let değer = dönüştür(değer);
    başlangıç + ((değer - en_az) / (en_çok - en_az)) as f32 * uzunluk
}

fn arcsinh_bölmeleri(aralık: Aralık, eşik: f64) -> Vec<f64> {
    fn pozitif(en_az: f64, en_çok: f64) -> Vec<f64> {
        if !en_az.is_finite() || !en_çok.is_finite() || en_az <= 0.0 || en_az > en_çok {
            return Vec::new();
        }
        let ilk_üs = en_az.log10().floor() as i32;
        let son_üs = en_çok.log10().floor() as i32;
        let mut sonuç = Vec::new();
        for üs in ilk_üs..=son_üs {
            let taban = 10_f64.powi(üs);
            for katsayı in 1..10 {
                let değer = f64::from(katsayı) * taban;
                if değer >= en_az * (1.0 - 1e-12) && değer <= en_çok * (1.0 + 1e-12) {
                    sonuç.push(artıma_yuvarla(değer, taban));
                }
            }
        }
        sonuç
    }

    let mut sonuç = if aralık.en_az < -eşik {
        let mut negatif = pozitif(eşik.max(-aralık.en_çok), -aralık.en_az);
        negatif.reverse();
        negatif.into_iter().map(|değer| -değer).collect()
    } else if aralık.en_az <= -eşik && -eşik <= aralık.en_çok {
        vec![-eşik]
    } else {
        Vec::new()
    };
    if aralık.en_az <= 0.0 && aralık.en_çok >= 0.0 {
        sonuç.push(0.0);
    }
    if aralık.en_çok > eşik {
        sonuç.extend(pozitif(eşik.max(aralık.en_az), aralık.en_çok));
    } else if aralık.en_az <= eşik && eşik <= aralık.en_çok {
        sonuç.push(eşik);
    }
    sonuç
}

fn zaman_bölmeleri(
    aralık: Aralık,
    boyut: f32,
    en_az_boşluk: f32,
    milisaniye: bool,
    zaman_dilimi: crate::ZamanDilimi,
    sabit_artım: Option<f64>,
) -> (Vec<f64>, f64) {
    const SANİYE_ADIMLARI: &[f64] = &[
        0.001,
        0.002,
        0.0025,
        0.005,
        0.01,
        0.02,
        0.025,
        0.05,
        0.1,
        0.2,
        0.25,
        0.5,
        1.0,
        5.0,
        10.0,
        15.0,
        30.0,
        60.0,
        300.0,
        600.0,
        900.0,
        1_800.0,
        3_600.0,
        7_200.0,
        10_800.0,
        14_400.0,
        21_600.0,
        28_800.0,
        43_200.0,
        86_400.0,
        172_800.0,
        259_200.0,
        345_600.0,
        432_000.0,
        518_400.0,
        604_800.0,
        691_200.0,
        777_600.0,
        864_000.0,
        1_296_000.0,
        2_592_000.0,
        5_184_000.0,
        7_776_000.0,
        10_368_000.0,
        15_552_000.0,
        31_536_000.0,
        63_072_000.0,
        157_680_000.0,
        315_360_000.0,
        788_400_000.0,
        1_576_800_000.0,
        3_153_600_000.0,
    ];
    const MİLİSANİYE_ADIMLARI: &[f64] = &[
        1.0,
        2.0,
        5.0,
        10.0,
        20.0,
        25.0,
        50.0,
        100.0,
        200.0,
        250.0,
        500.0,
        1_000.0,
        5_000.0,
        10_000.0,
        15_000.0,
        30_000.0,
        60_000.0,
        300_000.0,
        600_000.0,
        900_000.0,
        1_800_000.0,
        3_600_000.0,
        7_200_000.0,
        10_800_000.0,
        14_400_000.0,
        21_600_000.0,
        28_800_000.0,
        43_200_000.0,
        86_400_000.0,
        172_800_000.0,
        259_200_000.0,
        345_600_000.0,
        432_000_000.0,
        518_400_000.0,
        604_800_000.0,
        691_200_000.0,
        777_600_000.0,
        864_000_000.0,
        1_296_000_000.0,
        2_592_000_000.0,
        5_184_000_000.0,
        7_776_000_000.0,
        10_368_000_000.0,
        15_552_000_000.0,
        31_536_000_000.0,
        63_072_000_000.0,
        157_680_000_000.0,
        315_360_000_000.0,
        788_400_000_000.0,
        1_576_800_000_000.0,
        3_153_600_000_000.0,
    ];
    let birim = if milisaniye { 1_000.0 } else { 1.0 };
    let zaman_adımları = if milisaniye {
        MİLİSANİYE_ADIMLARI
    } else {
        SANİYE_ADIMLARI
    };
    let hedef =
        (aralık.en_çok - aralık.en_az) * f64::from(en_az_boşluk) / f64::from(boyut.max(1.0));
    let adım = sabit_artım
        .filter(|artım| artım.is_finite() && *artım > 0.0)
        .map(|artım| artım * birim)
        .unwrap_or_else(|| {
            zaman_adımları
                .iter()
                .copied()
                .find(|adım| *adım >= hedef)
                .unwrap_or_else(|| zaman_adımları.last().copied().unwrap_or(birim))
        });
    let saniye_adımı = adım / birim;
    if saniye_adımı >= 2_592_000.0 {
        let ay_adımı = if saniye_adımı >= 31_536_000.0 {
            let yıl_adımı = (saniye_adımı / 31_536_000.0).round().max(1.0) as i64;
            yıl_adımı.saturating_mul(12)
        } else {
            (saniye_adımı / 2_592_000.0).round().max(1.0) as i64
        };
        return (takvim_ay_bölmeleri(aralık, birim, ay_adımı), adım);
    }
    let başlangıç_saniyesi = aralık.en_az / birim;
    let ofset = f64::from(crate::zaman::zaman_dilimi_ofseti(
        zaman_dilimi,
        başlangıç_saniyesi,
    )) * birim;
    let ilk = ((aralık.en_az + ofset) / adım).ceil() * adım - ofset;
    let mut sonuç = Vec::new();
    let mut değer = ilk;
    while değer <= aralık.en_çok && sonuç.len() < 10_000 {
        sonuç.push(değer);
        değer += adım;
    }
    (sonuç, adım)
}

fn takvim_ay_bölmeleri(aralık: Aralık, birim: f64, ay_adımı: i64) -> Vec<f64> {
    if !birim.is_finite() || birim <= 0.0 || ay_adımı <= 0 {
        return Vec::new();
    }
    let en_az = aralık.en_az / birim;
    let en_çok = aralık.en_çok / birim;
    let Some((yıl, ay, _, _, _, _)) = crate::zaman::utc_alanları(en_az) else {
        return Vec::new();
    };
    let mut ay_indeksi = yıl
        .saturating_mul(12)
        .saturating_add(i64::from(ay).saturating_sub(1));
    ay_indeksi = ay_indeksi.div_euclid(ay_adımı).saturating_mul(ay_adımı);
    let mut sonuç = Vec::new();
    for _ in 0..10_000 {
        let bölme_yılı = ay_indeksi.div_euclid(12);
        let ay_sıfır = ay_indeksi.rem_euclid(12);
        let Ok(bölme_ayı) = u32::try_from(ay_sıfır.saturating_add(1)) else {
            break;
        };
        let Some(zaman) = crate::zaman::utc_zaman_damgası(bölme_yılı, bölme_ayı, 1) else {
            break;
        };
        if zaman > en_çok {
            break;
        }
        if zaman >= en_az {
            sonuç.push(zaman * birim);
        }
        ay_indeksi = ay_indeksi.saturating_add(ay_adımı);
    }
    sonuç
}

fn logaritmik_bölmeler(aralık: Aralık, taban: f64) -> Vec<f64> {
    if !taban.is_finite() || taban <= 1.0 || aralık.en_az <= 0.0 {
        return Vec::new();
    }
    let ilk = aralık.en_az.log(taban).floor() as i32;
    let son = aralık.en_çok.log(taban).ceil() as i32;
    let çarpanlar: &[f64] = if (taban - 10.0).abs() <= f64::EPSILON {
        &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]
    } else {
        &[1.0]
    };
    let mut sonuç = Vec::new();
    for üs in ilk..=son {
        let kuvvet = taban.powi(üs);
        for çarpan in çarpanlar {
            let değer = kuvvet * çarpan;
            if değer >= aralık.en_az && değer <= aralık.en_çok {
                sonuç.push(değer);
            }
        }
    }
    sonuç
}

fn logaritmik_otomatik_aralık<'a>(
    değerler: impl Iterator<Item = &'a Option<f64>>,
    taban: f64,
    tam_büyüklükler: bool,
) -> Option<Aralık> {
    if !taban.is_finite() || taban <= 1.0 {
        return None;
    }
    let mut en_az = f64::INFINITY;
    let mut en_çok = f64::NEG_INFINITY;
    for değer in değerler.flatten().filter(|değer| **değer > 0.0) {
        en_az = en_az.min(*değer);
        en_çok = en_çok.max(*değer);
    }
    if !en_az.is_finite() || !en_çok.is_finite() {
        return None;
    }
    logaritmik_aralık_sınırlardan(en_az, en_çok, taban, tam_büyüklükler)
}

fn logaritmik_aralık_sınırlardan(
    mut en_az: f64,
    mut en_çok: f64,
    taban: f64,
    tam_büyüklükler: bool,
) -> Option<Aralık> {
    if !taban.is_finite()
        || taban <= 1.0
        || !en_az.is_finite()
        || !en_çok.is_finite()
        || en_az <= 0.0
        || en_çok <= 0.0
    {
        return None;
    }
    if en_az == en_çok {
        en_az /= taban;
        en_çok *= taban;
    }
    let (alt, üst) = if tam_büyüklükler {
        let alt_üs = en_az.log(taban).floor() as i32;
        let üst_üs = en_çok.log(taban).ceil() as i32;
        (taban.powi(alt_üs), taban.powi(üst_üs))
    } else {
        let alt_adım = taban.powi(en_az.log(taban).floor() as i32);
        let üst_adım = taban.powi(en_çok.log(taban).floor() as i32);
        (
            (en_az / alt_adım).floor() * alt_adım,
            (en_çok / üst_adım).ceil() * üst_adım,
        )
    };
    Aralık::yeni(alt, üst).ok()
}

fn arcsinh_otomatik_aralık(değerler: impl Iterator<Item = f64>) -> Option<Aralık> {
    let (en_az, en_çok) = sonlu_sınırlar(değerler)?;
    let büyüt = |değer: f64, alt: bool| {
        if değer == 0.0 {
            0.0
        } else {
            let üs = değer.abs().log10();
            let üs = if (değer < 0.0) == alt {
                üs.ceil()
            } else {
                üs.floor()
            };
            değer.signum() * 10_f64.powf(üs)
        }
    };
    let mut alt = büyüt(en_az, true);
    let mut üst = büyüt(en_çok, false);
    if alt == üst {
        if alt < 0.0 {
            alt *= 10.0;
            üst /= 10.0;
        } else if alt > 0.0 {
            alt /= 10.0;
            üst *= 10.0;
        } else {
            alt = -1.0;
            üst = 1.0;
        }
    }
    Aralık::yeni(alt, üst).ok()
}

fn tam_x_aralığı(veri: &HizalıVeri) -> Result<Aralık, UplotHatası> {
    let Some(ilk) = veri.x().first().copied() else {
        return Err(UplotHatası::YetersizVeri { uzunluk: 0 });
    };
    let son = veri.x().last().copied().unwrap_or(ilk);
    if ilk == son {
        Aralık::yeni(ilk - 0.5, son + 0.5)
    } else {
        Aralık::yeni(ilk, son)
    }
}

fn görünür_x_indeksleri(x: &[f64], aralık: Aralık) -> std::ops::Range<usize> {
    let başlangıç = x.partition_point(|değer| *değer < aralık.en_az);
    let bitiş = x.partition_point(|değer| *değer <= aralık.en_çok);
    başlangıç..bitiş
}

fn space_between_grup_boyutu(
    x: &[f64],
    görünür_aralık: Aralık,
    piksel_boyutu: f32,
    genişlik_oranı: f32,
) -> f32 {
    let grup_sayısı = x.len().max(1) as f64;
    let oran = f64::from(genişlik_oranı.clamp(0.0, 1.0));
    let veri_boyutu = if x.len() > 1 {
        let ham_açıklık = x
            .first()
            .zip(x.last())
            .map_or(1.0, |(ilk, son)| (son - ilk).abs().max(f64::EPSILON));
        ham_açıklık * oran / (grup_sayısı - oran).max(f64::EPSILON)
    } else {
        oran / (1.0 - oran).max(f64::EPSILON)
    };
    let görünür_açıklık = (görünür_aralık.en_çok - görünür_aralık.en_az).max(f64::EPSILON);
    (veri_boyutu / görünür_açıklık * f64::from(piksel_boyutu)) as f32
}

/// uPlot `closestIdx()` gibi sıralı hizalı X sütununda ikili arama yapar.
/// Eşit uzaklıkta soldaki (daha küçük) indeks kazanır.
fn en_yakın_x_indeksi(x: &[f64], aralık: Aralık, hedef: f64) -> Option<usize> {
    let görünür = görünür_x_indeksleri(x, aralık);
    let kesit = x.get(görünür.clone())?;
    if kesit.is_empty() {
        return None;
    }
    let sağ_göreli = kesit.partition_point(|değer| *değer < hedef);
    let sağ = (sağ_göreli < kesit.len()).then(|| görünür.start + sağ_göreli);
    let sol = if sağ_göreli == 0 {
        None
    } else {
        görünür.start.checked_add(sağ_göreli.saturating_sub(1))
    };
    match (sol, sağ) {
        (Some(sol), Some(sağ)) => {
            let sol_uzaklık = (x.get(sol)? - hedef).abs();
            let sağ_uzaklık = (x.get(sağ)? - hedef).abs();
            Some(if sol_uzaklık <= sağ_uzaklık {
                sol
            } else {
                sağ
            })
        }
        (Some(sol), None) => Some(sol),
        (None, Some(sağ)) => Some(sağ),
        (None, None) => None,
    }
}

/// JavaScript `Intl.NumberFormat()` varsayılanındaki en çok üç ondalık
/// basamaklı kompakt tooltip değerini, platform yerel ayarından bağımsız üretir.
fn tooltip_sayısını_biçimlendir(değer: f64) -> String {
    let biçimli = format!("{değer:.3}");
    biçimli
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string()
}

/// Kaynak candlestick demosundaki `fmtUSD()` biçimini yerel ayardan
/// bağımsız üretir. Para imi, işaretten önce gelir (`$-1.00`).
pub(crate) fn usd_biçimle(değer: f64, ondalık: usize) -> String {
    if !değer.is_finite() {
        return "—".to_string();
    }

    let işaret = if değer.is_sign_negative() { "-" } else { "" };
    let ham = format!("{:.*}", ondalık, değer.abs());
    let (tam, kesir) = ham
        .split_once('.')
        .map_or((ham.as_str(), None), |(tam, kesir)| (tam, Some(kesir)));
    let mut ters_gruplu = String::with_capacity(tam.len() + tam.len() / 3);
    for (indeks, karakter) in tam.chars().rev().enumerate() {
        if indeks > 0 && indeks.is_multiple_of(3) {
            ters_gruplu.push(',');
        }
        ters_gruplu.push(karakter);
    }
    let gruplu = ters_gruplu.chars().rev().collect::<String>();

    kesir.map_or_else(
        || format!("${işaret}{gruplu}"),
        |kesir| format!("${işaret}{gruplu}.{kesir}"),
    )
}

/// İkili arama konumundan iki yana yalnız gerektiği kadar ilerleyerek null
/// koşusunu atlar. Dolu hizalı serilerde sorgu O(log N), null koşusunda
/// O(log N + K) olur.
fn en_yakın_dolu_x_indeksi(
    x: &[f64],
    y: &[Option<f64>],
    aralık: Aralık,
    hedef: f64,
    yön: NullAtlamaYönü,
) -> Option<usize> {
    let görünür = görünür_x_indeksleri(x, aralık);
    let kesit = x.get(görünür.clone())?;
    if kesit.is_empty() {
        return None;
    }

    if yön == NullAtlamaYönü::Önceki {
        let göreli_bitiş = kesit.partition_point(|değer| *değer <= hedef);
        let mut aday = görünür.start.checked_add(göreli_bitiş)?.checked_sub(1);
        while let Some(indeks) = aday.filter(|indeks| *indeks >= görünür.start) {
            if y.get(indeks).is_some_and(Option::is_some) {
                return Some(indeks);
            }
            aday = indeks.checked_sub(1);
        }
        return None;
    }

    let sağ_göreli = kesit.partition_point(|değer| *değer < hedef);
    let mut sol = if sağ_göreli == 0 {
        None
    } else {
        görünür.start.checked_add(sağ_göreli.saturating_sub(1))
    };
    let mut sağ = (sağ_göreli < kesit.len()).then(|| görünür.start + sağ_göreli);
    loop {
        let aday = match (sol, sağ) {
            (Some(sol_indeksi), Some(sağ_indeksi)) => {
                let sol_uzaklık = (x.get(sol_indeksi)? - hedef).abs();
                let sağ_uzaklık = (x.get(sağ_indeksi)? - hedef).abs();
                if sol_uzaklık <= sağ_uzaklık {
                    sol = sol_indeksi
                        .checked_sub(1)
                        .filter(|indeks| *indeks >= görünür.start);
                    sol_indeksi
                } else {
                    sağ = sağ_indeksi
                        .checked_add(1)
                        .filter(|indeks| *indeks < görünür.end);
                    sağ_indeksi
                }
            }
            (Some(sol_indeksi), None) => {
                sol = sol_indeksi
                    .checked_sub(1)
                    .filter(|indeks| *indeks >= görünür.start);
                sol_indeksi
            }
            (None, Some(sağ_indeksi)) => {
                sağ = sağ_indeksi
                    .checked_add(1)
                    .filter(|indeks| *indeks < görünür.end);
                sağ_indeksi
            }
            (None, None) => return None,
        };
        if y.get(aday).is_some_and(Option::is_some) {
            return Some(aday);
        }
    }
}

fn çizilecek_indeksler(
    x: &[f64],
    y: &[Option<f64>],
    aralık: Aralık,
    piksel_genişliği: f32,
) -> Vec<usize> {
    let eşik = (piksel_genişliği.max(1.0) as usize).saturating_mul(4);
    let görünür_çekirdek = görünür_x_indeksleri(x, aralık);
    // uPlot path çağrısı gibi görünüm sınırını kesen çizgi parçasını korumak
    // için her iki taraftan bir dış komşu da çizim adayına katılır.
    let görünür = görünür_çekirdek.start.saturating_sub(1)
        ..görünür_çekirdek.end.saturating_add(1).min(x.len());
    let görünür_sayı = görünür.end.saturating_sub(görünür.start);
    let Some(görünür_y) = y.get(görünür.clone()) else {
        return Vec::new();
    };
    if görünür_sayı.saturating_sub(1) < eşik {
        return görünür.collect();
    }

    let mut sonuç = Vec::with_capacity(eşik);
    let mut kova = None::<(usize, usize, usize, usize, usize, f64, f64)>;
    let mut boşlukta = false;
    let Some(görünür_x) = x.get(görünür.clone()) else {
        return sonuç;
    };
    for (göreli, (x_değeri, y_değeri)) in görünür_x.iter().zip(görünür_y).enumerate() {
        let indeks = görünür.start.saturating_add(göreli);
        let Some(y_değeri) = y_değeri else {
            if let Some((_, ilk, son, en_az_i, en_çok_i, _, _)) = kova.take() {
                kova_indekslerini_ekle(&mut sonuç, ilk, en_az_i, en_çok_i, son);
            }
            if !boşlukta {
                sonuç.push(indeks);
                boşlukta = true;
            }
            continue;
        };
        boşlukta = false;
        let oran = (*x_değeri - aralık.en_az) / (aralık.en_çok - aralık.en_az);
        let yeni_kova = (oran * f64::from(piksel_genişliği))
            .round()
            .clamp(0.0, f64::from(piksel_genişliği.max(0.0))) as usize;
        match kova.as_mut() {
            Some((kimlik, _ilk, son, en_az_i, en_çok_i, en_az, en_çok)) if *kimlik == yeni_kova =>
            {
                *son = indeks;
                if *y_değeri < *en_az {
                    *en_az = *y_değeri;
                    *en_az_i = indeks;
                }
                if *y_değeri > *en_çok {
                    *en_çok = *y_değeri;
                    *en_çok_i = indeks;
                }
            }
            _ => {
                if let Some((_, ilk, son, en_az_i, en_çok_i, _, _)) = kova.take() {
                    kova_indekslerini_ekle(&mut sonuç, ilk, en_az_i, en_çok_i, son);
                }
                kova = Some((
                    yeni_kova, indeks, indeks, indeks, indeks, *y_değeri, *y_değeri,
                ));
            }
        }
    }
    if let Some((_, ilk, son, en_az_i, en_çok_i, _, _)) = kova {
        kova_indekslerini_ekle(&mut sonuç, ilk, en_az_i, en_çok_i, son);
    }
    sonuç
}

fn kova_indekslerini_ekle(
    sonuç: &mut Vec<usize>,
    ilk: usize,
    en_az: usize,
    en_çok: usize,
    son: usize,
) {
    // Resmî `_drawAcc`: giriş → min → max → çıkış. Ekstremumların veri
    // zamanına göre sıralanması aynı piksel kovasındaki dikey zarfı bozar.
    for aday in [ilk, en_az, en_çok, son] {
        if sonuç.last().copied() != Some(aday) {
            sonuç.push(aday);
        }
    }
}

/// uPlot'un sayısal eksen yaklaşımı gibi görünür aralık ve piksel yoğunluğuna
/// göre 1/2/2.5/5 × 10ⁿ ailesinden uygun artımı seçer.
fn uygun_artım(aralık: Aralık, boyut: f32, en_az_boşluk: f32) -> f64 {
    let uzunluk = aralık.en_çok - aralık.en_az;
    if !uzunluk.is_finite() || uzunluk <= 0.0 || !boyut.is_finite() || boyut <= 0.0 {
        return 1.0;
    }
    let hedef = uzunluk * f64::from(en_az_boşluk.max(1.0)) / f64::from(boyut);
    if !hedef.is_finite() || hedef <= 0.0 {
        return 1.0;
    }
    let taban = 10_f64.powf(hedef.log10().floor());
    for çarpan in [1.0_f64, 2.0, 2.5, 5.0, 10.0] {
        let aday = taban * çarpan;
        if aday >= hedef && aday.is_finite() {
            return aday;
        }
    }
    hedef
}

fn sonlu_aralık(değerler: impl Iterator<Item = f64>) -> Option<Aralık> {
    let (en_az, en_çok) = sonlu_sınırlar(değerler)?;
    Aralık::yeni(en_az, en_çok).ok()
}

fn sonlu_sınırlar(değerler: impl Iterator<Item = f64>) -> Option<(f64, f64)> {
    let mut en_az = f64::INFINITY;
    let mut en_çok = f64::NEG_INFINITY;
    for değer in değerler.filter(|değer| değer.is_finite()) {
        en_az = en_az.min(değer);
        en_çok = en_çok.max(değer);
    }
    (en_az.is_finite() && en_çok.is_finite()).then_some((en_az, en_çok))
}

fn güzel_sayı(fark: f64, yuvarla: bool) -> Option<f64> {
    if !fark.is_finite() || fark <= 0.0 {
        return None;
    }
    let üs = fark.log10().floor();
    let kuvvet = 10_f64.powf(üs);
    if !kuvvet.is_finite() || kuvvet <= 0.0 {
        return None;
    }
    let kesir = fark / kuvvet;
    let güzel_kesir = if yuvarla {
        if kesir < 1.5 {
            1.0
        } else if kesir < 3.0 {
            if kesir > 2.25 { 2.5 } else { 2.0 }
        } else if kesir < 7.0 {
            5.0
        } else {
            10.0
        }
    } else if kesir <= 1.0 {
        1.0
    } else if kesir <= 2.0 {
        2.0
    } else if kesir <= 5.0 {
        5.0
    } else {
        10.0
    };
    let sonuç = güzel_kesir * kuvvet;
    (sonuç.is_finite() && sonuç > 0.0).then_some(sonuç)
}

fn güzel_ölçek(
    veri_aralığı: Aralık, boyut: f32, en_az_boşluk: f32
) -> Option<(Aralık, f64)> {
    if !boyut.is_finite() || boyut <= 0.0 || !en_az_boşluk.is_finite() || en_az_boşluk <= 0.0 {
        return None;
    }
    let en_az = veri_aralığı.en_az
        * if veri_aralığı.en_az < 0.0 {
            1.02
        } else if veri_aralığı.en_az > 0.0 {
            0.98
        } else {
            1.0
        };
    let en_çok = veri_aralığı.en_çok
        * if veri_aralığı.en_çok < 0.0 {
            0.98
        } else if veri_aralığı.en_çok > 0.0 {
            1.02
        } else {
            1.0
        };
    let en_fazla_etiket = (boyut / en_az_boşluk).floor().clamp(2.0, 10_000.0) as u32;
    let güzel_aralık = güzel_sayı(en_çok - en_az, false)?;
    let artım = güzel_sayı(güzel_aralık / f64::from(en_fazla_etiket - 1), true)?;
    let alt = artıma_yuvarla((en_az / artım).floor() * artım, artım);
    let üst = artıma_yuvarla((en_çok / artım).ceil() * artım, artım);
    Some((Aralık::yeni(alt, üst).ok()?, artım))
}

fn eksen_bölmeleri(aralık: Aralık, boyut: f32, en_az_boşluk: f32) -> Vec<f64> {
    let artım = uygun_artım(aralık, boyut, en_az_boşluk);
    eksen_bölmeleri_artımla(aralık, artım)
}

fn eksen_bölmeleri_artımla(aralık: Aralık, artım: f64) -> Vec<f64> {
    if !artım.is_finite() || artım <= 0.0 {
        return Vec::new();
    }
    let tolerans = artım.abs() * 1e-9;
    let mut değer = ((aralık.en_az - tolerans) / artım).ceil() * artım;
    let mut bölmeler = Vec::new();
    for _ in 0..1_000 {
        if değer > aralık.en_çok + tolerans {
            break;
        }
        let yuvarlanmış = artıma_yuvarla(değer, artım);
        bölmeler.push(if yuvarlanmış.abs() <= tolerans {
            0.0
        } else {
            yuvarlanmış
        });
        değer += artım;
    }
    bölmeler
}

fn yaklaşık_metin_genişliği(metin: &str, boyut: f32) -> f32 {
    metin
        .chars()
        .map(|karakter| {
            let em = match karakter {
                '0'..='9' => 0.556,
                '.' | ',' | ':' | ';' | '!' | '|' => 0.278,
                '-' | '−' | '+' | '=' => 0.584,
                'e' | 'a' | 's' | 'x' | 'y' | 'z' => 0.5,
                'i' | 'l' | 'I' | 'j' | 't' => 0.278,
                'm' | 'w' | 'M' | 'W' => 0.833,
                ' ' => 0.278,
                _ => 0.667,
            };
            em * boyut
        })
        .sum()
}

fn üç_anlamlı_basamak(değer: f64) -> String {
    if değer.is_nan() {
        return "NaN".to_string();
    }
    if değer == f64::INFINITY {
        return "Infinity".to_string();
    }
    if değer == f64::NEG_INFINITY {
        return "-Infinity".to_string();
    }
    if değer == 0.0 {
        return "0.00".to_string();
    }

    let ilk_üs = değer.abs().log10().floor() as i32;
    let yuvarlanmış = if (-6..3).contains(&ilk_üs) {
        let çarpan = 10_f64.powi(2 - ilk_üs);
        (değer * çarpan).round() / çarpan
    } else {
        değer
    };
    let üs = yuvarlanmış.abs().log10().floor() as i32;
    if !(-6..3).contains(&üs) {
        let bilimsel = format!("{yuvarlanmış:.2e}");
        let Some((mantis, üs)) = bilimsel.split_once('e') else {
            return bilimsel;
        };
        let Ok(üs) = üs.parse::<i32>() else {
            return bilimsel;
        };
        format!("{mantis}e{üs:+}")
    } else {
        let ondalık = usize::try_from((2 - üs).max(0)).unwrap_or(0);
        format!("{yuvarlanmış:.ondalık$}")
    }
}

fn artıma_yuvarla(değer: f64, artım: f64) -> f64 {
    let basamak = ondalık_basamak(artım);
    let kuvvet = 10_f64.powf(f64::from(basamak));
    (değer * kuvvet).round() / kuvvet
}

fn ondalık_basamak(artım: f64) -> u32 {
    let mut ölçekli = artım.abs();
    for basamak in 0..=12 {
        if (ölçekli - ölçekli.round()).abs() <= 1e-9 {
            return basamak;
        }
        ölçekli *= 10.0;
    }
    12
}

fn eksen_değerini_yaz(değer: f64, artım: f64) -> String {
    let basamak = usize::try_from(ondalık_basamak(artım).max(2)).unwrap_or(12);
    format!("{değer:.basamak$}")
}

fn eksen_değerini_artıma_göre_yaz(değer: f64, artım: f64) -> String {
    let basamak = usize::try_from(ondalık_basamak(artım)).unwrap_or(12);
    let mut sayı = format!("{değer:.basamak$}");
    if sayı.contains('.') {
        while sayı.ends_with('0') {
            sayı.pop();
        }
        if sayı.ends_with('.') {
            sayı.pop();
        }
    }
    sayı
}

fn kompakt_sayı(değer: f64) -> String {
    if !değer.is_finite() {
        return "—".to_string();
    }
    let mut ölçekli = değer;
    let mut sonek = "";
    for (eşik, aday) in [(1e9, "B"), (1e6, "M"), (1e3, "K")] {
        if değer.abs() >= eşik {
            ölçekli = değer / eşik;
            sonek = aday;
            break;
        }
    }
    let basamak = if sonek.is_empty() || ölçekli.abs() >= 100.0 {
        0
    } else if ölçekli.abs() >= 10.0 {
        1
    } else {
        2
    };
    let mut sayı = format!("{ölçekli:.basamak$}");
    if sayı.contains('.') {
        while sayı.ends_with('0') {
            sayı.pop();
        }
        if sayı.ends_with('.') {
            sayı.pop();
        }
    }
    format!("{sayı}{sonek}")
}

fn ölçek_eksen_değerini_yaz(
    değer: f64,
    artım: f64,
    birim: &str,
    dağılım: Option<YÖlçekDağılımı>,
    biçim: YÖlçekEtiketBiçimi,
) -> String {
    let sayı = match biçim {
        YÖlçekEtiketBiçimi::ArtımaGöre => eksen_değerini_artıma_göre_yaz(değer, artım),
        YÖlçekEtiketBiçimi::Bilimsel => format!("{değer:e}"),
        YÖlçekEtiketBiçimi::İkiliÜs => ikili_üs_etiketi(değer),
        YÖlçekEtiketBiçimi::İkiliŞapka => ikili_şapka_etiketi(değer),
        YÖlçekEtiketBiçimi::Kompakt => kompakt_sayı(değer),
        YÖlçekEtiketBiçimi::Otomatik => match dağılım {
            Some(YÖlçekDağılımı::ArcSinh { .. }) => format!("{değer}"),
            Some(YÖlçekDağılımı::Logaritmik { taban }) if (taban - 2.0).abs() <= f64::EPSILON => {
                ikili_üs_etiketi(değer)
            }
            Some(YÖlçekDağılımı::Logaritmik { .. }) if değer.abs() >= 1.0 => {
                format!("{değer:.0}")
            }
            Some(YÖlçekDağılımı::Logaritmik { .. } | YÖlçekDağılımı::Weibull) => {
                format!("{değer:e}")
            }
            Some(YÖlçekDağılımı::Özel(_)) => eksen_değerini_yaz(değer, artım),
            _ => eksen_değerini_yaz(değer, artım),
        },
    };
    if birim.is_empty() {
        sayı
    } else if birim == "%" || birim.starts_with('°') {
        format!("{sayı}{birim}")
    } else {
        format!("{sayı} {birim}")
    }
}

/// uPlot log eksen filtresi: ızgara her büyüklükte tüm bölmeleri korur, eksen
/// metinlerini ise kullanılabilir piksel alanına göre seyreltir.
fn log_etiketi_göster(
    değer: f64,
    aralık: Aralık,
    boyut: f32,
    dağılım: Option<YÖlçekDağılımı>,
    biçim: YÖlçekEtiketBiçimi,
    en_az_boşluk: f32,
) -> bool {
    if matches!(dağılım, Some(YÖlçekDağılımı::ArcSinh { .. })) {
        if değer == 0.0 {
            return true;
        }
        let üs = değer.abs().log10();
        return üs.is_finite() && (üs - üs.round()).abs() <= 1e-9;
    }
    let Some(YÖlçekDağılımı::Logaritmik { taban }) = dağılım else {
        return true;
    };
    if değer <= 0.0
        || aralık.en_az <= 0.0
        || !boyut.is_finite()
        || boyut <= 0.0
        || !en_az_boşluk.is_finite()
        || en_az_boşluk <= 0.0
    {
        return true;
    }
    let üs = değer.log(taban);
    if !üs.is_finite() {
        return false;
    }
    let tam_kuvvet = (üs - üs.round()).abs() <= 1e-9;
    let özel_kuvvet_biçimi = matches!(
        biçim,
        YÖlçekEtiketBiçimi::Bilimsel
            | YÖlçekEtiketBiçimi::İkiliÜs
            | YÖlçekEtiketBiçimi::İkiliŞapka
    );

    if (taban - 10.0).abs() <= f64::EPSILON && !özel_kuvvet_biçimi {
        let açıklık = aralık.en_çok.log10() - aralık.en_az.log10();
        if !açıklık.is_finite() || açıklık <= 0.0 {
            return true;
        }
        let piksel_farkı =
            |aday: f64| f64::from(boyut) * (10.0_f64.log10() - aday.log10()).abs() / açıklık;
        let büyüklük = 10_f64.powf(değer.log10().floor());
        let öncül = (değer / büyüklük).round() as i32;
        let en_az_boşluk = f64::from(en_az_boşluk);
        if piksel_farkı(9.0) >= en_az_boşluk {
            return (1..=9).contains(&öncül);
        }
        if piksel_farkı(7.0) >= en_az_boşluk {
            return matches!(öncül, 1 | 2 | 3 | 5 | 7);
        }
        if piksel_farkı(5.0) >= en_az_boşluk {
            return matches!(öncül, 1 | 2 | 5);
        }
        if öncül != 1 {
            return false;
        }
    } else if !tam_kuvvet {
        return false;
    }

    let en_az_üs = aralık.en_az.log(taban).floor() as i32;
    let en_çok_üs = aralık.en_çok.log(taban).ceil() as i32;
    let üs = üs.round() as i32;
    let açıklık = en_çok_üs.saturating_sub(en_az_üs).max(1);
    let adım = (f64::from(açıklık) * f64::from(en_az_boşluk) / f64::from(boyut))
        .ceil()
        .max(1.0) as i32;
    en_çok_üs.saturating_sub(üs).rem_euclid(adım) == 0
}

fn ikili_üs_etiketi(değer: f64) -> String {
    if !değer.is_finite() || değer <= 0.0 {
        return "—".to_string();
    }
    let üs = değer.log2().round() as i32;
    let mut sonuç = String::from("2");
    if üs < 0 {
        sonuç.push('⁻');
    }
    for rakam in üs.unsigned_abs().to_string().bytes() {
        sonuç.push(match rakam {
            b'0' => '⁰',
            b'1' => '¹',
            b'2' => '²',
            b'3' => '³',
            b'4' => '⁴',
            b'5' => '⁵',
            b'6' => '⁶',
            b'7' => '⁷',
            b'8' => '⁸',
            b'9' => '⁹',
            _ => '�',
        });
    }
    sonuç
}

fn ikili_şapka_etiketi(değer: f64) -> String {
    if !değer.is_finite() || değer <= 0.0 {
        return "—".to_string();
    }
    format!("2^{}", değer.log2().round() as i32)
}

fn renk_rgb(renk: &str) -> Option<(u8, u8, u8)> {
    let ham = renk.strip_prefix('#')?;
    if ham.len() != 6 {
        return None;
    }
    let kırmızı = u8::from_str_radix(ham.get(0..2)?, 16).ok()?;
    let yeşil = u8::from_str_radix(ham.get(2..4)?, 16).ok()?;
    let mavi = u8::from_str_radix(ham.get(4..6)?, 16).ok()?;
    Some((kırmızı, yeşil, mavi))
}

fn renk_alfa(renk: &str, alfa: u8) -> String {
    renk_rgb(renk).map_or_else(
        || renk.to_string(),
        |(r, g, b)| format!("#{r:02x}{g:02x}{b:02x}{alfa:02x}"),
    )
}

fn renk_opaklığı(renk: &str, opaklık: f32) -> String {
    let Some(ham) = renk.strip_prefix('#') else {
        return renk.to_string();
    };
    let temel = match ham.len() {
        6 => ham,
        8 => ham.get(0..6).unwrap_or(ham),
        _ => return renk.to_string(),
    };
    let mevcut = if ham.len() == 8 {
        ham.get(6..8)
            .and_then(|değer| u8::from_str_radix(değer, 16).ok())
            .unwrap_or(255)
    } else {
        255
    };
    let alfa = (f32::from(mevcut) * opaklık.clamp(0.0, 1.0)).round() as u8;
    format!("#{temel}{alfa:02x}")
}

fn odaklı_seri_stili(
    seri: &crate::SeriSeçenekleri,
    düzen: Option<crate::OdakDüzeni>,
    odak: Option<usize>,
    seri_indeksi: usize,
) -> (String, Option<String>, f32) {
    let (Some(düzen), Some(odak)) = (düzen, odak) else {
        return (seri.renk.clone(), seri.dolgu.clone(), seri.çizgi_kalınlığı);
    };
    let odaklı = odak == seri_indeksi;
    let kalınlık = if odaklı {
        düzen.odak_kalınlığı.unwrap_or(seri.çizgi_kalınlığı)
    } else {
        seri.çizgi_kalınlığı
    };
    match düzen.stil {
        crate::OdakStili::Opaklık if !odaklı => (
            renk_opaklığı(&seri.renk, düzen.alfa),
            seri.dolgu
                .as_ref()
                .map(|renk| renk_opaklığı(renk, düzen.alfa)),
            kalınlık,
        ),
        crate::OdakStili::OdakDışıSiyah if !odaklı => (
            "#000000".to_string(),
            seri.dolgu.as_ref().map(|_| "#0000001a".to_string()),
            kalınlık,
        ),
        crate::OdakStili::OdaklıMacenta if odaklı => {
            ("#ff00ff".to_string(), seri.dolgu.clone(), kalınlık)
        }
        _ => (seri.renk.clone(), seri.dolgu.clone(), kalınlık),
    }
}

fn medyan(sıralı: &[f64]) -> Option<f64> {
    let sağ = sıralı.get(sıralı.len() / 2).copied()?;
    let sol = sıralı.get(sıralı.len().saturating_sub(1) / 2).copied()?;
    Some((sol + sağ) / 2.0)
}

fn yıldız_çokgeni(merkez: Nokta, uçlar: usize, dış: f32, iç: f32) -> Vec<Nokta> {
    let nokta_sayısı = uçlar.saturating_mul(2);
    let mut noktalar = Vec::with_capacity(nokta_sayısı);
    for indeks in 0..nokta_sayısı {
        let açı = -std::f32::consts::FRAC_PI_2
            + indeks as f32 * std::f32::consts::PI / uçlar.max(1) as f32;
        let yarıçap = if indeks.is_multiple_of(2) { dış } else { iç };
        noktalar.push(Nokta::yeni(
            merkez.x + açı.cos() * yarıçap,
            merkez.y + açı.sin() * yarıçap,
        ));
    }
    noktalar
}

fn piksele_hizala(değer: f32, adım: f32, cihaz_piksel_oranı: f32) -> f32 {
    if adım > 0.0
        && adım.is_finite()
        && değer.is_finite()
        && cihaz_piksel_oranı > 0.0
        && cihaz_piksel_oranı.is_finite()
    {
        let mantıksal_adım = adım / cihaz_piksel_oranı;
        (değer / mantıksal_adım).round() * mantıksal_adım
    } else {
        değer
    }
}

#[allow(clippy::too_many_arguments)]
fn boşluk_tekil_indeksleri(
    x: &[f64],
    y: &[Option<f64>],
    görünür_başlangıç: usize,
    görünür_bitiş: usize,
    x_aralığı: Aralık,
    sol: f32,
    genişlik: f32,
    piksel_hizası: f32,
    cihaz_piksel_oranı: f32,
) -> Vec<usize> {
    if x.is_empty() || y.is_empty() || görünür_başlangıç >= görünür_bitiş {
        return Vec::new();
    }
    let son_indeks = x.len().min(y.len()).saturating_sub(1);
    let ilk_görünür = görünür_başlangıç.min(son_indeks);
    let son_görünür = görünür_bitiş.saturating_sub(1).min(son_indeks);
    let piksel = |indeks: usize| {
        x.get(indeks).map_or(sol, |değer| {
            piksele_hizala(
                sol + x_aralığı.konum(*değer, 0.0, genişlik),
                piksel_hizası,
                cihaz_piksel_oranı,
            )
        })
    };

    // uPlot getOuterIdxs() görünümün iki yanından bir örnek alır ve kenardaki
    // null koşusunun sonuna kadar genişler. linear.findGaps() bundan sonra
    // ilk/son non-null değere kırpar.
    let mut yol_başı = ilk_görünür.saturating_sub(1);
    while yol_başı > 0 && y.get(yol_başı).is_some_and(Option::is_none) {
        yol_başı -= 1;
    }
    let mut yol_sonu = son_görünür.saturating_add(1).min(son_indeks);
    while yol_sonu < son_indeks && y.get(yol_sonu).is_some_and(Option::is_none) {
        yol_sonu += 1;
    }
    while yol_başı <= yol_sonu && y.get(yol_başı).is_none_or(Option::is_none) {
        yol_başı += 1;
    }
    while yol_sonu >= yol_başı && y.get(yol_sonu).is_none_or(Option::is_none) {
        if yol_sonu == 0 {
            return Vec::new();
        }
        yol_sonu -= 1;
    }
    if yol_başı > yol_sonu {
        return Vec::new();
    }

    let mut boşluklar = Vec::<(f32, f32)>::new();
    let mut indeks = yol_başı;
    while indeks <= yol_sonu {
        if y.get(indeks).is_some_and(Option::is_none) {
            let başlangıç = indeks;
            while indeks < yol_sonu && y.get(indeks + 1).is_some_and(Option::is_none) {
                indeks += 1;
            }
            let son = indeks;
            let ilk_piksel = başlangıç
                .checked_sub(1)
                .map_or_else(|| piksel(başlangıç), piksel);
            let son_piksel = son
                .checked_add(1)
                .filter(|sonraki| *sonraki <= son_indeks)
                .map_or_else(|| piksel(son), piksel);
            if son_piksel >= ilk_piksel {
                boşluklar.push((ilk_piksel, son_piksel));
            }
        }
        indeks = indeks.saturating_add(1);
    }
    if boşluklar.is_empty() {
        return Vec::new();
    }

    let en_yakın_indeks = |hedef_piksel: f32| {
        let hedef_oran = ((hedef_piksel - sol) / genişlik).clamp(0.0, 1.0);
        let hedef_x =
            x_aralığı.en_az + f64::from(hedef_oran) * (x_aralığı.en_çok - x_aralığı.en_az);
        let ekleme = x.partition_point(|değer| *değer < hedef_x);
        match (
            ekleme
                .checked_sub(1)
                .and_then(|indeks| x.get(indeks).map(|değer| (indeks, *değer))),
            x.get(ekleme).map(|değer| (ekleme, *değer)),
        ) {
            (Some((sol_indeks, sol_değer)), Some((_, sağ_değer)))
                if (sol_değer - hedef_x).abs() <= (sağ_değer - hedef_x).abs() =>
            {
                sol_indeks
            }
            (_, Some((sağ_indeks, _))) => sağ_indeks,
            (Some((sol_indeks, _)), None) => sol_indeks,
            (None, None) => 0,
        }
    };

    let mut filtreli = Vec::new();
    if boşluklar
        .first()
        .is_some_and(|boşluk| boşluk.0 == piksel(ilk_görünür))
    {
        filtreli.push(ilk_görünür);
    }
    for çift in boşluklar.windows(2) {
        let [bu_boşluk, sonraki_boşluk] = çift else {
            continue;
        };
        if bu_boşluk.1 != sonraki_boşluk.0 {
            continue;
        }
        let mut yaklaşık = en_yakın_indeks(bu_boşluk.1);
        if y.get(yaklaşık).is_none_or(Option::is_none) {
            for uzaklık in 1..100 {
                if yaklaşık
                    .checked_add(uzaklık)
                    .and_then(|aday| y.get(aday).map(|değer| (aday, değer)))
                    .is_some_and(|(aday, değer)| {
                        if değer.is_some() {
                            yaklaşık = aday;
                            true
                        } else {
                            false
                        }
                    })
                {
                    break;
                }
                if yaklaşık
                    .checked_sub(uzaklık)
                    .and_then(|aday| y.get(aday).map(|değer| (aday, değer)))
                    .is_some_and(|(aday, değer)| {
                        if değer.is_some() {
                            yaklaşık = aday;
                            true
                        } else {
                            false
                        }
                    })
                {
                    break;
                }
            }
        }
        filtreli.push(yaklaşık);
    }
    if boşluklar
        .last()
        .is_some_and(|boşluk| boşluk.1 == piksel(son_görünür))
    {
        filtreli.push(son_görünür);
    }
    filtreli.sort_unstable();
    filtreli.dedup();
    filtreli
}

#[cfg(test)]
mod eksen_testleri {
    use super::*;

    fn standart_nokta_dolguları(grafik: &Grafik) -> Vec<String> {
        grafik
            .çiz()
            .komutlar()
            .iter()
            .filter_map(|komut| match komut {
                Komut::Daireler { dolgu, çizgi, .. } if çizgi == "#123456" => Some(dolgu.clone()),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn nokta_görünürlükleri_varsayılan_açık_ve_bağımsızdır() -> Result<(), UplotHatası> {
        let seçenekler = GrafikSeçenekleri::yeni(400, 240)?.x_zaman(false).seri(
            crate::SeriSeçenekleri::yeni("seri")
                .renk("#123456")
                .noktaları_göster(true),
        );
        assert!(seçenekler.kırılım_noktaları_görünür);
        assert!(seçenekler.imleç_noktaları_görünür);
        let veri = HizalıVeri::yeni(
            vec![0.0, 1.0, 2.0],
            vec![vec![Some(1.0), Some(2.0), Some(1.5)]],
        )?;
        let mut grafik = Grafik::yeni(seçenekler, veri)?;

        assert_eq!(standart_nokta_dolguları(&grafik), ["#ffffff"]);
        assert!(grafik.kırılım_noktalarını_göster(false));
        assert!(!grafik.kırılım_noktalarını_göster(false));
        assert!(standart_nokta_dolguları(&grafik).is_empty());
        assert!(grafik.imleç_noktaları_görünür());

        assert!(grafik.imleç_noktalarını_göster(false));
        assert!(!grafik.imleç_noktalarını_göster(false));
        assert!(!grafik.imleç_noktaları_görünür());
        assert!(!grafik.kırılım_noktaları_görünür());

        assert!(grafik.kırılım_noktalarını_göster(true));
        assert_eq!(standart_nokta_dolguları(&grafik), ["#ffffff"]);
        assert!(!grafik.imleç_noktaları_görünür());
        Ok(())
    }

    #[test]
    fn nokta_filtresi_aynı_pikselde_buluşan_boşlukların_tekilini_seçer() -> Result<(), UplotHatası>
    {
        let x = [0.0, 1.0, 2.0, 3.0, 4.0];
        let y = [Some(1.0), None, Some(2.0), None, Some(3.0)];
        assert_eq!(
            boşluk_tekil_indeksleri(
                &x,
                &y,
                0,
                x.len(),
                Aralık::yeni(0.0, 4.0)?,
                0.0,
                2.0,
                1.0,
                1.0,
            ),
            vec![0, 2, 4]
        );
        Ok(())
    }

    #[test]
    fn piksel_hizası_fiziksel_gpui_ölçeğine_göre_hesaplanır() {
        assert_eq!(piksele_hizala(10.26, 1.0, 1.0), 10.0);
        assert_eq!(piksele_hizala(10.26, 1.0, 2.0), 10.5);
        assert_eq!(piksele_hizala(10.26, 0.0, 2.0), 10.26);
    }

    #[test]
    fn çubuk_vuruş_dizini_yalnız_hedef_hücredeki_dikdörtgeni_döndürür() {
        let anahtar = ÇubukVuruşAnahtarı {
            genişlik: 800,
            yükseklik: 400,
            x_aralığı: Aralık {
                en_az: -0.5,
                en_çok: 1.5,
            },
            y_aralığı: Aralık {
                en_az: 0.0,
                en_çok: 10.0,
            },
            görünür_seriler: vec![true, true],
        };
        let kayıtlar = vec![
            ÇubukVuruşKaydı {
                seri: 0,
                indeks: 0,
                konum: Nokta::yeni(100.0, 100.0),
                genişlik: 40.0,
                yükseklik: 200.0,
                değer: 5.0,
            },
            ÇubukVuruşKaydı {
                seri: 1,
                indeks: 0,
                konum: Nokta::yeni(150.0, 60.0),
                genişlik: 40.0,
                yükseklik: 240.0,
                değer: 8.0,
            },
        ];
        let dizin = ÇubukVuruşDizini::yeni(anahtar, kayıtlar);
        assert_eq!(dizin.vuruş(120.0, 150.0).map(|vuruş| vuruş.seri), Some(0));
        assert_eq!(dizin.vuruş(170.0, 150.0).map(|vuruş| vuruş.seri), Some(1));
        assert!(dizin.vuruş(300.0, 150.0).is_none());
    }

    #[test]
    fn sıralı_x_imleci_ikili_arama_ve_null_koşusu_kuralını_korur() {
        let aralık = Aralık {
            en_az: 0.0,
            en_çok: 4.0,
        };
        let x = [0.0, 1.0, 1.0, 2.0, 3.0, 4.0];
        assert_eq!(en_yakın_x_indeksi(&x, aralık, 1.0), Some(1));
        assert_eq!(en_yakın_x_indeksi(&x, aralık, 1.5), Some(2));

        let y = [Some(0.0), None, None, None, Some(3.0), Some(4.0)];
        assert_eq!(
            en_yakın_dolu_x_indeksi(&x, &y, aralık, 2.0, NullAtlamaYönü::EnYakın),
            Some(4)
        );
        assert_eq!(
            en_yakın_dolu_x_indeksi(&x, &y, aralık, 2.0, NullAtlamaYönü::Önceki),
            Some(0)
        );
    }

    #[test]
    fn çizgi_seyrekleştirme_görünür_x_dilimi_ve_dış_komşuları_tarar() {
        let x = (0..=1_000).map(f64::from).collect::<Vec<_>>();
        let y = x.iter().copied().map(Some).collect::<Vec<_>>();
        let dar = Aralık {
            en_az: 400.0,
            en_çok: 420.0,
        };
        let dar_indeksler = çizilecek_indeksler(&x, &y, dar, 100.0);
        assert_eq!(dar_indeksler.first().copied(), Some(399));
        assert_eq!(dar_indeksler.last().copied(), Some(421));
        assert_eq!(dar_indeksler.len(), 23);

        let yoğun = çizilecek_indeksler(
            &x,
            &y,
            Aralık {
                en_az: 0.0,
                en_çok: 1_000.0,
            },
            10.0,
        );
        assert!(yoğun.len() <= 44);
        assert!(yoğun.windows(2).all(|çift| çift.first() < çift.get(1)));
    }

    #[test]
    fn datum_delta_biçimi_javascript_to_precision_üç_kuralını_korur() {
        assert_eq!(üç_anlamlı_basamak(0.0), "0.00");
        assert_eq!(üç_anlamlı_basamak(12.345), "12.3");
        assert_eq!(üç_anlamlı_basamak(0.000_001_234), "0.00000123");
        assert_eq!(üç_anlamlı_basamak(0.000_000_123_4), "1.23e-7");
        assert_eq!(üç_anlamlı_basamak(12_345.0), "1.23e+4");
        assert_eq!(üç_anlamlı_basamak(-9_876.0), "-9.88e+3");
        assert_eq!(üç_anlamlı_basamak(999.9), "1.00e+3");
    }

    #[test]
    fn bölmeler_sıfıra_hizalanır_ve_yakınlaştıkça_ondalık_detayı_artırır() {
        let tam = Aralık {
            en_az: -1.2,
            en_çok: 1.2,
        };
        let yakın = Aralık {
            en_az: -0.011,
            en_çok: 0.013,
        };
        let tam_artım = uygun_artım(tam, 304.0, 30.0);
        let yakın_artım = uygun_artım(yakın, 304.0, 30.0);
        let yakın_bölmeler = eksen_bölmeleri(yakın, 304.0, 30.0);
        let hizalı_tam_bölmeler = eksen_bölmeleri(tam, 593.0, 30.0);

        assert!(yakın_artım < tam_artım);
        assert!(yakın_bölmeler.contains(&0.0));
        assert!(hizalı_tam_bölmeler.contains(&-1.2));
        assert!(hizalı_tam_bölmeler.contains(&1.2));
        assert_eq!(eksen_değerini_yaz(0.0, yakın_artım), "0.0000");
        assert!(yakın_bölmeler.windows(2).all(|çift| {
            çift
                .first()
                .zip(çift.get(1))
                .is_some_and(|(sol, sağ)| sol < sağ)
        }));
    }

    #[test]
    fn güzel_sayı_kaynak_eşiklerini_korur() {
        assert_eq!(güzel_sayı(149.0, true), Some(100.0));
        assert_eq!(güzel_sayı(150.0, true), Some(200.0));
        assert_eq!(güzel_sayı(225.0, true), Some(200.0));
        assert_eq!(güzel_sayı(226.0, true), Some(250.0));
        assert_eq!(güzel_sayı(299.0, true), Some(250.0));
        assert_eq!(güzel_sayı(300.0, true), Some(500.0));
        assert_eq!(güzel_sayı(699.0, true), Some(500.0));
        assert_eq!(güzel_sayı(700.0, true), Some(1_000.0));
        assert_eq!(güzel_sayı(200.0, false), Some(200.0));
        assert_eq!(güzel_sayı(201.0, false), Some(500.0));
        assert_eq!(güzel_sayı(0.0, false), None);
    }

    #[test]
    fn güzel_ölçek_kaynak_boyut_matrisiyle_eşleşir() {
        let veri = Aralık {
            en_az: -123.0,
            en_çok: 230.0,
        };
        let örnekler = [
            (4.0, -500.0, 500.0, 500.0),
            (104.0, -250.0, 250.0, 250.0),
            (144.0, -200.0, 400.0, 200.0),
            (204.0, -200.0, 300.0, 100.0),
            (384.0, -150.0, 250.0, 50.0),
            (624.0, -150.0, 250.0, 25.0),
            (804.0, -140.0, 240.0, 20.0),
            (1_104.0, -130.0, 240.0, 10.0),
        ];
        for (plot_yüksekliği, alt, üst, artım) in örnekler {
            assert_eq!(
                güzel_ölçek(veri, plot_yüksekliği, 30.0).map(|(aralık, bulunan_artım)| (
                    aralık.en_az,
                    aralık.en_çok,
                    bulunan_artım
                )),
                Some((alt, üst, artım)),
                "plot yüksekliği: {plot_yüksekliği}"
            );
        }
    }

    #[test]
    fn kompakt_değerler_üç_anlamlı_basamağa_sığar() {
        assert_eq!(kompakt_sayı(99_949.0), "99.9K");
        assert_eq!(kompakt_sayı(-1_250.0), "-1.25K");
        assert_eq!(kompakt_sayı(42.0), "42");
    }

    #[test]
    fn log10_bölmeleri_kaynak_birden_dokuza_düzenini_korur() {
        let aralık = Aralık::yeni(1.0, 1_000.0);
        let Ok(aralık) = aralık else { return };
        assert_eq!(
            logaritmik_bölmeler(aralık, 10.0),
            [
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 20.0, 30.0, 40.0, 50.0, 60.0,
                70.0, 80.0, 90.0, 100.0, 200.0, 300.0, 400.0, 500.0, 600.0, 700.0, 800.0, 900.0,
                1_000.0
            ]
        );
        assert_eq!(
            ölçek_eksen_değerini_yaz(
                50_000.0,
                1.0,
                "",
                Some(YÖlçekDağılımı::Logaritmik { taban: 10.0 }),
                YÖlçekEtiketBiçimi::Otomatik,
            ),
            "50000"
        );
        assert_eq!(
            ölçek_eksen_değerini_yaz(
                2_f64.powi(-10),
                1.0,
                "",
                Some(YÖlçekDağılımı::Logaritmik { taban: 2.0 }),
                YÖlçekEtiketBiçimi::İkiliÜs,
            ),
            "2⁻¹⁰"
        );
        assert!(log_etiketi_göster(
            1e-6,
            Aralık {
                en_az: 1e-6,
                en_çok: 1e8,
            },
            600.0,
            Some(YÖlçekDağılımı::Logaritmik { taban: 10.0 }),
            YÖlçekEtiketBiçimi::Bilimsel,
            30.0,
        ));
        assert!(!log_etiketi_göster(
            2e-6,
            Aralık {
                en_az: 1e-6,
                en_çok: 1e8,
            },
            600.0,
            Some(YÖlçekDağılımı::Logaritmik { taban: 10.0 }),
            YÖlçekEtiketBiçimi::Bilimsel,
            30.0,
        ));
        assert!(log_etiketi_göster(
            2_f64.powi(20),
            Aralık {
                en_az: 2_f64.powi(-10),
                en_çok: 2_f64.powi(20),
            },
            204.0,
            Some(YÖlçekDağılımı::Logaritmik { taban: 2.0 }),
            YÖlçekEtiketBiçimi::İkiliŞapka,
            30.0,
        ));
        assert!(!log_etiketi_göster(
            2_f64.powi(18),
            Aralık {
                en_az: 2_f64.powi(-10),
                en_çok: 2_f64.powi(20),
            },
            204.0,
            Some(YÖlçekDağılımı::Logaritmik { taban: 2.0 }),
            YÖlçekEtiketBiçimi::İkiliŞapka,
            30.0,
        ));
    }

    #[test]
    fn log10_etiketleri_piksel_alanına_göre_kaynak_kümelerini_seçer() {
        let aralık = Aralık {
            en_az: 1.0,
            en_çok: 1_000.0,
        };
        let görünür = |boyut: f32| {
            (1..=9)
                .filter(|öncül| {
                    log_etiketi_göster(
                        f64::from(*öncül),
                        aralık,
                        boyut,
                        Some(YÖlçekDağılımı::Logaritmik { taban: 10.0 }),
                        YÖlçekEtiketBiçimi::Otomatik,
                        15.0,
                    )
                })
                .collect::<Vec<_>>()
        };
        assert_eq!(görünür(1_200.0), (1..=9).collect::<Vec<_>>());
        assert_eq!(görünür(600.0), [1, 2, 3, 5, 7]);
        assert_eq!(görünür(240.0), [1, 2, 5]);
        assert_eq!(görünür(90.0), [1]);
    }

    #[test]
    fn zaman_bölmeleri_saat_sınırlarına_hizalanır() {
        let aralık = Aralık::yeni(1_594_953_046.0, 1_595_039_415.0);
        let Ok(aralık) = aralık else { return };
        let (bölmeler, adım) =
            zaman_bölmeleri(aralık, 1_400.0, 50.0, false, crate::ZamanDilimi::Utc, None);
        assert_eq!(adım, 3_600.0);
        assert!(
            bölmeler
                .iter()
                .all(|değer| (*değer % 3_600.0).abs() <= f64::EPSILON)
        );
    }

    #[test]
    fn zaman_bölmeleri_kaynak_dört_sekiz_saat_ve_üç_gün_adımlarını_seçer() -> Result<(), UplotHatası>
    {
        let adımı_seç = |hedef: f64, milisaniye: bool| {
            let aralık = Aralık::yeni(0.0, hedef * 28.0)?;
            Ok::<_, UplotHatası>(
                zaman_bölmeleri(
                    aralık,
                    1_400.0,
                    50.0,
                    milisaniye,
                    crate::ZamanDilimi::Utc,
                    None,
                )
                .1,
            )
        };
        assert_eq!(adımı_seç(0.001, false)?, 0.001);
        assert_eq!(adımı_seç(1.1, false)?, 5.0);
        assert_eq!(adımı_seç(4.0 * 3_600.0, false)?, 4.0 * 3_600.0);
        assert_eq!(adımı_seç(8.0 * 3_600.0, false)?, 8.0 * 3_600.0);
        assert_eq!(adımı_seç(3.0 * 86_400.0, false)?, 3.0 * 86_400.0);
        assert_eq!(adımı_seç(60.0 * 86_400.0, false)?, 60.0 * 86_400.0);
        assert_eq!(adımı_seç(100.0 * 365.0 * 86_400.0, false)?, 3_153_600_000.0);
        assert_eq!(adımı_seç(2.1, true)?, 5.0);
        assert_eq!(adımı_seç(25.0, true)?, 25.0);
        Ok(())
    }

    #[test]
    fn aylık_zaman_bölmeleri_gerçek_takvim_sınırlarına_hizalanır() {
        let aralık = Aralık::yeni(1_483_228_800.0, 1_575_158_400.0);
        let Ok(aralık) = aralık else { return };
        let (bölmeler, adım) =
            zaman_bölmeleri(aralık, 1_850.0, 50.0, false, crate::ZamanDilimi::Utc, None);
        assert_eq!(adım, 2_592_000.0);
        assert!(!bölmeler.is_empty());
        assert!(bölmeler.iter().all(|değer| {
            crate::zaman::utc_alanları(*değer).is_some_and(|(_, _, gün, saat, dakika, saniye)| {
                gün == 1 && saat == 0 && dakika == 0 && saniye == 0
            })
        }));
    }
}
