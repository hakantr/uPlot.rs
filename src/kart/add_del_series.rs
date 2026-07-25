use super::{ortak_kart_etkileşimleri, veri_uretici::KanıtRastgele};
use crate::{GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası};

pub const ADD_DEL_SERIES_KANIT_TOHUMU: u32 = 0xADDE_1500;

pub const ADD_DEL_SERIES_KART_TANIM_ÖRNEĞİ: &str = r##"let (seçenekler, veri) = add_del_series_kartı()?;
let mut grafik = Grafik::yeni(seçenekler, veri)?;
grafik.seri_ekle(
    1,
    add_del_series_ek_seçeneği(0),
    add_del_series_ek_verisi(0),
)?;
// Olay sırası: AddSeries { seriesIdx: 2 } → SetData.
grafik.seri_sil(1)?;"##;

const DİNAMİK_SERİ_PALETİ: [(&str, &str, &str); 6] = [
    ("Orange", "#ffa500", "#ffa5001a"),
    ("Purple", "#8b5cf6", "#8b5cf61a"),
    ("Teal", "#0d9488", "#0d94881a"),
    ("Gold", "#ca8a04", "#ca8a041a"),
    ("Pink", "#db2777", "#db27771a"),
    ("Cyan", "#0891b2", "#0891b21a"),
];

/// İlk eklemede resmî turuncu stili korur; sonraki eklemelerde geliştiricinin
/// dinamik seri ayrımını görebilmesi için belirlenimci, dönen bir palet sunar.
pub fn add_del_series_ek_seçeneği(ekleme_sırası: u32) -> SeriSeçenekleri {
    let indeks = ekleme_sırası as usize % DİNAMİK_SERİ_PALETİ.len();
    let (etiket, renk, dolgu) =
        DİNAMİK_SERİ_PALETİ
            .get(indeks)
            .copied()
            .unwrap_or(("Orange", "#ffa500", "#ffa5001a"));
    SeriSeçenekleri::yeni(etiket).renk(renk).dolgu(dolgu)
}

fn seri_üret(rastgele: &mut KanıtRastgele) -> Vec<Option<f64>> {
    let değerler = (-10..=10).map(f64::from).collect::<Vec<_>>();
    (0..30)
        .map(|_| {
            let indeks = (rastgele.sonraki() * değerler.len() as f64).floor() as usize;
            değerler.get(indeks).copied()
        })
        .collect()
}

/// Resmî demonun her `Add Series` tıklamasında ürettiği turuncu seri için
/// belirlenimci kanıt akışı sağlar.
pub fn add_del_series_ek_verisi(ekleme_sırası: u32) -> Vec<Option<f64>> {
    let tohum = ADD_DEL_SERIES_KANIT_TOHUMU
        .wrapping_add(0x9E37_79B9_u32.wrapping_mul(ekleme_sırası.wrapping_add(1)));
    seri_üret(&mut KanıtRastgele::yeni(tohum))
}

