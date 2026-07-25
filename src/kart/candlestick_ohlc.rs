use super::{ortak_kart_etkileşimleri, veri_uretici::KanıtRastgele};
use crate::{
    Aralık, GrafikSeçenekleri, HizalıVeri, MumDüzeni, SeriSeçenekleri, UplotHatası,
    YÖlçekSeçenekleri,
};

const CANDLESTICK_JSON: &str = include_str!("veri/candlestick_ohlc.json");
pub const CANDLESTICK_KANIT_TOHUMU: u32 = 0x00CA_AD1E;

pub const CANDLESTICK_KART_TANIM_ÖRNEĞİ: &str = r##"let (seçenekler, veri) = candlestick_ohlc_kartı()?;
// OHLC gövdesi, fitil, hacim ve sütun hover geometrisi çekirdekte çözülür.
let grafik = Grafik::yeni(seçenekler, veri)?;"##;

/// `demos/candlestick-ohlc.html` içindeki Gold grafiğini üretir.
pub fn candlestick_ohlc_kartı() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let kaynak: Vec<Vec<f64>> = serde_json::from_str(CANDLESTICK_JSON).map_err(|hata| {
        UplotHatası::GeçersizKaynakVeri {
            varlık: "demos/candlestick-ohlc.html#data",
            açıklama: hata.to_string(),
        }
    })?;
    let Some(zamanlar) = kaynak.first().cloned() else {
        return Err(UplotHatası::GeçersizKaynakVeri {
            varlık: "demos/candlestick-ohlc.html#data",
            açıklama: "zaman serisi bulunamadı".to_string(),
        });
    };
    let uzunluk = zamanlar.len();
    if kaynak.len() != 5 || uzunluk == 0 || kaynak.iter().any(|seri| seri.len() != uzunluk) {
        return Err(UplotHatası::GeçersizKaynakVeri {
            varlık: "demos/candlestick-ohlc.html#data",
            açıklama: "OHLC serileri eşit uzunlukta değil".to_string(),
        });
    }
    let mut seriler = kaynak
        .iter()
        .skip(1)
        .map(|seri| seri.iter().copied().map(Some).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut rastgele = KanıtRastgele::yeni(CANDLESTICK_KANIT_TOHUMU);
    let hacim = (0..uzunluk)
        .map(|_| Some(10.0 + (rastgele.sonraki() * 241.0).floor()))
        .collect();
    seriler.push(hacim);
    let x = (0..uzunluk).map(|indeks| indeks as f64).collect();
    let veri = HizalıVeri::yeni(x, seriler)?;
    let hacim_ölçeği = YÖlçekSeçenekleri::yeni("vol")
        .aralık(Aralık::yeni(0.0, 2_000.0)?)
        .sağda(true)
        .eksen(true)
        .ızgara(false);
    let mut seçenekler = GrafikSeçenekleri::yeni(1_920, 600)?
        .başlık("Gold")
        .x_zaman(false)
        .mum_düzeni(MumDüzeni::yeni(zamanlar))
        .y_ölçeği(hacim_ölçeği)
        .etkileşimler(ortak_kart_etkileşimleri());
    for etiket in ["Open", "High", "Low", "Close"] {
        seçenekler = seçenekler.seri(
            SeriSeçenekleri::yeni(etiket)
                .renk("#000000")
                .çizgi_kalınlığı(0.0),
        );
    }
    seçenekler = seçenekler.seri(
        SeriSeçenekleri::yeni("Volume")
            .renk("#4ab650")
            .ölçek("vol")
            .çizgi_kalınlığı(0.0),
    );
    Ok((seçenekler, veri))
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut, grafik::usd_biçimle};

    #[test]
    fn kaynak_ohlc_verisi_ve_mum_geometrisi_korunur() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = candlestick_ohlc_kartı()?;
        assert_eq!(veri.uzunluk(), 218);
        let ilkler = veri
            .seriler()
            .iter()
            .filter_map(|seri| seri.first().copied().flatten())
            .collect::<Vec<_>>();
        assert_eq!(ilkler.first(), Some(&1_284.7));
        assert_eq!(ilkler.get(1), Some(&1_284.75));
        assert_eq!(ilkler.get(2), Some(&1_282.85));
        assert_eq!(ilkler.get(3), Some(&1_283.35));
        assert!(
            ilkler
                .get(4)
                .is_some_and(|hacim| (10.0..=250.0).contains(hacim))
        );
        let mut grafik = Grafik::yeni(seçenekler, veri)?;
        assert_eq!(grafik.mum_tarih_etiketi(0).as_deref(), Some("2019-01-01"));
        assert_eq!(grafik.mum_tarih_etiketi(217).as_deref(), Some("2019-10-25"));
        assert_eq!(usd_biçimle(1_284.7, 2), "$1,284.70");
        assert_eq!(usd_biçimle(-1_300.0, 0), "$-1,300");
        assert_eq!(grafik.görünür_x_aralığı(), Aralık::yeni(0.0, 217.0)?);
        let vuruş = grafik.kutu_bıyık_vuruşu(1_920, 600, 72.0, 100.0);
        assert!(vuruş.is_some(), "ilk mum sütunu vurulmalı");
        if let Some(vuruş) = vuruş {
            assert_eq!(vuruş.0, 0);
            assert_eq!(&vuruş.4[..4], &[1_284.7, 1_284.75, 1_282.85, 1_283.35]);
        }
        let sahne = grafik.çiz();
        assert!(sahne.komutlar().iter().any(|komut| {
            matches!(komut, Komut::Dikdörtgen { dolgu, .. } if dolgu == "#4ab650")
        }));
        assert!(sahne.komutlar().iter().any(|komut| {
            matches!(komut, Komut::Dikdörtgen { dolgu, .. } if dolgu == "#e54245")
        }));
        assert!(sahne.komutlar().iter().any(|komut| {
            matches!(
                komut,
                Komut::Dikdörtgen {
                    dolgu,
                    çizgi,
                    kalınlık,
                    ..
                } if dolgu == "#000000" && çizgi == "none" && *kalınlık == 0.0
            )
        }));

        let tam_mum_dikdörtgenleri = sahne
            .komutlar()
            .iter()
            .filter(|komut| matches!(komut, Komut::Dikdörtgen { .. }))
            .count();
        assert!(grafik.görünür_x_aralığını_ayarla(Aralık::yeni(100.0, 110.0)?, true));
        let yakın_sahne = grafik.çiz();
        let yakın_mum_dikdörtgenleri = yakın_sahne
            .komutlar()
            .iter()
            .filter(|komut| matches!(komut, Komut::Dikdörtgen { .. }))
            .count();
        assert!(
            yakın_mum_dikdörtgenleri * 5 < tam_mum_dikdörtgenleri,
            "yalnız görünür mumlar çizilmeli: {yakın_mum_dikdörtgenleri}/{tam_mum_dikdörtgenleri}"
        );
        Ok(())
    }
}
