use super::ortak_kart_etkileşimleri;
use super::veri_uretici::KanıtRastgele;
use crate::{GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası};

pub const SCROLL_SYNC_KANIT_TOHUMU: u32 = 0x5C20_1144;
pub const SCROLL_SYNC_KART_TANIM_ÖRNEĞİ: &str = r##"let (seçenekler, veri) = scroll_sync_kartı()?;
let grafik = Grafik::yeni(seçenekler, veri)?;

// GPUI masaüstü/web adaptörü kaydırma veya yerleşim değiştiğinde yüzey
// dikdörtgenini yeniler; istemci → sahne dönüşümü çekirdekte çözülür."##;

/// `demos/scroll-sync.html` içindeki `.syncRect()` örneğini aynı 30 x değeri,
/// −10…10 havuzu ve üç seriyle üretir. Kaynaktaki `Math.random()` yerine
/// tekrarlanabilir görsel/test kanıtı için sabit bir Rust tohumu kullanılır.
pub fn scroll_sync_kartı() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let x = (1..=30).map(f64::from).collect::<Vec<_>>();
    let değer_havuzu = (-10..=10).map(f64::from).collect::<Vec<_>>();
    let mut rng = KanıtRastgele::yeni(SCROLL_SYNC_KANIT_TOHUMU);
    let seriler = (0..3)
        .map(|_| {
            x.iter()
                .map(|_| {
                    let indeks = (rng.sonraki() * değer_havuzu.len() as f64).floor() as usize;
                    değer_havuzu.get(indeks).copied()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let veri = HizalıVeri::yeni(x, seriler)?;
    let seçenekler = GrafikSeçenekleri::yeni(400, 200)?
        .başlık(".syncRect()")
        .x_zaman(false)
        // Kaynak örnekte wheel/touch eklentisi yoktur; kapsayıcı doğal
        // olarak kayar. Ortak eklentiler API'de kalır ve geliştirici
        // tarafından sonradan açılabilir.
        .etkileşimler(
            ortak_kart_etkileşimleri()
                .tekerlek_etkileşimi(false)
                .dokunma_etkileşimi(false),
        )
        .seri(
            SeriSeçenekleri::yeni("Value")
                .renk("red")
                .dolgu("rgba(255,0,0,0.1)"),
        )
        .seri(
            SeriSeçenekleri::yeni("Value")
                .renk("green")
                .dolgu("rgba(0,255,0,0.1)"),
        )
        .seri(
            SeriSeçenekleri::yeni("Value")
                .renk("blue")
                .dolgu("rgba(0,0,255,0.1)"),
        );
    Ok((seçenekler, veri))
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut, YüzeyDikdörtgeni};

    #[test]
    fn kaynak_veri_boyutu_havuzu_ve_stilleri_korunur() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = scroll_sync_kartı()?;
        assert_eq!(veri.x().len(), 30);
        assert_eq!(veri.seriler().len(), 3);
        assert!(veri.seriler().iter().flatten().all(|değer| {
            değer.is_some_and(|değer| (-10.0..=10.0).contains(&değer) && değer.fract() == 0.0)
        }));
        assert!(seçenekler.seriler.iter().all(|seri| seri.etiket == "Value"));
        assert!(!seçenekler.etkileşimler.tekerlek_etkileşimi);
        assert!(!seçenekler.etkileşimler.dokunma_etkileşimi);
        let sahne = Grafik::yeni(seçenekler, veri)?.çiz();
        assert!(
            sahne.komutlar().iter().any(
                |komut| matches!(komut, Komut::Metin { içerik, .. } if içerik == ".syncRect()")
            )
        );
        assert!(sahne.komutlar().iter().any(
            |komut| matches!(komut, Komut::Alan { dolgu, .. } if dolgu == "rgba(255,0,0,0.1)")
        ));
        Ok(())
    }

    #[test]
    fn kaydırma_sonrası_aynı_görsel_nokta_aynı_sahne_konumuna_döner() {
        let ilk = YüzeyDikdörtgeni::yeni(20.0, 420.0, 400.0, 200.0);
        let kaydırılmış = YüzeyDikdörtgeni::yeni(20.0, 120.0, 400.0, 200.0);
        let ilk_konum = ilk.and_then(|yüzey| yüzey.sahne_konumu(220.0, 520.0, 400, 200));
        let son_konum = kaydırılmış.and_then(|yüzey| yüzey.sahne_konumu(220.0, 220.0, 400, 200));
        assert_eq!(ilk_konum, son_konum);
    }

    #[test]
    fn gpui_layout_kökeni_değişse_de_aynı_görsel_nokta_aynı_datuma_döner() -> Result<(), UplotHatası>
    {
        let (seçenekler, veri) = scroll_sync_kartı()?;
        let grafik = Grafik::yeni(seçenekler, veri)?;
        let önce = YüzeyDikdörtgeni::yeni(64.0, 480.0, 400.0, 200.0)
            .and_then(|yüzey| yüzey.sahne_konumu(264.0, 580.0, 400, 200));
        let sonra = YüzeyDikdörtgeni::yeni(64.0, 180.0, 400.0, 200.0)
            .and_then(|yüzey| yüzey.sahne_konumu(264.0, 280.0, 400, 200));
        assert_eq!(önce, sonra);

        let önce = önce.ok_or(UplotHatası::YetersizVeri { uzunluk: 0 })?;
        let sonra = sonra.ok_or(UplotHatası::YetersizVeri { uzunluk: 0 })?;
        let (sol, sağ, _, _) = grafik.çizim_alanı_boyutta(400, 200);
        let önce_oranı = f64::from(((önce.x - sol) / (sağ - sol)).clamp(0.0, 1.0));
        let sonra_oranı = f64::from(((sonra.x - sol) / (sağ - sol)).clamp(0.0, 1.0));
        assert_eq!(
            grafik.en_yakın_noktalar(önce_oranı),
            grafik.en_yakın_noktalar(sonra_oranı),
            "GPUI her pointer olayında güncel paint/layout kökenini kullandığında datum sabit kalmalı"
        );
        Ok(())
    }
}
