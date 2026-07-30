//! GPUI çizim yüzeyi ve etkileşim adaptörü.

#[cfg(feature = "gpui-svg")]
mod svg_kaydi;
#[cfg(feature = "gpui-svg")]
pub use svg_kaydi::{GpuiSvgKaydı, GpuiSvgKayıtAyarları};

use std::cell::{Cell, RefCell};
use std::collections::{HashMap, hash_map::DefaultHasher};
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;

use ::gpui::{
    AnyElement, App, BorderStyle, Bounds, ContentMask, Context, Corners, Entity, EventEmitter,
    FocusHandle, Hsla, IntoElement, KeyBinding, KeyDownEvent, KeyUpEvent, MouseButton,
    MouseDownEvent, MouseMoveEvent, MouseUpEvent, Path, PathBuilder, PinchEvent, Pixels, Render,
    Role, ScrollDelta, ScrollWheelEvent, SharedString, StyleRefinement, Task, TextAlign, TextRun,
    TouchPhase, WeakEntity, Window, canvas, div, linear_color_stop, linear_gradient, point,
    prelude::*, px, quad, rgb, rgba, size,
};

use crate::grafik::OransalGörünüm;
use crate::{
    Aralık, AçıklamaVuruşu, BoyutSenkronDüzeni, DağılımVuruşu, DoğrusalGradyan,
    EnYakınTooltipBilgisi, Grafik, HizalıVeri, Komut, MetinHizası, Nokta, Sahne, SeriBandı,
    SeriSeçenekleri, SeçimEylemi, TekerlekEkseni, UplotHatası, YüzeyDikdörtgeni,
    bilgi_kutusunu_yerleştir,
};
use web_time::Instant;

/// Yoğun yüzeyleri tek sprite'a indiren CPU rasterleştirici.
mod raster;

/// Yüzeylerin görünür alana uyarlanması.
#[path = "gpui/yerlesim.rs"]
mod yerleşim;
pub use yerleşim::{GörünürAlan, uyarlanan_alan};

::gpui::actions!(
    uplot_rs,
    [ÖlçümüTemizle, BirinciDatumuAyarla, İkinciDatumuAyarla]
);

/// uPlot.rs grafiklerinin varsayılan GPUI klavye eylemlerini kaydeder.
///
/// Uygulama başlatılırken bir kez çağrılmalıdır. Eylemler yalnız
/// `uplot_rs_grafik` bağlamında çalışır; uygulama isterse GPUI keymap
/// katmanında bunları geçersiz kılabilir.
pub fn başlat(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("escape", ÖlçümüTemizle, Some("uplot_rs_grafik")),
        KeyBinding::new("1", BirinciDatumuAyarla, Some("uplot_rs_grafik")),
        KeyBinding::new("2", İkinciDatumuAyarla, Some("uplot_rs_grafik")),
    ]);
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct GpuiYüzeyDönüşümü {
    ölçek: f32,
    köken_x: f32,
    köken_y: f32,
}

/// Veri yüzeyinin boyama bağlamı.
///
/// `pencere` geçmişte yakınlaştırmayı GPU dönüşümüne çeviriyordu; sahne artık
/// her görünüm değişiminde güncel pencere için kurulduğundan (uPlot
/// `s._paths = null`) hep birimdir ve [`GpuiBoyaGörünümü`] pratikte yalnız
/// çizim alanı kırpmasını taşır. [`Grafik::oransal_görünüm`] render yolunda
/// okunmaz; senkron grupları ve zoom-ranger için durur.
#[derive(Clone, Copy, Debug, PartialEq)]
struct GpuiVeriGörünümü {
    pencere: OransalGörünüm,
    çizim_alanı: (f32, f32, f32, f32),
}

#[derive(Clone, Copy)]
struct GpuiBoyaGörünümü {
    kesme_sınırları: Bounds<Pixels>,
    mantıksal_çizim_alanı: (f32, f32, f32, f32),
    x_ölçeği: f32,
    y_ölçeği: f32,
    x_kaydırması: f32,
    y_kaydırması: f32,
}

/// Sahne komutlarının geometrik noktalarının ötesine taşan azami görsel pay.
///
/// uPlot kırpma dikdörtgenini seri kalınlığının yarısı kadar genişletir
/// (`drawPath`: `plotLft - width / 2`, `plotWid + width`), nokta işaretleri
/// içinse nokta çapı kadar (`paths/points.js`: `lft - dia`, `wid + dia * 2`).
/// Aksi hâlde tam çizim sınırındaki çizgi yarım, sınırdaki nokta işareti ise
/// ortadan tıraşlanır.
fn kırpma_payı(sahne: &Sahne) -> f32 {
    let mut azami = 0.0_f32;
    let mut aday = |değer: f32| {
        if değer.is_finite() && değer > azami {
            azami = değer;
        }
    };
    for komut in sahne.komutlar() {
        match komut {
            Komut::Çizgi { kalınlık, .. }
            | Komut::KesikliÇizgi { kalınlık, .. }
            | Komut::Yol { kalınlık, .. }
            | Komut::GradyanYol { kalınlık, .. }
            | Komut::KesikliYol { kalınlık, .. }
            | Komut::Dikdörtgen { kalınlık, .. }
            | Komut::YuvarlatılmışDikdörtgen { kalınlık, .. } => aday(*kalınlık / 2.0),
            Komut::Daire {
                yarıçap, kalınlık,
            ..
            }
            | Komut::Daireler {
                yarıçap, kalınlık,
            ..
            } => aday(*yarıçap + *kalınlık / 2.0),
            Komut::DeğişkenDaireler {
                daireler, kalınlık,
            ..
            } => {
                let en_geniş = daireler
                    .iter()
                    .map(|(_, yarıçap)| *yarıçap)
                    .fold(0.0_f32, f32::max);
                aday(en_geniş + *kalınlık / 2.0);
            }
            _ => {}
        }
    }
    azami
}

impl GpuiBoyaGörünümü {
    fn hesapla(
        görünüm: GpuiVeriGörünümü,
        yüzey: GpuiYüzeyDönüşümü,
        kırpma_payı: f32,
    ) -> Option<Self> {
        let (sol, sağ, üst, alt) = görünüm.çizim_alanı;
        let genişlik = sağ - sol;
        let yükseklik = alt - üst;
        let kaynak_sol = sol + görünüm.pencere.sol * genişlik;
        let kaynak_sağ = sol + görünüm.pencere.sağ * genişlik;
        let kaynak_üst = üst + görünüm.pencere.üst * yükseklik;
        let kaynak_alt = üst + görünüm.pencere.alt * yükseklik;
        let kaynak_genişlik = kaynak_sağ - kaynak_sol;
        let kaynak_yükseklik = kaynak_alt - kaynak_üst;
        if kaynak_genişlik <= f32::EPSILON || kaynak_yükseklik <= f32::EPSILON {
            return None;
        }

        let x_ölçeği = genişlik / kaynak_genişlik;
        let y_ölçeği = yükseklik / kaynak_yükseklik;
        let hedef_sol = yüzey.köken_x + sol * yüzey.ölçek;
        let hedef_üst = yüzey.köken_y + üst * yüzey.ölçek;
        let kaynak_fiziksel_sol = yüzey.köken_x + kaynak_sol * yüzey.ölçek;
        let kaynak_fiziksel_üst = yüzey.köken_y + kaynak_üst * yüzey.ölçek;
        let x_kaydırması = hedef_sol - x_ölçeği * kaynak_fiziksel_sol;
        let y_kaydırması = hedef_üst - y_ölçeği * kaynak_fiziksel_üst;
        Some(Self {
            kesme_sınırları: Bounds::new(
                point(
                    px(hedef_sol - kırpma_payı * yüzey.ölçek),
                    px(hedef_üst - kırpma_payı * yüzey.ölçek),
                ),
                size(
                    px((genişlik + kırpma_payı * 2.0) * yüzey.ölçek),
                    px((yükseklik + kırpma_payı * 2.0) * yüzey.ölçek),
                ),
            ),
            mantıksal_çizim_alanı: görünüm.çizim_alanı,
            x_ölçeği,
            y_ölçeği,
            x_kaydırması,
            y_kaydırması,
        })
    }

    fn noktayı_dönüştür(self, nokta: ::gpui::Point<Pixels>) -> ::gpui::Point<Pixels> {
        point(
            px(self.x_ölçeği * f32::from(nokta.x) + self.x_kaydırması),
            px(self.y_ölçeği * f32::from(nokta.y) + self.y_kaydırması),
        )
    }

    fn sınırları_dönüştür(self, sınırlar: Bounds<Pixels>) -> Bounds<Pixels> {
        Bounds::from_corners(
            self.noktayı_dönüştür(sınırlar.origin),
            self.noktayı_dönüştür(point(sınırlar.right(), sınırlar.bottom())),
        )
    }
}

impl GpuiYüzeyDönüşümü {
    fn hesapla(
        kaynak_g: u32,
        kaynak_y: u32,
        hedef_x: f32,
        hedef_y: f32,
        hedef_g: f32,
        hedef_yükseklik: f32,
    ) -> Self {
        let ölçek = (hedef_g / kaynak_g as f32)
            .min(hedef_yükseklik / kaynak_y as f32)
            .max(0.01);
        let içerik_g = kaynak_g as f32 * ölçek;
        let içerik_y = kaynak_y as f32 * ölçek;
        let köken_x = hedef_x + (hedef_g - içerik_g) / 2.0;
        let köken_y = hedef_y + (hedef_yükseklik - içerik_y) / 2.0;
        Self {
            ölçek,
            köken_x,
            köken_y,
        }
    }
}

#[derive(Clone, PartialEq)]
struct İmleçDurumu {
    fare: Nokta,
    veri_x: f64,
    seri_x_değerleri: Vec<Option<f64>>,
    seri_değerleri: Vec<Option<f64>>,
    dağılım: Option<DağılımVuruşu>,
}

impl İmleçDurumu {
    fn lejant_verisi_aynı(&self, diğer: &Self) -> bool {
        self.veri_x == diğer.veri_x
            && self.seri_x_değerleri == diğer.seri_x_değerleri
            && self.seri_değerleri == diğer.seri_değerleri
            && self.dağılım == diğer.dağılım
    }
}

fn imleç_lejant_verisi_aynı(sol: Option<&İmleçDurumu>, sağ: Option<&İmleçDurumu>) -> bool {
    match (sol, sağ) {
        (Some(sol), Some(sağ)) => sol.lejant_verisi_aynı(sağ),
        (None, None) => true,
        _ => false,
    }
}

#[derive(Clone, Copy, Debug)]
pub enum GpuiGrafikOlayı {
    DurumDeğişti,
    /// İmlecin görsel konumu değişti. Senkron yüzeyler bu yüksek frekanslı
    /// olayı kullanır; dış lejantlar yalnız örnek değeri değiştiğinde yayılan
    /// [`GpuiGrafikOlayı::İmleçDeğişti`] olayını dinleyebilir.
    İmleçKonumuDeğişti,
    /// Görünür ölçek değişti. `fare_basma_bırakma` uPlot sync filtresinin
    /// seçim yakınlaştırmasına uygulanabilmesi için olay nedenini korur.
    GörünümDeğişti {
        fare_basma_bırakma: bool,
    },
    İmleçDeğişti,
    FareBırakıldı,
    /// `cursor-bind` Ctrl seçimi tamamlandı; üst uygulama metin UI'si açabilir.
    Açıklamaİstendi,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GpuiSeriEşleme {
    /// Kaynak seri sırasını hedefteki aynı indekse taşır.
    #[default]
    İndeks,
    /// Kaynak seri etiketini hedefte arar.
    Etiket,
}

/// Bir [`GpuiGrafikGrubu`] içindeki ortak davranışların ayarları.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GpuiGrafikGrupAyarları {
    pub imleç: bool,
    pub görünüm: bool,
    pub seçim_görünümü: bool,
    pub seri_görünürlüğü: bool,
    pub imleç_kilidi: bool,
    pub seri_eşleme: GpuiSeriEşleme,
}

impl Default for GpuiGrafikGrupAyarları {
    fn default() -> Self {
        Self {
            imleç: true,
            görünüm: true,
            seçim_görünümü: true,
            seri_görünürlüğü: true,
            imleç_kilidi: true,
            seri_eşleme: GpuiSeriEşleme::İndeks,
        }
    }
}

impl GpuiGrafikGrupAyarları {
    pub fn seri_eşleme(mut self, eşleme: GpuiSeriEşleme) -> Self {
        self.seri_eşleme = eşleme;
        self
    }

    pub fn seçim_görünümü(mut self, etkin: bool) -> Self {
        self.seçim_görünümü = etkin;
        self
    }

    pub fn imleç_kilidi(mut self, etkin: bool) -> Self {
        self.imleç_kilidi = etkin;
        self
    }
}

#[derive(Clone)]
struct GpuiGrafikGrupÜyesi {
    kimlik: String,
    grafik: Entity<GpuiGrafik>,
}

/// Birden fazla GPUI grafik yüzeyini normalize oranlarla birlikte yönetir.
///
/// Üyelerin piksel boyutları, veri aralıkları ve Y ölçekleri farklı olabilir.
/// Cursor, wheel, seçim, pan, eksen zoomu ve tam görünüm kaynak yüzeyin tam
/// ölçeklerindeki fiziksel oran penceresiyle hedeflere aktarılır.
pub struct GpuiGrafikGrubu {
    üyeler: Vec<GpuiGrafikGrupÜyesi>,
    ayarlar: GpuiGrafikGrupAyarları,
    etkin: bool,
    yayılıyor: bool,
    imleç_kilitli: bool,
    son_hata: Option<String>,
}

impl GpuiGrafikGrubu {
    pub fn yeni(ayarlar: GpuiGrafikGrupAyarları) -> Self {
        Self {
            üyeler: Vec::new(),
            ayarlar,
            etkin: true,
            yayılıyor: false,
            imleç_kilitli: false,
            son_hata: None,
        }
    }

    pub fn etkin(&self) -> bool {
        self.etkin
    }

    pub fn etkinliği_ayarla(&mut self, etkin: bool) -> bool {
        let değişti = self.etkin != etkin;
        self.etkin = etkin;
        değişti
    }

    pub fn ayarlar(&self) -> GpuiGrafikGrupAyarları {
        self.ayarlar
    }

    pub fn seçim_görünümünü_ayarla(&mut self, etkin: bool) -> bool {
        let değişti = self.ayarlar.seçim_görünümü != etkin;
        self.ayarlar.seçim_görünümü = etkin;
        değişti
    }

    pub fn son_hata(&self) -> Option<&str> {
        self.son_hata.as_deref()
    }

    pub fn üye_sayısı(&self) -> usize {
        self.üyeler.len()
    }

    pub fn grafik_ekle(
        &mut self,
        kimlik: impl Into<String>,
        grafik: Entity<GpuiGrafik>,
        cx: &mut Context<Self>,
    ) -> bool {
        let kimlik = kimlik.into();
        if self.üyeler.iter().any(|üye| üye.kimlik == kimlik) {
            return false;
        }
        let olay_kimliğini = kimlik.clone();
        cx.subscribe(&grafik, move |grup, _, olay: &GpuiGrafikOlayı, cx| {
            grup.olayı_yay(&olay_kimliğini, *olay, cx);
        })
        .detach();
        self.üyeler.push(GpuiGrafikGrupÜyesi { kimlik, grafik });
        true
    }

    fn olayı_yay(
        &mut self, kaynak_kimliği: &str, olay: GpuiGrafikOlayı, cx: &mut Context<Self>
    ) {
        if !self.etkin || self.yayılıyor {
            return;
        }
        let Some(kaynak) = self
            .üyeler
            .iter()
            .find(|üye| üye.kimlik == kaynak_kimliği)
            .map(|üye| üye.grafik.clone())
        else {
            return;
        };
        let hedefler = self
            .üyeler
            .iter()
            .filter(|üye| üye.kimlik != kaynak_kimliği)
            .map(|üye| üye.grafik.clone())
            .collect::<Vec<_>>();
        self.yayılıyor = true;
        match olay {
            GpuiGrafikOlayı::İmleçKonumuDeğişti if self.ayarlar.imleç => {
                let yayın = {
                    let kaynak = kaynak.read(cx);
                    kaynak.oransal_imleç_yayını().map(|(x, y, seri)| {
                        let etiket = seri.and_then(|indeks| {
                            kaynak
                                .grafik()
                                .seri_seçenekleri()
                                .get(indeks)
                                .map(|seri| seri.etiket.clone())
                        });
                        (x, y, seri, etiket)
                    })
                };
                for hedef in hedefler {
                    hedef.update(cx, |hedef, cx| {
                        if let Some((x, y, kaynak_serisi, kaynak_etiketi)) = yayın.as_ref() {
                            let odak_serisi = eşlenen_seri_indeksi(
                                hedef,
                                *kaynak_serisi,
                                kaynak_etiketi.as_deref(),
                                self.ayarlar.seri_eşleme,
                            );
                            hedef.senkron_imleci_ayarla(*x, Some(*y), odak_serisi, cx);
                        } else {
                            hedef.senkron_imleci_temizle(cx);
                        }
                    });
                }
            }
            GpuiGrafikOlayı::GörünümDeğişti {
                fare_basma_bırakma
            } if self.ayarlar.görünüm && (!fare_basma_bırakma || self.ayarlar.seçim_görünümü) =>
            {
                let görünüm = kaynak.read(cx).oransal_görünüm_yayını();
                for hedef in hedefler {
                    let sonuç = hedef.update(cx, |hedef, cx| {
                        // Grup zaten bütün hedefleri doğrudan güncelliyor.
                        // Hedefin aynı GörünümDeğişti olayını tekrar yayması,
                        // GPUI'nin ertelenmiş emit kuyruğunda kaynak/target
                        // ping-pong'u ve her üyede ayrı geçmiş girdisi üretir.
                        hedef.oransal_görünümü_sessiz_ayarla(görünüm, cx)
                    });
                    if let Err(hata) = sonuç {
                        self.son_hata = Some(format!("Grup görünümü uygulanamadı: {hata}"));
                    }
                }
            }
            GpuiGrafikOlayı::DurumDeğişti if self.ayarlar.seri_görünürlüğü => {
                let durum = {
                    let kaynak = kaynak.read(cx);
                    kaynak
                        .grafik()
                        .seri_seçenekleri()
                        .iter()
                        .enumerate()
                        .map(|(indeks, seri)| {
                            (seri.etiket.clone(), kaynak.grafik().seri_görünür_mü(indeks))
                        })
                        .collect::<Vec<_>>()
                };
                for hedef in hedefler {
                    hedef.update(cx, |hedef, cx| {
                        let mut değişti = false;
                        for (kaynak_indeksi, (etiket, görünür)) in durum.iter().enumerate() {
                            let hedef_indeksi = eşlenen_seri_indeksi(
                                hedef,
                                Some(kaynak_indeksi),
                                Some(etiket),
                                self.ayarlar.seri_eşleme,
                            );
                            if let Some(hedef_indeksi) = hedef_indeksi
                                && hedef
                                    .grafik
                                    .seri_görünürlüğünü_ayarla(hedef_indeksi, *görünür)
                                    .unwrap_or(false)
                            {
                                değişti = true;
                            }
                        }
                        if değişti {
                            // Grup hedefi kaynağa geri DurumDeğişti yaymaz;
                            // bütün üyeler bu turda zaten doğrudan güncellenir.
                            hedef.sahneyi_yenile(cx);
                            cx.notify();
                        }
                    });
                }
            }
            GpuiGrafikOlayı::FareBırakıldı if self.ayarlar.imleç_kilidi => {
                self.imleç_kilitli = !self.imleç_kilitli;
                for üye in &self.üyeler {
                    üye.grafik.update(cx, |grafik, cx| {
                        grafik.senkron_kilidi_ayarla(self.imleç_kilitli, cx);
                    });
                }
            }
            _ => {}
        }
        self.yayılıyor = false;
    }
}

fn eşlenen_seri_indeksi(
    hedef: &GpuiGrafik,
    kaynak_indeksi: Option<usize>,
    kaynak_etiketi: Option<&str>,
    eşleme: GpuiSeriEşleme,
) -> Option<usize> {
    match eşleme {
        GpuiSeriEşleme::İndeks => {
            kaynak_indeksi.filter(|indeks| *indeks < hedef.grafik().seri_seçenekleri().len())
        }
        GpuiSeriEşleme::Etiket => {
            let etiket = kaynak_etiketi?;
            hedef
                .grafik()
                .seri_seçenekleri()
                .iter()
                .position(|seri| seri.etiket == etiket)
        }
    }
}

/// Çekirdek [`Grafik`] durumunu GPUI canvas üzerinde gösteren hazır bileşen.
///
/// Bileşen platform olaylarını çekirdeğe iletir; yakınlaştırma, seçim, geçmiş
/// ve tekerlek normalizasyonunu uygulama kodunun tekrar etmesi gerekmez.
pub struct GpuiGrafik {
    grafik: Grafik,
    arka_plan_sahnesi: Rc<Sahne>,
    arka_plan_yüzeyi: Option<Entity<GpuiEtkileşimYüzeyi>>,
    ana_sahne: Rc<Sahne>,
    ana_yüzey: Option<Entity<GpuiAnaYüzey>>,
    eksen_sahnesi: Rc<Sahne>,
    eksen_yüzeyi: Option<Entity<GpuiEtkileşimYüzeyi>>,
    etkileşim_sahnesi: Rc<Sahne>,
    etkileşim_sahne_tamponu: Option<Sahne>,
    etkileşim_yüzeyi: Option<Entity<GpuiEtkileşimYüzeyi>>,
    ana_sahne_revizyonu: u64,
    görünüm_revizyonu: u64,
    etkileşim_sahne_revizyonu: u64,
    imleç: Option<İmleçDurumu>,
    seçim: Option<(Nokta, Nokta)>,
    açıklama_seçimi: bool,
    taşıma_başlangıcı: Option<Nokta>,
    dokunma_kaydırma: Option<(f64, f64)>,
    boşluk_basılı: bool,
    hata: Option<String>,
    çizim_sınırları: Rc<Cell<Option<Bounds<Pixels>>>>,
    veri_görünümü: Rc<Cell<GpuiVeriGörünümü>>,
    odak: Option<FocusHandle>,
    imleç_kilitli: bool,
    /// İmleç çizgisinin en yakın örneğin X konumuna oturup oturmayacağı.
    ///
    /// Ctrl basılıyken açılır. Kapalıyken çizgi fareyi kesintisiz izler;
    /// lejant ve odak değerleri her iki durumda da en yakın örnekten çözülür.
    imleç_değere_yapışsın: bool,
    boyut_senkron_katmanı: Option<BoyutSenkronDüzeni>,
    eksen_üzerinde: bool,
    açıklama_vuruşu: Option<AçıklamaVuruşu>,
    tooltip_tıklama_başlangıcı: Option<(Nokta, String)>,
    tooltip_tıklaması_sürüklendi: bool,
    bilgi_balonu_hazır: bool,
    bilgi_balonu_beklemesi: Option<Task<()>>,
    bilgi_balonu_son_hareket: Option<Instant>,
    #[cfg(test)]
    etkileşim_sahne_hazırlama_sayısı: u64,
}

struct GpuiAnaYüzey {
    sahne: Rc<Sahne>,
    çizim_sınırları: Rc<Cell<Option<Bounds<Pixels>>>>,
    yol_önbelleği: Rc<RefCell<GpuiYolÖnbelleği>>,
    duyarlı_grafik: Option<WeakEntity<GpuiGrafik>>,
    veri_görünümü: Rc<Cell<GpuiVeriGörünümü>>,
}

struct GpuiEtkileşimYüzeyi {
    sahne: Rc<Sahne>,
    yol_önbelleği: Rc<RefCell<GpuiYolÖnbelleği>>,
    /// Yalnız imleç katmanında doludur. uPlot imleç elemanlarını `.u-over`
    /// içinde tutar; o kap tam çizim dikdörtgeni ve `overflow: hidden`.
    /// Arka plan ve eksen katmanları çizim alanının dışına çizmek zorunda
    /// olduğu için kırpılmaz.
    çizim_görünümü: Option<Rc<Cell<GpuiVeriGörünümü>>>,
}

fn duyarlı_boyut_güncellenmeli(önceki: Option<Bounds<Pixels>>, güncel: Bounds<Pixels>) -> bool {
    önceki.is_none_or(|önceki| önceki.size != güncel.size)
}

/// Boyanan yol renklerinin test kaydı.
///
/// Sahne komutları doğruyken boyama aşamasında yanlış renk üretmek — gradyan
/// maskelerinin çakışıp bir dalın kaybolması gibi — komut testlerine
/// görünmüyor. `Window::rendered_frame` gpui'de `pub(crate)` olduğundan
/// boyanan primitive'lere dışarıdan da erişilemiyor. Bütün yol boyamaları
/// [`retained_yolu_boya`] üzerinden geçtiği için kayıt orada tek noktada
/// alınır.
///
/// `boya-gunlugu` özelliği kapalıyken bütün çağrılar boştur ve derleyici
/// tarafından elenir.
pub mod boya_günlüğü {
    #[cfg(feature = "boya-gunlugu")]
    use std::cell::RefCell;

    #[cfg(feature = "boya-gunlugu")]
    thread_local! {
        static KAYITLAR: RefCell<Vec<::gpui::Hsla>> = const { RefCell::new(Vec::new()) };
    }

    /// Kaydı sıfırlar. Ölçülecek kareden hemen önce çağrılır.
    pub fn temizle() {
        #[cfg(feature = "boya-gunlugu")]
        KAYITLAR.with(|kayıtlar| kayıtlar.borrow_mut().clear());
    }

    /// Son temizlemeden bu yana ekrana giden renkler, boyanma sırasıyla.
    ///
    /// Düz boyalar tek kayıt, gradyan şeritleri iki durak rengi bırakır.
    #[must_use]
    pub fn kayıtlar() -> Vec<::gpui::Hsla> {
        #[cfg(feature = "boya-gunlugu")]
        return KAYITLAR.with(|kayıtlar| kayıtlar.borrow().clone());
        #[cfg(not(feature = "boya-gunlugu"))]
        Vec::new()
    }

    /// Düz boyayı kaydeder; gradyanlar `yaz_renk` ile kendi duraklarını yazar.
    #[cfg_attr(not(feature = "boya-gunlugu"), expect(unused_variables))]
    pub(super) fn yaz(boya: ::gpui::Background) {
        #[cfg(feature = "boya-gunlugu")]
        if let Some(düz) = boya.as_solid() {
            yaz_renk(düz);
        }
    }

    #[cfg_attr(not(feature = "boya-gunlugu"), expect(unused_variables))]
    pub(super) fn yaz_renk(renk: ::gpui::Hsla) {
        #[cfg(feature = "boya-gunlugu")]
        KAYITLAR.with(|kayıtlar| kayıtlar.borrow_mut().push(renk));
    }
}

/// Yapışma açıkken ikinci eksenin oturacağı konumu seçer.
///
/// `oranlar` yapışılan X'teki seri değerlerinin eksen oranıdır; `None`
/// girdiler o X'te örneği olmayan serilerdir. Dönen değer, fareye
/// (`ham`) en yakın adayın çizim alanı içindeki konumudur.
///
/// Y oranı eksen başlangıcından ölçülür. Dikey ekran ekseninde sıfır
/// alttadır, bu yüzden normal yönelimde çevrilir; X dikeyken ikinci eksen
/// yataydır ve oran doğrudan kullanılır.
fn ikincil_yapışma_konumu(oranlar: &[Option<f64>], ham: f64, x_dikey: bool) -> Option<f64> {
    oranlar
        .iter()
        .filter_map(|oran| {
            let oran = (*oran)?;
            let konum = if x_dikey { oran } else { 1.0 - oran };
            konum.is_finite().then(|| (konum, (konum - ham).abs()))
        })
        .min_by(|(_, sol), (_, sağ)| sol.total_cmp(sağ))
        .map(|(konum, _)| konum)
}

/// Yüzeyin sıfır yüksekliğe çökmesini önleyen taban.
///
/// Grafik kökü sarmalayıcının yüksekliğini devralır; esnek bir kapta ölçüm
/// sıfır dönerse yüzey hiç görünmez. Taban bunu engeller, ama grafiğin kendi
/// ham yüksekliğini aşamaz: `sparklines` 150×30 hücrelere yerleşiyor ve sabit
/// 120 px taban her yüzeyi kendinden sonraki üç satırın üstüne taşırıyordu.
/// Sonra çizilen üstte kaldığından tabloda yalnız son satır görünür oluyordu.
const fn en_az_yüzey_yüksekliği(ham_yükseklik: u32) -> f32 {
    const TABAN: f32 = 120.0;
    if ham_yükseklik as f32 >= TABAN {
        TABAN
    } else {
        ham_yükseklik as f32
    }
}

impl GpuiEtkileşimYüzeyi {
    fn sahneyi_ayarla(&mut self, sahne: Rc<Sahne>) {
        self.yol_önbelleği
            .borrow_mut()
            .sahneyi_değiştir(&self.sahne, &sahne);
        self.sahne = sahne;
    }
}

impl Render for GpuiEtkileşimYüzeyi {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let sahne = self.sahne.clone();
        let yol_önbelleği = self.yol_önbelleği.clone();
        let çizim_görünümü = self.çizim_görünümü.clone();
        canvas(
            |_, _, _| {},
            move |sınırlar, _, pencere, uygulama| {
                let çizim_kırpması = çizim_görünümü
                    .as_ref()
                    .map(|görünüm| görünüm.get().çizim_alanı);
                sahneyi_önbellekli_boya(
                    &sahne,
                    sınırlar,
                    &mut yol_önbelleği.borrow_mut(),
                    None,
                    çizim_kırpması,
                    pencere,
                    uygulama,
                );
            },
        )
        .size_full()
    }
}

impl GpuiAnaYüzey {
    fn sahneyi_ayarla(
        &mut self, sahne: Rc<Sahne>, duyarlı_grafik: Option<WeakEntity<GpuiGrafik>>
    ) {
        self.yol_önbelleği
            .borrow_mut()
            .sahneyi_değiştir(&self.sahne, &sahne);
        self.sahne = sahne;
        self.duyarlı_grafik = duyarlı_grafik;
    }
}

#[derive(Default)]
struct GpuiYolÖnbelleği {
    sahne_boyutu: Option<(u32, u32)>,
    sınırlar: Option<Bounds<Pixels>>,
    yollar: Vec<Option<ÖnbellekliGpuiYol>>,
    ikincil_yollar: Vec<Option<ÖnbellekliGpuiYol>>,
    renkler: HashMap<String, Hsla>,
    veri_komutları: Vec<bool>,
    veri_komutu_çizim_alanı: Option<(u32, u32, u32, u32)>,
    /// Yoğun yüzeyin tek sprite'a indirilmiş hâli.
    ///
    /// `None` iken yüzey vektör yolundadır. Anahtar fiziksel çözünürlüktür;
    /// veri, ölçek/görünüm veya boyut değişimi `yüzeyi_hazırla` içinde
    /// önbelleği zaten düşürüyor, cihaz piksel oranı değişimi ise anahtarla
    /// yakalanıyor. Tetikleyici kümesi uPlot'un `_commit()` eşleniğidir.
    raster: Option<(u32, u32, Arc<::gpui::RenderImage>)>,
}

/// Kareler arası saklanan, pencere konumuna yerleştirilmiş yol.
///
/// Cihaz ölçeklemesini `Window::paint_path` her gönderimde yapar. Yüzeyin
/// pencere içindeki kökeni yalnız kaydırmada değiştiği için ötelemeyi de
/// burada tutmak, sabit karelerde köşe başına bir geçişi tamamen kaldırır;
/// geriye yalnız `paint_path`'in kendi ölçekleme geçişi kalır.
#[derive(Clone)]
struct ÖnbellekliGpuiYol {
    yol: Path<Pixels>,
    öteleme: ::gpui::Point<Pixels>,
}

impl ÖnbellekliGpuiYol {
    fn yeni(yol: Path<Pixels>) -> Self {
        Self {
            yol,
            öteleme: point(px(0.0), px(0.0)),
        }
    }

    /// Yolu istenen ötelemeye taşır ve boyanabilir kopyasını verir.
    fn boyanabilir(&mut self, hedef: ::gpui::Point<Pixels>) -> BoyanabilirGpuiYol {
        if self.öteleme != hedef {
            let fark = point(hedef.x - self.öteleme.x, hedef.y - self.öteleme.y);
            for köşe in &mut self.yol.vertices {
                köşe.xy_position += fark;
            }
            self.yol.bounds.origin += fark;
            self.öteleme = hedef;
        }
        BoyanabilirGpuiYol {
            mantıksal_sınırlar: self.yol.bounds,
            yol: self.yol.clone(),
        }
    }
}

