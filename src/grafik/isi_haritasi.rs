use super::Grafik;
use crate::{Aralık, IsıHaritasıDüzeni, IsıHücresiBoyutu, Komut, Nokta, Sahne};
use std::collections::BTreeMap;

impl Grafik {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn ısı_haritasını_çiz(
        &self,
        sahne: &mut Sahne,
        düzen: &IsıHaritasıDüzeni,
        x_aralığı: Aralık,
        y_aralığı: Aralık,
        sol: f32,
        sağ: f32,
        üst: f32,
        alt: f32,
    ) {
        let genişlik = sağ - sol;
        let yükseklik = alt - üst;
        let mut renk_grupları = BTreeMap::<String, Vec<Vec<Nokta>>>::new();
        for hücre in &düzen.hücreler {
            let merkez_x = self.x_konumu(x_aralığı, hücre.x, sol, genişlik);
            let merkez_y = alt - self.y_konumu("y", y_aralığı, hücre.y, 0.0, yükseklik);
            let hücre_genişliği = eksen_boyutu(hücre.genişlik, hücre.x, |değer| {
                self.x_konumu(x_aralığı, değer, sol, genişlik)
            });
            let hücre_yüksekliği = eksen_boyutu(hücre.yükseklik, hücre.y, |değer| {
                alt - self.y_konumu("y", y_aralığı, değer, 0.0, yükseklik)
            });
            if !merkez_x.is_finite()
                || !merkez_y.is_finite()
                || !hücre_genişliği.is_finite()
                || !hücre_yüksekliği.is_finite()
            {
                continue;
            }
            let x0 = (merkez_x - hücre_genişliği / 2.0).clamp(sol, sağ);
            let x1 = (merkez_x + hücre_genişliği / 2.0).clamp(sol, sağ);
            let y0 = (merkez_y - hücre_yüksekliği / 2.0).clamp(üst, alt);
            let y1 = (merkez_y + hücre_yüksekliği / 2.0).clamp(üst, alt);
            if x1 <= x0 || y1 <= y0 {
                continue;
            }
            renk_grupları
                .entry(hücre.renk.clone())
                .or_default()
                .push(vec![
                    Nokta::yeni(x0, y0),
                    Nokta::yeni(x1, y0),
                    Nokta::yeni(x1, y1),
                    Nokta::yeni(x0, y1),
                ]);
        }
        for (dolgu, çokgenler) in renk_grupları {
            sahne.ekle(Komut::Alan { çokgenler, dolgu });
        }
    }
}

fn eksen_boyutu(boyut: IsıHücresiBoyutu, merkez: f64, konum: impl Fn(f64) -> f32) -> f32 {
    match boyut {
        IsıHücresiBoyutu::Piksel(piksel) => piksel.abs(),
        IsıHücresiBoyutu::Veri(veri) => {
            let yarı = veri.abs() / 2.0;
            (konum(merkez + yarı) - konum(merkez - yarı)).abs()
        }
    }
}