/// `demos/add-del-series.html` başlangıç grafiğini üretir.
pub fn add_del_series_kartı() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let x = (1..=30).map(f64::from).collect::<Vec<_>>();
    let mut rastgele = KanıtRastgele::yeni(ADD_DEL_SERIES_KANIT_TOHUMU);
    let veri = HizalıVeri::yeni(x, (0..3).map(|_| seri_üret(&mut rastgele)).collect())?;
    let seçenekler = GrafikSeçenekleri::yeni(1_920, 600)?
        .başlık("Area Fill")
        .x_zaman(false)
        .seri_yaşam_döngüsünü_izle(true)
        .etkileşimler(ortak_kart_etkileşimleri())
        .seri(
            SeriSeçenekleri::yeni("Red")
                .renk("#ff0000")
                .dolgu("#ff00001a"),
        )
        .seri(
            SeriSeçenekleri::yeni("Green")
                .renk("#008000")
                .dolgu("#00ff001a"),
        )
        .seri(
            SeriSeçenekleri::yeni("Blue")
                .renk("#0000ff")
                .dolgu("#0000ff1a"),
        );
    Ok((seçenekler, veri))
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, SeriYaşamDöngüsüOlayı};

    #[test]
    fn kaynak_serileri_atomik_eklenip_silinir() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = add_del_series_kartı()?;
        let mut grafik = Grafik::yeni(seçenekler, veri)?;
        let kimlik = grafik.kimlik();
        assert_eq!(grafik.seri_seçenekleri().len(), 3);
        assert_eq!(
            grafik.seri_yaşam_döngüsü_olaylarını_al(),
            (0..=3)
                .map(|seri_indeksi| SeriYaşamDöngüsüOlayı::Eklendi {
                    seri_indeksi,
                    başlangıç: true,
                })
                .collect::<Vec<_>>()
        );
        assert!(grafik.seçim_yakınlaştır(0.2, 0.8)?);
        assert!(grafik.yakınlaştırılmış());
        let (_, yenilenen_veri) = add_del_series_kartı()?;
        grafik.veriyi_ayarla(yenilenen_veri)?;
        assert_eq!(grafik.kimlik(), kimlik);
        assert!(!grafik.yakınlaştırılmış());
        grafik.seri_ekle(
            1,
            add_del_series_ek_seçeneği(0),
            add_del_series_ek_verisi(0),
        )?;
        assert_eq!(grafik.kimlik(), kimlik);
        assert_eq!(grafik.seri_seçenekleri().len(), 4);
        assert_eq!(
            grafik
                .seri_seçenekleri()
                .get(1)
                .map(|seri| seri.etiket.as_str()),
            Some("Orange")
        );
        assert_eq!(
            grafik.seri_yaşam_döngüsü_olaylarını_al(),
            vec![
                SeriYaşamDöngüsüOlayı::Eklendi {
                    seri_indeksi: 2,
                    başlangıç: false,
                },
                SeriYaşamDöngüsüOlayı::VeriAyarlandı { seri_sayısı: 5 },
            ]
        );
        assert!(grafik.çiz().svg().contains("#ffa500"));
        grafik.seri_sil(1)?;
        assert_eq!(grafik.kimlik(), kimlik);
        assert_eq!(grafik.seri_seçenekleri().len(), 3);
        assert_eq!(
            grafik.seri_yaşam_döngüsü_olaylarını_al(),
            vec![
                SeriYaşamDöngüsüOlayı::Silindi { seri_indeksi: 2 },
                SeriYaşamDöngüsüOlayı::VeriAyarlandı { seri_sayısı: 4 },
            ]
        );
        assert!(matches!(
            grafik.seri_sil(9),
            Err(UplotHatası::GeçersizSeriİndeksi { .. })
        ));
        assert!(matches!(
            grafik.seri_ekle(
                9,
                SeriSeçenekleri::yeni("Geçersiz"),
                add_del_series_ek_verisi(1),
            ),
            Err(UplotHatası::GeçersizSeriİndeksi { .. })
        ));
        let önceki_veri = grafik.hizalı_veri().clone();
        let önceki_seriler = grafik.seri_seçenekleri().to_vec();
        assert!(
            grafik
                .seri_ekle(1, add_del_series_ek_seçeneği(2), vec![Some(1.0); 29],)
                .is_err()
        );
        assert_eq!(grafik.kimlik(), kimlik);
        assert_eq!(grafik.hizalı_veri(), &önceki_veri);
        assert_eq!(grafik.seri_seçenekleri(), önceki_seriler);
        assert!(grafik.seri_yaşam_döngüsü_olayları().is_empty());
        Ok(())
    }

    #[test]
    fn tekrarlı_ekleme_silme_sırası_veriyi_ve_renkleri_korur() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = add_del_series_kartı()?;
        let kaynak_seriler = veri.seriler().to_vec();
        let mut grafik = Grafik::yeni(seçenekler, veri)?;
        grafik.seri_yaşam_döngüsü_olaylarını_al();

        let ilk = add_del_series_ek_verisi(0);
        let ikinci = add_del_series_ek_verisi(1);
        grafik.seri_ekle(1, add_del_series_ek_seçeneği(0), ilk.clone())?;
        grafik.seri_ekle(1, add_del_series_ek_seçeneği(1), ikinci.clone())?;
        assert_eq!(
            grafik
                .seri_seçenekleri()
                .iter()
                .map(|seri| seri.renk.as_str())
                .collect::<Vec<_>>(),
            vec!["#ff0000", "#8b5cf6", "#ffa500", "#008000", "#0000ff"]
        );
        assert_eq!(grafik.hizalı_veri().seriler().get(1), Some(&ikinci));
        assert_eq!(grafik.hizalı_veri().seriler().get(2), Some(&ilk));
        assert_eq!(grafik.hizalı_veri().seriler().get(3), kaynak_seriler.get(1));

        grafik.seri_sil(1)?;
        grafik.seri_sil(1)?;
        assert_eq!(grafik.hizalı_veri().seriler(), kaynak_seriler.as_slice());
        assert_eq!(
            grafik
                .seri_seçenekleri()
                .iter()
                .map(|seri| seri.renk.as_str())
                .collect::<Vec<_>>(),
            vec!["#ff0000", "#008000", "#0000ff"]
        );
        Ok(())
    }
}