#[derive(Clone)]
struct BoyanabilirGpuiYol {
    mantıksal_sınırlar: Bounds<Pixels>,
    yol: Path<Pixels>,
}

impl GpuiYolÖnbelleği {
    fn renk(&mut self, kod: &str) -> Hsla {
        if let Some(renk) = self.renkler.get(kod) {
            return *renk;
        }
        let renk = renk_çöz(kod);
        self.renkler.insert(kod.to_owned(), renk);
        renk
    }

    fn yüzeyi_hazırla(&mut self, sahne: &Sahne, sınırlar: Bounds<Pixels>) {
        let sahne_boyutu = sahne.boyut();
        let boyut_değişti = self
            .sınırlar
            .is_some_and(|önceki| önceki.size != sınırlar.size);
        if self.sahne_boyutu != Some(sahne_boyutu) || boyut_değişti {
            self.sahne_boyutu = Some(sahne_boyutu);
            self.yollar.clear();
            self.ikincil_yollar.clear();
            self.veri_komutları.clear();
            self.veri_komutu_çizim_alanı = None;
            self.raster = None;
        }
        self.sınırlar = Some(sınırlar);
        if self.yollar.len() != sahne.komutlar().len() {
            self.yollar.resize_with(sahne.komutlar().len(), || None);
            self.ikincil_yollar
                .resize_with(sahne.komutlar().len(), || None);
        }
    }

    fn veri_komutlarını_hazırla(&mut self, sahne: &Sahne, çizim_alanı: (f32, f32, f32, f32)) {
        let anahtar = (
            çizim_alanı.0.to_bits(),
            çizim_alanı.1.to_bits(),
            çizim_alanı.2.to_bits(),
            çizim_alanı.3.to_bits(),
        );
        if self.veri_komutu_çizim_alanı == Some(anahtar)
            && self.veri_komutları.len() == sahne.komutlar().len()
        {
            return;
        }
        self.veri_komutları = sahne
            .komutlar()
            .iter()
            .map(|komut| komut_çizim_alanında_mı(komut, çizim_alanı))
            .collect();
        self.veri_komutu_çizim_alanı = Some(anahtar);
    }

    fn sahneyi_değiştir(&mut self, eski: &Sahne, yeni: &Sahne) -> usize {
        self.veri_komutları.clear();
        self.veri_komutu_çizim_alanı = None;
        self.raster = None;
        if eski.boyut() != yeni.boyut() {
            self.sahne_boyutu = Some(yeni.boyut());
            self.sınırlar = None;
            self.yollar.clear();
            self.ikincil_yollar.clear();
            return 0;
        }

        let mut korunan = 0;
        for (indeks, (eski_kimlik, yeni_kimlik)) in eski
            .geometri_kimlikleri()
            .iter()
            .zip(yeni.geometri_kimlikleri())
            .enumerate()
        {
            if eski_kimlik != yeni_kimlik {
                if let Some(yol) = self.yollar.get_mut(indeks) {
                    *yol = None;
                }
                if let Some(yol) = self.ikincil_yollar.get_mut(indeks) {
                    *yol = None;
                }
                continue;
            }
            if self.yollar.get(indeks).is_some_and(Option::is_some) {
                korunan += 1;
            }
        }
        self.yollar.truncate(yeni.komutlar().len());
        self.ikincil_yollar.truncate(yeni.komutlar().len());
        self.yollar.resize_with(yeni.komutlar().len(), || None);
        self.ikincil_yollar
            .resize_with(yeni.komutlar().len(), || None);
        korunan
    }

    fn yol(
        &mut self,
        indeks: usize,
        öteleme: ::gpui::Point<Pixels>,
        oluştur: impl FnOnce() -> Option<Path<Pixels>>,
    ) -> Option<BoyanabilirGpuiYol> {
        if let Some(yol) = self.yollar.get_mut(indeks).and_then(Option::as_mut) {
            return Some(yol.boyanabilir(öteleme));
        }
        let mut yol = ÖnbellekliGpuiYol::yeni(oluştur()?);
        let boyanabilir = yol.boyanabilir(öteleme);
        if let Some(hedef) = self.yollar.get_mut(indeks) {
            *hedef = Some(yol);
        }
        Some(boyanabilir)
    }

    fn ikincil_yol(
        &mut self,
        indeks: usize,
        öteleme: ::gpui::Point<Pixels>,
        oluştur: impl FnOnce() -> Option<Path<Pixels>>,
    ) -> Option<BoyanabilirGpuiYol> {
        if let Some(yol) = self.ikincil_yollar.get_mut(indeks).and_then(Option::as_mut) {
            return Some(yol.boyanabilir(öteleme));
        }
        let mut yol = ÖnbellekliGpuiYol::yeni(oluştur()?);
        let boyanabilir = yol.boyanabilir(öteleme);
        if let Some(hedef) = self.ikincil_yollar.get_mut(indeks) {
            *hedef = Some(yol);
        }
        Some(boyanabilir)
    }
}

impl Render for GpuiAnaYüzey {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let sahne = self.sahne.clone();
        let çizim_sınırları = self.çizim_sınırları.clone();
        let yol_önbelleği = self.yol_önbelleği.clone();
        let duyarlı_grafik = self.duyarlı_grafik.clone();
        let veri_görünümü = self.veri_görünümü.clone();
        canvas(
            move |sınırlar, _, uygulama| {
                let önceki_sınırlar = çizim_sınırları.replace(Some(sınırlar));
                let Some(grafik) = duyarlı_grafik else {
                    return;
                };
                // Kaydırma yüzeyin pencere içindeki kökenini değiştirir, grafik
                // boyutunu değiştirmez. Köken değişiminde `boyutu_ayarla` için
                // ertelenmiş bir GPUI görevi üretmek ana sahneyi değiştirmese bile
                // ana iş parçacığına gereksiz iş yükler.
                if !duyarlı_boyut_güncellenmeli(önceki_sınırlar, sınırlar) {
                    return;
                }
                let genişlik = f32::from(sınırlar.size.width).round().max(160.0) as u32;
                let yükseklik = f32::from(sınırlar.size.height).round().max(120.0) as u32;
                uygulama.defer(move |uygulama| {
                    if let Some(grafik) = grafik.upgrade() {
                        grafik.update(uygulama, |grafik, cx| {
                            let _ = grafik.boyutu_ayarla(genişlik, yükseklik, cx);
                        });
                    }
                });
            },
            move |sınırlar, _, pencere, uygulama| {
                sahneyi_önbellekli_boya(
                    &sahne,
                    sınırlar,
                    &mut yol_önbelleği.borrow_mut(),
                    Some(veri_görünümü.get()),
                    None,
                    pencere,
                    uygulama,
                );
            },
        )
        .size_full()
    }
}

impl GpuiGrafik {
    fn ölçüm_datumunu_imleçte_ayarla(&mut self, datum: usize, cx: &mut Context<Self>) {
        if !self.grafik.ölçüm_datumları_etkin() {
            return;
        }
        let Some(imleç) = self.imleç.as_ref() else {
            return;
        };
        let (sol, sağ, üst, alt) = self.çizim_alanı();
        let yatay = f64::from((imleç.fare.x - sol) / (sağ - sol));
        let dikey = f64::from((imleç.fare.y - üst) / (alt - üst));
        self.grafik.ölçüm_datumunu_ayarla(datum, yatay, dikey);
        cx.stop_propagation();
        self.grafik_bildir(cx);
    }

    pub fn yeni(grafik: Grafik) -> Self {
        let boyut_senkron_katmanı = grafik.boyut_senkron_düzeni();
        let arka_plan_sahnesi = Rc::new(grafik.gpui_arka_plan_sahnesini_çiz());
        let ana_sahne = Rc::new(grafik.gpui_görünür_veri_sahnesini_çiz());
        let eksen_sahnesi = Rc::new(grafik.gpui_eksen_sahnesini_çiz());
        let veri_görünümü = Rc::new(Cell::new(GpuiVeriGörünümü {
            pencere: OransalGörünüm::default(),
            çizim_alanı: grafik.çizim_alanı_boyutta(grafik.boyut().0, grafik.boyut().1),
        }));
        Self {
            grafik,
            arka_plan_sahnesi,
            arka_plan_yüzeyi: None,
            ana_sahne,
            ana_yüzey: None,
            eksen_sahnesi,
            eksen_yüzeyi: None,
            etkileşim_sahnesi: Rc::new(Sahne::yeni(1, 1)),
            etkileşim_sahne_tamponu: Some(Sahne::yeni(1, 1)),
            etkileşim_yüzeyi: None,
            ana_sahne_revizyonu: 1,
            görünüm_revizyonu: 1,
            etkileşim_sahne_revizyonu: 0,
            imleç: None,
            seçim: None,
            açıklama_seçimi: false,
            taşıma_başlangıcı: None,
            dokunma_kaydırma: None,
            boşluk_basılı: false,
            hata: None,
            çizim_sınırları: Rc::new(Cell::new(None)),
            veri_görünümü,
            odak: None,
            imleç_kilitli: boyut_senkron_katmanı.is_some(),
            imleç_değere_yapışsın: false,
            boyut_senkron_katmanı,
            eksen_üzerinde: false,
            açıklama_vuruşu: None,
            tooltip_tıklama_başlangıcı: None,
            tooltip_tıklaması_sürüklendi: false,
            bilgi_balonu_hazır: false,
            bilgi_balonu_beklemesi: None,
            bilgi_balonu_son_hareket: None,
            #[cfg(test)]
            etkileşim_sahne_hazırlama_sayısı: 0,
        }
    }

    pub fn grafik(&self) -> &Grafik {
        &self.grafik
    }

    /// İmlecin çizim alanı içindeki güncel konumu, kaynak boyutunda.
    ///
    /// Yapışma açıkken en yakın örneğin üstündedir, kapalıyken fareyi izler.
    /// İmleç katmanını kendi çizen tüketiciler ve yapışmayı doğrulayan
    /// testler buradan okur.
    pub fn imleç_konumu(&self) -> Option<Nokta> {
        self.imleç.as_ref().map(|imleç| imleç.fare)
    }

    /// İmleç yüzeyin üzerinde mi.
    ///
    /// Fare yüzeyi terk ettiğinde temizlenir; uPlot'un `mouseleave` ile
    /// cursor'ı gizlemesinin karşılığıdır. Testler ve tüketiciler imleç
    /// katmanının gerçekten söndüğünü buradan doğrular.
    pub const fn imleç_etkin_mi(&self) -> bool {
        self.imleç.is_some()
    }

    /// Yüzeyin son yerleşimde ölçülen çizim alanı, pencere koordinatında.
    ///
    /// Ana yüzeyin `canvas` prepaint aşamasında yazılır; ilk yerleşim
    /// tamamlanana kadar ve sanallaştırılmış listelerde görünür alana
    /// girmemiş yüzeylerde `None`'dır. Yerleşimi doğrulayan testler
    /// buradan okur: sahne komutları doğruyken yüzeyin yanlış boyutta
    /// yerleşmesi yalnız bu ölçümde görünür.
    pub fn ölçülen_alan(&self) -> Option<Bounds<Pixels>> {
        self.çizim_sınırları.get()
    }

    pub fn grafik_kimliği(&self) -> u64 {
        self.grafik.kimlik()
    }

    /// Retained ana ve etkileşim katmanlarının güncel revizyonlarını döndürür.
    ///
    /// Pointer hareketinde ana revizyon sabit kalır; yalnız hafif etkileşim
    /// katmanı değişir. `setData`, `setSeries` ve resize ana revizyonu artırır.
    /// Zoom/pan ana geometriyi değiştirmez; yalnız GPU görünüm matrisi yenilenir.
    pub const fn sahne_revizyonları(&self) -> (u64, u64) {
        (self.ana_sahne_revizyonu, self.etkileşim_sahne_revizyonu)
    }

    pub fn seri_yaşam_döngüsü_olaylarını_al(
        &mut self,
    ) -> Vec<crate::SeriYaşamDöngüsüOlayı> {
        self.grafik.seri_yaşam_döngüsü_olaylarını_al()
    }

    pub fn hata(&self) -> Option<&str> {
        self.hata.as_deref()
    }

    pub fn lejant(&self) -> Option<(f64, f64)> {
        self.imleç.as_ref().and_then(|imleç| {
            imleç
                .seri_değerleri
                .first()
                .copied()
                .flatten()
                .map(|y| (imleç.veri_x, y))
        })
    }

    pub fn lejant_değerleri(&self) -> Option<(Option<f64>, Vec<Option<f64>>)> {
        if !self.grafik.lejant_canlı() {
            return None;
        }
        self.imleç
            .as_ref()
            .map(|imleç| {
                let gösterim_değerleri = self
                    .grafik
                    .x_konum_oranı(imleç.veri_x)
                    .and_then(|oran| self.grafik.en_yakın_noktalar(oran))
                    .map(|(_, değerler)| değerler)
                    .unwrap_or_else(|| imleç.seri_değerleri.clone());
                (Some(imleç.veri_x), gösterim_değerleri)
            })
            .or_else(|| {
                self.grafik
                    .boşta_lejant_değerleri()
                    .map(|değerler| (None, değerler))
            })
    }

    pub fn senkron_yayını(&self) -> Option<(f64, f64, Option<usize>)> {
        let imleç = self.imleç.as_ref()?;
        let (sol, sağ, üst, alt) = self.çizim_alanı();
        let genişlik = sağ - sol;
        let yükseklik = alt - üst;
        if genişlik <= 0.0 || yükseklik <= 0.0 {
            return None;
        }
        Some((
            f64::from((imleç.fare.x - sol) / genişlik),
            f64::from((imleç.fare.y - üst) / yükseklik),
            self.grafik.odak_serisi(),
        ))
    }

    /// Grup üyeleri arasında piksel boyutundan bağımsız imleç yayını.
    pub fn oransal_imleç_yayını(&self) -> Option<(f64, f64, Option<usize>)> {
        self.senkron_yayını()
    }

    /// Grafiğin tam ölçekleri içindeki normalize görünüm penceresini yayınlar.
    pub fn oransal_görünüm_yayını(&self) -> OransalGörünüm {
        self.grafik.oransal_görünüm()
    }

    fn etkin_en_yakın_tooltip(&self) -> Option<EnYakınTooltipBilgisi> {
        let imleç = self.imleç.as_ref()?;
        let seri = self.grafik.odak_serisi()?;
        let (sol, sağ, _, _) = self.çizim_alanı();
        let yatay_oran = f64::from(((imleç.fare.x - sol) / (sağ - sol)).clamp(0.0, 1.0));
        self.grafik.en_yakın_tooltip(yatay_oran, seri)
    }

    pub fn senkron_veri_yayını(&self) -> Option<(f64, f64, Option<usize>)> {
        let imleç = self.imleç.as_ref()?;
        let (sol, sağ, üst, alt) = self.çizim_alanı();
        let yatay = f64::from((imleç.fare.x - sol) / (sağ - sol));
        let dikey = f64::from((imleç.fare.y - üst) / (alt - üst));
        let (_, y_oranı) = self.grafik.fiziksel_oranları_mantıksala(yatay, dikey);
        let y_aralığı = self.grafik.görünür_y_aralığı();
        let y = y_aralığı.en_çok - y_oranı * (y_aralığı.en_çok - y_aralığı.en_az);
        Some((imleç.veri_x, y, self.grafik.odak_serisi()))
    }

    pub fn senkron_veri_imleci_ayarla(
        &mut self,
        x: f64,
        y: f64,
        odak_serisi: Option<usize>,
        cx: &mut Context<Self>,
    ) -> bool {
        if self.imleç_kilitli || !x.is_finite() || !y.is_finite() {
            return false;
        }
        let (sol, sağ, üst, alt) = self.çizim_alanı();
        let (Some(x_oranı), Some(y_oranı)) = (
            self.grafik.x_konum_oranı(x),
            self.grafik.seri_y_konum_oranı(0, y),
        ) else {
            return false;
        };
        let x_dikey = self.grafik.x_dikey_mi();
        let fare = if x_dikey {
            Nokta::yeni(
                sol + y_oranı as f32 * (sağ - sol),
                alt - x_oranı as f32 * (alt - üst),
            )
        } else {
            Nokta::yeni(
                sol + x_oranı as f32 * (sağ - sol),
                alt - y_oranı as f32 * (alt - üst),
            )
        };
        let eksen_uzunluğu = if x_dikey { alt - üst } else { sağ - sol };
        let Some(çözüm) = self.grafik.imleç_çözümü(x_oranı, f64::from(eksen_uzunluğu))
        else {
            return false;
        };
        self.imleç = Some(İmleçDurumu {
            fare,
            veri_x: çözüm.ortak_x,
            seri_x_değerleri: çözüm
                .seriler
                .iter()
                .map(|örnek| örnek.map(|örnek| örnek.x))
                .collect(),
            seri_değerleri: çözüm
                .seriler
                .iter()
                .map(|örnek| örnek.map(|örnek| örnek.değer))
                .collect(),
            dağılım: None,
        });
        if self.grafik.imleç_odağını_seriye_ayarla(odak_serisi) {
            self.veri_sahnesini_yenile(cx);
        }
        self.etkileşim_yüzeyini_yenile(cx);
        true
    }

    pub fn senkron_veri_x_imleci_ayarla(
        &mut self,
        x: f64,
        odak_serisi: Option<usize>,
        cx: &mut Context<Self>,
    ) -> bool {
        let Some(x_oranı) = self.grafik.x_konum_oranı(x) else {
            return false;
        };
        self.senkron_imleci_ayarla(x_oranı, None, odak_serisi, cx)
    }

    pub fn senkron_imleci_ayarla(
        &mut self,
        yatay_oran: f64,
        dikey_oran: Option<f64>,
        odak_serisi: Option<usize>,
        cx: &mut Context<Self>,
    ) -> bool {
        if self.imleç_kilitli
            || !yatay_oran.is_finite()
            || dikey_oran.is_some_and(|oran| !oran.is_finite())
        {
            return false;
        }
        let (sol, sağ, üst, alt) = self.çizim_alanı();
        let yatay_oran = yatay_oran.clamp(0.0, 1.0);
        let dikey_oran = dikey_oran.map(|oran| oran.clamp(0.0, 1.0));
        let fiziksel_dikey = dikey_oran.unwrap_or(0.5);
        let (x_oranı, _) = self
            .grafik
            .fiziksel_oranları_mantıksala(yatay_oran, fiziksel_dikey);
        let x_uzunluğu = if self.grafik.x_dikey_mi() {
            alt - üst
        } else {
            sağ - sol
        };
        let Some(çözüm) = self.grafik.imleç_çözümü(x_oranı, f64::from(x_uzunluğu)) else {
            return false;
        };
        let seri_x_değerleri = çözüm
            .seriler
            .iter()
            .map(|örnek| örnek.map(|örnek| örnek.x))
            .collect();
        let seri_değerleri = çözüm
            .seriler
            .iter()
            .map(|örnek| örnek.map(|örnek| örnek.değer))
            .collect();
        let fare = Nokta::yeni(
            sol + yatay_oran as f32 * (sağ - sol),
            dikey_oran.map_or(-10.0, |oran| üst + oran as f32 * (alt - üst)),
        );
        self.açıklama_vuruşu = None;
        self.imleç = Some(İmleçDurumu {
            fare,
            veri_x: çözüm.ortak_x,
            seri_x_değerleri,
            seri_değerleri,
            dağılım: None,
        });
        if self.grafik.imleç_odağını_seriye_ayarla(odak_serisi) {
            self.veri_sahnesini_yenile(cx);
        }
        self.etkileşim_yüzeyini_yenile(cx);
        true
    }

    /// Normalize grup görünümünü bu grafiğin kendi tam ölçeklerine uygular.
    pub fn oransal_görünümü_ayarla(
        &mut self,
        görünüm: OransalGörünüm,
        geçmişe_ekle: bool,
        cx: &mut Context<Self>,
    ) -> Result<bool, UplotHatası> {
        let değişti = self.grafik.oransal_görünümü_ayarla(görünüm, geçmişe_ekle)?;
        if değişti {
            self.görünüm_bildir(false, cx);
        }
        Ok(değişti)
    }

    fn oransal_görünümü_sessiz_ayarla(
        &mut self,
        görünüm: OransalGörünüm,
        cx: &mut Context<Self>,
    ) -> Result<bool, UplotHatası> {
        let değişti = self.grafik.oransal_görünümü_ayarla(görünüm, false)?;
        if değişti {
            self.görünümü_sessiz_bildir(cx);
        }
        Ok(değişti)
    }

    /// Görünümü senkron grubu hedefine yayar ve olay yankısı üretmez.
    ///
    /// GPUI `emit` çağrıları `pending_effects` kuyruğuna yazılır; kaynak
    /// tarafın `senkronlanıyor` bayrağı hedefler olaylarını yaydığında çoktan
    /// sıfırlanmış olur. Hedefin `GörünümDeğişti` yayması bu yüzden her üyeyi
    /// bir tur daha gereksiz çalıştırır ve üyelerde ayrı geçmiş girdisi
    /// bırakır. Grup hedefleri zaten aynı turda doğrudan güncellendiğinden
    /// yeniden yayın taşımaz.
    pub fn görünür_aralıkları_sessiz_ayarla(
        &mut self,
        x: Aralık,
        y: Aralık,
        geçmişe_ekle: bool,
        cx: &mut Context<Self>,
    ) -> bool {
        let değişti = self.grafik.görünür_aralıkları_ayarla(x, y, geçmişe_ekle);
        if değişti {
            self.görünümü_sessiz_bildir(cx);
        }
        değişti
    }

    /// [`Self::görünür_aralıkları_sessiz_ayarla`] karşılığı; yalnız X taşır.
    pub fn görünür_x_aralığını_sessiz_ayarla(
        &mut self,
        x: Aralık,
        geçmişe_ekle: bool,
        cx: &mut Context<Self>,
    ) -> bool {
        let değişti = self.grafik.görünür_x_aralığını_ayarla(x, geçmişe_ekle);
        if değişti {
            self.görünümü_sessiz_bildir(cx);
        }
        değişti
    }

    pub fn senkron_imleci_temizle(&mut self, cx: &mut Context<Self>) -> bool {
        if self.imleç_kilitli || self.imleç.is_none() {
            return false;
        }
        self.imleç = None;
        if self.grafik.imleç_odağını_temizle() {
            self.veri_sahnesini_yenile(cx);
        }
        self.etkileşim_yüzeyini_yenile(cx);
        true
    }

    pub fn senkron_kilidi_ayarla(&mut self, kilitli: bool, cx: &mut Context<Self>) -> bool {
        let değişti = self.imleç_kilitli != kilitli;
        self.imleç_kilitli = kilitli;
        if değişti {
            cx.notify();
        }
        değişti
    }

    pub fn grafiği_ayarla(&mut self, grafik: Grafik, cx: &mut Context<Self>) {
        let imleci_koru = self
            .grafik
            .tooltip_düzeni()
            .is_some_and(|düzen| düzen.imleç_durumunu_koru)
            && grafik
                .tooltip_düzeni()
                .is_some_and(|düzen| düzen.imleç_durumunu_koru);
        let korunmuş_imleç = imleci_koru.then(|| self.imleç.clone()).flatten();
        let korunmuş_kilit = imleci_koru && self.imleç_kilitli;
        self.grafik = grafik;
        self.imleç = korunmuş_imleç;
        self.seçim = None;
        self.açıklama_seçimi = false;
        self.taşıma_başlangıcı = None;
        self.dokunma_kaydırma = None;
        self.boşluk_basılı = false;
        self.hata = None;
        self.boyut_senkron_katmanı = self.grafik.boyut_senkron_düzeni();
        self.imleç_kilitli = korunmuş_kilit || self.boyut_senkron_katmanı.is_some();
        self.açıklama_vuruşu = None;
        self.grafik_bildir(cx);
    }

    pub fn veriyi_ayarla(
        &mut self,
        veri: HizalıVeri,
        cx: &mut Context<Self>,
    ) -> Result<(), UplotHatası> {
        let korunacak_imleç = self.imleç.as_ref().map(|imleç| imleç.fare);
        self.grafik.veriyi_ayarla(veri)?;
        self.açıklama_vuruşu = None;
        if let Some(fare) = korunacak_imleç {
            self.canlı_imleci_yenile(fare);
        }
        self.grafik_bildir(cx);
        Ok(())
    }

    pub fn veriyi_y_aralığında_ayarla(
        &mut self,
        veri: HizalıVeri,
        aralık: Aralık,
        cx: &mut Context<Self>,
    ) -> Result<(), UplotHatası> {
        let korunacak_imleç = self.imleç.as_ref().map(|imleç| imleç.fare);
        self.grafik.veriyi_y_aralığında_ayarla(veri, aralık)?;
        self.açıklama_vuruşu = None;
        if let Some(fare) = korunacak_imleç {
            self.canlı_imleci_yenile(fare);
        }
        self.grafik_bildir(cx);
        Ok(())
    }

    pub fn veriyi_y_sunumunda_ayarla(
        &mut self,
        veri: HizalıVeri,
        aralık: crate::Aralık,
        özel_etiketler: Vec<(f64, String)>,
        dolgu_tabanları: Vec<f64>,
        cx: &mut Context<Self>,
    ) -> Result<(), UplotHatası> {
        let korunacak_imleç = self.imleç.as_ref().map(|imleç| imleç.fare);
        self.grafik
            .veriyi_y_sunumunda_ayarla(veri, aralık, özel_etiketler, dolgu_tabanları)?;
        self.açıklama_vuruşu = None;
        if let Some(fare) = korunacak_imleç {
            self.canlı_imleci_yenile(fare);
        }
        self.grafik_bildir(cx);
        Ok(())
    }

    pub fn canlı_veriyi_ayarla(
        &mut self,
        veri: HizalıVeri,
        cx: &mut Context<Self>,
    ) -> Result<(), UplotHatası> {
        let korunacak_imleç = self.imleç.as_ref().map(|imleç| imleç.fare);
        self.grafik.canlı_veriyi_ayarla(veri)?;
        self.açıklama_vuruşu = None;
        if let Some(fare) = korunacak_imleç {
            self.canlı_imleci_yenile(fare);
        }
        self.grafik_bildir(cx);
        Ok(())
    }

    pub fn canlı_veriyi_x_etiket_çarpanında_ayarla(
        &mut self,
        veri: HizalıVeri,
        çarpan: f64,
        cx: &mut Context<Self>,
    ) -> Result<(), UplotHatası> {
        let korunacak_imleç = self.imleç.as_ref().map(|imleç| imleç.fare);
        self.grafik
            .canlı_veriyi_x_etiket_çarpanında_ayarla(veri, çarpan)?;
        self.açıklama_vuruşu = None;
        if let Some(fare) = korunacak_imleç {
            self.canlı_imleci_yenile(fare);
        }
        self.grafik_bildir(cx);
        Ok(())
    }

    /// uPlot `setData()` sırasında yaptığı gibi aynı hafif cursor katmanını
    /// korur ve sabit fare konumundaki canlı değerleri yeni veriden çözer.
    fn canlı_imleci_yenile(&mut self, fare: Nokta) {
        if self
            .imleç
            .as_ref()
            .is_some_and(|imleç| imleç.dağılım.is_some())
            || !self.grafik_alanında(fare)
        {
            self.imleç = None;
            return;
        }
        let (sol, sağ, üst, alt) = self.çizim_alanı();
        let yatay = f64::from((fare.x - sol) / (sağ - sol));
        let dikey = f64::from((fare.y - üst) / (alt - üst));
        let x_dikey = self.grafik.x_dikey_mi();
        let x_oranı = if x_dikey { 1.0 - dikey } else { yatay };
        let x_uzunluğu = if x_dikey { alt - üst } else { sağ - sol };
        let Some(çözüm) = self.grafik.imleç_çözümü(x_oranı, f64::from(x_uzunluğu)) else {
            self.imleç = None;
            return;
        };
        let seri_x_değerleri = çözüm
            .seriler
            .iter()
            .map(|örnek| örnek.map(|örnek| örnek.x))
            .collect();
        let seri_değerleri = çözüm
            .seriler
            .iter()
            .map(|örnek| örnek.map(|örnek| örnek.değer))
            .collect();
        // Veri yenilenirken çizgi, kullanıcının bıraktığı fare konumunda
        // kalır; değerler aşağıda en yakın örnekten yeniden çözülür.
        let x_konumu = x_oranı as f32;
        self.imleç = Some(İmleçDurumu {
            fare: if x_dikey {
                Nokta::yeni(fare.x, alt - x_konumu * (alt - üst))
            } else {
                Nokta::yeni(sol + x_konumu * (sağ - sol), fare.y)
            },
            veri_x: çözüm.ortak_x,
            seri_x_değerleri,
            seri_değerleri,
            dağılım: None,
        });
    }

    pub fn canlı_veriyi_x_aralığında_ayarla(
        &mut self,
        veri: HizalıVeri,
        aralık: Aralık,
        cx: &mut Context<Self>,
    ) -> Result<bool, UplotHatası> {
        let görünür_değişti = self.grafik.canlı_veriyi_x_aralığında_ayarla(veri, aralık)?;
        self.açıklama_vuruşu = None;
        if görünür_değişti {
            self.grafik_bildir(cx);
        }
        Ok(görünür_değişti)
    }

    pub fn canlı_x_aralığını_ayarla(
        &mut self,
        aralık: Aralık,
        cx: &mut Context<Self>,
    ) -> bool {
        let değişti = self.grafik.canlı_x_aralığını_ayarla(aralık);
        if değişti {
            self.grafik_bildir(cx);
        }
        değişti
    }

    pub fn canlı_y_aralığını_ayarla(
        &mut self,
        aralık: Aralık,
        cx: &mut Context<Self>,
    ) -> bool {
        let değişti = self.grafik.canlı_y_aralığını_ayarla(aralık);
        if değişti {
            self.grafik_bildir(cx);
        }
        değişti
    }

    pub fn y_ölçek_aralıklarını_ayarla(
        &mut self,
        aralıklar: &[(&str, Aralık)],
        cx: &mut Context<Self>,
    ) -> Result<bool, UplotHatası> {
        let değişti = self.grafik.y_ölçek_aralıklarını_ayarla(aralıklar)?;
        if değişti {
            self.grafik_bildir(cx);
        }
        Ok(değişti)
    }

    pub fn boyutu_ayarla(
        &mut self,
        genişlik: u32,
        yükseklik: u32,
        cx: &mut Context<Self>,
    ) -> Result<bool, UplotHatası> {
        let değişti = self.grafik.boyutu_ayarla(genişlik, yükseklik)?;
        if değişti {
            self.grafik_bildir(cx);
        }
        Ok(değişti)
    }

    pub fn seri_ekle(
        &mut self,
        indeks: usize,
        seçenek: SeriSeçenekleri,
        değerler: Vec<Option<f64>>,
        cx: &mut Context<Self>,
    ) -> Result<(), UplotHatası> {
        self.grafik.seri_ekle(indeks, seçenek, değerler)?;
        self.imleç = None;
        self.açıklama_vuruşu = None;
        self.seçim = None;
        self.açıklama_seçimi = false;
        self.grafik_bildir(cx);
        Ok(())
    }

    pub fn seri_sil(&mut self, indeks: usize, cx: &mut Context<Self>) -> Result<(), UplotHatası> {
        self.grafik.seri_sil(indeks)?;
        self.imleç = None;
        self.açıklama_vuruşu = None;
        self.seçim = None;
        self.açıklama_seçimi = false;
        self.grafik_bildir(cx);
        Ok(())
    }

    /// Lejant satırındaki `setSeries(i, {focus: true})` karşılığını uygular;
    /// `None` odağı bırakır.
    ///
    /// uPlot odak alfasını yalnız `cursor.focus` kurulmuş grafiklerde boyar,
    /// bu yüzden sunumu olmayan grafiklerde sahne yenilenmez. Odak yalnız
    /// seri stilini değiştirdiğinden veri katmanı yeniden çözülür; imleç ve
    /// seçim katmanlarına dokunulmaz.
    pub fn odak_serisini_ayarla(&mut self, seri: Option<usize>, cx: &mut Context<Self>) -> bool {
        if !self.grafik.odak_sunumu_var_mı() {
            return false;
        }
        let değişti = self.grafik.imleç_odağını_seriye_ayarla(seri);
        if değişti {
            self.veri_sahnesini_yenile(cx);
        }
        değişti
    }

    /// Web tarafındaki lejant düğmeleriyle aynı görünürlük değişimini GPUI
    /// uygulamalarına sunar ve yalnız gerekli sahne katmanlarını yeniler.
    pub fn seri_görünürlüğünü_ayarla(
        &mut self,
        indeks: usize,
        görünür: bool,
        cx: &mut Context<Self>,
    ) -> Result<bool, UplotHatası> {
        let değişti = self.grafik.seri_görünürlüğünü_ayarla(indeks, görünür)?;
        if değişti {
            self.grafik_bildir(cx);
        }
        Ok(değişti)
    }

