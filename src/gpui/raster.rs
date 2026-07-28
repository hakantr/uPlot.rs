//! Yoğun yüzeyleri kare başına yeniden gönderilen geometri yerine tek
//! sprite'a indiren CPU rasterleştirici.
//!
//! GPUI retained bir sahne modeli kullanır: görünen her primitive'in
//! geometrisi her karenin sahnesine kopyalanır ve GPU'ya yüklenir. Maliyet
//! köşe sayısıyla doğrusaldır ve **her kare** ödenir. uPlot'un canvas'ı ise
//! `_commit()` başına bir kez çizer ve piksellerini korur; imleç hareketinde
//! canvas'a hiç dokunmaz.
//!
//! Bu modül aynı davranışı GPUI içinde kurar. Köşe bütçesini aşan bir yüzey,
//! içeriği kayıpsız rasterleştirilebiliyorsa bir kez BGRA tamponuna çizilir
//! ve kareler `Window::paint_image` ile tek sprite gönderir. Yenileme
//! tetikleyicisi uPlot'un `_commit()` eşleniğidir: veri, ölçek/görünüm,
//! yüzey boyutu veya cihaz piksel oranı.
//!
//! Kapsam bilinçli olarak dar: yalnız eksen hizalı dolu dikdörtgenler.
//! Onlar için piksel kapsaması analitik olarak hesaplanabildiğinden çıktı
//! vektör yoluyla aynıdır. Kenar yumuşatmalı çizgi ve eğri için CPU
//! rasterleştirme, lyon + GPU'nun yaptığı işi daha yavaş ve daha düşük
//! kalitede tekrarlamak olurdu; o yüzeyler vektör yolunda kalır.

use std::sync::Arc;

use ::gpui::{Hsla, RenderImage};
use image::{Frame, RgbaImage};

use crate::cizim::{Komut, Sahne};

/// Bu köşe sayısının üstündeki yüzeyler raster adayıdır.
///
/// Ölçüm: kart bazlı kök render bütçesinde LatencyHeatmap yüzeyleri ~51K
/// köşeyle 4,93 ms'ye çıkarken, ~1K köşeli yüzeyler 0,7–1,0 ms bandında
/// kalıyor. Eşik o iki bandın arasına, hafif kartlara hiç dokunmayacak
/// şekilde konumlandı.
pub(super) const RASTER_NOKTA_EŞİĞİ: usize = 8_000;

/// Eksen hizalı dikdörtgen: sol, sağ, üst, alt.
type Dikdörtgen = [f32; 4];

/// Sahne kayıpsız rasterleştirilebiliyorsa taşıdığı nokta sayısını verir.
///
/// `None` dönerse yüzey vektör yolunda kalır.
pub(super) fn nokta_sayısı_rasterlenebilirse(sahne: &Sahne) -> Option<usize> {
    let mut nokta = 0usize;
    for komut in sahne.komutlar() {
        match komut {
            Komut::ArkaPlan { .. } => nokta += 1,
            Komut::Alan { çokgenler, .. } => {
                for çokgen in çokgenler {
                    dikdörtgene_çöz(çokgen)?;
                    nokta += çokgen.len();
                }
            }
            // Dolgusuz kenarlık CPU tarafında ayrıca rasterleştirme ister;
            // kapsam dışı tutuluyor.
            Komut::Dikdörtgen { kalınlık, .. } if *kalınlık <= 0.0 => nokta += 1,
            _ => return None,
        }
    }
    Some(nokta)
}

/// Dört noktalı bir çokgen eksen hizalı dikdörtgense sınırlarını verir.
fn dikdörtgene_çöz(çokgen: &[crate::Nokta]) -> Option<Dikdörtgen> {
    let [a, b, c, d] = çokgen else {
        return None;
    };
    let hizalı = (a.y - b.y).abs() <= f32::EPSILON
        && (b.x - c.x).abs() <= f32::EPSILON
        && (c.y - d.y).abs() <= f32::EPSILON
        && (d.x - a.x).abs() <= f32::EPSILON;
    if !hizalı {
        return None;
    }
    Some([a.x.min(c.x), a.x.max(c.x), a.y.min(c.y), a.y.max(c.y)])
}

