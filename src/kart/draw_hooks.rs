use super::ortak_kart_etkileşimleri;
use crate::{
    Aralık, GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası, ÇizimKancasıDüzeni,
};

pub const DRAW_HOOKS_KART_TANIM_ÖRNEĞİ: &str = r##"let (seçenekler, veri) = draw_hooks_kartı()?;
let grafik = Grafik::yeni(seçenekler, veri)?;"##;

pub fn draw_hooks_kartı() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let veri = HizalıVeri::yeni(
        vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 9.0, 10.0],
        vec![
            vec![40.0, 43.0, 60.0, 65.0, 71.0, 73.0, 40.0, 22.0, 20.0],
            vec![30.0, 23.0, 35.0, 27.0, 11.0, 13.0, 30.0, 32.0, 30.0],
            vec![15.0, 13.0, 39.0, 17.0, 21.0, 53.0, 10.0, 11.0, 13.0],
        ]
        .into_iter()
        .map(|seri| seri.into_iter().map(Some).collect())
        .collect(),
    )?;
    let kancalar = ÇizimKancasıDüzeni::default()
        .gradyan("#666666", "#000000")
        .seri_medyanları(50.0, 6.0)
        .yıldız_noktalar(6, 8.0, 4.0)
        .çizim_süresi_metni(true)
        .çizim_süresi_stili("#ffffff", 12.0);
    let seçenekler = GrafikSeçenekleri::yeni(600, 400)?
        .başlık("Draw Hooks")
        .x_zaman(false)
        .y_aralığı(Aralık::yeni(0.0, 80.0)?)
        .ızgara_rengi("#000000")
        .eksen_göstergeleri(true)
        .x_eksen_rengi("#000000")
        .birincil_y_eksen_rengi("#000000")
        .x_eksen_çentik_uzunluğu(10.0)
        .birincil_y_eksen_çentik_uzunluğu(10.0)
        .imleç_noktalarını_göster(false)
        .çizim_kancaları(kancalar)
        .etkileşimler(ortak_kart_etkileşimleri())
        .seri(SeriSeçenekleri::yeni("blah").renk("#ff3333"))
        .seri(SeriSeçenekleri::yeni("yerp").renk("#33ccff"))
        .seri(SeriSeçenekleri::yeni("zort").renk("#ffcc33"));
    Ok((seçenekler, veri))
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{GradyanRenkDurağı, Grafik, Komut, Nokta};

    #[test]
    fn kaynak_veri_ve_dört_çizim_kancası_korunur() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = draw_hooks_kartı()?;
        assert_eq!(veri.x(), &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 9.0, 10.0]);
        assert_eq!(
            veri.seriler(),
            &[
                vec![
                    Some(40.0),
                    Some(43.0),
                    Some(60.0),
                    Some(65.0),
                    Some(71.0),
                    Some(73.0),
                    Some(40.0),
                    Some(22.0),
                    Some(20.0),
                ],
                vec![
                    Some(30.0),
                    Some(23.0),
                    Some(35.0),
                    Some(27.0),
                    Some(11.0),
                    Some(13.0),
                    Some(30.0),
                    Some(32.0),
                    Some(30.0),
                ],
                vec![
                    Some(15.0),
                    Some(13.0),
                    Some(39.0),
                    Some(17.0),
                    Some(21.0),
                    Some(53.0),
                    Some(10.0),
                    Some(11.0),
                    Some(13.0),
                ],
            ]
        );
        assert!(seçenekler.eksen_göstergeleri);
        assert_eq!(seçenekler.x_eksen_rengi, "#000000");
        assert_eq!(seçenekler.birincil_y_eksen_rengi, "#000000");
        assert_eq!(seçenekler.x_eksen_çentik_uzunluğu, 10.0);
        assert_eq!(seçenekler.birincil_y_eksen_çentik_uzunluğu, 10.0);
        assert!(!seçenekler.imleç_noktaları_görünür);

        let sahne = Grafik::yeni(seçenekler, veri)?.çiz();
        let komutlar = sahne.komutlar();
        let son = komutlar.last();
        assert!(matches!(
            son,
            Some(Komut::Metin {
                içerik,
                renk,
                boyut,
                ..
            }) if içerik.starts_with("Time to Draw: ")
                && içerik.ends_with("ms")
                && renk == "#ffffff"
                && (*boyut - 12.0).abs() <= f32::EPSILON
        ));
        let gradyanlar = komutlar
            .iter()
            .filter_map(|komut| match komut {
                Komut::GradyanAlan {
                    çokgenler, gradyan
                } => Some((çokgenler, gradyan)),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(gradyanlar.len(), 1);
        for (çokgenler, gradyan) in gradyanlar {
            assert_eq!(çokgenler.len(), 1);
            assert!(çokgenler.iter().all(|çokgen| çokgen.len() == 4));
            assert_eq!(gradyan.başlangıç, Nokta::yeni(0.0, 0.0));
            assert_eq!(
                gradyan.duraklar,
                vec![
                    GradyanRenkDurağı {
                        oran: 0.0,
                        renk: "#666666".to_string(),
                    },
                    GradyanRenkDurağı {
                        oran: 1.0,
                        renk: "#000000".to_string(),
                    },
                ]
            );
        }
        assert_eq!(
            komutlar
                .iter()
                .filter(|komut| matches!(komut, Komut::Alan { .. }))
                .count(),
            27
        );
        for (seri_rengi, medyan_rengi) in [
            ("#ff3333", "#ff333333"),
            ("#33ccff", "#33ccff33"),
            ("#ffcc33", "#ffcc3333"),
        ] {
            assert!(komutlar.iter().any(|komut| matches!(
                komut,
                Komut::Çizgi { renk: çizgi, kalınlık, .. }
                    if çizgi == medyan_rengi && (*kalınlık - 50.0).abs() <= f32::EPSILON
            )));
            let yol = komutlar
                .iter()
                .position(|komut| matches!(komut, Komut::Yol { renk, .. } if renk == seri_rengi));
            let ilk_yıldız = komutlar.iter().position(
                |komut| matches!(komut, Komut::Alan { dolgu, .. } if dolgu == seri_rengi),
            );
            let medyan = komutlar.iter().position(|komut| {
                matches!(
                    komut,
                    Komut::Çizgi {
                        renk,
                        kalınlık,
                        ..
                    } if renk == medyan_rengi && (*kalınlık - 50.0).abs() <= f32::EPSILON
                )
            });
            assert!(yol < ilk_yıldız && ilk_yıldız < medyan);
        }
        for çokgenler in komutlar.iter().filter_map(|komut| match komut {
            Komut::Alan { çokgenler, .. } => Some(çokgenler),
            _ => None,
        }) {
            assert_eq!(çokgenler.len(), 1);
            for çokgen in çokgenler {
                assert_eq!(çokgen.len(), 12);
                let merkez_y =
                    çokgen.iter().map(|nokta| nokta.y).sum::<f32>() / çokgen.len() as f32;
                assert!(
                    çokgen
                        .first()
                        .is_some_and(|ilk| (merkez_y - ilk.y - 8.0).abs() < 0.01)
                );
            }
        }
        Ok(())
    }

    #[test]
    fn set_data_medyan_önbelleğini_yeniler() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = draw_hooks_kartı()?;
        let mut grafik = Grafik::yeni(seçenekler, veri)?;
        let önceki_y = grafik
            .çiz()
            .komutlar()
            .iter()
            .find_map(|komut| match komut {
                Komut::Çizgi {
                    başlangıç,
                    renk,
                    kalınlık,
                    ..
                } if renk == "#ff333333" && (*kalınlık - 50.0).abs() <= f32::EPSILON => {
                    Some(başlangıç.y)
                }
                _ => None,
            });
        assert!(önceki_y.is_some(), "ilk medyan çizgisi bulunamadı");
        grafik.veriyi_ayarla(HizalıVeri::yeni(
            vec![1.0, 2.0, 3.0],
            vec![
                vec![Some(10.0), Some(10.0), Some(10.0)],
                vec![Some(20.0), Some(20.0), Some(20.0)],
                vec![Some(70.0), Some(70.0), Some(70.0)],
            ],
        )?)?;
        let yeni_y = grafik
            .çiz()
            .komutlar()
            .iter()
            .find_map(|komut| match komut {
                Komut::Çizgi {
                    başlangıç,
                    renk,
                    kalınlık,
                    ..
                } if renk == "#ff333333" && (*kalınlık - 50.0).abs() <= f32::EPSILON => {
                    Some(başlangıç.y)
                }
                _ => None,
            });
        assert!(yeni_y.is_some(), "yenilenen medyan çizgisi bulunamadı");
        assert_ne!(önceki_y, yeni_y);
        Ok(())
    }
}