    /// uPlot `delBand()` / `addBand()` eşdeğerini aynı GPUI yüzeyinde uygular.
    pub fn bantları_ayarla(&mut self, bantlar: Vec<SeriBandı>, cx: &mut Context<Self>) -> bool {
        let değişti = self.grafik.bantları_ayarla(bantlar);
        if değişti {
            self.grafik_bildir(cx);
        }
        değişti
    }

    /// CSS bulunmayan GPUI yüzeylerinde seri çizgi/dolgu rengini çalışma
    /// anında değiştirir.
    pub fn seri_renklerini_ayarla(
        &mut self,
        indeks: usize,
        çizgi: impl Into<String>,
        dolgu: Option<String>,
        cx: &mut Context<Self>,
    ) -> Result<bool, UplotHatası> {
        let değişti = self.grafik.seri_renklerini_ayarla(indeks, çizgi, dolgu)?;
        if değişti {
            self.grafik_bildir(cx);
        }
        Ok(değişti)
    }

    /// GPUI çubuk serilerinin nokta başına dinamik dolgu/vuruş paletini
    /// değiştirir.
    pub fn seri_çubuk_renklerini_ayarla(
        &mut self,
        indeks: usize,
        dolgular: Vec<String>,
        çizgiler: Vec<String>,
        cx: &mut Context<Self>,
    ) -> Result<bool, UplotHatası> {
        let değişti = self
            .grafik
            .seri_çubuk_renklerini_ayarla(indeks, dolgular, çizgiler)?;
        if değişti {
            self.grafik_bildir(cx);
        }
        Ok(değişti)
    }

    pub fn boşlukları_birleştir_ayarla(
        &mut self,
        birleştir: bool,
        cx: &mut Context<Self>,
    ) -> bool {
        let değişti = self.grafik.boşlukları_birleştir_ayarla(birleştir);
        if değişti {
            self.imleç = None;
            self.grafik_bildir(cx);
        }
        değişti
    }

    pub fn tekerlek_etkileşimi_ayarla(&mut self, etkin: bool, cx: &mut Context<Self>) -> bool {
        let değişti = self.grafik.tekerlek_etkileşimi_ayarla(etkin);
        if değişti {
            cx.notify();
        }
        değişti
    }

    pub fn tekerlek_odaksız_etkileşimi_ayarla(
        &mut self,
        etkin: bool,
        cx: &mut Context<Self>,
    ) -> bool {
        let değişti = self.grafik.tekerlek_odaksız_etkileşimi_ayarla(etkin);
        if değişti {
            cx.notify();
        }
        değişti
    }

    pub fn kırılım_noktalarını_göster(
        &mut self,
        görünür: bool,
        cx: &mut Context<Self>,
    ) -> bool {
        let değişti = self.grafik.kırılım_noktalarını_göster(görünür);
        if değişti {
            self.grafik_bildir(cx);
        }
        değişti
    }

    pub fn imleç_noktalarını_göster(
        &mut self, görünür: bool, cx: &mut Context<Self>
    ) -> bool {
        let değişti = self.grafik.imleç_noktalarını_göster(görünür);
        if değişti {
            self.etkileşim_yüzeyini_yenile(cx);
            cx.notify();
        }
        değişti
    }

    pub fn y_arcsinh_eşiği_ayarla(
        &mut self,
        anahtar: &str,
        eşik: f64,
        cx: &mut Context<Self>,
    ) -> bool {
        let değişti = self.grafik.y_arcsinh_eşiği_ayarla(anahtar, eşik);
        if değişti {
            self.grafik_bildir(cx);
        }
        değişti
    }

    pub fn önceki_görünüm(&mut self, cx: &mut Context<Self>) -> bool {
        let değişti = self.grafik.önceki_görünüm();
        if değişti {
            self.görünüm_bildir(false, cx);
        }
        değişti
    }

    pub fn tam_görünüm(&mut self, cx: &mut Context<Self>) -> bool {
        let değişti = self.grafik.tam_görünüm();
        if değişti {
            self.görünüm_bildir(false, cx);
        }
        değişti
    }

    pub fn görünür_aralıkları_ayarla(
        &mut self,
        x: Aralık,
        y: Aralık,
        geçmişe_ekle: bool,
        cx: &mut Context<Self>,
    ) -> bool {
        let değişti = self.grafik.görünür_aralıkları_ayarla(x, y, geçmişe_ekle);
        if değişti {
            self.görünüm_bildir(false, cx);
        }
        değişti
    }

    pub fn görünür_x_aralığını_ayarla(
        &mut self,
        x: Aralık,
        geçmişe_ekle: bool,
        cx: &mut Context<Self>,
    ) -> bool {
        let değişti = self.grafik.görünür_x_aralığını_ayarla(x, geçmişe_ekle);
        if değişti {
            self.görünüm_bildir(false, cx);
        }
        değişti
    }

    fn çizim_alanı(&self) -> (f32, f32, f32, f32) {
        let (genişlik, yükseklik) = self.grafik.boyut();
        self.grafik.çizim_alanı_boyutta(genişlik, yükseklik)
    }

    fn etkileşim_sahnesini_doldur(&self, sahne: &mut Sahne) {
        let (genişlik, yükseklik) = self.grafik.boyut();
        sahne.yeniden_kullan(genişlik, yükseklik);
        let (sol, sağ, üst, alt) = self.çizim_alanı();
        if let Some(düzen) = self.boyut_senkron_katmanı {
            let çizim_genişliği = sağ - sol;
            let çizim_yüksekliği = alt - üst;
            let imleç = Nokta::yeni(
                sol + çizim_genişliği * düzen.imleç_x_oranı,
                üst + çizim_yüksekliği * düzen.imleç_y_oranı,
            );
            sahne.ekle(Komut::Dikdörtgen {
                konum: Nokta::yeni(
                    sol + çizim_genişliği * düzen.seçim_x_oranı,
                    üst + çizim_yüksekliği * düzen.seçim_y_oranı,
                ),
                genişlik: çizim_genişliği * düzen.seçim_genişlik_oranı,
                yükseklik: çizim_yüksekliği * düzen.seçim_yükseklik_oranı,
                dolgu: "#00000012".into(),
                çizgi: "#00000000".into(),
                kalınlık: 0.0,
            });
            sahne.ekle(Komut::KesikliÇizgi {
                başlangıç: Nokta::yeni(imleç.x, üst),
                bitiş: Nokta::yeni(imleç.x, alt),
                renk: "#607d8b".into(),
                kalınlık: 1.0,
                kesik: 4.0,
            });
            sahne.ekle(Komut::KesikliÇizgi {
                başlangıç: Nokta::yeni(sol, imleç.y),
                bitiş: Nokta::yeni(sağ, imleç.y),
                renk: "#607d8b".into(),
                kalınlık: 1.0,
                kesik: 4.0,
            });
            if self
                .grafik
                .seri_seçenekleri()
                .first()
                .is_some_and(|seri| seri.göster)
            {
                sahne.ekle(Komut::Daire {
                    merkez: Nokta::yeni(
                        sol + çizim_genişliği * düzen.hover_x_oranı,
                        üst + çizim_yüksekliği * düzen.hover_y_oranı,
                    ),
                    yarıçap: 2.5,
                    dolgu: "red".into(),
                    çizgi: "red".into(),
                    kalınlık: 0.0,
                });
            }
        }
        if let Some(imleç) = self.imleç.as_ref() {
            let timeline_sayısı = self.grafik.timeline_seri_sayısı();
            if timeline_sayısı > 0 {
                let yatay_oran = f64::from(
                    ((imleç.fare.x - sol) / (sağ - sol).max(f32::EPSILON)).clamp(0.0, 1.0),
                );
                let şerit_yüksekliği = (alt - üst) * 0.9 / timeline_sayısı as f32;
                let şerit_boşluğu = if timeline_sayısı > 1 {
                    (alt - üst) * 0.1 / timeline_sayısı.saturating_sub(1) as f32
                } else {
                    0.0
                };
                for vuruş in self
                    .grafik
                    .timeline_vuruşları_pikselde(yatay_oran, f64::from(sağ - sol))
                {
                    let x0 = self
                        .grafik
                        .x_konum_oranı(vuruş.başlangıç)
                        .map_or(sol, |oran| sol + oran as f32 * (sağ - sol))
                        .clamp(sol, sağ);
                    let x1 = self
                        .grafik
                        .x_konum_oranı(vuruş.bitiş)
                        .map_or(sağ, |oran| sol + oran as f32 * (sağ - sol))
                        .clamp(sol, sağ);
                    if x1 > x0 {
                        sahne.ekle(Komut::Dikdörtgen {
                            konum: Nokta::yeni(
                                x0,
                                üst + vuruş.seri as f32 * (şerit_yüksekliği + şerit_boşluğu),
                            ),
                            genişlik: x1 - x0,
                            yükseklik: şerit_yüksekliği,
                            dolgu: "#0000004d".into(),
                            çizgi: "#00000000".into(),
                            kalınlık: 0.0,
                        });
                    }
                }
                return;
            }
            if let Some(vuruş) = &imleç.dağılım {
                sahne.ekle(Komut::Daire {
                    merkez: vuruş.merkez,
                    yarıçap: vuruş.boyut / 2.0,
                    dolgu: "#ffffff66".into(),
                    çizgi: "#111111".into(),
                    kalınlık: 2.0,
                });
                return;
            }
            if let Some((_, _, konum, genişlik, yükseklik, _)) = self.grafik.çubuk_vuruşu(
                self.grafik.boyut().0,
                self.grafik.boyut().1,
                imleç.fare.x,
                imleç.fare.y,
            ) {
                sahne.ekle(Komut::Dikdörtgen {
                    konum,
                    genişlik,
                    yükseklik,
                    dolgu: "#ffffff4d".into(),
                    çizgi: "#ffffff00".into(),
                    kalınlık: 0.0,
                });
                return;
            }
            if self.grafik.çubuk_grafiği() {
                return;
            }
            if let Some((indeks, konum, genişlik, yükseklik, değerler)) =
                self.grafik.kutu_bıyık_vuruşu(
                    self.grafik.boyut().0,
                    self.grafik.boyut().1,
                    imleç.fare.x,
                    imleç.fare.y,
                )
            {
                sahne.ekle(Komut::Dikdörtgen {
                    konum,
                    genişlik,
                    yükseklik,
                    dolgu: "#33ccff4d".into(),
                    çizgi: "#33ccff00".into(),
                    kalınlık: 0.0,
                });
                let (satırlar, imleç_ofseti, kaynak_mum_konumu) =
                    if let Some(tarih) = self.grafik.mum_tarih_etiketi(indeks) {
                        let [açılış, yüksek, düşük, kapanış, hacim] = değerler;
                        (
                            Some([
                                format!("Date: {tarih}"),
                                format!("Open: {}", crate::grafik::usd_biçimle(açılış, 2)),
                                format!("High: {}", crate::grafik::usd_biçimle(yüksek, 2)),
                                format!("Low: {}", crate::grafik::usd_biçimle(düşük, 2)),
                                format!("Close: {}", crate::grafik::usd_biçimle(kapanış, 2)),
                                format!("Volume: {hacim:.0}"),
                            ]),
                            0.0,
                            true,
                        )
                    } else if let Some(framework) = self.grafik.kutu_bıyık_kategorisi(indeks) {
                        let değer_yaz = |değer: f64| {
                            if değer.is_finite() {
                                format!("{değer:.2}")
                            } else {
                                "—".to_string()
                            }
                        };
                        let [medyan, q1, q3, en_az, en_çok] = değerler;
                        (
                            Some([
                                format!("Lib: {framework}"),
                                format!("Median: {}", değer_yaz(medyan)),
                                format!("q1: {}", değer_yaz(q1)),
                                format!("q3: {}", değer_yaz(q3)),
                                format!("min: {}", değer_yaz(en_az)),
                                format!("max: {}", değer_yaz(en_çok)),
                            ]),
                            14.0,
                            false,
                        )
                    } else {
                        (None, 0.0, false)
                    };
                if let Some(satırlar) = satırlar {
                    let en_uzun = satırlar
                        .iter()
                        .map(|satır| satır.chars().count())
                        .max()
                        .unwrap_or(0);
                    let kutu_genişliği = (en_uzun as f32 * 6.5 + 16.0)
                        .max(220.0)
                        .min((sağ - sol).max(1.0));
                    let kutu_yüksekliği = 106.0;
                    let kutu_x = if kaynak_mum_konumu {
                        imleç.fare.x
                    } else {
                        (imleç.fare.x + imleç_ofseti)
                            .min(sağ - kutu_genişliği)
                            .max(sol)
                    };
                    let kutu_y = if kaynak_mum_konumu {
                        imleç.fare.y
                    } else {
                        (imleç.fare.y + imleç_ofseti)
                            .min(alt - kutu_yüksekliği)
                            .max(üst)
                    };
                    sahne.ekle(Komut::Dikdörtgen {
                        konum: Nokta::yeni(kutu_x, kutu_y),
                        genişlik: kutu_genişliği,
                        yükseklik: kutu_yüksekliği,
                        dolgu: "#fff9c4eb".into(),
                        çizgi: "#00000033".into(),
                        kalınlık: 1.0,
                    });
                    for (satır, içerik) in satırlar.into_iter().enumerate() {
                        sahne.ekle(Komut::Metin {
                            konum: Nokta::yeni(kutu_x + 8.0, kutu_y + 16.0 + satır as f32 * 16.0),
                            içerik,
                            renk: "#111111".into(),
                            boyut: 11.0,
                            hiza: MetinHizası::Başlangıç,
                        });
                    }
                }
                return;
            }
            if self.grafik.kutu_bıyık_grafiği() || self.grafik.mum_grafiği() {
                return;
            }
            let x_dikey = self.grafik.x_dikey_mi();
            // Yapışma kapalıyken çizgi, imleç durumunda tutulan serbest fare
            // konumunu kullanır; açıkken en yakın örneğin X konumuna oturur.
            let serbest_konum = if x_dikey {
                imleç.fare.y
            } else {
                imleç.fare.x
            };
            let x_konumu = if self.imleç_değere_yapışsın {
                self.grafik
                    .x_konum_oranı(imleç.veri_x)
                    .map_or(serbest_konum, |oran| {
                        if x_dikey {
                            alt - oran as f32 * (alt - üst)
                        } else {
                            sol + oran as f32 * (sağ - sol)
                        }
                    })
            } else {
                serbest_konum
            };
            sahne.ekle(if x_dikey {
                Komut::KesikliÇizgi {
                    başlangıç: Nokta::yeni(sol, x_konumu),
                    bitiş: Nokta::yeni(sağ, x_konumu),
                    renk: "#6b7280".into(),
                    kalınlık: 1.0,
                    kesik: 4.0,
                }
            } else {
                Komut::KesikliÇizgi {
                    başlangıç: Nokta::yeni(x_konumu, üst),
                    bitiş: Nokta::yeni(x_konumu, alt),
                    renk: "#6b7280".into(),
                    kalınlık: 1.0,
                    kesik: 4.0,
                }
            });
            if !self.grafik.eksen_göstergeleri_etkin() && self.grafik.imleç_y_görünür() {
                sahne.ekle(if x_dikey {
                    Komut::KesikliÇizgi {
                        başlangıç: Nokta::yeni(imleç.fare.x, üst),
                        bitiş: Nokta::yeni(imleç.fare.x, alt),
                        renk: "#6b7280".into(),
                        kalınlık: 1.0,
                        kesik: 4.0,
                    }
                } else {
                    Komut::KesikliÇizgi {
                        başlangıç: Nokta::yeni(sol, imleç.fare.y),
                        bitiş: Nokta::yeni(sağ, imleç.fare.y),
                        renk: "#6b7280".into(),
                        kalınlık: 1.0,
                        kesik: 4.0,
                    }
                });
            } else {
                let x_metni = format!("{}", imleç.veri_x);
                let x_rozet_genişliği = (x_metni.chars().count() as f32 * 7.0 + 16.0).max(24.0);
                sahne.ekle(Komut::Dikdörtgen {
                    konum: Nokta::yeni(x_konumu - x_rozet_genişliği / 2.0, alt + 6.0),
                    genişlik: x_rozet_genişliği,
                    yükseklik: 22.0,
                    dolgu: "#111111".into(),
                    çizgi: "#111111".into(),
                    kalınlık: 0.0,
                });
                sahne.ekle(Komut::Metin {
                    konum: Nokta::yeni(x_konumu, alt + 21.0),
                    içerik: x_metni,
                    renk: "#ffffff".into(),
                    boyut: 11.0,
                    hiza: MetinHizası::Orta,
                });
            }
            for (seri_indeksi, değer) in imleç.seri_değerleri.iter().enumerate() {
                let Some(değer) = değer else {
                    continue;
                };
                let Some(seri) = self.grafik.seri_seçenekleri().get(seri_indeksi) else {
                    continue;
                };
                let seri_x = imleç
                    .seri_x_değerleri
                    .get(seri_indeksi)
                    .copied()
                    .flatten()
                    .unwrap_or(imleç.veri_x);
                let seri_rengi = self
                    .grafik
                    .seri_imleç_rengi(seri_indeksi, seri_x, *değer)
                    .unwrap_or_else(|| seri.renk.clone());
                let Some(y_oranı) = self.grafik.seri_y_konum_oranı(seri_indeksi, *değer) else {
                    continue;
                };
                let y_konumu = if x_dikey {
                    sol + y_oranı as f32 * (sağ - sol)
                } else {
                    alt - y_oranı as f32 * (alt - üst)
                };
                let seri_x_konumu = self.grafik.x_konum_oranı(seri_x).map_or(x_konumu, |oran| {
                    if x_dikey {
                        alt - oran as f32 * (alt - üst)
                    } else {
                        sol + oran as f32 * (sağ - sol)
                    }
                });
                let seri_noktası = if x_dikey {
                    Nokta::yeni(y_konumu, seri_x_konumu)
                } else {
                    Nokta::yeni(seri_x_konumu, y_konumu)
                };
                if self.grafik.eksen_göstergeleri_etkin() {
                    sahne.ekle(Komut::KesikliÇizgi {
                        başlangıç: Nokta::yeni(sol, y_konumu),
                        bitiş: Nokta::yeni(sağ, y_konumu),
                        renk: seri_rengi.clone().into(),
                        kalınlık: 1.0,
                        kesik: 4.0,
                    });
                    let değer_metni = format!("{değer}");
                    let rozet_genişliği =
                        (değer_metni.chars().count() as f32 * 7.0 + 16.0).max(24.0);
                    let eksen_sağı = sol - seri_indeksi as f32 * 50.0;
                    let rozet_x = eksen_sağı - rozet_genişliği;
                    sahne.ekle(Komut::Dikdörtgen {
                        konum: Nokta::yeni(rozet_x, y_konumu - 11.0),
                        genişlik: rozet_genişliği,
                        yükseklik: 22.0,
                        dolgu: seri_rengi.clone().into(),
                        çizgi: seri_rengi.clone().into(),
                        kalınlık: 0.0,
                    });
                    sahne.ekle(Komut::Metin {
                        konum: Nokta::yeni(rozet_x + rozet_genişliği / 2.0, y_konumu + 4.0),
                        içerik: değer_metni,
                        renk: "#ffffff".into(),
                        boyut: 11.0,
                        hiza: MetinHizası::Orta,
                    });
                }
                if self.grafik.imleç_noktaları_görünür() {
                    let boyut = seri.imleç_nokta_boyutu.unwrap_or(5.0);
                    let dolgu = seri
                        .imleç_nokta_dolgusu
                        .clone()
                        .unwrap_or_else(|| seri_rengi.clone());
                    let çizgi = seri
                        .imleç_nokta_çizgisi
                        .clone()
                        .unwrap_or_else(|| seri_rengi.clone());
                    sahne.ekle(Komut::Daire {
                        merkez: seri_noktası,
                        yarıçap: boyut / 2.0,
                        dolgu: dolgu.into(),
                        çizgi: çizgi.into(),
                        kalınlık: seri.imleç_nokta_kalınlığı.unwrap_or(0.0),
                    });
                }
            }
        }
        if let Some((başlangıç, bitiş)) = self.seçim {
            let (dolgu, çizgi) = if self.açıklama_seçimi {
                ("#ffff004d", "#ffff0000")
            } else {
                ("#3b82f633", "#3b82f6")
            };
            let xy = self.grafik.etkileşim_seçenekleri().seçim_xy_yakınlaştır;
            let x_dikey = self.grafik.x_dikey_mi();
            sahne.ekle(Komut::Dikdörtgen {
                konum: if xy {
                    Nokta::yeni(başlangıç.x.min(bitiş.x), başlangıç.y.min(bitiş.y))
                } else if x_dikey {
                    Nokta::yeni(sol, başlangıç.y.min(bitiş.y))
                } else {
                    Nokta::yeni(başlangıç.x.min(bitiş.x), üst)
                },
                genişlik: if xy {
                    (bitiş.x - başlangıç.x).abs()
                } else if x_dikey {
                    sağ - sol
                } else {
                    (bitiş.x - başlangıç.x).abs()
                },
                yükseklik: if xy || x_dikey {
                    (bitiş.y - başlangıç.y).abs()
                } else {
                    alt - üst
                },
                dolgu: dolgu.into(),
                çizgi: çizgi.into(),
                kalınlık: 1.0,
            });
        }
        if let Some(vuruş) = self.açıklama_vuruşu.as_ref() {
            for komut in self
                .grafik
                .açıklama_vurgu_sahnesi_boyutta(genişlik, yükseklik, vuruş)
                .komutlar()
            {
                sahne.ekle(komut.clone());
            }
        }
    }

    #[cfg(any(test, feature = "gpui-svg"))]
    fn etkileşim_sahnesi(&self) -> Sahne {
        let mut sahne = Sahne::yeni(1, 1);
        self.etkileşim_sahnesini_doldur(&mut sahne);
        sahne
    }

    fn etkileşim_sahnesini_hazırla(&mut self) -> Option<Rc<Sahne>> {
        #[cfg(test)]
        {
            self.etkileşim_sahne_hazırlama_sayısı =
                self.etkileşim_sahne_hazırlama_sayısı.saturating_add(1);
        }
        let mut yeni = self
            .etkileşim_sahne_tamponu
            .take()
            .unwrap_or_else(|| Sahne::yeni(1, 1));
        self.etkileşim_sahnesini_doldur(&mut yeni);
        if yeni == *self.etkileşim_sahnesi {
            self.etkileşim_sahne_tamponu = Some(yeni);
            return None;
        }
        let eski = std::mem::replace(&mut self.etkileşim_sahnesi, Rc::new(yeni));
        self.etkileşim_sahne_revizyonu = self.etkileşim_sahne_revizyonu.saturating_add(1);
        Some(eski)
    }

    fn etkileşim_sahne_tamponunu_geri_al(&mut self, eski: Rc<Sahne>) {
        if let Ok(eski) = Rc::try_unwrap(eski) {
            self.etkileşim_sahne_tamponu = Some(eski);
        }
    }

    #[cfg(test)]
    fn etkileşim_sahnesini_yenile(&mut self) -> bool {
        let Some(eski) = self.etkileşim_sahnesini_hazırla() else {
            return false;
        };
        self.etkileşim_sahne_tamponunu_geri_al(eski);
        true
    }

    fn etkileşim_yüzeyini_yenile(&mut self, cx: &mut Context<Self>) -> bool {
        let Some(eski) = self.etkileşim_sahnesini_hazırla() else {
            return false;
        };
        if let Some(yüzey) = self.etkileşim_yüzeyi.as_ref() {
            let sahne = self.etkileşim_sahnesi.clone();
            yüzey.update(cx, |yüzey, cx| {
                yüzey.sahneyi_ayarla(sahne);
                cx.notify();
            });
        }
        self.etkileşim_sahne_tamponunu_geri_al(eski);
        true
    }

    fn sahne_konumu(&self, pencere_konumu: ::gpui::Point<Pixels>) -> Option<Nokta> {
        let sınırlar = self.çizim_sınırları.get()?;
        let (kaynak_g, kaynak_y) = self.grafik.boyut();
        YüzeyDikdörtgeni::yeni(
            f64::from(f32::from(sınırlar.origin.x)),
            f64::from(f32::from(sınırlar.origin.y)),
            f64::from(f32::from(sınırlar.size.width)),
            f64::from(f32::from(sınırlar.size.height)),
        )?
        .sahne_konumu(
            f64::from(f32::from(pencere_konumu.x)),
            f64::from(f32::from(pencere_konumu.y)),
            kaynak_g,
            kaynak_y,
        )
    }

    fn grafik_alanında(&self, nokta: Nokta) -> bool {
        let (sol, sağ, üst, alt) = self.çizim_alanı();
        (sol..=sağ).contains(&nokta.x) && (üst..=alt).contains(&nokta.y)
    }

    fn imleç_ızgarasına_oturt(&self, nokta: Nokta) -> Option<Nokta> {
        let (sol, sağ, üst, alt) = self.çizim_alanı();
        let genişlik = sağ - sol;
        let yükseklik = alt - üst;
        let içerik_ölçeği = self.çizim_sınırları.get().map_or(1.0, |sınırlar| {
            let (kaynak_g, kaynak_y) = self.grafik.boyut();
            (f32::from(sınırlar.size.width) / kaynak_g as f32)
                .min(f32::from(sınırlar.size.height) / kaynak_y as f32)
                .max(f32::EPSILON)
        });
        let (yatay, dikey) = self.grafik.imleç_oranlarını_uyarla(
            f64::from((nokta.x - sol) / genişlik),
            f64::from((nokta.y - üst) / yükseklik),
            f64::from(genişlik * içerik_ölçeği),
            f64::from(yükseklik * içerik_ölçeği),
        )?;
        Some(Nokta::yeni(
            sol + yatay as f32 * genişlik,
            üst + dikey as f32 * yükseklik,
        ))
    }

    /// İmleç katmanını fare konumuna göre günceller.
    ///
    /// `değere_yapış` açıkken imleç en yakın örneğin üstüne oturur: X ekseni
    /// örneğin X'ine, ikinci eksen o X'teki en yakın seri değerine. Kapalıyken
    /// fareyi kesintisiz izler. Lejant ve odak değerleri her iki durumda da en
    /// yakın örnekten çözülür — uPlot'ta olduğu gibi çizginin serbest olması
    /// okunan değeri değiştirmez.
    fn imleci_güncelle(
        &mut self,
        pencere_konumu: ::gpui::Point<Pixels>,
        değere_yapış: bool,
    ) -> bool {
        if self.imleç_kilitli {
            return false;
        }
        self.imleç_değere_yapışsın = değere_yapış;
        let Some(fare) = self.sahne_konumu(pencere_konumu) else {
            self.imleç = None;
            self.açıklama_vuruşu = None;
            return self.grafik.imleç_odağını_temizle();
        };
        if !self.grafik_alanında(fare) {
            self.imleç = None;
            self.açıklama_vuruşu = None;
            return self.grafik.imleç_odağını_temizle();
        }
        self.açıklama_vuruşu = self.grafik.açıklama_vuruşu_boyutta(
            self.grafik.boyut().0,
            self.grafik.boyut().1,
            fare.x,
            fare.y,
        );
        let (sol, sağ, üst, alt) = self.çizim_alanı();
        if let Some(vuruş) = self.grafik.dağılım_vuruşu_boyutta(
            self.grafik.boyut().0,
            self.grafik.boyut().1,
            fare.x,
            fare.y,
        ) {
            let mut değerler = vec![None; self.grafik.seri_seçenekleri().len()];
            if let Some(hedef) = değerler.get_mut(vuruş.seri) {
                *hedef = Some(vuruş.y);
            }
            self.imleç = Some(İmleçDurumu {
                fare,
                veri_x: vuruş.x,
                seri_x_değerleri: değerler
                    .iter()
                    .map(|değer| değer.map(|_| vuruş.x))
                    .collect(),
                seri_değerleri: değerler,
                dağılım: Some(vuruş),
            });
            return false;
        }
        let Some(fare) = self.imleç_ızgarasına_oturt(fare) else {
            self.imleç = None;
            return self.grafik.imleç_odağını_temizle();
        };
        let yatay = f64::from((fare.x - sol) / (sağ - sol));
        let dikey = f64::from((fare.y - üst) / (alt - üst));
        let x_dikey = self.grafik.x_dikey_mi();
        let x_oranı = if x_dikey { 1.0 - dikey } else { yatay };
        let x_uzunluğu = if x_dikey { alt - üst } else { sağ - sol };
        let Some(çözüm) = self.grafik.imleç_çözümü(x_oranı, f64::from(x_uzunluğu)) else {
            self.imleç = None;
            return self.grafik.imleç_odağını_temizle();
        };
        let odak_değişti = self.grafik.imleç_odağını_çözümle_güncelle(
            yatay,
            dikey,
            if x_dikey {
                f64::from(sağ - sol)
            } else {
                f64::from(alt - üst)
            },
            &çözüm,
        );
        let seri_x_değerleri = çözüm
            .seriler
            .iter()
            .map(|örnek| örnek.map(|örnek| örnek.x))
            .collect();
        let seri_değerleri = çözüm
            .seriler
            .iter()
            .map(|örnek| örnek.map(|örnek| örnek.değer))
            .collect();
        let x_konumu = if değere_yapış {
            self.grafik.x_konum_oranı(çözüm.imleç_x).unwrap_or(x_oranı) as f32
        } else {
            x_oranı as f32
        };
        // Yapışma açıkken ikinci eksen de örneğe oturur. Yalnız X yapışınca
        // imleç noktası veri noktasının hizasına gelmiyor, yanından geçen bir
        // kesişim gösteriyordu; uPlot'un hover noktası zaten örneğin
        // üstündedir. Aday, yapışılan X'teki seri değerleri arasından fareye
        // en yakın olandır.
        let ikincil_ham = if x_dikey { yatay } else { dikey };
        let ikincil_konum = if değere_yapış {
            let oranlar = çözüm
                .seriler
                .iter()
                .enumerate()
                .map(|(seri, örnek)| {
                    let örnek = (*örnek)?;
                    self.grafik.seri_y_konum_oranı(seri, örnek.değer)
                })
                .collect::<Vec<_>>();
            ikincil_yapışma_konumu(&oranlar, ikincil_ham, x_dikey).unwrap_or(ikincil_ham) as f32
        } else {
            ikincil_ham as f32
        };
        self.imleç = Some(İmleçDurumu {
            fare: if x_dikey {
                Nokta::yeni(
                    sol + ikincil_konum * (sağ - sol),
                    alt - x_konumu * (alt - üst),
                )
            } else {
                Nokta::yeni(
                    sol + x_konumu * (sağ - sol),
                    üst + ikincil_konum * (alt - üst),
                )
            },
            veri_x: çözüm.ortak_x,
            seri_x_değerleri,
            seri_değerleri,
            dağılım: None,
        });
        odak_değişti
    }

    fn bilgi_balonu_beklemesini_yenile(&mut self, cx: &mut Context<Self>) {
        self.bilgi_balonu_hazır = false;
        self.bilgi_balonu_son_hareket = Some(cx.background_executor().now());
        if !self.grafik.etkileşim_seçenekleri().imleç_bilgi_kutusu
            || self.imleç.is_none()
            || self.seçim.is_some()
            || self.taşıma_başlangıcı.is_some()
            || self.grafik.eksen_sürükleniyor()
        {
            self.bilgi_balonu_son_hareket = None;
            self.bilgi_balonu_beklemesi = None;
            return;
        }

        // Her pointer olayında yeni Task üretmek yerine tek bekleyici son
        // hareket zamanını izler. Hareket sürerse yalnız kalan süreyi yeniden
        // bekler; imleç bir saniye durduğunda kendini tamamlar.
        if self.bilgi_balonu_beklemesi.is_some() {
            return;
        }
        self.bilgi_balonu_beklemesi = Some(cx.spawn(async move |bu, cx| {
            let eşik = Duration::from_secs(1);
            let mut kalan = eşik;
            loop {
                cx.background_executor().timer(kalan).await;
                let sonraki = bu
                    .update(cx, |bu, cx| {
                        if bu.imleç.is_none()
                            || !bu.grafik.etkileşim_seçenekleri().imleç_bilgi_kutusu
                            || bu.seçim.is_some()
                            || bu.taşıma_başlangıcı.is_some()
                            || bu.grafik.eksen_sürükleniyor()
                        {
                            return None;
                        }
                        let şimdi = cx.background_executor().now();
                        let geçen = bu
                            .bilgi_balonu_son_hareket
                            .map_or(eşik, |son| şimdi.saturating_duration_since(son));
                        if geçen >= eşik {
                            bu.bilgi_balonu_hazır = true;
                            bu.bilgi_balonu_son_hareket = None;
                            cx.notify();
                            None
                        } else {
                            Some(eşik.saturating_sub(geçen))
                        }
                    })
                    .ok()
                    .flatten();
                let Some(yeni_kalan) = sonraki else {
                    break;
                };
                kalan = yeni_kalan;
            }
            let _ = bu.update(cx, |bu, _cx| {
                bu.bilgi_balonu_beklemesi = None;
            });
        }));
    }