/// Sahneyi verilen fiziksel çözünürlükte BGRA tamponuna çizer.
///
/// `ölçek`, sahne koordinatlarından fiziksel piksele geçiş çarpanıdır.
/// `renk_çöz`, sahne renk kodlarını çözen çağrıdır; adaptörün renk önbelleğini
/// paylaşır.
pub(super) fn rasterleştir(
    sahne: &Sahne,
    fiziksel_genişlik: u32,
    fiziksel_yükseklik: u32,
    ölçek: f32,
    mut renk_çöz: impl FnMut(&str) -> Hsla,
) -> Option<Arc<RenderImage>> {
    if fiziksel_genişlik == 0 || fiziksel_yükseklik == 0 {
        return None;
    }
    let piksel_sayısı = (fiziksel_genişlik as usize).checked_mul(fiziksel_yükseklik as usize)?;
    // Düz alfalı BGRA; `RenderImage` bu düzeni bekliyor.
    let mut tampon = vec![0u8; piksel_sayısı.checked_mul(4)?];

    for komut in sahne.komutlar() {
        match komut {
            Komut::ArkaPlan { renk } => {
                let (r, g, b, a) = bileşenler(renk_çöz(renk));
                dikdörtgen_harmanla(
                    &mut tampon,
                    fiziksel_genişlik,
                    fiziksel_yükseklik,
                    [
                        0.0,
                        fiziksel_genişlik as f32,
                        0.0,
                        fiziksel_yükseklik as f32,
                    ],
                    (r, g, b, a),
                );
            }
            Komut::Alan { çokgenler, dolgu } => {
                let (r, g, b, a) = bileşenler(renk_çöz(dolgu));
                if a <= 0.0 {
                    continue;
                }
                for çokgen in çokgenler {
                    let [sol, sağ, üst, alt] = dikdörtgene_çöz(çokgen)?;
                    dikdörtgen_harmanla(
                        &mut tampon,
                        fiziksel_genişlik,
                        fiziksel_yükseklik,
                        [sol * ölçek, sağ * ölçek, üst * ölçek, alt * ölçek],
                        (r, g, b, a),
                    );
                }
            }
            Komut::Dikdörtgen {
                konum,
                genişlik,
                yükseklik,
                dolgu,
                kalınlık,
                ..
            } if *kalınlık <= 0.0 => {
                let (r, g, b, a) = bileşenler(renk_çöz(dolgu));
                if a <= 0.0 {
                    continue;
                }
                dikdörtgen_harmanla(
                    &mut tampon,
                    fiziksel_genişlik,
                    fiziksel_yükseklik,
                    [
                        konum.x * ölçek,
                        (konum.x + genişlik) * ölçek,
                        konum.y * ölçek,
                        (konum.y + yükseklik) * ölçek,
                    ],
                    (r, g, b, a),
                );
            }
            _ => return None,
        }
    }

    let görsel = RgbaImage::from_raw(fiziksel_genişlik, fiziksel_yükseklik, tampon)?;
    Some(Arc::new(RenderImage::new(vec![Frame::new(görsel)])))
}

fn bileşenler(renk: Hsla) -> (f32, f32, f32, f32) {
    let rgb = renk.to_rgb();
    (rgb.r, rgb.g, rgb.b, rgb.a)
}

/// Eksen hizalı dikdörtgeni analitik piksel kapsamasıyla harmanlar.
///
/// Kapsama, pikselin dikdörtgenle kesişme alanıdır; eksen hizalı geometride
/// bu tam değerdir, dolayısıyla kenar yumuşatma vektör yoluyla eşdeğerdir.
fn dikdörtgen_harmanla(
    tampon: &mut [u8],
    genişlik: u32,
    yükseklik: u32,
    [sol, sağ, üst, alt]: Dikdörtgen,
    (r, g, b, a): (f32, f32, f32, f32),
) {
    let sol_k = sol.max(0.0);
    let sağ_k = sağ.min(genişlik as f32);
    let üst_k = üst.max(0.0);
    let alt_k = alt.min(yükseklik as f32);
    if sağ_k <= sol_k || alt_k <= üst_k {
        return;
    }
    let ilk_x = sol_k.floor() as u32;
    let son_x = (sağ_k.ceil() as u32).min(genişlik);
    let ilk_y = üst_k.floor() as u32;
    let son_y = (alt_k.ceil() as u32).min(yükseklik);

    for y in ilk_y..son_y {
        let dikey = ((alt_k.min((y + 1) as f32)) - (üst_k.max(y as f32))).max(0.0);
        if dikey <= 0.0 {
            continue;
        }
        for x in ilk_x..son_x {
            let yatay = ((sağ_k.min((x + 1) as f32)) - (sol_k.max(x as f32))).max(0.0);
            if yatay <= 0.0 {
                continue;
            }
            let kapsama = yatay * dikey;
            if kapsama <= 0.0 {
                continue;
            }
            let kaynak_alfa = a * kapsama;
            if kaynak_alfa <= 0.0 {
                continue;
            }
            let taban = ((y as usize) * (genişlik as usize) + x as usize) * 4;
            let Some(piksel) = tampon
                .get_mut(taban..taban + 4)
                .and_then(|dilim| <&mut [u8; 4]>::try_from(dilim).ok())
            else {
                continue;
            };
            harmanla(piksel, (r, g, b), kaynak_alfa);
        }
    }
}

/// Düz alfalı BGRA piksele source-over harmanlama.
fn harmanla(piksel: &mut [u8; 4], (r, g, b): (f32, f32, f32), kaynak_alfa: f32) {
    let hedef_alfa = f32::from(piksel[3]) / 255.0;
    let sonuç_alfa = kaynak_alfa + hedef_alfa * (1.0 - kaynak_alfa);
    if sonuç_alfa <= 0.0 {
        *piksel = [0; 4];
        return;
    }
    let karıştır = |kaynak: f32, hedef: u8| {
        let hedef = f32::from(hedef) / 255.0;
        let değer = (kaynak * kaynak_alfa + hedef * hedef_alfa * (1.0 - kaynak_alfa)) / sonuç_alfa;
        (değer.clamp(0.0, 1.0) * 255.0).round() as u8
    };
    // BGRA düzeni.
    *piksel = [
        karıştır(b, piksel[0]),
        karıştır(g, piksel[1]),
        karıştır(r, piksel[2]),
        (sonuç_alfa.clamp(0.0, 1.0) * 255.0).round() as u8,
    ];
}
