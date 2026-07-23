use super::Grafik;
use crate::{Aralık, Komut, MetinHizası, Nokta, Sahne, TimelineDüzeni};

impl Grafik {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn timeline_çiz(
        &self,
        sahne: &mut Sahne,
        düzen: &TimelineDüzeni,
        x_aralığı: Aralık,
        sol: f32,
        sağ: f32,
        üst: f32,
        alt: f32,
    ) {
        let seri_sayısı = düzen.seri_etiketleri.len();
        if seri_sayısı == 0 {
            return;
        }
        let genişlik = sağ - sol;
        let yükseklik = alt - üst;
        let şerit_yüksekliği = yükseklik * düzen.şerit_oranı / seri_sayısı as f32;
        let şerit_boşluğu = if seri_sayısı > 1 {
            yükseklik * (1.0 - düzen.şerit_oranı) / seri_sayısı.saturating_sub(1) as f32
        } else {
            0.0
        };

        for (indeks, etiket) in düzen.seri_etiketleri.iter().enumerate() {
            let şerit_üstü = üst + indeks as f32 * (şerit_yüksekliği + şerit_boşluğu);
            sahne.ekle(Komut::Metin {
                konum: Nokta::yeni(sol - 15.0, şerit_üstü + şerit_yüksekliği / 2.0 + 5.0),
                içerik: etiket.clone(),
                renk: "#4b5563".to_string(),
                boyut: 12.0,
                hiza: MetinHizası::Bitiş,
            });
        }

        for hücre in &düzen.hücreler {
            if hücre.bitiş < x_aralığı.en_az || hücre.başlangıç > x_aralığı.en_çok {
                continue;
            }
            let mut x0 = self.x_konumu(x_aralığı, hücre.başlangıç, sol, genişlik);
            let mut x1 = self.x_konumu(x_aralığı, hücre.bitiş, sol, genişlik);
            if let Some(azami) = hücre.azami_genişlik
                && x1 - x0 > azami
            {
                let merkez = (x0 + x1) / 2.0;
                x0 = merkez - azami / 2.0;
                x1 = merkez + azami / 2.0;
            }
            x0 = x0.clamp(sol, sağ);
            x1 = x1.clamp(sol, sağ);
            if x1 <= x0 {
                continue;
            }
            let şerit_üstü =
                üst + hücre.seri_indeksi as f32 * (şerit_yüksekliği + şerit_boşluğu);
            let kalınlık = hücre.çizgi_kalınlığı.min((x1 - x0) / 2.0);
            sahne.ekle(Komut::Dikdörtgen {
                konum: Nokta::yeni(x0, şerit_üstü),
                genişlik: x1 - x0,
                yükseklik: şerit_yüksekliği,
                dolgu: hücre.dolgu.clone(),
                çizgi: hücre.çizgi.clone(),
                kalınlık,
            });
            let metin_genişliği = hücre.değer.chars().count() as f32 * 7.0 + kalınlık * 2.0 + 4.0;
            if x1 - x0 >= metin_genişliği {
                sahne.ekle(Komut::Metin {
                    konum: Nokta::yeni(
                        x0 + kalınlık + 2.0,
                        şerit_üstü + şerit_yüksekliği / 2.0 + 5.0,
                    ),
                    içerik: hücre.değer.clone(),
                    renk: "#111111".to_string(),
                    boyut: 14.0,
                    hiza: MetinHizası::Başlangıç,
                });
            }
        }
    }
}