    fn bilgi_balonu_beklemesini_iptal_et(&mut self) -> bool {
        let görünürdü = self.bilgi_balonu_hazır;
        self.bilgi_balonu_hazır = false;
        self.bilgi_balonu_beklemesi = None;
        self.bilgi_balonu_son_hareket = None;
        görünürdü
    }

    fn bilgi_balonu_seri_indeksi(&self, imleç: &İmleçDurumu) -> Option<usize> {
        self.grafik.odak_serisi().or_else(|| {
            (!self.grafik.en_yakın_tooltip_etkin())
                .then(|| {
                    let (sol, sağ, üst, alt) = self.çizim_alanı();
                    imleç
                        .seri_değerleri
                        .iter()
                        .enumerate()
                        .filter_map(|(indeks, değer)| {
                            let değer = (*değer)?;
                            let oran = self.grafik.seri_y_konum_oranı(indeks, değer)? as f32;
                            let mesafe = if self.grafik.x_dikey_mi() {
                                (imleç.fare.x - (sol + (sağ - sol) * oran)).abs()
                            } else {
                                (imleç.fare.y - (alt - (alt - üst) * oran)).abs()
                            };
                            Some((indeks, mesafe))
                        })
                        .min_by(|sol, sağ| sol.1.total_cmp(&sağ.1))
                        .map(|(indeks, _)| indeks)
                })
                .flatten()
        })
    }

    fn tekerlek_yakınlaştır(
        &mut self,
        olay: &ScrollWheelEvent,
        şimdi: Instant,
        odaklı: bool,
    ) -> bool {
        let Some(fare) = self.sahne_konumu(olay.position) else {
            return false;
        };
        if !self.grafik_alanında(fare) {
            return false;
        }
        if cfg!(any(target_os = "windows", target_family = "wasm"))
            && self.grafik.etkileşim_seçenekleri().dokunma_etkileşimi
        {
            match olay.touch_phase {
                TouchPhase::Started => {
                    let _ = self.grafik.taşımayı_başlat();
                    self.dokunma_kaydırma = Some((0.0_f64, 0.0_f64));
                    return false;
                }
                TouchPhase::Ended | TouchPhase::Cancelled if self.dokunma_kaydırma.is_some() => {
                    self.dokunma_kaydırma = None;
                    self.grafik.taşımayı_bitir();
                    return false;
                }
                TouchPhase::Moved => {}
                _ => return false,
            }
        }
        if !self
            .grafik
            .etkileşim_seçenekleri()
            .tekerlek_odaksız_etkileşim
            && !odaklı
        {
            return false;
        }
        let (sol, sağ, üst, alt) = self.çizim_alanı();
        if let Some((birikmiş_x, birikmiş_y)) = self.dokunma_kaydırma.as_mut() {
            let (x, y) = match olay.delta {
                ScrollDelta::Pixels(delta) => {
                    (f64::from(f32::from(delta.x)), f64::from(f32::from(delta.y)))
                }
                ScrollDelta::Lines(delta) => (f64::from(delta.x * 16.0), f64::from(delta.y * 16.0)),
            };
            *birikmiş_x += x / f64::from(sağ - sol);
            *birikmiş_y += y / f64::from(alt - üst);
            return match self.grafik.taşı(*birikmiş_x, *birikmiş_y) {
                Ok(değişti) => {
                    self.hata = None;
                    değişti
                }
                Err(hata) => {
                    self.hata = Some(format!("Dokunma taşıması uygulanamadı: {hata}"));
                    false
                }
            };
        }
        let eksen = Self::tekerlek_ekseni(olay.modifiers.shift, olay.modifiers.control);
        let (delta, hassas) = match olay.delta {
            ScrollDelta::Pixels(delta) => {
                let x = f64::from(f32::from(delta.x));
                let y = f64::from(f32::from(delta.y));
                (
                    if eksen == TekerlekEkseni::X && x.abs() > y.abs() {
                        x
                    } else {
                        y
                    },
                    true,
                )
            }
            ScrollDelta::Lines(delta) => {
                let x = f64::from(delta.x);
                let y = f64::from(delta.y);
                (
                    if eksen == TekerlekEkseni::X && x.abs() > y.abs() {
                        x
                    } else {
                        y
                    },
                    false,
                )
            }
        };
        let yatay = f64::from((fare.x - sol) / (sağ - sol));
        let dikey = f64::from((fare.y - üst) / (alt - üst));
        match self
            .grafik
            .tekerlek_eksende_zamanda(yatay, dikey, delta, hassas, eksen, şimdi)
        {
            Ok(değişti) => {
                self.hata = None;
                değişti
            }
            Err(hata) => {
                self.hata = Some(format!("Tekerlek yakınlaştırması uygulanamadı: {hata}"));
                false
            }
        }
    }

    fn tekerlek_ekseni(shift: bool, control: bool) -> TekerlekEkseni {
        match (shift, control) {
            (true, false) => TekerlekEkseni::X,
            (false, true) => TekerlekEkseni::Y,
            _ => TekerlekEkseni::İkisi,
        }
    }

    fn tekerlek_olayı_etkin(ayarlar: crate::EtkileşimSeçenekleri, grafik_odaklı: bool) -> bool {
        ayarlar.tekerlek_etkileşimi && (ayarlar.tekerlek_odaksız_etkileşim || grafik_odaklı)
    }

    fn dokunma_yakınlaştır(&mut self, olay: &PinchEvent) -> bool {
        if matches!(olay.phase, TouchPhase::Ended | TouchPhase::Cancelled) {
            self.grafik.dokunmayı_bitir();
            return false;
        }
        if olay.phase == TouchPhase::Started && !self.grafik.dokunmayı_başlat() {
            return false;
        }
        let Some(fare) = self.sahne_konumu(olay.position) else {
            return false;
        };
        if !self.grafik_alanında(fare) {
            return false;
        }
        let (sol, sağ, üst, alt) = self.çizim_alanı();
        let yatay = f64::from((fare.x - sol) / (sağ - sol));
        let dikey = f64::from((fare.y - üst) / (alt - üst));
        let çarpan = f64::from((1.0 + olay.delta).max(0.01));
        match self.grafik.dokunma_yakınlaştır(yatay, dikey, çarpan) {
            Ok(değişti) => {
                self.hata = None;
                değişti
            }
            Err(hata) => {
                self.hata = Some(format!("Dokunma yakınlaştırması uygulanamadı: {hata}"));
                false
            }
        }
    }

    fn seçimi_tamamla(&mut self, cx: &mut Context<Self>) -> bool {
        let açıklama_seçimi = std::mem::take(&mut self.açıklama_seçimi);
        let Some((başlangıç, bitiş)) = self.seçim.take() else {
            return false;
        };
        let ayarlar = self.grafik.etkileşim_seçenekleri();
        let x_farkı = (bitiş.x - başlangıç.x).abs();
        let y_farkı = (bitiş.y - başlangıç.y).abs();
        let kaynak_sıfır_eşik = ayarlar.imleç_bağları.ctrl_seçim_ölçeğini_durdur
            || ayarlar.imleç_bağları.tıklamayı_ilet;
        let eşik = if kaynak_sıfır_eşik {
            f64::from(f32::EPSILON)
        } else {
            4.0
        };
        let (yatay_etkin, dikey_etkin) =
            self.grafik
                .fiziksel_seçim_eksenleri(f64::from(x_farkı), f64::from(y_farkı), eşik);
        if !yatay_etkin && !dikey_etkin {
            cx.emit(GpuiGrafikOlayı::FareBırakıldı);
            return false;
        }
        let (sol, sağ, üst, alt) = self.çizim_alanı();
        let sonuç = if ayarlar.seçim_xy_yakınlaştır {
            self.grafik
                .fiziksel_seçim_yakınlaştır_eksenlerde(
                    f64::from((başlangıç.x - sol) / (sağ - sol)),
                    f64::from((başlangıç.y - üst) / (alt - üst)),
                    f64::from((bitiş.x - sol) / (sağ - sol)),
                    f64::from((bitiş.y - üst) / (alt - üst)),
                    yatay_etkin,
                    dikey_etkin,
                )
                .map(|değişti| değişti.then_some(SeçimEylemi::Yakınlaştırıldı))
        } else {
            let (başlangıç_oranı, bitiş_oranı) = if self.grafik.x_dikey_mi() {
                (
                    f64::from((alt - başlangıç.y) / (alt - üst)),
                    f64::from((alt - bitiş.y) / (alt - üst)),
                )
            } else {
                (
                    f64::from((başlangıç.x - sol) / (sağ - sol)),
                    f64::from((bitiş.x - sol) / (sağ - sol)),
                )
            };
            self.grafik
                .seçimi_bitir(başlangıç_oranı, bitiş_oranı, açıklama_seçimi)
                .map(Some)
        };
        match sonuç {
            Ok(Some(SeçimEylemi::Açıklamaİstendi)) => {
                self.hata = None;
                cx.emit(GpuiGrafikOlayı::Açıklamaİstendi);
                false
            }
            Ok(Some(_)) => {
                self.hata = None;
                true
            }
            Ok(None) => {
                self.hata = None;
                false
            }
            Err(hata) => {
                self.hata = Some(format!("Seçilen aralık uygulanamadı: {hata}"));
                false
            }
        }
    }

    fn sahneyi_yenile(&mut self, cx: &mut Context<Self>) {
        let _ölçüm = crate::izleme::Ölçüm::başlat(crate::izleme::Yuva::TamSahne);
        self.açıklama_vuruşu = None;
        self.arka_plan_sahnesi = Rc::new(self.grafik.gpui_arka_plan_sahnesini_çiz());
        self.ana_sahne = Rc::new(self.grafik.gpui_görünür_veri_sahnesini_çiz());
        self.ana_sahne_revizyonu = self.ana_sahne_revizyonu.saturating_add(1);
        self.görünümü_yenile();
        if let Some(yüzey) = self.arka_plan_yüzeyi.as_ref() {
            let sahne = self.arka_plan_sahnesi.clone();
            yüzey.update(cx, |yüzey, cx| {
                yüzey.sahneyi_ayarla(sahne);
                cx.notify();
            });
        }
        let duyarlı_grafik = self.grafik.duyarlı_boyut_mu().then(|| cx.weak_entity());
        if let Some(yüzey) = self.ana_yüzey.as_ref() {
            let sahne = self.ana_sahne.clone();
            yüzey.update(cx, |yüzey, cx| {
                yüzey.sahneyi_ayarla(sahne, duyarlı_grafik);
                cx.notify();
            });
        }
        self.eksen_sahnesini_yenile(cx);
        self.etkileşim_yüzeyini_yenile(cx);
    }

    /// Veri yüzeyini güncel görünüm penceresi için yeniden kurar. Sabit arka
    /// planı ve eksen/grid katmanını ellemez; odak değişimi ile görünüm
    /// değişimi aynı yolu paylaşır.
    fn veri_sahnesini_yenile(&mut self, cx: &mut Context<Self>) {
        let _ölçüm = crate::izleme::Ölçüm::başlat(crate::izleme::Yuva::VeriSahnesi);
        self.ana_sahne = Rc::new(self.grafik.gpui_görünür_veri_sahnesini_çiz());
        self.ana_sahne_revizyonu = self.ana_sahne_revizyonu.saturating_add(1);
        self.görünümü_yenile();
        let duyarlı_grafik = self.grafik.duyarlı_boyut_mu().then(|| cx.weak_entity());
        if let Some(yüzey) = self.ana_yüzey.as_ref() {
            let sahne = self.ana_sahne.clone();
            yüzey.update(cx, |yüzey, cx| {
                yüzey.sahneyi_ayarla(sahne, duyarlı_grafik);
                cx.notify();
            });
        }
    }

    fn eksen_sahnesini_yenile(&mut self, cx: &mut Context<Self>) {
        self.eksen_sahnesi = Rc::new(self.grafik.gpui_eksen_sahnesini_çiz());
        if let Some(yüzey) = self.eksen_yüzeyi.as_ref() {
            let sahne = self.eksen_sahnesi.clone();
            yüzey.update(cx, |yüzey, cx| {
                yüzey.sahneyi_ayarla(sahne);
                cx.notify();
            });
        }
    }

    fn görünümü_yenile(&mut self) {
        self.veri_görünümü.set(GpuiVeriGörünümü {
            // Veri sahnesi güncel görünüm penceresi için kurulduğundan
            // yüzeye ek bir yakınlaştırma dönüşümü uygulanmaz; pencere
            // birimdir ve `GpuiBoyaGörünümü` yalnız kırpmayı taşır.
            pencere: OransalGörünüm::default(),
            çizim_alanı: self.çizim_alanı(),
        });
        self.görünüm_revizyonu = self.görünüm_revizyonu.saturating_add(1);
    }

    fn grafik_bildir(&mut self, cx: &mut Context<Self>) {
        self.sahneyi_yenile(cx);
        Self::bildir(cx);
    }

    fn görünüm_bildir(&mut self, fare_basma_bırakma: bool, cx: &mut Context<Self>) {
        self.görünümü_sessiz_bildir(cx);
        cx.emit(GpuiGrafikOlayı::GörünümDeğişti {
            fare_basma_bırakma
        });
    }

    /// Veri ve eksen katmanlarını güncel pencereye göre tazeler; olay yaymaz.
    fn görünümü_sessiz_bildir(&mut self, cx: &mut Context<Self>) {
        self.açıklama_vuruşu = None;
        // Görünüm değişince imleç katmanı eski piksel konumunda kalıyordu:
        // zoom sonrası vurgu ve lejant, fare kıpırdayana kadar yeni ölçeğin
        // yanlış noktasını gösteriyordu. İmleç aynı fare konumundan yeni
        // görünüme göre yeniden çözülür.
        if let Some(fare) = self.imleç.as_ref().map(|imleç| imleç.fare) {
            self.canlı_imleci_yenile(fare);
        }
        self.veri_sahnesini_yenile(cx);
        self.eksen_sahnesini_yenile(cx);
        cx.notify();
    }

    fn bildir(cx: &mut Context<Self>) {
        cx.emit(GpuiGrafikOlayı::DurumDeğişti);
        cx.notify();
    }

    #[cfg(test)]
    fn imleç_bildir(cx: &mut Context<Self>) {
        cx.emit(GpuiGrafikOlayı::İmleçKonumuDeğişti);
        cx.notify();
    }
}

impl EventEmitter<GpuiGrafikOlayı> for GpuiGrafik {}

impl Render for GpuiGrafik {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let _ölçüm = crate::izleme::Ölçüm::başlat(crate::izleme::Yuva::GrafikRender);
        if self
            .grafik
            .cihaz_piksel_oranını_ayarla(window.scale_factor())
        {
            self.sahneyi_yenile(cx);
        }
        let odak = self
            .odak
            .get_or_insert_with(|| cx.focus_handle().tab_stop(true))
            .clone();
        let arka_plan_yüzeyi = self
            .arka_plan_yüzeyi
            .get_or_insert_with(|| {
                let sahne = self.arka_plan_sahnesi.clone();
                cx.new(|_| GpuiEtkileşimYüzeyi {
                    sahne,
                    yol_önbelleği: Rc::new(RefCell::new(GpuiYolÖnbelleği::default())),
                    çizim_görünümü: None,
                })
            })
            .clone();
        let ana_yüzey = self
            .ana_yüzey
            .get_or_insert_with(|| {
                let sahne = self.ana_sahne.clone();
                let çizim_sınırları = self.çizim_sınırları.clone();
                let veri_görünümü = self.veri_görünümü.clone();
                let duyarlı_grafik = self.grafik.duyarlı_boyut_mu().then(|| cx.weak_entity());
                cx.new(|_| GpuiAnaYüzey {
                    sahne,
                    çizim_sınırları,
                    yol_önbelleği: Rc::new(RefCell::new(GpuiYolÖnbelleği::default())),
                    duyarlı_grafik,
                    veri_görünümü,
                })
            })
            .clone();
        // İlk yerleşimde etkileşim yüzeyi henüz yoktur. Sahneyi bir kez
        // hazırlayıp yüzeye doğrudan veriyoruz; `cx.notify()` ile ikinci bir
        // ilk-frame renderı üretmiyoruz. Sonraki etkileşimler yüzeyi olay
        // işleyicilerinden, yalnız durum gerçekten değiştiğinde günceller.
        if self.etkileşim_yüzeyi.is_none()
            && let Some(eski) = self.etkileşim_sahnesini_hazırla()
        {
            self.etkileşim_sahne_tamponunu_geri_al(eski);
        }
        let etkileşim_yüzeyi = self
            .etkileşim_yüzeyi
            .get_or_insert_with(|| {
                let sahne = self.etkileşim_sahnesi.clone();
                let çizim_görünümü = self.veri_görünümü.clone();
                cx.new(|_| GpuiEtkileşimYüzeyi {
                    sahne,
                    yol_önbelleği: Rc::new(RefCell::new(GpuiYolÖnbelleği::default())),
                    çizim_görünümü: Some(çizim_görünümü),
                })
            })
            .clone();
        let eksen_yüzeyi = self
            .eksen_yüzeyi
            .get_or_insert_with(|| {
                let sahne = self.eksen_sahnesi.clone();
                cx.new(|_| GpuiEtkileşimYüzeyi {
                    sahne,
                    yol_önbelleği: Rc::new(RefCell::new(GpuiYolÖnbelleği::default())),
                    çizim_görünümü: None,
                })
            })
            .clone();
        let taşıyor = self.taşıma_başlangıcı.is_some();
        let taşımaya_hazır = self.boşluk_basılı && self.grafik.yakınlaştırılmış();
        let eksen_sürükleniyor = self.grafik.eksen_sürükleniyor();
        let eksen_imleci = self.eksen_üzerinde || eksen_sürükleniyor;
        let standart_bilgi_kutusu = self
            .imleç
            .as_ref()
            .filter(|_| self.bilgi_balonu_hazır)
            .filter(|_| self.grafik.etkileşim_seçenekleri().imleç_bilgi_kutusu)
            .filter(|_| self.grafik.tooltip_düzeni().is_none())
            .and_then(|imleç| {
                let seri_indeksi = self.bilgi_balonu_seri_indeksi(imleç);
                let y = imleç.dağılım.as_ref().map(|vuruş| vuruş.y).or_else(|| {
                    seri_indeksi
                        .and_then(|indeks| imleç.seri_değerleri.get(indeks))
                        .copied()
                        .flatten()
                })?;
                let sınırlar = self.çizim_sınırları.get()?;
                let (kaynak_g, kaynak_y) = self.grafik.boyut();
                let ölçek = (f32::from(sınırlar.size.width) / kaynak_g as f32)
                    .min(f32::from(sınırlar.size.height) / kaynak_y as f32)
                    .max(0.01);
                let yatay_pay = (f32::from(sınırlar.size.width) - kaynak_g as f32 * ölçek) / 2.0;
                let dikey_pay = (f32::from(sınırlar.size.height) - kaynak_y as f32 * ölçek) / 2.0;
                let (çizim_sol, çizim_sağ, çizim_üst, çizim_alt) = self.çizim_alanı();
                let yatay_oran = f64::from(
                    ((imleç.fare.x - çizim_sol) / (çizim_sağ - çizim_sol)).clamp(0.0, 1.0),
                );
                let en_yakın =
                    seri_indeksi.and_then(|seri| self.grafik.en_yakın_tooltip(yatay_oran, seri));
                if self.grafik.en_yakın_tooltip_etkin() && en_yakın.is_none() {
                    return None;
                }
                let kenarlık = en_yakın.as_ref().map_or_else(
                    || "#000000".to_string(),
                    |bilgi| bilgi.kenarlık_rengi.clone(),
                );
                let bağlantı = en_yakın
                    .as_ref()
                    .map(|bilgi| bilgi.karşılaştırma_url.clone());
                let (bağlantı_sol, bağlantı_üst) = en_yakın
                    .as_ref()
                    .and_then(|bilgi| {
                        let x = self.grafik.x_konum_oranı(bilgi.zaman)?;
                        let y = self.grafik.seri_y_konum_oranı(bilgi.seri, bilgi.değer)?;
                        let (_, _, çizim_üst, çizim_alt) = self.çizim_alanı();
                        Some((
                            çizim_sol + (çizim_sağ - çizim_sol) * x as f32,
                            çizim_alt - (çizim_alt - çizim_üst) * y as f32,
                        ))
                    })
                    .unwrap_or((imleç.fare.x, imleç.fare.y));
                let metin = en_yakın.map_or_else(
                    || {
                        imleç.dağılım.as_ref().map_or_else(
                            || {
                                format!(
                                    "{},{y} at {},{}",
                                    imleç.veri_x,
                                    ((imleç.fare.x - çizim_sol) * ölçek).round(),
                                    ((imleç.fare.y - çizim_üst) * ölçek).round()
                                )
                            },
                            |vuruş| {
                                format!(
                                    "Country: {} · Population: {} · GDP: ${} · Income: ${}",
                                    vuruş.etiket.as_deref().unwrap_or("--"),
                                    vuruş.değer.map_or_else(
                                        || "--".to_string(),
                                        |değer| değer.to_string()
                                    ),
                                    vuruş.x,
                                    vuruş.y
                                )
                            },
                        )
                    },
                    |bilgi| bilgi.metin,
                );
                let ölçüm_metni = SharedString::from(metin.replace(['\r', '\n'], " "));
                let metin_koşusu = TextRun {
                    len: ölçüm_metni.len(),
                    font: window.text_style().font(),
                    color: renk_çöz("#ffffff"),
                    background_color: None,
                    underline: None,
                    strikethrough: None,
                };
                let (metin_kimliği, metin_uzunluğu) =
                    tek_satır_metin_kimliği(ölçüm_metni.as_ref());
                let metin_çizgisi = window.text_system().shape_line_by_hash(
                    metin_kimliği,
                    metin_uzunluğu,
                    px(12.0),
                    &[metin_koşusu],
                    None,
                    || ölçüm_metni,
                );
                let kutu_genişliği = f64::from(f32::from(metin_çizgisi.width()) + 18.0);
                let kutu_yüksekliği = 26.0;
                let plot_sınırı = YüzeyDikdörtgeni::yeni(
                    f64::from(yatay_pay + çizim_sol * ölçek),
                    f64::from(dikey_pay + çizim_üst * ölçek),
                    f64::from((çizim_sağ - çizim_sol) * ölçek),
                    f64::from((çizim_alt - çizim_üst) * ölçek),
                )?;
                let yerleşim = bilgi_kutusunu_yerleştir(
                    plot_sınırı,
                    f64::from(yatay_pay + bağlantı_sol * ölçek),
                    f64::from(dikey_pay + bağlantı_üst * ölçek),
                    kutu_genişliği,
                    kutu_yüksekliği,
                    12.0,
                )?;
                Some((
                    yerleşim.sol as f32,
                    yerleşim.üst as f32,
                    metin,
                    kenarlık,
                    bağlantı,
                    yerleşim.azami_genişlik as f32,
                ))
            });
        let açıklama_bilgi_kutusu = self
            .açıklama_vuruşu
            .as_ref()
            .filter(|vuruş| vuruş.etiket_üzerinde && !vuruş.açıklama.is_empty())
            .and_then(|vuruş| {
                let imleç = self.imleç.as_ref()?;
                let sınırlar = self.çizim_sınırları.get()?;
                let (kaynak_g, kaynak_y) = self.grafik.boyut();
                let ölçek = (f32::from(sınırlar.size.width) / kaynak_g as f32)
                    .min(f32::from(sınırlar.size.height) / kaynak_y as f32)
                    .max(0.01);
                let yatay_pay = (f32::from(sınırlar.size.width) - kaynak_g as f32 * ölçek) / 2.0;
                let dikey_pay = (f32::from(sınırlar.size.height) - kaynak_y as f32 * ölçek) / 2.0;
                Some((
                    (yatay_pay + imleç.fare.x * ölçek + 12.0)
                        .clamp(4.0, (f32::from(sınırlar.size.width) - 190.0).max(4.0)),
                    (dikey_pay + imleç.fare.y * ölçek + 12.0)
                        .clamp(4.0, (f32::from(sınırlar.size.height) - 42.0).max(4.0)),
                    vuruş.açıklama.clone(),
                    vuruş.çizgi.clone(),
                    None,
                    (f32::from(sınırlar.size.width) - 8.0).max(0.0),
                ))
            });
        let bilgi_kutusu = açıklama_bilgi_kutusu.or(standart_bilgi_kutusu);
        let tooltip_kutuları = self
            .imleç
            .as_ref()
            .filter(|_| self.bilgi_balonu_hazır)
            .filter(|_| self.grafik.etkileşim_seçenekleri().imleç_bilgi_kutusu)
            .and_then(|imleç| {
                let sınırlar = self.çizim_sınırları.get()?;
                let (kaynak_g, kaynak_y) = self.grafik.boyut();
                let ölçek = (f32::from(sınırlar.size.width) / kaynak_g as f32)
                    .min(f32::from(sınırlar.size.height) / kaynak_y as f32)
                    .max(0.01);
                let yatay_pay = (f32::from(sınırlar.size.width) - kaynak_g as f32 * ölçek) / 2.0;
                let dikey_pay = (f32::from(sınırlar.size.height) - kaynak_y as f32 * ölçek) / 2.0;
                let (sol, sağ, üst, alt) = self.çizim_alanı();
                let yatay_oran = f64::from(((imleç.fare.x - sol) / (sağ - sol)).clamp(0.0, 1.0));
                let dikey_oran = f64::from(((imleç.fare.y - üst) / (alt - üst)).clamp(0.0, 1.0));
                Some(
                    self.grafik
                        .tooltip_bilgileri(yatay_oran, dikey_oran)
                        .into_iter()
                        .map(|bilgi| {
                            let kaynak_x =
                                sol + (sağ - sol) * bilgi.yatay_oran.clamp(0.0, 1.0) as f32;
                            let kaynak_y =
                                üst + (alt - üst) * bilgi.dikey_oran.clamp(0.0, 1.0) as f32;
                            let seri_tooltipi = bilgi.seri.is_some();
                            let fiziksel_x = yatay_pay + kaynak_x * ölçek;
                            let fiziksel_y = dikey_pay + kaynak_y * ölçek;
                            let kutu_sol = if seri_tooltipi {
                                fiziksel_x.round()
                            } else {
                                fiziksel_x
                            };
                            let kutu_üst = if seri_tooltipi {
                                fiziksel_y.round()
                            } else {
                                fiziksel_y
                            };
                            (
                                kutu_sol,
                                kutu_üst,
                                bilgi.metin,
                                bilgi.arka_plan_rengi,
                                bilgi.metin_rengi,
                            )
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .unwrap_or_default();
        let bilgi_katmanı = div()
            .absolute()
            .top_0()
            .left_0()
            .size_full()
            .child(
                etkileşim_yüzeyi.cached(
                    StyleRefinement::default()
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full(),
                ),
            )
            .when_some(
                bilgi_kutusu,
                |yüzey, (sol, üst, metin, kenarlık, bağlantı, azami_genişlik)| {
                    yüzey.child(
                        div()
                            .absolute()
                            .left(px(sol))
                            .top(px(üst))
                            .max_w(px(azami_genişlik))
                            .px_2()
                            .py_1()
                            .border_1()
                            .border_color(renk_çöz(&kenarlık))
                            .rounded_sm()
                            .bg(if bağlantı.is_some() {
                                rgb(0xffffff)
                            } else {
                                rgba(0x000000cc)
                            })
                            .text_color(if bağlantı.is_some() {
                                rgb(0x111111)
                            } else {
                                rgb(0xffffff)
                            })
                            .text_xs()
                            .child(metin),
                    )
                },
            )
            .children(tooltip_kutuları.into_iter().map(
                |(sol, üst, metin, arka_plan, metin_rengi)| {
                    div()
                        .absolute()
                        .left(px(sol))
                        .top(px(üst))
                        .px_2()
                        .py_1()
                        .rounded_sm()
                        .bg(renk_çöz(&arka_plan))
                        .text_color(renk_çöz(&metin_rengi))
                        .text_xs()
                        .child(metin)
                },
            ))
            .into_any_element();
        let yüzey_stili = || {
            StyleRefinement::default()
                .absolute()
                .top_0()
                .left_0()
                .size_full()
        };
        let mut arka_plan_katmanı = Some(arka_plan_yüzeyi.cached(yüzey_stili()).into_any_element());
        let mut eksen_katmanı = Some(eksen_yüzeyi.cached(yüzey_stili()).into_any_element());
        let mut veri_katmanı = Some(ana_yüzey.cached(yüzey_stili()).into_any_element());
        let mut bilgi_katmanı = Some(bilgi_katmanı);
        let mut çizim_katmanları = Vec::<AnyElement>::with_capacity(4);
        for katman in self.grafik.katman_sırası() {
            let öğe = match katman {
                crate::ÇizimKatmanı::ArkaPlan => arka_plan_katmanı.take(),
                crate::ÇizimKatmanı::IzgaraEksen => eksen_katmanı.take(),
                crate::ÇizimKatmanı::Veri => veri_katmanı.take(),
                crate::ÇizimKatmanı::Bilgi => bilgi_katmanı.take(),
            };
            if let Some(öğe) = öğe {
                çizim_katmanları.push(öğe);
            }
        }
        div()
            .id("uplot-rs-gpui-grafik")
            .relative()
            .role(Role::Group)
            .aria_label("Etkileşimli uPlot.rs grafiği")
            .track_focus(&odak)
            .key_context("uplot_rs_grafik")
            .size_full()
            .min_h(px(en_az_yüzey_yüksekliği(self.grafik.boyut().1)))
            .overflow_hidden()
            .when(taşıyor, |yüzey| yüzey.cursor_grabbing())
            .when(!taşıyor && taşımaya_hazır, |yüzey| yüzey.cursor_grab())
            .when(!taşıyor && !taşımaya_hazır && eksen_imleci, |yüzey| {
                yüzey.cursor_move()
            })
            .when(!taşıyor && self.açıklama_vuruşu.is_some(), |yüzey| {
                yüzey.cursor_pointer()
            })
            .on_action(cx.listener(|bu, _: &ÖlçümüTemizle, _, cx| {
                if bu.grafik.ölçüm_datumlarını_temizle() {
                    cx.stop_propagation();
                    bu.grafik_bildir(cx);
                }
            }))
            .on_action(cx.listener(|bu, _: &BirinciDatumuAyarla, _, cx| {
                bu.ölçüm_datumunu_imleçte_ayarla(1, cx);
            }))
            .on_action(cx.listener(|bu, _: &İkinciDatumuAyarla, _, cx| {
                bu.ölçüm_datumunu_imleçte_ayarla(2, cx);
            }))
            .on_key_down(cx.listener(|bu, olay: &KeyDownEvent, _, cx| {
                let tuş = olay.keystroke.key.as_str();
                if tuş == "space" && !bu.boşluk_basılı {
                    bu.boşluk_basılı = true;
                    bu.seçim = None;
                    bu.açıklama_seçimi = false;
                    cx.stop_propagation();
                    bu.etkileşim_yüzeyini_yenile(cx);
                    cx.notify();
                }
            }))
            .on_key_up(cx.listener(|bu, olay: &KeyUpEvent, _, cx| {
                if olay.keystroke.key.as_str() == "space" && bu.boşluk_basılı {
                    bu.boşluk_basılı = false;
                    bu.taşıma_başlangıcı = None;
                    bu.grafik.taşımayı_bitir();
                    cx.stop_propagation();
                    bu.etkileşim_yüzeyini_yenile(cx);
                    cx.notify();
                }
            }))
            .on_mouse_move(cx.listener(|bu, olay: &MouseMoveEvent, _window, cx| {
                // Önceki seri vektörlerini her pointer olayında deep-clone
                // etmeyiz. Durumu geçici olarak sahiplenmek karşılaştırmayı
                // tahsissiz tutar; aşağıdaki dallar yeni durumu yerleştirir.
                let mut önceki_imleç = bu.imleç.take();
                let mut önceki_açıklama = bu.açıklama_vuruşu.take();
                let önceki_seçim = bu.seçim;
                let önceki_eksen = bu.eksen_üzerinde;
                let önceki_hata = bu.hata.clone();
                let bilgi_balonu_görünürdü = bu.bilgi_balonu_hazır;
                let mut ana_sahne_değişti = false;
                let mut görünüm_değişti = false;
                let mut imleç_korundu = false;
                if bu.grafik.eksen_sürükleniyor()
                    && let Some(konum) = bu.sahne_konumu(olay.position)
                {
                    match bu
                        .grafik
                        .eksen_sürükle(konum.x, konum.y, olay.modifiers.shift)
                    {
                        Ok(değişti) => {
                            bu.hata = None;
                            ana_sahne_değişti = değişti;
                            görünüm_değişti = değişti;
                        }
                        Err(hata) => {
                            bu.hata = Some(format!("Eksen ölçeği sürüklenemedi: {hata}"));
                        }
                    }
                    bu.imleç = None;
                    bu.açıklama_vuruşu = None;
                } else if let Some(başlangıç) = bu.taşıma_başlangıcı
                    && let Some(konum) = bu.sahne_konumu(olay.position)
                {
                    let (sol, sağ, üst, alt) = bu.çizim_alanı();
                    let yatay = f64::from((konum.x - başlangıç.x) / (sağ - sol));
                    let dikey = f64::from((konum.y - başlangıç.y) / (alt - üst));
                    match bu.grafik.taşı(yatay, dikey) {
                        Ok(değişti) => {
                            bu.hata = None;
                            ana_sahne_değişti = değişti;
                            görünüm_değişti = değişti;
                        }
                        Err(hata) => {
                            bu.hata = Some(format!("Grafik görünümü taşınamadı: {hata}"));
                        }
                    }
                    bu.imleç = None;
                    bu.açıklama_vuruşu = None;
                } else if bu.imleç_kilitli {
                    // Kilitli imleç dışarıdan gelen senkron konumunu korur.
                    // Yukarıdaki `take`, normal akışta pahalı vektör
                    // klonlarını önler; bu dalda sahipliği geri veriyoruz.
                    bu.imleç = önceki_imleç.take();
                    bu.açıklama_vuruşu = önceki_açıklama.take();
                    imleç_korundu = true;
                } else {
                    // Ctrl basılıyken imleç çizgisi örnek konumlarına oturur;
                    // basılı değilken fareyi kesintisiz izler.
                    ana_sahne_değişti = bu.imleci_güncelle(olay.position, olay.modifiers.control);
                }
                if bu.taşıma_başlangıcı.is_none()
                    && !bu.grafik.eksen_sürükleniyor()
                    && olay.dragging()
                    && let Some((başlangıç, _)) = bu.seçim
                    && let Some(konum) = bu.sahne_konumu(olay.position)
                {
                    let (sol, sağ, üst, alt) = bu.çizim_alanı();
                    let xy = bu.grafik.etkileşim_seçenekleri().seçim_xy_yakınlaştır;
                    let ham_bitiş = if xy {
                        Nokta::yeni(konum.x.clamp(sol, sağ), konum.y.clamp(üst, alt))
                    } else if bu.grafik.x_dikey_mi() {
                        Nokta::yeni(başlangıç.x, konum.y.clamp(üst, alt))
                    } else {
                        Nokta::yeni(konum.x.clamp(sol, sağ), başlangıç.y)
                    };
                    let bitiş = bu.imleç_ızgarasına_oturt(ham_bitiş).unwrap_or(ham_bitiş);
                    bu.seçim = Some((başlangıç, bitiş));
                }
                if olay.dragging()
                    && let Some((başlangıç, _)) = bu.tooltip_tıklama_başlangıcı.as_ref()
                    && let Some(konum) = bu.sahne_konumu(olay.position)
                    && (başlangıç.x != konum.x || başlangıç.y != konum.y)
                {
                    bu.tooltip_tıklaması_sürüklendi = true;
                }
                if !bu.grafik.eksen_sürükleniyor()
                    && let Some(konum) = bu.sahne_konumu(olay.position)
                {
                    let (genişlik, yükseklik) = bu.grafik.boyut();
                    bu.eksen_üzerinde = bu
                        .grafik
                        .eksen_vuruşu_boyutta(genişlik, yükseklik, konum.x, konum.y)
                        .is_some();
                }
                let imleç_değişti = !imleç_korundu && bu.imleç != önceki_imleç;
                let lejant_değişti = !imleç_korundu
                    && !imleç_lejant_verisi_aynı(önceki_imleç.as_ref(), bu.imleç.as_ref());
                let açıklama_değişti = !imleç_korundu && bu.açıklama_vuruşu != önceki_açıklama;
                let seçim_değişti = bu.seçim != önceki_seçim;
                let etkileşim_değişti = imleç_değişti || açıklama_değişti || seçim_değişti;
                if imleç_değişti {
                    bu.bilgi_balonu_beklemesini_yenile(cx);
                }
                crate::izleme::fare_sahne_kararı(ana_sahne_değişti);
                if ana_sahne_değişti {
                    if görünüm_değişti {
                        bu.görünüm_bildir(false, cx);
                    } else {
                        bu.veri_sahnesini_yenile(cx);
                    }
                }
                if etkileşim_değişti {
                    bu.etkileşim_yüzeyini_yenile(cx);
                }
                if imleç_değişti {
                    cx.emit(GpuiGrafikOlayı::İmleçKonumuDeğişti);
                }
                if lejant_değişti || (ana_sahne_değişti && !görünüm_değişti) {
                    cx.emit(GpuiGrafikOlayı::İmleçDeğişti);
                }
                if önceki_eksen != bu.eksen_üzerinde
                    || önceki_hata != bu.hata
                    || açıklama_değişti
                    || (bilgi_balonu_görünürdü && imleç_değişti)
                {
                    cx.notify();
                }
            }))
            .on_scroll_wheel(cx.listener(|bu, olay: &ScrollWheelEvent, window, cx| {
                let ayarlar = bu.grafik.etkileşim_seçenekleri();
                let odaklı = bu.odak.as_ref().is_some_and(|odak| odak.is_focused(window));
                let grafik_üzerinde = bu
                    .sahne_konumu(olay.position)
                    .is_some_and(|konum| bu.grafik_alanında(konum));
                let tekerlek_kabul_edildi =
                    grafik_üzerinde && Self::tekerlek_olayı_etkin(ayarlar, odaklı);
                let dokunma_kabul_edildi = grafik_üzerinde
                    && cfg!(any(target_os = "windows", target_family = "wasm"))
                    && ayarlar.dokunma_etkileşimi
                    && (bu.dokunma_kaydırma.is_some()
                        || matches!(
                            olay.touch_phase,
                            TouchPhase::Started | TouchPhase::Ended | TouchPhase::Cancelled
                        ));
                if !tekerlek_kabul_edildi && !dokunma_kabul_edildi {
                    return;
                }
                let bilgi_balonu_görünürdü = bu.bilgi_balonu_beklemesini_iptal_et();
                cx.stop_propagation();
                let datum_değişti = bu.grafik.ölçüm_datumlarını_temizle();
                let şimdi = cx.background_executor().now();
                let görünüm_değişti = bu.tekerlek_yakınlaştır(olay, şimdi, odaklı);
                if görünüm_değişti {
                    bu.görünüm_bildir(false, cx);
                } else if datum_değişti {
                    bu.grafik_bildir(cx);
                } else if bilgi_balonu_görünürdü {
                    cx.notify();
                }
            }))
            .on_pinch(cx.listener(|bu, olay: &PinchEvent, _, cx| {
                let bilgi_balonu_görünürdü = bu.bilgi_balonu_beklemesini_iptal_et();
                let datum_değişti = bu.grafik.ölçüm_datumlarını_temizle();
                let görünüm_değişti = bu.dokunma_yakınlaştır(olay);
                if görünüm_değişti {
                    bu.görünüm_bildir(false, cx);
                } else if datum_değişti {
                    bu.grafik_bildir(cx);
                } else if bilgi_balonu_görünürdü {
                    cx.notify();
                }
            }))
            // uPlot cursor'ı `mouseleave` ile gizler. GPUI'de bunun karşılığı
            // `on_mouse_exit` değil: o olay yalnız fare **pencereyi** terk
            // ettiğinde üretilir ve `on_mouse_move` de hitbox ile filtrelidir,
            // yani yüzey sınırından çıkışta ikisi de çağrılmaz. `on_hover`
            // hareketi filtresiz dinleyip hover geçişini bildirir ve pencere
            // çıkışını da kapsar. Bu ayrım olmadan imleç çizgisi terk edilen
            // yüzeyde kalıyordu — `sparklines` tablosunda 20 küçük yüzeyin
            // her biri kendi çizgisini bırakıyordu.
            .on_hover(cx.listener(|bu, üzerinde: &bool, pencere, cx| {
                if *üzerinde || bu.imleç_kilitli {
                    return;
                }
                // Seçim ve taşıma sürerken fare yüzeyin dışına çıkabilir;
                // sürükleme kendi bırakma olayıyla temizlenir.
                if bu.seçim.is_some() || bu.taşıma_başlangıcı.is_some() {
                    return;
                }
                // `on_hover` hitbox'ı üst bir katman gölgelediğinde de `false`
                // bildirir; fare hâlâ yüzeyin üstündeyse imleci temizlemek
                // çizgiyi hareket ederken söndürür. Ölçülen alan tek
                // güvenilir ölçüttür.
                if bu
                    .çizim_sınırları
                    .get()
                    .is_some_and(|alan| alan.contains(&pencere.mouse_position()))
                {
                    return;
                }
                bu.bilgi_balonu_beklemesini_iptal_et();
                bu.imleç = None;
                bu.açıklama_vuruşu = None;
                bu.eksen_üzerinde = false;
                if bu.grafik.imleç_odağını_temizle() {
                    bu.veri_sahnesini_yenile(cx);
                }
                bu.etkileşim_yüzeyini_yenile(cx);
                cx.emit(GpuiGrafikOlayı::İmleçKonumuDeğişti);
                cx.emit(GpuiGrafikOlayı::İmleçDeğişti);
                cx.notify();
            }))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|bu, olay: &MouseDownEvent, window, cx| {
                    let bilgi_balonu_görünürdü = bu.bilgi_balonu_beklemesini_iptal_et();
                    let önceki_seçim = bu.seçim;
                    let önceki_taşıma = bu.taşıma_başlangıcı;
                    let önceki_eksen_sürükleniyordu = bu.grafik.eksen_sürükleniyor();
                    bu.tooltip_tıklaması_sürüklendi = false;
                    bu.tooltip_tıklama_başlangıcı =
                        bu.sahne_konumu(olay.position).and_then(|konum| {
                            if !bu.grafik_alanında(konum) {
                                return None;
                            }
                            bu.etkin_en_yakın_tooltip()
                                .map(|bilgi| (konum, bilgi.karşılaştırma_url))
                        });
                    let mut ana_sahne_değişti = false;
                    let mut görünüm_değişti = false;
                    if let Some(odak) = bu.odak.as_ref() {
                        odak.focus(window, cx);
                    }
                    let ayarlar = bu.grafik.etkileşim_seçenekleri();
                    let eksen_başladı = bu.sahne_konumu(olay.position).is_some_and(|konum| {
                        let (genişlik, yükseklik) = bu.grafik.boyut();
                        bu.grafik
                            .eksen_sürüklemeyi_başlat(genişlik, yükseklik, konum.x, konum.y)
                    });
                    if eksen_başladı {
                        bu.seçim = None;
                        bu.taşıma_başlangıcı = None;
                        bu.açıklama_seçimi = false;
                    } else if bu.boşluk_basılı
                        && let Some(konum) = bu.sahne_konumu(olay.position)
                        && bu.grafik_alanında(konum)
                        && bu.grafik.taşımayı_başlat()
                    {
                        bu.taşıma_başlangıcı = Some(konum);
                        bu.seçim = None;
                        bu.açıklama_seçimi = false;
                        bu.imleç = None;
                        bu.açıklama_vuruşu = None;
                    } else if olay.click_count >= 2 && ayarlar.çift_tıkla_tam_görünüm {
                        let datum_değişti = bu.grafik.ölçüm_datumlarını_temizle();
                        let tam_görünüm_değişti = bu.grafik.tam_görünüm();
                        ana_sahne_değişti = datum_değişti || tam_görünüm_değişti;
                        görünüm_değişti = tam_görünüm_değişti;
                        bu.seçim = None;
                        bu.açıklama_seçimi = false;
                    } else if ayarlar.seçim_yakınlaştır
                        && let Some(konum) = bu.sahne_konumu(olay.position)
                        && bu.grafik_alanında(konum)
                    {
                        let konum = bu.imleç_ızgarasına_oturt(konum).unwrap_or(konum);
                        bu.seçim = Some((konum, konum));
                        bu.açıklama_seçimi = ayarlar.imleç_bağları.ctrl_seçim_ölçeğini_durdur
                            && olay.modifiers.control;
                    }
                    if ana_sahne_değişti {
                        if görünüm_değişti {
                            bu.görünüm_bildir(false, cx);
                        } else {
                            bu.grafik_bildir(cx);
                        }
                    } else if önceki_seçim != bu.seçim
                        || önceki_taşıma != bu.taşıma_başlangıcı
                        || önceki_eksen_sürükleniyordu != bu.grafik.eksen_sürükleniyor()
                        || bilgi_balonu_görünürdü
                    {
                        bu.etkileşim_yüzeyini_yenile(cx);
                        cx.notify();
                    }
                }),
            )
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|bu, olay: &MouseUpEvent, _, cx| {
                    let sürüklendi = std::mem::take(&mut bu.tooltip_tıklaması_sürüklendi);
                    let tooltip_tıklaması = if sürüklendi {
                        bu.tooltip_tıklama_başlangıcı = None;
                        None
                    } else {
                        bu.tooltip_tıklama_başlangıcı
                            .take()
                            .and_then(|(başlangıç, url)| {
                                let bitiş = bu.sahne_konumu(olay.position)?;
                                let aynı_konum = başlangıç.x == bitiş.x && başlangıç.y == bitiş.y;
                                let aynı_url = bu
                                    .etkin_en_yakın_tooltip()
                                    .is_some_and(|bilgi| bilgi.karşılaştırma_url == url);
                                (aynı_konum && aynı_url).then_some(url)
                            })
                    };
                    if bu.grafik.eksen_sürükleniyor() {
                        bu.grafik.eksen_sürüklemeyi_bitir();
                        cx.notify();
                        return;
                    }
                    if bu.taşıma_başlangıcı.take().is_some() {
                        bu.grafik.taşımayı_bitir();
                        cx.notify();
                        return;
                    }
                    let seçim_vardı = bu.seçim.is_some();
                    let ana_sahne_değişti = bu.seçimi_tamamla(cx);
                    if seçim_vardı {
                        bu.etkileşim_yüzeyini_yenile(cx);
                    }
                    if ana_sahne_değişti {
                        bu.görünüm_bildir(true, cx);
                    } else if seçim_vardı {
                        cx.notify();
                    }
                    if let Some(url) = tooltip_tıklaması {
                        cx.open_url(&url);
                    }
                }),
            )
            .on_mouse_up_out(
                MouseButton::Left,
                cx.listener(|bu, _: &MouseUpEvent, _, cx| {
                    bu.tooltip_tıklama_başlangıcı = None;
                    bu.tooltip_tıklaması_sürüklendi = false;
                    let mut durum_değişti = false;
                    if bu.grafik.eksen_sürükleniyor() {
                        bu.grafik.eksen_sürüklemeyi_bitir();
                        bu.eksen_üzerinde = false;
                        durum_değişti = true;
                    }
                    if bu.taşıma_başlangıcı.take().is_some() {
                        bu.grafik.taşımayı_bitir();
                        durum_değişti = true;
                    }
                    let seçim_vardı = bu.seçim.is_some();
                    let görünüm_değişti = bu.seçimi_tamamla(cx);
                    if görünüm_değişti {
                        bu.etkileşim_yüzeyini_yenile(cx);
                        bu.görünüm_bildir(true, cx);
                    } else if durum_değişti || seçim_vardı {
                        bu.etkileşim_yüzeyini_yenile(cx);
                        cx.notify();
                    }
                }),
            )
            .children(çizim_katmanları)
    }
}

/// Ortak sahne komutlarını GPUI canvas üzerine boyar.
pub fn sahneyi_boya(
    sahne: &Sahne,
    sınırlar: Bounds<Pixels>,
    pencere: &mut Window,
    uygulama: &mut App,
) {
    let mut yol_önbelleği = GpuiYolÖnbelleği::default();
    sahneyi_önbellekli_boya(
        sahne,
        sınırlar,
        &mut yol_önbelleği,
        None,
        None,
        pencere,
        uygulama,
    );
}

fn komut_çizim_alanında_mı(komut: &Komut, (sol, sağ, üst, alt): (f32, f32, f32, f32)) -> bool {
    if matches!(komut, Komut::ArkaPlan { .. }) {
        return false;
    }
    let içeride = |x: f32, y: f32| x >= sol && x <= sağ && y >= üst && y <= alt;
    let kesişiyor = |en_sol: f32, en_sağ: f32, en_üst: f32, en_alt: f32| {
        en_sağ >= sol && en_sol <= sağ && en_alt >= üst && en_üst <= alt
    };
    match komut {
        Komut::Çizgi {
            başlangıç, bitiş,
        ..
        }
        | Komut::KesikliÇizgi {
            başlangıç, bitiş,
        ..
        } => kesişiyor(
            başlangıç.x.min(bitiş.x),
            başlangıç.x.max(bitiş.x),
            başlangıç.y.min(bitiş.y),
            başlangıç.y.max(bitiş.y),
        ),
        Komut::Yol { parçalar, .. }
        | Komut::GradyanYol { parçalar, .. }
        | Komut::KesikliYol { parçalar, .. } => nokta_sınırları(parçalar.iter().flatten())
            .is_some_and(|(a, b, c, d)| kesişiyor(a, b, c, d)),
        Komut::Alan { çokgenler, .. } | Komut::GradyanAlan { çokgenler, .. } => {
            nokta_sınırları(çokgenler.iter().flatten())
                .is_some_and(|(a, b, c, d)| kesişiyor(a, b, c, d))
        }
        Komut::Daire {
            merkez, yarıçap, ..
        } => kesişiyor(
            merkez.x - yarıçap,
            merkez.x + yarıçap,
            merkez.y - yarıçap,
            merkez.y + yarıçap,
        ),
        Komut::Daireler {
            merkezler, yarıçap,
        ..
        } => nokta_sınırları(merkezler.iter()).is_some_and(|(a, b, c, d)| {
            kesişiyor(a - yarıçap, b + yarıçap, c - yarıçap, d + yarıçap)
        }),
        Komut::DeğişkenDaireler { daireler, .. } => daireler.iter().any(|(merkez, yarıçap)| {
            kesişiyor(
                merkez.x - yarıçap,
                merkez.x + yarıçap,
                merkez.y - yarıçap,
                merkez.y + yarıçap,
            )
        }),
        Komut::Dikdörtgen {
            konum,
            genişlik,
            yükseklik,
            ..
        }
        | Komut::YuvarlatılmışDikdörtgen {
            konum,
            genişlik,
            yükseklik,
            ..
        } => kesişiyor(
            konum.x.min(konum.x + genişlik),
            konum.x.max(konum.x + genişlik),
            konum.y.min(konum.y + yükseklik),
            konum.y.max(konum.y + yükseklik),
        ),
        Komut::Metin { konum, .. } | Komut::DöndürülmüşMetin { konum, .. } => {
            içeride(konum.x, konum.y)
        }
        Komut::ArkaPlan { .. } => false,
    }
}

fn nokta_sınırları<'a>(
    noktalar: impl Iterator<Item = &'a Nokta>,
) -> Option<(f32, f32, f32, f32)> {
    noktalar.fold(None, |sınırlar, nokta| {
        Some(match sınırlar {
            None => (nokta.x, nokta.x, nokta.y, nokta.y),
            Some((sol, sağ, üst, alt)) => (
                sol.min(nokta.x),
                sağ.max(nokta.x),
                üst.min(nokta.y),
                alt.max(nokta.y),
            ),
        })
    })
}

fn retained_yolu_boya(
    yol: BoyanabilirGpuiYol,
    boya: impl Into<::gpui::Background>,
    görünüm: Option<GpuiBoyaGörünümü>,
    hedef_köken: ::gpui::Point<Pixels>,
    pencere: &mut Window,
) {
    crate::izleme::yol_boyandı(yol.yol.vertices.len());
    let boya = boya.into();
    boya_günlüğü::yaz(boya);
    let yerleşik = yol.yol;
    if let Some(mut görünüm) = görünüm {
        görünüm.kesme_sınırları.origin += hedef_köken;
        pencere.with_content_mask(
            Some(ContentMask {
                bounds: görünüm.kesme_sınırları,
            }),
            |pencere| {
                pencere.paint_path(yerleşik, boya);
            },
        );
    } else {
        pencere.paint_path(yerleşik, boya);
    }
}

/// Yüzeyi tek sprite olarak boyar; uygun değilse `false` döner.
///
/// Uygunluk iki koşula bağlı: köşe bütçesinin aşılması ve sahnenin kayıpsız
/// rasterleştirilebilmesi. İkincisi sağlanmazsa yüzey vektör yolunda kalır,
/// yani politika kart bazlı değil ölçüm bazlıdır ve yeni kartlar da
/// kendiliğinden kapsanır.
fn raster_yüzeyi_boya(
    sahne: &Sahne,
    sınırlar: Bounds<Pixels>,
    yol_önbelleği: &mut GpuiYolÖnbelleği,
    çizim_kırpması: Option<(f32, f32, f32, f32)>,
    pencere: &mut Window,
) -> bool {
    let fiziksel_ölçek = pencere.scale_factor();
    let fiziksel_genişlik = (f32::from(sınırlar.size.width) * fiziksel_ölçek)
        .round()
        .max(0.0) as u32;
    let fiziksel_yükseklik = (f32::from(sınırlar.size.height) * fiziksel_ölçek)
        .round()
        .max(0.0) as u32;
    if fiziksel_genişlik == 0 || fiziksel_yükseklik == 0 {
        return false;
    }

    let önbellekte = yol_önbelleği
        .raster
        .as_ref()
        .filter(|(g, y, _)| *g == fiziksel_genişlik && *y == fiziksel_yükseklik)
        .map(|(_, _, görsel)| Arc::clone(görsel));

    let görsel = match önbellekte {
        Some(görsel) => görsel,
        None => {
            let Some(nokta) = raster::nokta_sayısı_rasterlenebilirse(sahne) else {
                return false;
            };
            if nokta < raster::RASTER_NOKTA_EŞİĞİ {
                return false;
            }
            let (kaynak_g, kaynak_y) = sahne.boyut();
            let dönüşüm = GpuiYüzeyDönüşümü::hesapla(
                kaynak_g,
                kaynak_y,
                0.0,
                0.0,
                f32::from(sınırlar.size.width),
                f32::from(sınırlar.size.height),
            );
            let üretilen = {
                let renkler = &mut *yol_önbelleği;
                raster::rasterleştir(
                    sahne,
                    fiziksel_genişlik,
                    fiziksel_yükseklik,
                    dönüşüm.ölçek * fiziksel_ölçek,
                    |kod| renkler.renk(kod),
                )
            };
            let Some(üretilen) = üretilen else {
                return false;
            };
            yol_önbelleği.raster =
                Some((fiziksel_genişlik, fiziksel_yükseklik, Arc::clone(&üretilen)));
            üretilen
        }
    };

    let boya = |pencere: &mut Window| {
        // `image_bounds` görselin pencere içindeki yerleşim dikdörtgeni,
        // `bounds` ise görünür kırpma; ikisi de yüzeyin kendisi. Yerleşimi
        // pencere kökenine vermek sprite'ı sol üste çakar.
        let _ = pencere.paint_image(sınırlar, sınırlar, Corners::default(), görsel, 0, false);
    };
    if let Some((k_sol, k_sağ, k_üst, k_alt)) = çizim_kırpması {
        let (kaynak_g, kaynak_y) = sahne.boyut();
        let dönüşüm = GpuiYüzeyDönüşümü::hesapla(
            kaynak_g,
            kaynak_y,
            f32::from(sınırlar.origin.x),
            f32::from(sınırlar.origin.y),
            f32::from(sınırlar.size.width),
            f32::from(sınırlar.size.height),
        );
        pencere.with_content_mask(
            Some(ContentMask {
                bounds: Bounds::new(
                    point(
                        px(dönüşüm.köken_x + k_sol * dönüşüm.ölçek),
                        px(dönüşüm.köken_y + k_üst * dönüşüm.ölçek),
                    ),
                    size(
                        px((k_sağ - k_sol) * dönüşüm.ölçek),
                        px((k_alt - k_üst) * dönüşüm.ölçek),
                    ),
                ),
            }),
            boya,
        );
    } else {
        boya(pencere);
    }
    true
}

fn sahneyi_önbellekli_boya(
    sahne: &Sahne,
    sınırlar: Bounds<Pixels>,
    yol_önbelleği: &mut GpuiYolÖnbelleği,
    veri_görünümü: Option<GpuiVeriGörünümü>,
    // uPlot imleç elemanlarını `.u-over` içinde tutar; o kap tam çizim
    // dikdörtgeni ve `overflow: hidden`. Kırpma verilirse bu yüzeyin tüm
    // komutları ızgara sınırına hapsedilir.
    çizim_kırpması: Option<(f32, f32, f32, f32)>,
    pencere: &mut Window,
    uygulama: &mut App,
) {
    let _ölçüm = crate::izleme::Ölçüm::başlat(crate::izleme::Yuva::YüzeyBoyama);
    // GPUI cached-view anahtarı kaydırma sırasında bounds/content-mask ile
    // değişebilir. Yüzey bütünüyle görünür alanın dışındaysa retained komutları
    // yeniden sahneye eklemek hiçbir piksel üretemez; özellikle aynı sayfadaki
    // çok yüzeyli örneklerde bu kısa devre off-screen grafik maliyetini kaldırır.
    if sınırlar
        .intersect(&pencere.content_mask().bounds)
        .is_empty()
    {
        return;
    }
    yol_önbelleği.yüzeyi_hazırla(sahne, sınırlar);
    let (kaynak_g, kaynak_y) = sahne.boyut();
    let dönüşüm = GpuiYüzeyDönüşümü::hesapla(
        kaynak_g,
        kaynak_y,
        f32::from(sınırlar.origin.x),
        f32::from(sınırlar.origin.y),
        f32::from(sınırlar.size.width),
        f32::from(sınırlar.size.height),
    );
    let ölçek = dönüşüm.ölçek;
    let köken_x = dönüşüm.köken_x;
    let köken_y = dönüşüm.köken_y;
    let dönüştür = |nokta: Nokta| {
        point(
            px(dönüşüm.köken_x + nokta.x * dönüşüm.ölçek),
            px(dönüşüm.köken_y + nokta.y * dönüşüm.ölçek),
        )
    };
    // Retained yollar pencere kökeninden bağımsız, yüzey-yerel
    // koordinatlarda tessellate edilir. Scroll yalnız aşağıdaki son GPU
    // dönüşümünün hedef kökenini değiştirir.
    let yerel_dönüşüm = GpuiYüzeyDönüşümü::hesapla(
        kaynak_g,
        kaynak_y,
        0.0,
        0.0,
        f32::from(sınırlar.size.width),
        f32::from(sınırlar.size.height),
    );
    let yolu_dönüştür = |nokta: Nokta| {
        point(
            px(yerel_dönüşüm.köken_x + nokta.x * yerel_dönüşüm.ölçek),
            px(yerel_dönüşüm.köken_y + nokta.y * yerel_dönüşüm.ölçek),
        )
    };
    // uPlot'un canvas'ı `_commit()` başına bir kez çizip piksellerini korur;
    // GPUI ise her karede tüm geometriyi yeniden gönderir. Köşe bütçesini
    // aşan ve kayıpsız rasterleştirilebilen yüzeyler tek sprite'a indirilerek
    // aynı davranışa getirilir.
    if raster_yüzeyi_boya(sahne, sınırlar, yol_önbelleği, çizim_kırpması, pencere) {
        return;
    }
    let pay = kırpma_payı(sahne);
    let boya_görünümü =
        veri_görünümü.and_then(|görünüm| GpuiBoyaGörünümü::hesapla(görünüm, dönüşüm, pay));
    let yol_boya_görünümü = veri_görünümü
        .and_then(|görünüm| GpuiBoyaGörünümü::hesapla(görünüm, yerel_dönüşüm, pay));
    let hedef_köken = sınırlar.origin;
    if let Some(görünüm) = boya_görünümü {
        yol_önbelleği.veri_komutlarını_hazırla(sahne, görünüm.mantıksal_çizim_alanı);
    }

    let çizim_maskesi = çizim_kırpması.map(|(k_sol, k_sağ, k_üst, k_alt)| ContentMask {
        bounds: Bounds::new(
            point(
                px(köken_x + (k_sol - pay) * ölçek),
                px(köken_y + (k_üst - pay) * ölçek),
            ),
            size(
                px((k_sağ - k_sol + pay * 2.0) * ölçek),
                px((k_alt - k_üst + pay * 2.0) * ölçek),
            ),
        ),
    });
    pencere.with_content_mask(çizim_maskesi, |pencere| {
        for (komut_indeksi, komut) in sahne.komutlar().iter().enumerate() {
            let veri_komutu = yol_önbelleği
                .veri_komutları
                .get(komut_indeksi)
                .copied()
                .unwrap_or(false);
            let komut_görünümü = boya_görünümü.filter(|_| veri_komutu);
            let yol_komut_görünümü = yol_boya_görünümü.filter(|_| veri_komutu);
            match komut {
                Komut::ArkaPlan { renk } => {
                    pencere.paint_quad(quad(
                        Bounds::new(
                            point(px(köken_x), px(köken_y)),
                            size(px(kaynak_g as f32 * ölçek), px(kaynak_y as f32 * ölçek)),
                        ),
                        px(0.0),
                        yol_önbelleği.renk(renk),
                        px(0.0),
                        rgba(0x00000000),
                        BorderStyle::default(),
                    ));
                }
                Komut::Çizgi {
                    başlangıç,
                    bitiş,
                    renk,
                    kalınlık,
                } => {
                    if let Some(yol) = yol_önbelleği.yol(komut_indeksi, hedef_köken, || {
                        let mut yol = PathBuilder::stroke(px(*kalınlık * ölçek));
                        yol.move_to(yolu_dönüştür(*başlangıç));
                        yol.line_to(yolu_dönüştür(*bitiş));
                        yol.build().ok()
                    }) {
                        retained_yolu_boya(
                            yol,
                            yol_önbelleği.renk(renk),
                            yol_komut_görünümü,
                            hedef_köken,
                            pencere,
                        );
                    }
                }
                Komut::KesikliÇizgi {
                    başlangıç,
                    bitiş,
                    renk,
                    kalınlık,
                    kesik,
                } => {
                    if let Some(yol) = yol_önbelleği.yol(komut_indeksi, hedef_köken, || {
                        let mut yol = PathBuilder::stroke(px(*kalınlık * ölçek))
                            .dash_array(&[px(*kesik * ölçek), px(*kesik * ölçek)]);
                        yol.move_to(yolu_dönüştür(*başlangıç));
                        yol.line_to(yolu_dönüştür(*bitiş));
                        yol.build().ok()
                    }) {
                        retained_yolu_boya(
                            yol,
                            yol_önbelleği.renk(renk),
                            yol_komut_görünümü,
                            hedef_köken,
                            pencere,
                        );
                    }
                }
                Komut::Yol {
                    parçalar,
                    renk,
                    kalınlık,
                } => {
                    if let Some(yol) = yol_önbelleği.yol(komut_indeksi, hedef_köken, || {
                        let mut yol = PathBuilder::stroke(px(*kalınlık * ölçek));
                        for parça in parçalar {
                            let mut noktalar = parça.iter();
                            if let Some(ilk) = noktalar.next() {
                                yol.move_to(yolu_dönüştür(*ilk));
                            }
                            for nokta in noktalar {
                                yol.line_to(yolu_dönüştür(*nokta));
                            }
                        }
                        yol.build().ok()
                    }) {
                        retained_yolu_boya(
                            yol,
                            yol_önbelleği.renk(renk),
                            yol_komut_görünümü,
                            hedef_köken,
                            pencere,
                        );
                    }
                }
                Komut::GradyanYol {
                    parçalar,
                    gradyan,
                    kalınlık,
                } => {
                    if let Some(yol) = yol_önbelleği.yol(komut_indeksi, hedef_köken, || {
                        let mut yol = PathBuilder::stroke(px(*kalınlık * ölçek));
                        for parça in parçalar {
                            let mut noktalar = parça.iter();
                            if let Some(ilk) = noktalar.next() {
                                yol.move_to(yolu_dönüştür(*ilk));
                            }
                            for nokta in noktalar {
                                yol.line_to(yolu_dönüştür(*nokta));
                            }
                        }
                        yol.build().ok()
                    }) {
                        gradyan_yolunu_boya(
                            yol,
                            gradyan,
                            &yolu_dönüştür,
                            yol_komut_görünümü,
                            yol_önbelleği,
                            hedef_köken,
                            pencere,
                        );
                    }
                }
                Komut::KesikliYol {
                    parçalar,
                    renk,
                    kalınlık,
                    çizgi,
                    boşluk,
                } => {
                    if let Some(yol) = yol_önbelleği.yol(komut_indeksi, hedef_köken, || {
                        let mut yol = PathBuilder::stroke(px(*kalınlık * ölçek))
                            .dash_array(&[px(*çizgi * ölçek), px(*boşluk * ölçek)]);
                        for parça in parçalar {
                            let mut noktalar = parça.iter();
                            if let Some(ilk) = noktalar.next() {
                                yol.move_to(yolu_dönüştür(*ilk));
                            }
                            for nokta in noktalar {
                                yol.line_to(yolu_dönüştür(*nokta));
                            }
                        }
                        yol.build().ok()
                    }) {
                        retained_yolu_boya(
                            yol,
                            yol_önbelleği.renk(renk),
                            yol_komut_görünümü,
                            hedef_köken,
                            pencere,
                        );
                    }
                }
                Komut::Alan { çokgenler, dolgu } => {
                    if let Some(yol) = yol_önbelleği.yol(komut_indeksi, hedef_köken, || {
                        let mut yol = PathBuilder::fill();
                        for çokgen in çokgenler {
                            let mut noktalar = çokgen.iter();
                            if let Some(ilk) = noktalar.next() {
                                yol.move_to(yolu_dönüştür(*ilk));
                            }
                            for nokta in noktalar {
                                yol.line_to(yolu_dönüştür(*nokta));
                            }
                            if çokgen.len() >= 3 {
                                yol.close();
                            }
                        }
                        yol.build().ok()
                    }) {
                        retained_yolu_boya(
                            yol,
                            yol_önbelleği.renk(dolgu),
                            yol_komut_görünümü,
                            hedef_köken,
                            pencere,
                        );
                    }
                }
                Komut::GradyanAlan {
                    çokgenler, gradyan
                } => {
                    if let Some(yol) = yol_önbelleği.yol(komut_indeksi, hedef_köken, || {
                        let mut yol = PathBuilder::fill();
                        for çokgen in çokgenler {
                            let mut noktalar = çokgen.iter();
                            if let Some(ilk) = noktalar.next() {
                                yol.move_to(yolu_dönüştür(*ilk));
                            }
                            for nokta in noktalar {
                                yol.line_to(yolu_dönüştür(*nokta));
                            }
                            if çokgen.len() >= 3 {
                                yol.close();
                            }
                        }
                        yol.build().ok()
                    }) {
                        gradyan_yolunu_boya(
                            yol,
                            gradyan,
                            &yolu_dönüştür,
                            yol_komut_görünümü,
                            yol_önbelleği,
                            hedef_köken,
                            pencere,
                        );
                    }
                }
                Komut::Daire {
                    merkez,
                    yarıçap,
                    dolgu,
                    çizgi,
                    kalınlık,
                } => {
                    let dolgu_yolu = yol_önbelleği.yol(komut_indeksi, hedef_köken, || {
                        let merkez = yolu_dönüştür(*merkez);
                        let yarıçap = px(*yarıçap * ölçek);
                        let yarıçaplar = point(yarıçap, yarıçap);
                        let sol = point(merkez.x - yarıçap, merkez.y);
                        let sağ = point(merkez.x + yarıçap, merkez.y);
                        let mut yol = PathBuilder::fill();
                        yol.move_to(sol);
                        yol.arc_to(yarıçaplar, px(0.0), false, true, sağ);
                        yol.arc_to(yarıçaplar, px(0.0), false, true, sol);
                        yol.close();
                        yol.build().ok()
                    });
                    let çizgi_yolu = (*kalınlık > 0.0)
                        .then(|| {
                            yol_önbelleği.ikincil_yol(komut_indeksi, hedef_köken, || {
                                let merkez = yolu_dönüştür(*merkez);
                                let yarıçap = px(*yarıçap * ölçek);
                                let yarıçaplar = point(yarıçap, yarıçap);
                                let sol = point(merkez.x - yarıçap, merkez.y);
                                let sağ = point(merkez.x + yarıçap, merkez.y);
                                let mut yol = PathBuilder::stroke(px(*kalınlık * ölçek));
                                yol.move_to(sol);
                                yol.arc_to(yarıçaplar, px(0.0), false, true, sağ);
                                yol.arc_to(yarıçaplar, px(0.0), false, true, sol);
                                yol.close();
                                yol.build().ok()
                            })
                        })
                        .flatten();
                    if let Some(dolgu_yolu) = dolgu_yolu {
                        retained_yolu_boya(
                            dolgu_yolu,
                            yol_önbelleği.renk(dolgu),
                            yol_komut_görünümü,
                            hedef_köken,
                            pencere,
                        );
                    }
                    if let Some(çizgi_yolu) = çizgi_yolu {
                        retained_yolu_boya(
                            çizgi_yolu,
                            yol_önbelleği.renk(çizgi),
                            yol_komut_görünümü,
                            hedef_köken,
                            pencere,
                        );
                    }
                }
                Komut::Daireler {
                    merkezler,
                    yarıçap,
                    dolgu,
                    çizgi,
                    kalınlık,
                    kesme_sınırları,
                } => {
                    let dolgu_yolu = yol_önbelleği.yol(komut_indeksi, hedef_köken, || {
                        let mut yol = PathBuilder::fill();
                        let yarıçap = px(*yarıçap * ölçek);
                        let yarıçaplar = point(yarıçap, yarıçap);
                        for merkez in merkezler {
                            let merkez = yolu_dönüştür(*merkez);
                            let sol = point(merkez.x - yarıçap, merkez.y);
                            let sağ = point(merkez.x + yarıçap, merkez.y);
                            yol.move_to(sol);
                            yol.arc_to(yarıçaplar, px(0.0), false, true, sağ);
                            yol.arc_to(yarıçaplar, px(0.0), false, true, sol);
                            yol.close();
                        }
                        yol.build().ok()
                    });
                    let çizgi_yolu = (*kalınlık > 0.0)
                        .then(|| {
                            yol_önbelleği.ikincil_yol(komut_indeksi, hedef_köken, || {
                                let mut yol = PathBuilder::stroke(px(*kalınlık * ölçek));
                                let yarıçap = px(*yarıçap * ölçek);
                                let yarıçaplar = point(yarıçap, yarıçap);
                                for merkez in merkezler {
                                    let merkez = yolu_dönüştür(*merkez);
                                    let sol = point(merkez.x - yarıçap, merkez.y);
                                    let sağ = point(merkez.x + yarıçap, merkez.y);
                                    yol.move_to(sol);
                                    yol.arc_to(yarıçaplar, px(0.0), false, true, sağ);
                                    yol.arc_to(yarıçaplar, px(0.0), false, true, sol);
                                    yol.close();
                                }
                                yol.build().ok()
                            })
                        })
                        .flatten();
                    if let Some(dolgu_yolu) = dolgu_yolu {
                        let dolgu_boyası = yol_önbelleği.renk(dolgu);
                        let çizgi_boyası = yol_önbelleği.renk(çizgi);
                        if let Some((başlangıç, bitiş)) = kesme_sınırları {
                            let mut başlangıç = dönüştür(*başlangıç);
                            let mut bitiş = dönüştür(*bitiş);
                            if let Some(görünüm) = komut_görünümü {
                                başlangıç = görünüm.noktayı_dönüştür(başlangıç);
                                bitiş = görünüm.noktayı_dönüştür(bitiş);
                            }
                            let sol = başlangıç.x.min(bitiş.x);
                            let üst = başlangıç.y.min(bitiş.y);
                            let sınırlar = Bounds::new(
                                point(sol, üst),
                                size(
                                    başlangıç.x.max(bitiş.x) - sol,
                                    başlangıç.y.max(bitiş.y) - üst,
                                ),
                            );
                            pencere.with_content_mask(
                                Some(ContentMask { bounds: sınırlar }),
                                |pencere| {
                                    retained_yolu_boya(
                                        dolgu_yolu.clone(),
                                        dolgu_boyası,
                                        yol_komut_görünümü,
                                        hedef_köken,
                                        pencere,
                                    );
                                    if let Some(çizgi_yolu) = çizgi_yolu.as_ref() {
                                        retained_yolu_boya(
                                            çizgi_yolu.clone(),
                                            çizgi_boyası,
                                            yol_komut_görünümü,
                                            hedef_köken,
                                            pencere,
                                        );
                                    }
                                },
                            );
                        } else {
                            retained_yolu_boya(
                                dolgu_yolu,
                                dolgu_boyası,
                                yol_komut_görünümü,
                                hedef_köken,
                                pencere,
                            );
                            if let Some(çizgi_yolu) = çizgi_yolu {
                                retained_yolu_boya(
                                    çizgi_yolu,
                                    çizgi_boyası,
                                    yol_komut_görünümü,
                                    hedef_köken,
                                    pencere,
                                );
                            }
                        }
                    }
                }
                Komut::DeğişkenDaireler {
                    daireler,
                    dolgu,
                    çizgi,
                    kalınlık,
                    kesme_sınırları,
                } => {
                    let dolgu_yolu = yol_önbelleği.yol(komut_indeksi, hedef_köken, || {
                        let mut yol = PathBuilder::fill();
                        for (merkez, yarıçap) in daireler {
                            let merkez = yolu_dönüştür(*merkez);
                            let yarıçap = px(*yarıçap * ölçek);
                            let yarıçaplar = point(yarıçap, yarıçap);
                            let sol = point(merkez.x - yarıçap, merkez.y);
                            let sağ = point(merkez.x + yarıçap, merkez.y);
                            yol.move_to(sol);
                            yol.arc_to(yarıçaplar, px(0.0), false, true, sağ);
                            yol.arc_to(yarıçaplar, px(0.0), false, true, sol);
                            yol.close();
                        }
                        yol.build().ok()
                    });
                    let çizgi_yolu = (*kalınlık > 0.0)
                        .then(|| {
                            yol_önbelleği.ikincil_yol(komut_indeksi, hedef_köken, || {
                                let mut yol = PathBuilder::stroke(px(*kalınlık * ölçek));
                                for (merkez, yarıçap) in daireler {
                                    let merkez = yolu_dönüştür(*merkez);
                                    let yarıçap = px(*yarıçap * ölçek);
                                    let yarıçaplar = point(yarıçap, yarıçap);
                                    let sol = point(merkez.x - yarıçap, merkez.y);
                                    let sağ = point(merkez.x + yarıçap, merkez.y);
                                    yol.move_to(sol);
                                    yol.arc_to(yarıçaplar, px(0.0), false, true, sağ);
                                    yol.arc_to(yarıçaplar, px(0.0), false, true, sol);
                                    yol.close();
                                }
                                yol.build().ok()
                            })
                        })
                        .flatten();
                    if let Some(dolgu_yolu) = dolgu_yolu {
                        let dolgu_boyası = yol_önbelleği.renk(dolgu);
                        let çizgi_boyası = yol_önbelleği.renk(çizgi);
                        if let Some((başlangıç, bitiş)) = kesme_sınırları {
                            let mut başlangıç = dönüştür(*başlangıç);
                            let mut bitiş = dönüştür(*bitiş);
                            if let Some(görünüm) = komut_görünümü {
                                başlangıç = görünüm.noktayı_dönüştür(başlangıç);
                                bitiş = görünüm.noktayı_dönüştür(bitiş);
                            }
                            let sol = başlangıç.x.min(bitiş.x);
                            let üst = başlangıç.y.min(bitiş.y);
                            let sınırlar = Bounds::new(
                                point(sol, üst),
                                size(
                                    başlangıç.x.max(bitiş.x) - sol,
                                    başlangıç.y.max(bitiş.y) - üst,
                                ),
                            );
                            pencere.with_content_mask(
                                Some(ContentMask { bounds: sınırlar }),
                                |pencere| {
                                    retained_yolu_boya(
                                        dolgu_yolu.clone(),
                                        dolgu_boyası,
                                        yol_komut_görünümü,
                                        hedef_köken,
                                        pencere,
                                    );
                                    if let Some(çizgi_yolu) = çizgi_yolu.as_ref() {
                                        retained_yolu_boya(
                                            çizgi_yolu.clone(),
                                            çizgi_boyası,
                                            yol_komut_görünümü,
                                            hedef_köken,
                                            pencere,
                                        );
                                    }
                                },
                            );
                        } else {
                            retained_yolu_boya(
                                dolgu_yolu,
                                dolgu_boyası,
                                yol_komut_görünümü,
                                hedef_köken,
                                pencere,
                            );
                            if let Some(çizgi_yolu) = çizgi_yolu {
                                retained_yolu_boya(
                                    çizgi_yolu,
                                    çizgi_boyası,
                                    yol_komut_görünümü,
                                    hedef_köken,
                                    pencere,
                                );
                            }
                        }
                    }
                }
                Komut::Dikdörtgen {
                    konum,
                    genişlik,
                    yükseklik,
                    dolgu,
                    çizgi,
                    kalınlık,
                } => {
                    let mut konum = dönüştür(*konum);
                    let mut boyut = size(px(*genişlik * ölçek), px(*yükseklik * ölçek));
                    if let Some(görünüm) = komut_görünümü {
                        konum = görünüm.noktayı_dönüştür(konum);
                        boyut.width *= görünüm.x_ölçeği;
                        boyut.height *= görünüm.y_ölçeği;
                    }
                    let mut boya = |pencere: &mut Window| {
                        pencere.paint_quad(quad(
                            Bounds::new(konum, boyut),
                            px(0.0),
                            yol_önbelleği.renk(dolgu),
                            px(*kalınlık * ölçek),
                            yol_önbelleği.renk(çizgi),
                            BorderStyle::default(),
                        ));
                    };
                    if let Some(görünüm) = komut_görünümü {
                        pencere.with_content_mask(
                            Some(ContentMask {
                                bounds: görünüm.kesme_sınırları,
                            }),
                            boya,
                        );
                    } else {
                        boya(pencere);
                    }
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
                    let mut konum = dönüştür(*konum);
                    let mut boyut = size(px(*genişlik * ölçek), px(*yükseklik * ölçek));
                    let yarıçap_ölçeği = if let Some(görünüm) = komut_görünümü {
                        konum = görünüm.noktayı_dönüştür(konum);
                        boyut.width *= görünüm.x_ölçeği;
                        boyut.height *= görünüm.y_ölçeği;
                        görünüm.x_ölçeği.min(görünüm.y_ölçeği)
                    } else {
                        1.0
                    };
                    let mut boya = |pencere: &mut Window| {
                        pencere.paint_quad(quad(
                            Bounds::new(konum, boyut),
                            Corners {
                                top_left: px(yarıçaplar.üst_sol * ölçek * yarıçap_ölçeği),
                                top_right: px(yarıçaplar.üst_sağ * ölçek * yarıçap_ölçeği),
                                bottom_right: px(yarıçaplar.alt_sağ * ölçek * yarıçap_ölçeği),
                                bottom_left: px(yarıçaplar.alt_sol * ölçek * yarıçap_ölçeği),
                            },
                            yol_önbelleği.renk(dolgu),
                            px(*kalınlık * ölçek),
                            yol_önbelleği.renk(çizgi),
                            BorderStyle::default(),
                        ));
                    };
                    if let Some(görünüm) = komut_görünümü {
                        pencere.with_content_mask(
                            Some(ContentMask {
                                bounds: görünüm.kesme_sınırları,
                            }),
                            boya,
                        );
                    } else {
                        boya(pencere);
                    }
                }
                Komut::Metin {
                    konum,
                    içerik,
                    renk,
                    boyut,
                    hiza,
                } => {
                    // GPUI `shape_line` çok satırlı metni panic ile reddeder. Sahne
                    // kaynağı dış veri/başlık içerebildiğinden adaptör sınırında
                    // satır sonlarını güvenli tek satır boşluğuna dönüştürürüz.
                    let (metin_kimliği, metin_uzunluğu) = tek_satır_metin_kimliği(içerik);
                    let koşu = TextRun {
                        len: metin_uzunluğu,
                        font: pencere.text_style().font(),
                        color: yol_önbelleği.renk(renk),
                        background_color: None,
                        underline: None,
                        strikethrough: None,
                    };
                    let çizgi = pencere.text_system().shape_line_by_hash(
                        metin_kimliği,
                        metin_uzunluğu,
                        px(*boyut * ölçek),
                        &[koşu],
                        None,
                        || SharedString::from(içerik.replace(['\r', '\n'], " ")),
                    );
                    let genişlik = f32::from(çizgi.width());
                    let mut dayanak = dönüştür(*konum);
                    if let Some(görünüm) = komut_görünümü {
                        dayanak = görünüm.noktayı_dönüştür(dayanak);
                    }
                    let x = match hiza {
                        MetinHizası::Başlangıç => f32::from(dayanak.x),
                        MetinHizası::Orta => f32::from(dayanak.x) - genişlik / 2.0,
                        MetinHizası::Bitiş => f32::from(dayanak.x) - genişlik,
                    };
                    let başlangıç = point(px(x), dayanak.y - px(*boyut * ölçek));
                    let mut boya = |pencere: &mut Window| {
                        let _ = çizgi.paint(
                            başlangıç,
                            px(*boyut * 1.25 * ölçek),
                            TextAlign::Left,
                            None,
                            pencere,
                            uygulama,
                        );
                    };
                    if let Some(görünüm) = komut_görünümü {
                        pencere.with_content_mask(
                            Some(ContentMask {
                                bounds: görünüm.kesme_sınırları,
                            }),
                            boya,
                        );
                    } else {
                        boya(pencere);
                    }
                }
                Komut::DöndürülmüşMetin {
                    konum,
                    içerik,
                    renk,
                    boyut,
                    ..
                } => {
                    // GPUI 0.2 metin primitifinde dönüşüm yoktur. Glifleri tek tek
                    // dikey bir satırda boyamak, eksen etiketini ayrı bir DOM/öğe
                    // ağına çevirmeden aynı hafif sahne katmanında tutar.
                    let karakter_sayısı = içerik.chars().count();
                    let adım = *boyut * 0.9;
                    let başlangıç_y =
                        konum.y - (karakter_sayısı.saturating_sub(1) as f32 * adım) / 2.0;
                    // Yazı tipi ve renk etiket boyunca sabittir; karakter başına
                    // `text_style()` klonlamak ve renk tablosunu yeniden aramak
                    // yalnız glif sayısı kadar tekrar üretir.
                    let yazı_tipi = pencere.text_style().font();
                    let metin_rengi = yol_önbelleği.renk(renk);
                    for (indeks, karakter) in içerik.chars().rev().enumerate() {
                        let mut hasher = DefaultHasher::new();
                        karakter.hash(&mut hasher);
                        let metin_kimliği = hasher.finish();
                        let metin_uzunluğu = karakter.len_utf8();
                        let koşu = TextRun {
                            len: metin_uzunluğu,
                            font: yazı_tipi.clone(),
                            color: metin_rengi,
                            background_color: None,
                            underline: None,
                            strikethrough: None,
                        };
                        let çizgi = pencere.text_system().shape_line_by_hash(
                            metin_kimliği,
                            metin_uzunluğu,
                            px(*boyut * ölçek),
                            &[koşu],
                            None,
                            || SharedString::from(karakter.to_string()),
                        );
                        let y = başlangıç_y + indeks as f32 * adım;
                        let mut dayanak = dönüştür(Nokta::yeni(konum.x, y));
                        if let Some(görünüm) = komut_görünümü {
                            dayanak = görünüm.noktayı_dönüştür(dayanak);
                        }
                        let başlangıç = point(
                            dayanak.x - çizgi.width() / 2.0,
                            dayanak.y - px(*boyut * ölçek),
                        );
                        let mut boya = |pencere: &mut Window| {
                            let _ = çizgi.paint(
                                başlangıç,
                                px(*boyut * 1.25 * ölçek),
                                TextAlign::Left,
                                None,
                                pencere,
                                uygulama,
                            );
                        };
                        if let Some(görünüm) = komut_görünümü {
                            pencere.with_content_mask(
                                Some(ContentMask {
                                    bounds: görünüm.kesme_sınırları,
                                }),
                                boya,
                            );
                        } else {
                            boya(pencere);
                        }
                    }
                }
            }
        }
    });
}

/// Tahsisat ve cache regresyon testleri için normal retained boya hazırlığı.
///
/// Bu tür yalnız [`crate::diagnostics`] üzerinden sunulur; grafik oluşturma
/// API'si değildir. İlk `tur` yolu kurar, sonraki turlar aynı fiziksel GPUI
/// path'ini allocation yapmadan paylaşır.
#[doc(hidden)]
pub struct GpuiRetainedBoyaÖlçer {
    sahne: Sahne,
    sınırlar: Bounds<Pixels>,
    önbellek: GpuiYolÖnbelleği,
}

#[doc(hidden)]
impl GpuiRetainedBoyaÖlçer {
    pub fn yeni() -> Self {
        let mut sahne = Sahne::yeni(320, 180);
        sahne.ekle(Komut::Yol {
            parçalar: vec![vec![Nokta::yeni(10.0, 20.0), Nokta::yeni(200.0, 80.0)]],
            renk: "#305cde".into(),
            kalınlık: 2.0,
        });
        Self {
            sahne,
            sınırlar: Bounds::new(point(px(0.0), px(0.0)), size(px(320.0), px(180.0))),
            önbellek: GpuiYolÖnbelleği::default(),
        }
    }

    pub fn tur(&mut self) -> usize {
        self.önbellek.yüzeyi_hazırla(&self.sahne, self.sınırlar);
        self.önbellek
            .yol(0, point(px(0.0), px(0.0)), || {
                Some(Path::new(point(px(1.0), px(2.0))))
            })
            .map_or(0, |yol| yol.yol.vertices.len())
    }
}

impl Default for GpuiRetainedBoyaÖlçer {
    fn default() -> Self {
        Self::yeni()
    }
}

#[allow(clippy::too_many_arguments)]
fn gradyan_yolunu_boya(
    yol: BoyanabilirGpuiYol,
    gradyan: &DoğrusalGradyan,
    dönüştür: &impl Fn(Nokta) -> ::gpui::Point<Pixels>,
    görünüm: Option<GpuiBoyaGörünümü>,
    renk_önbelleği: &mut GpuiYolÖnbelleği,
    hedef_köken: ::gpui::Point<Pixels>,
    pencere: &mut Window,
) {
    let Some(ilk) = gradyan.duraklar.first() else {
        return;
    };
    if gradyan.duraklar.len() == 1 {
        retained_yolu_boya(
            yol,
            renk_önbelleği.renk(&ilk.renk),
            görünüm,
            hedef_köken,
            pencere,
        );
        return;
    }
    let mut başlangıç = dönüştür(gradyan.başlangıç);
    let mut bitiş = dönüştür(gradyan.bitiş);
    // Gradyan ekseni `yolu_dönüştür` ile yüzey-yerel koordinatta gelir; yol ise
    // önbellekten `hedef_köken`e ötelenmiş çıkar ve sınırları mutlaktır. İkisi
    // karışırsa maske aralıkları farklı uzaylardan hesaplanıp çakışır: yüzen
    // çubuk gradyanında kırmızı ve yeşil aynı şeride düşüyor, sonra boyanan
    // yeşil kırmızıyı tamamen örtüyordu. `boya_maskeli_aralık` ötelemeyi zaten
    // kendisi uyguladığından hesap yerel uzayda yapılır.
    let mut yol_sınırları = yol.mantıksal_sınırlar;
    yol_sınırları.origin.x -= hedef_köken.x;
    yol_sınırları.origin.y -= hedef_köken.y;
    if let Some(görünüm) = görünüm {
        başlangıç = görünüm.noktayı_dönüştür(başlangıç);
        bitiş = görünüm.noktayı_dönüştür(bitiş);
        yol_sınırları = görünüm.sınırları_dönüştür(yol_sınırları);
    }
    let dx = f32::from(bitiş.x - başlangıç.x);
    let dy = f32::from(bitiş.y - başlangıç.y);
    let yatay = dx.abs() >= dy.abs();
    let eksen_başlangıcı = if yatay {
        f32::from(başlangıç.x)
    } else {
        f32::from(başlangıç.y)
    };
    let eksen_bitişi = if yatay {
        f32::from(bitiş.x)
    } else {
        f32::from(bitiş.y)
    };
    let eksen_farkı = eksen_bitişi - eksen_başlangıcı;
    if eksen_farkı.abs() <= f32::EPSILON {
        retained_yolu_boya(
            yol,
            renk_önbelleği.renk(&ilk.renk),
            görünüm,
            hedef_köken,
            pencere,
        );
        return;
    }
    let sınır_başı = if yatay {
        f32::from(yol_sınırları.left())
    } else {
        f32::from(yol_sınırları.top())
    };
    let sınır_sonu = if yatay {
        f32::from(yol_sınırları.right())
    } else {
        f32::from(yol_sınırları.bottom())
    };
    let açı = if yatay {
        if eksen_farkı >= 0.0 { 90.0 } else { 270.0 }
    } else if eksen_farkı >= 0.0 {
        180.0
    } else {
        0.0
    };

    let oranlar = gradyan
        .duraklar
        .iter()
        .map(|durak| durak.oran)
        .collect::<Vec<_>>();
    for şerit in gradyan_şeritleri(
        &oranlar,
        eksen_başlangıcı,
        eksen_bitişi,
        sınır_başı,
        sınır_sonu,
    ) {
        let Some(boya) = şerit.boya(&gradyan.duraklar, açı, renk_önbelleği) else {
            continue;
        };
        boya_maskeli_aralık(
            &yol,
            yatay,
            şerit.başlangıç,
            şerit.bitiş,
            boya,
            görünüm,
            yol_sınırları,
            hedef_köken,
            pencere,
        );
    }
}

/// Bir gradyan şeridinin kaplayacağı aralık ve boyası.
///
/// Eksenin dışında kalan bölgeler uç durakların düz rengiyle dolar; eksen
/// içindeki her durak çifti kendi doğrusal geçişini alır.
#[derive(Debug, Clone, Copy, PartialEq)]
struct GradyanŞeridi {
    başlangıç: f32,
    bitiş: f32,
    sol_durak: usize,
    sağ_durak: usize,
    /// Geçişin yol sınırlarına göre başlangıç ve bitiş yüzdesi. Düz
    /// şeritlerde kullanılmaz.
    sol_yüzde: f32,
    sağ_yüzde: f32,
}

impl GradyanŞeridi {
    fn boya(
        &self,
        duraklar: &[crate::GradyanRenkDurağı],
        açı: f32,
        renk_önbelleği: &mut GpuiYolÖnbelleği,
    ) -> Option<::gpui::Background> {
        let sol = duraklar.get(self.sol_durak)?;
        if self.sol_durak == self.sağ_durak {
            return Some(renk_önbelleği.renk(&sol.renk).into());
        }
        let sağ = duraklar.get(self.sağ_durak)?;
        let sol_rengi = renk_önbelleği.renk(&sol.renk);
        let sağ_rengi = renk_önbelleği.renk(&sağ.renk);
        // Geçiş şeridi `Background::as_solid` vermez; durak renkleri günlüğe
        // burada yazılır, yoksa gradyanla boyanan hiçbir renk görünmez.
        boya_günlüğü::yaz_renk(sol_rengi);
        boya_günlüğü::yaz_renk(sağ_rengi);
        Some(linear_gradient(
            açı,
            linear_color_stop(sol_rengi, self.sol_yüzde),
            linear_color_stop(sağ_rengi, self.sağ_yüzde),
        ))
    }
}

/// Gradyan duraklarını yol sınırları içinde boyanacak şeritlere böler.
///
/// Eksen noktaları ve sınırlar **aynı** koordinat uzayında olmalıdır. Uzaylar
/// karıştığında şeritler örtüşür ve son boyanan renk öncekileri gizler:
/// `sparklines-bars` yüzen çubuklarında kırmızı ile yeşil aynı şeride düşüp
/// çubukların tamamı yeşil çiziliyordu. Şeritler bu yüzden sınırlara kısılır;
/// döndürülen aralıklar sınırları boşluksuz ve örtüşmesiz kaplar.
fn gradyan_şeritleri(
    oranlar: &[f32],
    eksen_başlangıcı: f32,
    eksen_bitişi: f32,
    sınır_başı: f32,
    sınır_sonu: f32,
) -> Vec<GradyanŞeridi> {
    let eksen_farkı = eksen_bitişi - eksen_başlangıcı;
    let sınır_uzunluğu = (sınır_sonu - sınır_başı).max(f32::EPSILON);
    let konum = |oran: f32| {
        (eksen_başlangıcı + oran.clamp(0.0, 1.0) * eksen_farkı).clamp(sınır_başı, sınır_sonu)
    };
    let mut şeritler = Vec::with_capacity(oranlar.len().saturating_add(1));
    // Geçiş şeritlerinde yüzdeler durakların kendi konumlarından gelir; şerit
    // sınırlara kısılsa da renk rampası yerinde kalır. Düz şeritler onları
    // kullanmadığından kendi aralıklarını verir.
    let mut ekle = |başlangıç: f32,
                    bitiş: f32,
                    sol_durak: usize,
                    sağ_durak: usize,
                    yüzdeler: Option<(f32, f32)>| {
        let (başlangıç, bitiş) = (başlangıç.min(bitiş), başlangıç.max(bitiş));
        if bitiş - başlangıç <= f32::EPSILON {
            return;
        }
        let (sol_yüzde, sağ_yüzde) = yüzdeler.unwrap_or((
            (başlangıç - sınır_başı) / sınır_uzunluğu,
            (bitiş - sınır_başı) / sınır_uzunluğu,
        ));
        şeritler.push(GradyanŞeridi {
            başlangıç,
            bitiş,
            sol_durak,
            sağ_durak,
            sol_yüzde,
            sağ_yüzde,
        });
    };

    let son_indeks = oranlar.len().saturating_sub(1);
    let (Some(ilk_oran), Some(son_oran)) = (oranlar.first(), oranlar.last()) else {
        return şeritler;
    };
    let ilk_konum = konum(*ilk_oran);
    let son_konum = konum(*son_oran);
    // Eksenin gerisinde kalan bölge ilk durağın düz rengiyle dolar.
    if eksen_farkı >= 0.0 {
        ekle(sınır_başı, ilk_konum, 0, 0, None);
    } else {
        ekle(ilk_konum, sınır_sonu, 0, 0, None);
    }
    for (sıra, çift) in oranlar.windows(2).enumerate() {
        let (Some(sol), Some(sağ)) = (çift.first(), çift.get(1)) else {
            continue;
        };
        let (sol_konum, sağ_konum) = (konum(*sol), konum(*sağ));
        let yüzdeler = (
            (sol_konum - sınır_başı) / sınır_uzunluğu,
            (sağ_konum - sınır_başı) / sınır_uzunluğu,
        );
        ekle(
            sol_konum,
            sağ_konum,
            sıra,
            sıra.saturating_add(1),
            Some(yüzdeler),
        );
    }
    // Eksenin ötesinde kalan bölge son durağın düz rengiyle dolar.
    if eksen_farkı >= 0.0 {
        ekle(son_konum, sınır_sonu, son_indeks, son_indeks, None);
    } else {
        ekle(sınır_başı, son_konum, son_indeks, son_indeks, None);
    }
    şeritler
}

#[allow(clippy::too_many_arguments)]
fn boya_maskeli_aralık(
    yol: &BoyanabilirGpuiYol,
    yatay: bool,
    başlangıç: f32,
    bitiş: f32,
    boya: impl Into<::gpui::Background>,
    görünüm: Option<GpuiBoyaGörünümü>,
    yol_sınırları: Bounds<Pixels>,
    hedef_köken: ::gpui::Point<Pixels>,
    pencere: &mut Window,
) {
    let (başlangıç, bitiş) = (başlangıç.min(bitiş), başlangıç.max(bitiş));
    if bitiş - başlangıç <= f32::EPSILON {
        return;
    }
    let mut sınırlar = if yatay {
        Bounds::new(
            point(px(başlangıç), yol_sınırları.top()),
            size(px(bitiş - başlangıç), yol_sınırları.size.height),
        )
    } else {
        Bounds::new(
            point(yol_sınırları.left(), px(başlangıç)),
            size(yol_sınırları.size.width, px(bitiş - başlangıç)),
        )
    };
    sınırlar.origin += hedef_köken;
    let boya = boya.into();
    pencere.with_content_mask(Some(ContentMask { bounds: sınırlar }), |pencere| {
        retained_yolu_boya(yol.clone(), boya, görünüm, hedef_köken, pencere);
    });
}

fn tek_satır_metin_kimliği(metin: &str) -> (u64, usize) {
    let mut hasher = DefaultHasher::new();
    for bayt in metin.bytes() {
        if matches!(bayt, b'\r' | b'\n') {
            b' '.hash(&mut hasher);
        } else {
            bayt.hash(&mut hasher);
        }
    }
    (hasher.finish(), metin.len())
}

/// Seri seçeneklerindeki CSS renk kodunu GPUI rengine çevirir.
///
/// Hex (`#rgb`, `#rgba`, `#rrggbb`, `#rrggbbaa`), `rgb()`/`rgba()` ve temel
/// adlandırılmış renkleri tanır; tanınmayan kod siyaha düşer. Lejant ve seri
/// düğmesi kuran tüketiciler aynı rengi yüzeydeki çizgiyle birebir eşlemek
/// için buna ihtiyaç duyar.
pub fn renk_çöz(kod: &str) -> Hsla {
    let kod = kod.trim().to_ascii_lowercase();
    if let Some(ham) = kod.strip_prefix('#') {
        return match ham.len() {
            3 => kısa_hex_rengi(ham, false),
            4 => kısa_hex_rengi(ham, true),
            6 => u32::from_str_radix(ham, 16)
                .map_or_else(|_| rgb(0x000000).into(), |sayı| rgb(sayı).into()),
            8 => u32::from_str_radix(ham, 16)
                .map_or_else(|_| rgb(0x000000).into(), |sayı| rgba(sayı).into()),
            _ => rgb(0x000000).into(),
        };
    }
    if let Some(renk) = css_rgb_rengi(&kod) {
        return renk;
    }
    let sayı = match kod.as_str() {
        "transparent" => 0x00000000,
        "white" => 0xffffffff,
        "red" => 0xff0000ff,
        "green" => 0x008000ff,
        "blue" => 0x0000ffff,
        "yellow" => 0xffff00ff,
        "orange" => 0xffa500ff,
        "purple" => 0x800080ff,
        "magenta" | "fuchsia" => 0xff00ffff,
        "cyan" | "aqua" => 0x00ffffff,
        "gray" | "grey" => 0x808080ff,
        "brown" => 0xa52a2aff,
        "teal" => 0x008080ff,
        "pink" => 0xffc0cbff,
        _ => 0x000000ff,
    };
    rgba(sayı).into()
}

fn kısa_hex_rengi(ham: &str, alfa_var: bool) -> Hsla {
    let mut rakamlar = ham.chars().filter_map(|rakam| rakam.to_digit(16));
    let Some(r) = rakamlar.next() else {
        return rgb(0x000000).into();
    };
    let Some(g) = rakamlar.next() else {
        return rgb(0x000000).into();
    };
    let Some(b) = rakamlar.next() else {
        return rgb(0x000000).into();
    };
    let a = if alfa_var {
        let Some(a) = rakamlar.next() else {
            return rgb(0x000000).into();
        };
        a * 17
    } else {
        255
    };
    rgba((r * 17) << 24 | (g * 17) << 16 | (b * 17) << 8 | a).into()
}

fn css_rgb_rengi(kod: &str) -> Option<Hsla> {
    let içerik = kod
        .strip_prefix("rgba(")
        .or_else(|| kod.strip_prefix("rgb("))?
        .strip_suffix(')')?;
    let normal = içerik.replace(',', " ");
    let (kanallar, eğik_alfa) = normal
        .split_once('/')
        .map_or((normal.as_str(), None), |(kanallar, alfa)| {
            (kanallar, Some(alfa.trim()))
        });
    let parçalar = kanallar.split_whitespace().collect::<Vec<_>>();
    let (r, g, b, eski_alfa) = match parçalar.as_slice() {
        [r, g, b] => (*r, *g, *b, None),
        [r, g, b, alfa] => (*r, *g, *b, Some(*alfa)),
        _ => return None,
    };
    let r = css_renk_kanalı(r)?;
    let g = css_renk_kanalı(g)?;
    let b = css_renk_kanalı(b)?;
    let a = eğik_alfa.or(eski_alfa).map_or(Some(255), css_alfa_kanalı)?;
    Some(rgba(u32::from(r) << 24 | u32::from(g) << 16 | u32::from(b) << 8 | u32::from(a)).into())
}

fn css_renk_kanalı(değer: &str) -> Option<u8> {
    if let Some(yüzde) = değer.strip_suffix('%') {
        return yüzde
            .parse::<f32>()
            .ok()
            .map(|oran| (oran.clamp(0.0, 100.0) * 2.55).round() as u8);
    }
    değer
        .parse::<f32>()
        .ok()
        .map(|kanal| kanal.clamp(0.0, 255.0).round() as u8)
}

fn css_alfa_kanalı(değer: &str) -> Option<u8> {
    if let Some(yüzde) = değer.strip_suffix('%') {
        return yüzde
            .parse::<f32>()
            .ok()
            .map(|oran| (oran.clamp(0.0, 100.0) * 2.55).round() as u8);
    }
    değer
        .parse::<f32>()
        .ok()
        .map(|alfa| (alfa.clamp(0.0, 1.0) * 255.0).round() as u8)
}

#[cfg(test)]
mod testler {
    use super::*;

    /// Grafik kökünün taban yüksekliği yüzeyi kendi hücresinden taşırmamalı.
    ///
    /// `sparklines` 10×2 tablosu 150×30 hücrelere yerleşir. Sabit 120 px taban
    /// her yüzeyi 90 px aşağı taşırıp kendinden sonraki üç satırın üstüne
    /// yazıyordu; sonra çizilen üstte kaldığından tabloda yalnız son satır
    /// görünüyordu. Taban artık ham yüksekliği aşamaz.
    #[test]
    fn taban_yükseklik_ham_yüksekliği_aşmaz() {
        assert_eq!(en_az_yüzey_yüksekliği(30), 30.0);
        assert_eq!(en_az_yüzey_yüksekliği(119), 119.0);
        assert_eq!(en_az_yüzey_yüksekliği(120), 120.0);
        // Uzun yüzeylerde taban değişmez; esnek kapta sıfıra çökmeyi önler.
        assert_eq!(en_az_yüzey_yüksekliği(600), 120.0);
    }

    /// Ctrl yapışması ikinci eksende de fareye en yakın örneğe oturmalı.
    ///
    /// Yalnız X yapıştığında imleç noktası veri noktasının hizasına gelmiyor,
    /// yanından geçen bir kesişim gösteriyordu.
    #[test]
    fn ikincil_yapışma_en_yakın_örneği_seçer() {
        // Normal yönelim: y oranı 0 = alt, ekran dikeyi 0 = üst.
        let oranlar = [Some(0.25), Some(0.80), None];
        // Fare üst bölgede (0,15) → çevrilmiş konumları 0,75 ve 0,20;
        // en yakın 0,20 olan ikinci seridir.
        assert_eq!(
            ikincil_yapışma_konumu(&oranlar, 0.15, false),
            Some(0.19999999999999996)
        );
        // Fare alt bölgede (0,70) → 0,75 kazanır.
        assert_eq!(ikincil_yapışma_konumu(&oranlar, 0.70, false), Some(0.75));

        // X dikeyken ikinci eksen yataydır; oran çevrilmez.
        assert_eq!(ikincil_yapışma_konumu(&oranlar, 0.30, true), Some(0.25));

        // Örneği olmayan seriler ve sonsuz oranlar aday değildir.
        assert_eq!(ikincil_yapışma_konumu(&[None, None], 0.5, false), None);
        assert_eq!(
            ikincil_yapışma_konumu(&[Some(f64::INFINITY)], 0.5, false),
            None
        );
        assert_eq!(ikincil_yapışma_konumu(&[], 0.5, false), None);
    }

    /// Gradyan şeritleri yol sınırlarını boşluksuz ve örtüşmesiz kaplamalı.
    ///
    /// `sparklines-bars` yüzen çubuklarında eksen noktaları yüzey-yerel,
    /// yol sınırları ise mutlak koordinattaydı. Karışım şeritleri aynı
    /// yere düşürüyor, son boyanan yeşil kırmızıyı örtüyordu. Testin ikinci
    /// yarısı o karışımı taklit ederek invaryantın gerçekten koruduğunu
    /// gösterir.
    #[test]
    fn gradyan_şeritleri_sınırları_örtüşmeden_kaplar() {
        // `sparklines-bars` yüzen çubuk gradyanı: ayrık kırmızı/yeşil,
        // eksen 392 → 152, yol sınırı 8 → 392.
        let oranlar = [0.0, 1.0, 1.0];
        let şeritler = gradyan_şeritleri(&oranlar, 392.0, 152.0, 8.0, 392.0);
        assert!(!şeritler.is_empty(), "şerit üretilmedi");

        let mut sıralı = şeritler.clone();
        sıralı.sort_by(|a, b| a.başlangıç.total_cmp(&b.başlangıç));
        let ilk = sıralı.first().map(|şerit| şerit.başlangıç);
        let son = sıralı.last().map(|şerit| şerit.bitiş);
        assert_eq!(ilk, Some(8.0), "şeritler sınırın başından başlamalı");
        assert_eq!(son, Some(392.0), "şeritler sınırın sonunda bitmeli");
        for çift in sıralı.windows(2) {
            let (Some(önce), Some(sonra)) = (çift.first(), çift.get(1)) else {
                continue;
            };
            assert!(
                (sonra.başlangıç - önce.bitiş).abs() <= f32::EPSILON,
                "şeritler arasında boşluk veya örtüşme var: {önce:?} → {sonra:?}"
            );
        }
        // Her durak en az bir şeritte temsil edilmeli; ayrık gradyanın
        // negatif dalı ilk durağı, pozitif dalı son durağı kullanır.
        let temsil_edilen = |şeritler: &[GradyanŞeridi]| {
            (0..oranlar.len())
                .filter(|indeks| {
                    şeritler
                        .iter()
                        .any(|şerit| şerit.sol_durak == *indeks || şerit.sağ_durak == *indeks)
                })
                .count()
        };
        assert_eq!(
            temsil_edilen(&şeritler),
            oranlar.len(),
            "her durak boyanmalı: {şeritler:?}"
        );

        // Karışık uzay: sınırlar 352 px ötelenmiş, eksen yerel kalmış.
        let karışık = gradyan_şeritleri(&oranlar, 392.0, 152.0, 360.0, 744.0);
        assert!(
            karışık.iter().all(|şerit| şerit.başlangıç >= 360.0),
            "kısıtlama olmadan şeritler sınırın dışına taşardı: {karışık:?}"
        );
        // Karışımın gerçek zararı: eksen sınırın gerisinde kaldığından son
        // durak hiç şerit almıyor. Yüzeyde bu, ayrık gradyanın bir dalının
        // tamamen kaybolması olarak görünüyordu.
        assert!(
            temsil_edilen(&karışık) < oranlar.len(),
            "karışık uzayda durak kaybı beklenir, invaryant bunu yakalar: {karışık:?}"
        );
    }

    /// Retained yol önbellekten `hedef_köken`e ötelenmiş çıkar; sınırları da
    /// öyle. Gradyan ekseni ise `yolu_dönüştür` ile yüzey-yerel hesaplanır.
    /// `gradyan_yolunu_boya` bu ötelemeyi sınırlardan geri düşer — iki uzay
    /// karışırsa maske aralıkları çakışıp son boyanan renk öncekileri örter.
    #[test]
    fn boyanabilir_yol_sınırları_hedef_kökene_ötelenir() {
        let mut kurucu = PathBuilder::fill();
        kurucu.move_to(point(px(0.0), px(0.0)));
        kurucu.line_to(point(px(10.0), px(0.0)));
        kurucu.line_to(point(px(10.0), px(10.0)));
        kurucu.close();
        let yol = kurucu.build();
        assert!(yol.is_ok(), "üçgen yol kurulamadı");
        let Ok(yol) = yol else { return };

        let mut önbellekli = ÖnbellekliGpuiYol::yeni(yol);
        let boyanabilir = önbellekli.boyanabilir(point(px(305.0), px(352.0)));
        assert_eq!(boyanabilir.mantıksal_sınırlar.origin.x, px(305.0));
        assert_eq!(boyanabilir.mantıksal_sınırlar.origin.y, px(352.0));

        // Aynı önbellek başka bir kökene taşındığında sınırlar onu izler.
        let boyanabilir = önbellekli.boyanabilir(point(px(0.0), px(0.0)));
        assert_eq!(boyanabilir.mantıksal_sınırlar.origin.x, px(0.0));
        assert_eq!(boyanabilir.mantıksal_sınırlar.origin.y, px(0.0));
    }

    /// Dağılım yüzeyleri on binlerce daireyi tek `PathBuilder` yoluna yazar.
    /// `build()` bu ölçekte başarısız olursa çağrı yerindeki `.ok()` hatayı
    /// yutar ve yüzey sessizce boş çizilir.
    /// GPUI yol kurucusunun toplu daire sınırını belgeler.
    ///
    /// Ölçüm 4.000 dairenin tek yolda kurulduğunu, 8.000'in kurulamadığını
    /// gösterdi. `build()` bu sınırın üstünde `Err` döndürür ve `Komut::Daireler`
    /// çizimindeki `.ok()` hatayı yutar; yüzey sessizce boş çizilir. 40.000
    /// noktalı dağılım yüzeyinin boş görünmesinin nedeni budur.
    #[test]
    fn iki_bin_daire_tek_yola_kurulabilir() {
        let mut yol = PathBuilder::fill();
        let yarıçap = px(2.5);
        let yarıçaplar = point(yarıçap, yarıçap);
        for indeks in 0..2_000 {
            let merkez = point(
                px((indeks % 100) as f32 * 8.0),
                px((indeks / 100) as f32 * 8.0),
            );
            let sol = point(merkez.x - yarıçap, merkez.y);
            let sağ = point(merkez.x + yarıçap, merkez.y);
            yol.move_to(sol);
            yol.arc_to(yarıçaplar, px(0.0), false, true, sağ);
            yol.arc_to(yarıçaplar, px(0.0), false, true, sol);
            yol.close();
        }
        assert!(
            yol.build().is_ok(),
            "parça boyutundaki daire yığını kurulamadı: dağılım yüzeyleri boş çizilir"
        );
    }

    #[::gpui::test]
    fn gpui_test_context_retained_sahne_ve_hover_katmanını_ayrı_tutar(
        cx: &mut ::gpui::TestAppContext,
    ) {
        let kart = test_çizgi_kartı(800, 600);
        assert!(kart.is_ok(), "test grafiği seçenekleri oluşturulamadı");
        let Ok((seçenekler, veri)) = kart else {
            return;
        };
        let grafik = Grafik::yeni(seçenekler, veri);
        assert!(grafik.is_ok(), "test grafiği oluşturulamadı");
        let Ok(grafik) = grafik else {
            return;
        };
        let (grafik, cx) = cx.add_window_view(|_, _| GpuiGrafik::yeni(grafik));
        let (ana_önce, etkileşim_önce, sınırlar) = grafik.read_with(cx, |grafik, _| {
            let (ana, etkileşim) = grafik.sahne_revizyonları();
            (ana, etkileşim, grafik.çizim_sınırları.get())
        });
        assert!(sınırlar.is_some(), "GPUI canvas sınırları hazırlanmadı");
        let Some(sınırlar) = sınırlar else {
            return;
        };

        cx.simulate_mouse_move(sınırlar.center(), None, ::gpui::Modifiers::none());

        grafik.read_with(cx, |grafik, _| {
            let (ana_sonra, etkileşim_sonra) = grafik.sahne_revizyonları();
            assert_eq!(ana_sonra, ana_önce);
            assert!(etkileşim_sonra > etkileşim_önce);
        });
    }

    #[::gpui::test]
    fn değişmeyen_entity_renderı_etkileşim_sahnesini_tekrar_kurmaz(
        cx: &mut ::gpui::TestAppContext,
    ) {
        let kart = test_çizgi_kartı(800, 600);
        assert!(kart.is_ok());
        let Ok((seçenekler, veri)) = kart else {
            return;
        };
        let grafik = Grafik::yeni(seçenekler, veri);
        assert!(grafik.is_ok());
        let Ok(grafik) = grafik else { return };
        let (grafik, cx) = cx.add_window_view(|_, _| GpuiGrafik::yeni(grafik));
        let ilk = grafik.read_with(cx, |grafik, _| grafik.etkileşim_sahne_hazırlama_sayısı);

        grafik.update(cx, |_, cx| cx.notify());
        cx.run_until_parked();

        grafik.read_with(cx, |grafik, _| {
            assert_eq!(grafik.etkileşim_sahne_hazırlama_sayısı, ilk);
        });
    }

    #[::gpui::test]
    fn bilgi_balonu_en_yakın_imleçte_bir_saniye_sonra_hazır_olur(
        cx: &mut ::gpui::TestAppContext,
    ) {
        let kart = test_çizgi_kartı(800, 600);
        assert!(kart.is_ok());
        let Ok((seçenekler, veri)) = kart else {
            return;
        };
        let etkileşimler = seçenekler.etkileşimler.imleç_bilgi_kutusu(true);
        let grafik = Grafik::yeni(seçenekler.etkileşimler(etkileşimler), veri);
        assert!(grafik.is_ok());
        let Ok(grafik) = grafik else { return };
        let (grafik, cx) = cx.add_window_view(|_, _| GpuiGrafik::yeni(grafik));
        let sınırlar = grafik.read_with(cx, |grafik, _| grafik.çizim_sınırları.get());
        assert!(sınırlar.is_some());
        let Some(sınırlar) = sınırlar else {
            return;
        };

        cx.simulate_mouse_move(sınırlar.center(), None, ::gpui::Modifiers::none());
        assert!(!grafik.read_with(cx, |grafik, _| grafik.bilgi_balonu_hazır));
        cx.executor().advance_clock(Duration::from_millis(999));
        cx.run_until_parked();
        assert!(!grafik.read_with(cx, |grafik, _| grafik.bilgi_balonu_hazır));
        cx.executor().advance_clock(Duration::from_millis(1));
        cx.run_until_parked();
        grafik.read_with(cx, |grafik, _| {
            assert!(grafik.bilgi_balonu_hazır);
            let imleç = grafik.imleç.as_ref();
            assert!(imleç.is_some());
            assert!(imleç.is_some_and(|imleç| imleç.seri_değerleri.iter().any(Option::is_some)));
        });

        cx.simulate_mouse_move(
            ::gpui::point(sınırlar.center().x + px(1.0), sınırlar.center().y),
            None,
            ::gpui::Modifiers::none(),
        );
        assert!(!grafik.read_with(cx, |grafik, _| grafik.bilgi_balonu_hazır));
    }

    #[test]
    fn bilgi_balonu_imlece_dikey_olarak_en_yakın_seriyi_seçer() -> Result<(), UplotHatası> {
        let seçenekler = crate::GrafikSeçenekleri::yeni(800, 600)?
            .x_zaman(false)
            .y_aralığı(Aralık::yeni(0.0, 10.0)?)
            .seri(SeriSeçenekleri::yeni("alt"))
            .seri(SeriSeçenekleri::yeni("üst"));
        let veri = HizalıVeri::yeni(
            vec![0.0, 1.0],
            vec![vec![Some(1.0), Some(1.0)], vec![Some(9.0), Some(9.0)]],
        )?;
        let bileşen = GpuiGrafik::yeni(Grafik::yeni(seçenekler, veri)?);
        let (sol, sağ, üst, _) = bileşen.çizim_alanı();
        let imleç = İmleçDurumu {
            fare: Nokta::yeni((sol + sağ) / 2.0, üst),
            veri_x: 1.0,
            seri_x_değerleri: vec![Some(1.0), Some(1.0)],
            seri_değerleri: vec![Some(1.0), Some(9.0)],
            dağılım: None,
        };

        assert_eq!(bileşen.bilgi_balonu_seri_indeksi(&imleç), Some(1));
        Ok(())
    }

    /// uPlot her `setScale`'de seri yollarını geçersizleştirir; veri katmanı
    /// da yakınlaştırmada yeniden kurulmalı, aksi hâlde kontur kalınlığı
    /// anizotropik ölçeklenir ve piksel kovası seyreltmesi ilk yoğunlukta
    /// donar.
    #[::gpui::test]
    fn tekerlek_zoom_veri_katmanını_yeniden_kurar(cx: &mut ::gpui::TestAppContext) {
        let kart = test_çizgi_kartı(800, 600);
        assert!(kart.is_ok(), "test grafiği seçenekleri oluşturulamadı");
        let Ok((seçenekler, veri)) = kart else {
            return;
        };
        let grafik = Grafik::yeni(seçenekler, veri);
        assert!(grafik.is_ok(), "test grafiği oluşturulamadı");
        let Ok(grafik) = grafik else { return };
        let (grafik, cx) = cx.add_window_view(|_, _| GpuiGrafik::yeni(grafik));
        let (ana_sahne, ana_revizyon, görünüm_revizyonu) = grafik.read_with(cx, |grafik, _| {
            (
                grafik.ana_sahne.clone(),
                grafik.ana_sahne_revizyonu,
                grafik.görünüm_revizyonu,
            )
        });
        grafik.update(cx, |grafik, cx| {
            let değişti = grafik.grafik.tekerlek(0.5, 0.5, 180.0, true);
            assert!(değişti.is_ok(), "tekerlek yakınlaştırması uygulanamadı");
            let Ok(değişti) = değişti else { return };
            assert!(değişti);
            grafik.görünüm_bildir(false, cx);
        });

        grafik.read_with(cx, |grafik, _| {
            assert!(
                !Rc::ptr_eq(&grafik.ana_sahne, &ana_sahne),
                "yakınlaştırma veri sahnesini yeniden kurmalı"
            );
            assert!(grafik.ana_sahne_revizyonu > ana_revizyon);
            assert!(grafik.görünüm_revizyonu > görünüm_revizyonu);
            assert!(grafik.grafik.yakınlaştırılmış());
        });
    }

    /// Sahne güncel pencere için kurulduğundan yüzeye ikinci bir
    /// yakınlaştırma dönüşümü uygulanmamalı; aksi hâlde geometri iki kez
    /// ölçeklenir.
    #[test]
    fn zoom_sonrası_görünüm_penceresi_birim_kalır() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = test_çizgi_kartı(800, 600)?;
        let grafik = Grafik::yeni(seçenekler, veri)?;
        let mut bileşen = GpuiGrafik::yeni(grafik);
        let görünüm_revizyonu = bileşen.görünüm_revizyonu;

        assert!(
            bileşen
                .grafik
                .görünür_x_aralığını_ayarla(Aralık::yeni(0.5, 1.5)?, true,)
        );
        bileşen.görünümü_yenile();

        assert!(bileşen.görünüm_revizyonu > görünüm_revizyonu);
        let pencere = bileşen.veri_görünümü.get().pencere;
        assert!(pencere.sol.abs() <= 0.0001);
        assert!((pencere.sağ - 1.0).abs() <= 0.0001);
        assert!(pencere.üst.abs() <= 0.0001);
        assert!((pencere.alt - 1.0).abs() <= 0.0001);
        Ok(())
    }

    #[test]
    fn yalnız_x_zoomu_gpui_yüzeyinde_y_verisini_yeniden_taramaz() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = test_çizgi_kartı(800, 600)?;
        let mut grafik = Grafik::yeni(seçenekler, veri)?;

        assert!(grafik.tekerlek_eksende(0.5, 0.5, 1.0, false, TekerlekEkseni::X,)?);
        let pencere = grafik.oransal_görünüm();

        assert!(pencere.sol > 0.0 || pencere.sağ < 1.0);
        assert!(pencere.üst.abs() <= f32::EPSILON);
        assert!((pencere.alt - 1.0).abs() <= f32::EPSILON);
        Ok(())
    }

    #[test]
    fn gpui_dört_katmanlı_çekirdek_sırasını_korur() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = test_çizgi_kartı(800, 600)?;
        let varsayılan = Grafik::yeni(seçenekler.clone(), veri.clone())?;
        let ızgara_üstte = Grafik::yeni(
            seçenekler.katman_sırası([
                crate::ÇizimKatmanı::ArkaPlan,
                crate::ÇizimKatmanı::Veri,
                crate::ÇizimKatmanı::IzgaraEksen,
                crate::ÇizimKatmanı::Bilgi,
            ]),
            veri,
        )?;

        assert_eq!(varsayılan.katman_sırası(), &crate::VARSAYILAN_KATMAN_SIRASI);
        assert_eq!(
            ızgara_üstte.katman_sırası(),
            &[
                crate::ÇizimKatmanı::ArkaPlan,
                crate::ÇizimKatmanı::Veri,
                crate::ÇizimKatmanı::IzgaraEksen,
                crate::ÇizimKatmanı::Bilgi,
            ]
        );
        let bileşen = GpuiGrafik::yeni(varsayılan);
        assert!(
            bileşen
                .arka_plan_sahnesi
                .komutlar()
                .iter()
                .any(|komut| matches!(komut, Komut::ArkaPlan { .. }))
        );
        assert!(
            bileşen
                .ana_sahne
                .komutlar()
                .iter()
                .all(|komut| !matches!(komut, Komut::ArkaPlan { .. }))
        );
        Ok(())
    }

    #[test]
    fn zoom_matrisi_kaynak_pencereyi_sabit_maskeye_taşır() {
        let görünüm = GpuiVeriGörünümü {
            pencere: OransalGörünüm {
                sol: 0.25,
                sağ: 0.75,
                üst: 0.2,
                alt: 0.8,
            },
            çizim_alanı: (100.0, 700.0, 50.0, 550.0),
        };
        let yüzey = GpuiYüzeyDönüşümü {
            ölçek: 1.0,
            köken_x: 0.0,
            köken_y: 0.0,
        };
        let boya = GpuiBoyaGörünümü::hesapla(görünüm, yüzey, 0.0);
        assert!(boya.is_some(), "geçerli zoom matrisi bekleniyordu");
        let Some(boya) = boya else { return };
        let kaynak_sol = point(px(250.0), px(150.0));
        let kaynak_sağ_alt = point(px(550.0), px(450.0));
        assert_eq!(
            boya.noktayı_dönüştür(kaynak_sol),
            point(px(100.0), px(50.0))
        );
        assert_eq!(
            boya.noktayı_dönüştür(kaynak_sağ_alt),
            point(px(700.0), px(550.0))
        );
        assert_eq!(
            boya.kesme_sınırları,
            Bounds::new(point(px(100.0), px(50.0)), size(px(600.0), px(500.0)))
        );
    }

    fn yol_sahnesi(renk: &str, kalınlık: f32, bitiş_x: f32) -> Sahne {
        let mut sahne = Sahne::yeni(320, 180);
        sahne.ekle(Komut::Yol {
            parçalar: vec![vec![Nokta::yeni(10.0, 20.0), Nokta::yeni(bitiş_x, 80.0)]],
            renk: renk.to_owned().into(),
            kalınlık,
        });
        sahne
    }

    fn test_çizgi_kartı(
        genişlik: u32,
        yükseklik: u32,
    ) -> Result<(crate::GrafikSeçenekleri, HizalıVeri), UplotHatası> {
        let seçenekler = crate::GrafikSeçenekleri::yeni(genişlik, yükseklik)?
            .başlık("GPUI çekirdek testi")
            .x_zaman(false)
            .etkileşimler(
                crate::EtkileşimSeçenekleri::default()
                    .tekerlek_etkileşimi(true)
                    .tekerlek_odaksız_etkileşim(true),
            )
            .seri(SeriSeçenekleri::yeni("Value").renk("red"));
        let veri = HizalıVeri::yeni(
            vec![0.0, 1.0, 2.0],
            vec![vec![Some(0.0), Some(1.0), Some(0.5)]],
        )?;
        Ok((seçenekler, veri))
    }

    fn test_grup_kartı(
        genişlik: u32,
        yükseklik: u32,
        x: Vec<f64>,
        y: Vec<Option<f64>>,
        etiket: &str,
    ) -> Result<Grafik, UplotHatası> {
        let seçenekler = crate::GrafikSeçenekleri::yeni(genişlik, yükseklik)?
            .başlık(etiket)
            .x_zaman(false)
            .etkileşimler(
                crate::EtkileşimSeçenekleri::default()
                    .tekerlek_etkileşimi(true)
                    .tekerlek_odaksız_etkileşim(true),
            )
            .seri(SeriSeçenekleri::yeni(etiket).renk("red"));
        Grafik::yeni(seçenekler, HizalıVeri::yeni(x, vec![y])?)
    }

    #[::gpui::test]
    fn grafik_grubu_farklı_boyut_ve_aralıklarda_tüm_oranları_korur(
        cx: &mut ::gpui::TestAppContext,
    ) {
        let kaynak = test_grup_kartı(
            1_920,
            400,
            vec![0.0, 1.0, 2.0, 3.0],
            vec![Some(0.0), Some(10.0), Some(5.0), Some(8.0)],
            "Kaynak",
        );
        let hedef = test_grup_kartı(
            600,
            240,
            vec![100.0, 200.0, 300.0, 400.0],
            vec![Some(-200.0), Some(0.0), Some(300.0), Some(100.0)],
            "Hedef",
        );
        assert!(kaynak.is_ok() && hedef.is_ok());
        let (Ok(kaynak), Ok(hedef)) = (kaynak, hedef) else {
            return;
        };
        let (kaynak, hedef, grup) = cx.update(|cx| {
            let kaynak = cx.new(|_| GpuiGrafik::yeni(kaynak));
            let hedef = cx.new(|_| GpuiGrafik::yeni(hedef));
            let grup = cx.new(|_| GpuiGrafikGrubu::yeni(GpuiGrafikGrupAyarları::default()));
            grup.update(cx, |grup, cx| {
                assert!(grup.grafik_ekle("kaynak", kaynak.clone(), cx));
                assert!(grup.grafik_ekle("hedef", hedef.clone(), cx));
            });
            (kaynak, hedef, grup)
        });
        assert_eq!(grup.read_with(cx, |grup, _| grup.üye_sayısı()), 2);

        kaynak.update(cx, |kaynak, cx| {
            assert!(kaynak.senkron_imleci_ayarla(0.73, Some(0.21), Some(0), cx));
            GpuiGrafik::imleç_bildir(cx);
        });
        let hedef_imleci = hedef.read_with(cx, |hedef, _| hedef.oransal_imleç_yayını());
        assert!(hedef_imleci.is_some(), "hedef imleci senkronlanmadı");
        let Some((x, y, seri)) = hedef_imleci else {
            return;
        };
        assert!((x - 0.73).abs() < 1e-6);
        assert!((y - 0.21).abs() < 1e-6);
        assert_eq!(seri, Some(0));

        kaynak.update(cx, |kaynak, cx| {
            let değişti =
                kaynak
                    .grafik
                    .tekerlek_eksende(0.68, 0.37, 1.0, false, TekerlekEkseni::İkisi);
            assert_eq!(değişti, Ok(true));
            kaynak.görünüm_bildir(false, cx);
        });
        let kaynak_görünümü = kaynak.read_with(cx, |kaynak, _| kaynak.oransal_görünüm_yayını());
        let hedef_görünümü = hedef.read_with(cx, |hedef, _| hedef.oransal_görünüm_yayını());
        for (kaynak, hedef) in [
            (kaynak_görünümü.sol, hedef_görünümü.sol),
            (kaynak_görünümü.sağ, hedef_görünümü.sağ),
            (kaynak_görünümü.üst, hedef_görünümü.üst),
            (kaynak_görünümü.alt, hedef_görünümü.alt),
        ] {
            assert!((kaynak - hedef).abs() < 1e-5);
        }

        kaynak.update(cx, |kaynak, cx| {
            let sonuç = kaynak.seri_görünürlüğünü_ayarla(0, false, cx);
            assert_eq!(sonuç, Ok(true));
        });
        assert!(!hedef.read_with(cx, |hedef, _| hedef.grafik().seri_görünür_mü(0)));
    }

    #[::gpui::test]
    fn grup_imleci_aynı_veri_indeksinde_de_oransal_konumu_izler(cx: &mut ::gpui::TestAppContext) {
        let kaynak = test_grup_kartı(
            800,
            400,
            vec![0.0, 1.0, 2.0],
            vec![Some(0.0), Some(1.0), Some(2.0)],
            "Kaynak",
        );
        let hedef = test_grup_kartı(
            600,
            240,
            vec![10.0, 20.0, 30.0],
            vec![Some(5.0), Some(6.0), Some(7.0)],
            "Hedef",
        );
        assert!(kaynak.is_ok() && hedef.is_ok());
        let (Ok(kaynak), Ok(hedef)) = (kaynak, hedef) else {
            return;
        };
        let (kaynak, cx) = cx.add_window_view(|_, _| GpuiGrafik::yeni(kaynak));
        let (hedef, _grup) = cx.update(|_, cx| {
            let hedef = cx.new(|_| GpuiGrafik::yeni(hedef));
            let grup = cx.new(|_| GpuiGrafikGrubu::yeni(GpuiGrafikGrupAyarları::default()));
            grup.update(cx, |grup, cx| {
                assert!(grup.grafik_ekle("kaynak", kaynak.clone(), cx));
                assert!(grup.grafik_ekle("hedef", hedef.clone(), cx));
            });
            (hedef, grup)
        });
        let sınırlar = kaynak.read_with(cx, |grafik, _| grafik.çizim_sınırları.get());
        assert!(sınırlar.is_some());
        let Some(sınırlar) = sınırlar else {
            return;
        };
        let ilk = point(sınırlar.center().x, sınırlar.center().y - px(12.0));
        let ikinci = point(sınırlar.center().x, sınırlar.center().y + px(12.0));

        cx.simulate_mouse_move(ilk, None, ::gpui::Modifiers::none());
        let ilk_y = hedef
            .read_with(cx, |grafik, _| grafik.oransal_imleç_yayını())
            .map(|(_, y, _)| y);
        cx.simulate_mouse_move(ikinci, None, ::gpui::Modifiers::none());
        let ikinci_y = hedef
            .read_with(cx, |grafik, _| grafik.oransal_imleç_yayını())
            .map(|(_, y, _)| y);

        assert!(ilk_y.is_some() && ikinci_y.is_some());
        assert_ne!(ilk_y.map(f64::to_bits), ikinci_y.map(f64::to_bits));
    }

    #[test]
    fn oransal_görünüm_dikey_ters_ve_logaritmik_ölçeklerde_geri_döner() -> Result<(), UplotHatası> {
        let tam = Aralık::yeni(1.0, 1_000.0)?;
        let seçenekler = crate::GrafikSeçenekleri::yeni(360, 720)?
            .x_zaman(false)
            .x_aralığı(tam)
            .x_logaritmik(10.0)
            .x_ters_yön(true)
            .x_dikey(true)
            .y_ölçeği(
                crate::YÖlçekSeçenekleri::yeni("y")
                    .aralık(tam)
                    .logaritmik(10.0)
                    .ters_yön(true),
            )
            .seri(SeriSeçenekleri::yeni("Value").renk("red"));
        let veri = HizalıVeri::yeni(
            vec![1.0, 10.0, 100.0, 1_000.0],
            vec![vec![Some(1.0), Some(10.0), Some(100.0), Some(1_000.0)]],
        )?;
        let mut grafik = Grafik::yeni(seçenekler, veri)?;
        let beklenen = OransalGörünüm {
            sol: 0.13,
            sağ: 0.79,
            üst: 0.24,
            alt: 0.91,
        };

        assert!(grafik.oransal_görünümü_ayarla(beklenen, true)?);
        let gerçek = grafik.oransal_görünüm();
        for (beklenen, gerçek) in [
            (beklenen.sol, gerçek.sol),
            (beklenen.sağ, gerçek.sağ),
            (beklenen.üst, gerçek.üst),
            (beklenen.alt, gerçek.alt),
        ] {
            assert!((beklenen - gerçek).abs() < 1e-5);
        }
        Ok(())
    }

    #[test]
    fn grafik_grubu_seriyi_etiket_veya_indeksle_eşleyebilir() -> Result<(), UplotHatası> {
        let seçenekler = crate::GrafikSeçenekleri::yeni(600, 240)?
            .x_zaman(false)
            .seri(SeriSeçenekleri::yeni("green").renk("green"))
            .seri(SeriSeçenekleri::yeni("red").renk("red"));
        let veri = HizalıVeri::yeni(
            vec![0.0, 1.0],
            vec![vec![Some(0.0), Some(1.0)], vec![Some(1.0), Some(0.0)]],
        )?;
        let hedef = GpuiGrafik::yeni(Grafik::yeni(seçenekler, veri)?);

        assert_eq!(
            eşlenen_seri_indeksi(&hedef, Some(0), Some("red"), GpuiSeriEşleme::İndeks),
            Some(0)
        );
        assert_eq!(
            eşlenen_seri_indeksi(&hedef, Some(0), Some("red"), GpuiSeriEşleme::Etiket),
            Some(1)
        );
        Ok(())
    }

    fn test_y_kaydırılmış_kartı() -> Result<(crate::GrafikSeçenekleri, HizalıVeri), UplotHatası> {
        let ham = vec![Some(1.0), Some(2.0), Some(3.0)];
        let seçenekler = crate::GrafikSeçenekleri::yeni(1_920, 600)?
            .x_zaman(false)
            .seri(
                SeriSeçenekleri::yeni("Core 1")
                    .renk("red")
                    .lejant_değerleri(ham.clone()),
            )
            .seri(
                SeriSeçenekleri::yeni("Core 2")
                    .renk("green")
                    .lejant_değerleri(ham.clone()),
            )
            .seri(
                SeriSeçenekleri::yeni("Core 3")
                    .renk("blue")
                    .lejant_değerleri(ham),
            );
        let veri = HizalıVeri::yeni(
            vec![0.0, 1.0, 2.0],
            vec![
                vec![Some(1.0), Some(2.0), Some(3.0)],
                vec![Some(11.0), Some(12.0), Some(13.0)],
                vec![Some(21.0), Some(22.0), Some(23.0)],
            ],
        )?;
        Ok((seçenekler, veri))
    }

    fn test_boyut_senkron_kartı() -> Result<(crate::GrafikSeçenekleri, HizalıVeri), UplotHatası> {
        let düzen = BoyutSenkronDüzeni::piksel_değerlerinden(
            725.0, 733.0, 200.0, 200.0, 100.0, 0.0, 100.0, 733.0, 363.0, 400.0,
        )
        .ok_or_else(|| UplotHatası::GeçersizKaynakVeri {
            varlık: "GPUI çekirdek testi",
            açıklama: "boyut senkron düzeni oluşturulamadı".to_string(),
        })?;
        let seçenekler = crate::GrafikSeçenekleri::yeni(800, 800)?
            .x_zaman(false)
            .boyut_senkronu(düzen)
            .seri(
                SeriSeçenekleri::yeni("Value")
                    .renk("red")
                    .dolgu("#ff00001a"),
            );
        let veri = HizalıVeri::yeni(
            vec![0.0, 1.0, 2.0],
            vec![vec![Some(0.0), Some(1.0), Some(2.0)]],
        )?;
        Ok((seçenekler, veri))
    }

    fn test_dikey_kartı() -> Result<(crate::GrafikSeçenekleri, HizalıVeri), UplotHatası> {
        let seçenekler = crate::GrafikSeçenekleri::yeni(320, 600)?
            .x_zaman(false)
            .x_dikey(true)
            .etkileşimler(crate::EtkileşimSeçenekleri::default().seçim_xy_yakınlaştır(true))
            .seri(SeriSeçenekleri::yeni("A").renk("red"))
            .seri(SeriSeçenekleri::yeni("B").renk("blue"));
        let veri = HizalıVeri::yeni(
            vec![0.0, 1.0, 2.0],
            vec![
                vec![Some(0.0), Some(1.0), Some(2.0)],
                vec![Some(2.0), Some(3.0), Some(4.0)],
            ],
        )?;
        Ok((seçenekler, veri))
    }

    fn test_cursor_snap_kartı() -> Result<(crate::GrafikSeçenekleri, HizalıVeri), UplotHatası> {
        let (seçenekler, veri) = test_çizgi_kartı(1_920, 600)?;
        Ok((seçenekler.imleç_ızgara_adımı(10.0), veri))
    }

    fn test_açıklama_kartı() -> Result<(crate::GrafikSeçenekleri, HizalıVeri), UplotHatası> {
        let açıklamalar = crate::AçıklamaDüzeni::default()
            .stil(crate::AçıklamaStili::yeni(
                "eqk",
                "rgb(76 175 80)",
                "rgb(76 175 80 / 20%)",
                crate::AçıklamaHizası::Alt,
            ))
            .işaret(
                crate::Açıklamaİşareti::yeni("eqk", 4.0, 4.0, "eqk_01")
                    .açıklama("Earthquake 01!"),
            );
        let seçenekler = crate::GrafikSeçenekleri::yeni(1_920, 600)?
            .x_zaman(false)
            .açıklamalar(açıklamalar)
            .seri(SeriSeçenekleri::yeni("Value").renk("red"));
        let veri = HizalıVeri::yeni((1..=30).map(f64::from).collect(), vec![vec![Some(0.0); 30]])?;
        Ok((seçenekler, veri))
    }

    fn önbelleğe_örnek_yol_ekle(önbellek: &mut GpuiYolÖnbelleği) {
        önbellek.yollar = vec![Some(örnek_önbellekli_yol())];
    }

    fn örnek_önbellekli_yol() -> ÖnbellekliGpuiYol {
        ÖnbellekliGpuiYol::yeni(Path::new(point(px(0.0), px(0.0))))
    }

    /// Tessellation önbelleği asıl kazanç: aynı komut için yol yalnız bir kez
    /// kurulur. Cihaz ölçeklemesini `Window::paint_path` her gönderimde
    /// yapar; ölçeklenmiş kopyayı saklamak upstream'de olmayan bir API
    /// gerektiriyordu ve ölçülen kazancı bakım yükünü karşılamadı.
    #[test]
    fn önbellekli_yol_aynı_komut_için_yeniden_kurulmaz() {
        let mut önbellek = GpuiYolÖnbelleği {
            yollar: vec![None],
            ikincil_yollar: vec![None],
            ..Default::default()
        };
        let ilk = önbellek.yol(0, point(px(0.0), px(0.0)), || {
            Some(Path::new(point(px(1.0), px(2.0))))
        });
        assert!(ilk.is_some());

        let yeniden_oluşturuldu = Cell::new(false);
        let ikinci = önbellek.yol(0, point(px(0.0), px(0.0)), || {
            yeniden_oluşturuldu.set(true);
            None
        });
        assert!(!yeniden_oluşturuldu.get(), "yol yeniden kurulmamalı");
        assert!(ikinci.is_some());
    }

    #[test]
    fn bin_retained_boya_hazırlığı_svg_serileştirmesini_çalıştırmaz() {
        crate::cizim::test_svg_serileştirme_sayacını_sıfırla();
        let sahne = yol_sahnesi("#ff0000", 2.0, 200.0);
        let sınırlar = Bounds::new(point(px(0.0), px(0.0)), size(px(320.0), px(180.0)));
        let mut önbellek = GpuiYolÖnbelleği::default();
        önbellek.yüzeyi_hazırla(&sahne, sınırlar);

        for _ in 0..1_000 {
            önbellek.yüzeyi_hazırla(&sahne, sınırlar);
            let yol = önbellek.yol(0, point(px(0.0), px(0.0)), || {
                Some(Path::new(point(px(1.0), px(2.0))))
            });
            assert!(yol.is_some());
        }

        assert_eq!(crate::cizim::test_svg_serileştirme_çağrıları(), 0);
    }

    #[test]
    fn hover_katmanı_ana_sahne_geometrisini_değiştirmez() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = test_çizgi_kartı(1_920, 600)?;
        let grafik = Grafik::yeni(seçenekler, veri)?;
        let mut bileşen = GpuiGrafik::yeni(grafik);
        let ana_komut_sayısı = bileşen.ana_sahne.komutlar().len();
        let başlangıç_revizyonları = bileşen.sahne_revizyonları();
        assert!(ana_komut_sayısı > 0);
        assert!(bileşen.etkileşim_sahnesi().komutlar().is_empty());

        let en_yakın = bileşen.grafik.en_yakın_noktalar(0.5);
        assert!(en_yakın.is_some());
        let Some((veri_x, seri_değerleri)) = en_yakın else {
            return Ok(());
        };
        let (sol, sağ, üst, alt) = bileşen.çizim_alanı();
        bileşen.imleç = Some(İmleçDurumu {
            fare: Nokta::yeni((sol + sağ) / 2.0, (üst + alt) / 2.0),
            veri_x,
            seri_x_değerleri: vec![Some(veri_x); seri_değerleri.len()],
            seri_değerleri,
            dağılım: None,
        });

        assert!(!bileşen.etkileşim_sahnesi().komutlar().is_empty());
        assert!(bileşen.etkileşim_sahnesini_yenile());
        let hover_revizyonları = bileşen.sahne_revizyonları();
        assert_eq!(hover_revizyonları.0, başlangıç_revizyonları.0);
        assert_eq!(
            hover_revizyonları.1,
            başlangıç_revizyonları.1.saturating_add(1)
        );
        assert!(!bileşen.etkileşim_sahnesini_yenile());
        assert_eq!(bileşen.sahne_revizyonları(), hover_revizyonları);
        assert_eq!(bileşen.ana_sahne.komutlar().len(), ana_komut_sayısı);
        Ok(())
    }

    #[test]
    fn scroll_sync_güncel_gpui_layout_kökenini_ana_sahneyi_değiştirmeden_kullanır()
    -> Result<(), UplotHatası> {
        let (seçenekler, veri) = test_çizgi_kartı(400, 200)?;
        let bileşen = GpuiGrafik::yeni(Grafik::yeni(seçenekler, veri)?);
        let ana_sahne = bileşen.ana_sahne.clone();
        let komut_sayısı = ana_sahne.komutlar().len();

        bileşen.çizim_sınırları.set(Some(Bounds::new(
            point(px(64.0), px(480.0)),
            size(px(400.0), px(200.0)),
        )));
        let önce = bileşen.sahne_konumu(point(px(264.0), px(580.0)));

        bileşen.çizim_sınırları.set(Some(Bounds::new(
            point(px(64.0), px(180.0)),
            size(px(400.0), px(200.0)),
        )));
        let sonra = bileşen.sahne_konumu(point(px(264.0), px(280.0)));

        assert_eq!(önce, Some(Nokta::yeni(200.0, 100.0)));
        assert_eq!(önce, sonra);
        assert!(Rc::ptr_eq(&bileşen.ana_sahne, &ana_sahne));
        assert_eq!(bileşen.ana_sahne.komutlar().len(), komut_sayısı);
        Ok(())
    }

    #[test]
    fn sine_stream_set_data_statik_gpui_yollarını_önbellekte_korur() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = test_çizgi_kartı(1_920, 600)?;
        let mut grafik = Grafik::yeni(seçenekler, veri)?;
        let eski = grafik.çiz();
        grafik.veriyi_ayarla(HizalıVeri::yeni(
            vec![0.0, 1.0, 2.0],
            vec![vec![Some(0.5), Some(0.25), Some(1.5)]],
        )?)?;
        let yeni = grafik.çiz();
        assert_eq!(eski.boyut(), yeni.boyut());

        let sınırlar = Bounds::new(point(px(0.0), px(0.0)), size(px(1_920.0), px(600.0)));
        let mut önbellek = GpuiYolÖnbelleği::default();
        önbellek.yüzeyi_hazırla(&eski, sınırlar);
        önbellek.yollar = vec![Some(örnek_önbellekli_yol()); eski.komutlar().len()];
        let korunan = önbellek.sahneyi_değiştir(&eski, &yeni);

        assert!(korunan > 0, "eksen/grid gibi statik yollar korunmalı");
        assert!(
            korunan < eski.komutlar().len(),
            "altı canlı seri ve dolguları yeni veriyle geçersizleşmeli"
        );
        Ok(())
    }

    #[test]
    fn y_kaydırılmış_hover_geometrisi_ile_ham_lejant_değeri_ayrılır() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = test_y_kaydırılmış_kartı()?;
        let grafik = Grafik::yeni(seçenekler, veri)?;
        let çözüm = grafik
            .imleç_çözümü(0.0, 1_000.0)
            .ok_or(UplotHatası::YetersizVeri { uzunluk: 0 })?;
        let seri_değerleri = çözüm
            .seriler
            .iter()
            .map(|örnek| örnek.map(|örnek| örnek.değer))
            .collect::<Vec<_>>();
        let üçüncü_geometri = seri_değerleri
            .get(2)
            .copied()
            .flatten()
            .ok_or(UplotHatası::YetersizVeri { uzunluk: 0 })?;
        let mut bileşen = GpuiGrafik::yeni(grafik);
        bileşen.imleç = Some(İmleçDurumu {
            fare: Nokta::yeni(64.0, 100.0),
            veri_x: çözüm.ortak_x,
            seri_x_değerleri: çözüm
                .seriler
                .iter()
                .map(|örnek| örnek.map(|örnek| örnek.x))
                .collect(),
            seri_değerleri,
            dağılım: None,
        });
        let (_, lejant) = bileşen
            .lejant_değerleri()
            .ok_or(UplotHatası::YetersizVeri { uzunluk: 0 })?;
        let üçüncü_lejant = lejant
            .get(2)
            .copied()
            .flatten()
            .ok_or(UplotHatası::YetersizVeri { uzunluk: 0 })?;
        assert_eq!(üçüncü_geometri, üçüncü_lejant + 20.0);
        Ok(())
    }

    #[test]
    fn resize_kalıcı_katmanları_ana_sahneden_ayırır_ve_oranları_korur() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = test_boyut_senkron_kartı()?;
        let grafik = Grafik::yeni(seçenekler, veri)?;
        let mut bileşen = GpuiGrafik::yeni(grafik);
        assert!(bileşen.imleç_kilitli);
        assert!(!bileşen.ana_sahne.test_svg().contains("#607d8b"));

        let oranları_oku = |bileşen: &GpuiGrafik| {
            let sahne = bileşen.etkileşim_sahnesi();
            let (sol, sağ, üst, alt) = bileşen.çizim_alanı();
            let imleç = sahne.komutlar().iter().find_map(|komut| match komut {
                Komut::KesikliÇizgi {
                    başlangıç,
                    bitiş,
                    renk,
                    ..
                } if renk == "#607d8b" && (başlangıç.x - bitiş.x).abs() <= f32::EPSILON => {
                    Some((
                        (başlangıç.x - sol) / (sağ - sol),
                        sahne.komutlar().iter().find_map(|aday| match aday {
                            Komut::KesikliÇizgi {
                                başlangıç,
                                bitiş,
                                renk,
                                ..
                            } if renk == "#607d8b"
                                && (başlangıç.y - bitiş.y).abs() <= f32::EPSILON =>
                            {
                                Some((başlangıç.y - üst) / (alt - üst))
                            }
                            _ => None,
                        })?,
                    ))
                }
                _ => None,
            });
            let seçim = sahne.komutlar().iter().find_map(|komut| match komut {
                Komut::Dikdörtgen {
                    konum,
                    genişlik,
                    yükseklik,
                    dolgu,
                    ..
                } if dolgu == "#00000012" => Some((
                    (konum.x - sol) / (sağ - sol),
                    (konum.y - üst) / (alt - üst),
                    *genişlik / (sağ - sol),
                    *yükseklik / (alt - üst),
                )),
                _ => None,
            });
            let hover = sahne.komutlar().iter().find_map(|komut| match komut {
                Komut::Daire { merkez, dolgu, .. } if dolgu == "red" => Some((
                    (merkez.x - sol) / (sağ - sol),
                    (merkez.y - üst) / (alt - üst),
                )),
                _ => None,
            });
            (imleç, seçim, hover)
        };

        let büyük = oranları_oku(&bileşen);
        assert!(büyük.0.is_some() && büyük.1.is_some() && büyük.2.is_some());
        assert!(bileşen.grafik.boyutu_ayarla(400, 400)?);
        let küçük = oranları_oku(&bileşen);
        assert_eq!(büyük, küçük);

        assert!(bileşen.grafik.seri_görünürlüğünü_ayarla(0, false)?);
        assert!(
            !bileşen
                .etkileşim_sahnesi()
                .komutlar()
                .iter()
                .any(|komut| matches!(komut, Komut::Daire { dolgu, .. } if dolgu == "red"))
        );
        Ok(())
    }

    #[test]
    fn dikey_x_yüzeyi_imleci_ve_xy_seçimini_fiziksel_yönelimde_çizer() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = test_dikey_kartı()?;
        assert!(seçenekler.etkileşimler.seçim_xy_yakınlaştır);
        let grafik = Grafik::yeni(seçenekler, veri)?;
        let mut bileşen = GpuiGrafik::yeni(grafik);
        let (sol, sağ, üst, alt) = bileşen.çizim_alanı();
        let veri_x = 1.0;
        let veri_y = 0.0;
        let x_oranı = bileşen.grafik.x_konum_oranı(veri_x).unwrap_or(0.5) as f32;
        let y_oranı = bileşen.grafik.seri_y_konum_oranı(0, veri_y).unwrap_or(0.5) as f32;
        bileşen.imleç = Some(İmleçDurumu {
            fare: Nokta::yeni(sol + y_oranı * (sağ - sol), alt - x_oranı * (alt - üst)),
            veri_x,
            seri_x_değerleri: vec![Some(veri_x), Some(veri_x)],
            seri_değerleri: vec![Some(veri_y), Some(2.0)],
            dağılım: None,
        });
        let imleç_sahnesi = bileşen.etkileşim_sahnesi();
        assert!(imleç_sahnesi.komutlar().iter().any(|komut| matches!(
            komut,
            Komut::KesikliÇizgi { başlangıç, bitiş, .. }
                if (başlangıç.y - bitiş.y).abs() <= f32::EPSILON
        )));
        assert!(imleç_sahnesi.komutlar().iter().any(|komut| matches!(
            komut,
            Komut::KesikliÇizgi { başlangıç, bitiş, .. }
                if (başlangıç.x - bitiş.x).abs() <= f32::EPSILON
        )));

        bileşen.imleç = None;
        bileşen.seçim = Some((Nokta::yeni(90.0, 120.0), Nokta::yeni(190.0, 320.0)));
        let seçim_sahnesi = bileşen.etkileşim_sahnesi();
        assert!(seçim_sahnesi.komutlar().iter().any(|komut| matches!(
            komut,
            Komut::Dikdörtgen { genişlik, yükseklik, .. }
                if (*genişlik - 100.0).abs() <= f32::EPSILON
                    && (*yükseklik - 200.0).abs() <= f32::EPSILON
        )));
        Ok(())
    }

    #[test]
    fn cursor_snap_duyarlı_yüzeyde_css_pikselini_ve_seçim_ucunu_korur() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = test_cursor_snap_kartı()?;
        let grafik = Grafik::yeni(seçenekler, veri)?;
        let bileşen = GpuiGrafik::yeni(grafik);
        bileşen.çizim_sınırları.set(Some(Bounds::new(
            point(px(0.0), px(0.0)),
            size(px(960.0), px(300.0)),
        )));
        let (sol, sağ, üst, alt) = bileşen.çizim_alanı();
        let ölçek = 0.5;
        let ham = Nokta::yeni(sol + 0.143 * (sağ - sol), üst + 0.167 * (alt - üst));
        let oturan = bileşen
            .imleç_ızgarasına_oturt(ham)
            .ok_or(UplotHatası::YetersizVeri { uzunluk: 0 })?;
        let css_x = (oturan.x - sol) * ölçek;
        let css_y = (oturan.y - üst) * ölçek;
        assert!((css_x / 10.0 - (css_x / 10.0).round()).abs() < 0.0001);
        assert!((css_y / 10.0 - (css_y / 10.0).round()).abs() < 0.0001);
        Ok(())
    }

    #[test]
    fn annotation_hover_yalnız_etkileşim_sahnesini_değiştirir() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = test_açıklama_kartı()?;
        let grafik = Grafik::yeni(seçenekler, veri)?;
        let mut bileşen = GpuiGrafik::yeni(grafik);
        let ana_sahne = bileşen.ana_sahne.clone();
        let (sol, sağ, _, alt) = bileşen.çizim_alanı();
        let deprem_x = sol + (sağ - sol) * (3.0 / 29.0);
        bileşen.açıklama_vuruşu = bileşen.grafik.açıklama_vuruşu_boyutta(
            bileşen.grafik.boyut().0,
            bileşen.grafik.boyut().1,
            deprem_x,
            alt - 9.0,
        );

        assert!(bileşen.açıklama_vuruşu.is_some());
        assert!(!bileşen.etkileşim_sahnesi().komutlar().is_empty());
        assert_eq!(bileşen.ana_sahne, ana_sahne);
        Ok(())
    }

    #[test]
    fn gpui_css_adlı_ve_modern_rgb_renklerini_korur() {
        let kırmızı = renk_çöz("red").to_rgb();
        assert!((kırmızı.r - 1.0).abs() < f32::EPSILON);
        assert!(kırmızı.g.abs() < f32::EPSILON);
        assert!(kırmızı.b.abs() < f32::EPSILON);
        assert!((kırmızı.a - 1.0).abs() < f32::EPSILON);

        let annotation = renk_çöz("rgb(255 193 7 / 20%)").to_rgb();
        assert!((annotation.r - 1.0).abs() < f32::EPSILON);
        assert!((annotation.g - 193.0 / 255.0).abs() < 0.0001);
        assert!((annotation.b - 7.0 / 255.0).abs() < 0.0001);
        assert!((annotation.a - 0.2).abs() < 0.0001);

        let eski = renk_çöz("rgba(255,0,0,0.1)").to_rgb();
        assert!((eski.a - 26.0 / 255.0).abs() < 0.0001);
        assert_eq!(renk_çöz("#0f08"), renk_çöz("rgba(0,255,0,0.5333333)"));
    }

    #[test]
    fn gpui_renk_önbelleği_aynı_css_rengini_bir_kez_saklar() {
        let mut önbellek = GpuiYolÖnbelleği::default();
        let ilk = önbellek.renk("rgb(255 193 7 / 20%)");
        let ikinci = önbellek.renk("rgb(255 193 7 / 20%)");
        assert_eq!(ilk, ikinci);
        assert_eq!(önbellek.renkler.len(), 1);
    }

    #[test]
    fn gpui_yol_önbelleği_renk_değişiminde_geometriyi_korur() {
        let eski = yol_sahnesi("#ff0000", 2.0, 200.0);
        let yeni = yol_sahnesi("#0000ff", 2.0, 200.0);
        let mut önbellek = GpuiYolÖnbelleği {
            sahne_boyutu: Some(eski.boyut()),
            sınırlar: None,
            yollar: Vec::new(),
            ikincil_yollar: Vec::new(),
            renkler: HashMap::new(),
            veri_komutları: Vec::new(),
            veri_komutu_çizim_alanı: None,
            raster: None,
        };
        önbelleğe_örnek_yol_ekle(&mut önbellek);

        assert_eq!(önbellek.sahneyi_değiştir(&eski, &yeni), 1);
        assert!(önbellek.yollar.first().is_some_and(Option::is_some));
    }

    #[test]
    fn gpui_yol_önbelleği_geometri_ve_kalınlık_değişiminde_geçersizleşir() {
        let eski = yol_sahnesi("#ff0000", 2.0, 200.0);
        for yeni in [
            yol_sahnesi("#ff0000", 3.0, 200.0),
            yol_sahnesi("#ff0000", 2.0, 240.0),
        ] {
            let mut önbellek = GpuiYolÖnbelleği {
                sahne_boyutu: Some(eski.boyut()),
                sınırlar: None,
                yollar: Vec::new(),
                ikincil_yollar: Vec::new(),
                renkler: HashMap::new(),
                veri_komutları: Vec::new(),
                veri_komutu_çizim_alanı: None,
                raster: None,
            };
            önbelleğe_örnek_yol_ekle(&mut önbellek);

            assert_eq!(önbellek.sahneyi_değiştir(&eski, &yeni), 0);
            assert!(önbellek.yollar.first().is_some_and(Option::is_none));
        }
    }

    #[test]
    fn yalnız_değişen_serinin_yolu_geçersizleşir() {
        let mut eski = yol_sahnesi("#ff0000", 2.0, 200.0);
        eski.ekle(Komut::Yol {
            parçalar: vec![vec![Nokta::yeni(10.0, 100.0), Nokta::yeni(200.0, 40.0)]],
            renk: "#00ff00".into(),
            kalınlık: 2.0,
        });
        let mut yeni = yol_sahnesi("#ff0000", 2.0, 200.0);
        yeni.ekle(Komut::Yol {
            parçalar: vec![vec![Nokta::yeni(10.0, 100.0), Nokta::yeni(240.0, 20.0)]],
            renk: "#00ff00".into(),
            kalınlık: 2.0,
        });
        let mut önbellek = GpuiYolÖnbelleği {
            sahne_boyutu: Some(eski.boyut()),
            sınırlar: None,
            yollar: vec![Some(örnek_önbellekli_yol()), Some(örnek_önbellekli_yol())],
            ikincil_yollar: vec![None, None],
            renkler: HashMap::new(),
            veri_komutları: Vec::new(),
            veri_komutu_çizim_alanı: None,
            raster: None,
        };

        assert_eq!(önbellek.sahneyi_değiştir(&eski, &yeni), 1);
        assert!(önbellek.yollar.first().is_some_and(Option::is_some));
        assert!(önbellek.yollar.get(1).is_some_and(Option::is_none));
    }

    #[test]
    fn önbellek_vektör_kapasitesini_sahne_geçişinde_yeniden_kullanır() {
        let eski = yol_sahnesi("#ff0000", 2.0, 200.0);
        let yeni = yol_sahnesi("#0000ff", 2.0, 200.0);
        let mut önbellek = GpuiYolÖnbelleği {
            sahne_boyutu: Some(eski.boyut()),
            sınırlar: None,
            yollar: Vec::with_capacity(32),
            ikincil_yollar: Vec::with_capacity(32),
            renkler: HashMap::new(),
            veri_komutları: Vec::new(),
            veri_komutu_çizim_alanı: None,
            raster: None,
        };
        önbelleğe_örnek_yol_ekle(&mut önbellek);
        let yol_kapasitesi = önbellek.yollar.capacity();
        let ikincil_kapasite = önbellek.ikincil_yollar.capacity();

        assert_eq!(önbellek.sahneyi_değiştir(&eski, &yeni), 1);
        assert_eq!(önbellek.yollar.capacity(), yol_kapasitesi);
        assert_eq!(önbellek.ikincil_yollar.capacity(), ikincil_kapasite);
    }

    #[test]
    fn gpui_yol_önbelleği_kaydırmada_geometriyi_korur_boyutta_geçersizleşir() {
        let sahne = yol_sahnesi("#ff0000", 2.0, 200.0);
        let ilk_sınırlar = Bounds::new(point(px(0.0), px(0.0)), size(px(320.0), px(180.0)));
        let kaydırılmış = Bounds::new(point(px(24.0), px(80.0)), size(px(320.0), px(180.0)));
        let yeniden_boyutlanmış =
            Bounds::new(point(px(24.0), px(80.0)), size(px(640.0), px(180.0)));
        let mut önbellek = GpuiYolÖnbelleği::default();
        önbellek.yüzeyi_hazırla(&sahne, ilk_sınırlar);
        önbelleğe_örnek_yol_ekle(&mut önbellek);

        önbellek.yüzeyi_hazırla(&sahne, kaydırılmış);
        assert!(önbellek.yollar.first().is_some_and(Option::is_some));

        önbellek.yüzeyi_hazırla(&sahne, yeniden_boyutlanmış);
        assert!(önbellek.yollar.first().is_some_and(Option::is_none));
    }

    #[test]
    fn kaydırma_duyarlı_boyut_güncellemesi_üretmez() {
        let ilk = Bounds::new(point(px(0.0), px(0.0)), size(px(320.0), px(180.0)));
        let kaydırılmış = Bounds::new(point(px(0.0), px(80.0)), size(px(320.0), px(180.0)));
        let yeniden_boyutlanmış = Bounds::new(point(px(0.0), px(80.0)), size(px(640.0), px(180.0)));

        assert!(duyarlı_boyut_güncellenmeli(None, ilk));
        assert!(!duyarlı_boyut_güncellenmeli(Some(ilk), kaydırılmış));
        assert!(duyarlı_boyut_güncellenmeli(
            Some(kaydırılmış),
            yeniden_boyutlanmış
        ));
    }

    #[test]
    fn wheel_modifierları_platformdan_ortak_eksenlere_eşlenir() {
        assert_eq!(
            GpuiGrafik::tekerlek_ekseni(false, false),
            TekerlekEkseni::İkisi
        );
        assert_eq!(GpuiGrafik::tekerlek_ekseni(true, false), TekerlekEkseni::X);
        assert_eq!(GpuiGrafik::tekerlek_ekseni(false, true), TekerlekEkseni::Y);
        assert_eq!(
            GpuiGrafik::tekerlek_ekseni(true, true),
            TekerlekEkseni::İkisi
        );
    }

    #[test]
    fn tekerlek_varsayılan_olarak_odak_gerektirir_ve_isteğe_bağlı_açılır() {
        let ayarlar = crate::EtkileşimSeçenekleri::default().tekerlek_etkileşimi(true);
        assert!(!GpuiGrafik::tekerlek_olayı_etkin(ayarlar, false));
        assert!(GpuiGrafik::tekerlek_olayı_etkin(ayarlar, true));

        let odaksız = ayarlar.tekerlek_odaksız_etkileşim(true);
        assert!(GpuiGrafik::tekerlek_olayı_etkin(odaksız, false));
    }
}
