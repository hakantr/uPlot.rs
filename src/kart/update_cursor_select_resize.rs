use crate::{
    BoyutSenkronDüzeni, GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası,
    ortak_kart_etkileşimleri,
};

pub const UPDATE_CURSOR_SELECT_RESIZE_ARALIK_MS: u64 = 100;
pub const UPDATE_CURSOR_SELECT_RESIZE_KART_TANIM_ÖRNEĞİ: &str = r#"let mut akış = BoyutSenkronAkışı::yeni();
let (seçenekler, veri) = update_cursor_select_resize_kartı(akış.boyut())?;
let mut grafik = Grafik::yeni(seçenekler, veri)?;

// Zamanlayıcı yalnız çekirdeğin ürettiği yeni boyutu yüzeye uygular.
let yeni_boyut = akış.ilerlet();
grafik.boyutu_ayarla(yeni_boyut, yeni_boyut)?;"#;

/// Resmî demodaki 800 → 390 → 400 → 810 → 800 boyut döngüsü.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoyutSenkronAkışı {
    boyut: u32,
    küçülüyor: bool,
}

impl BoyutSenkronAkışı {
    pub const fn yeni() -> Self {
        Self {
            boyut: 800,
            küçülüyor: true,
        }
    }

    pub const fn boyut(self) -> u32 {
        self.boyut
    }

    pub fn ilerlet(&mut self) -> u32 {
        if self.küçülüyor && self.boyut < 400 {
            self.küçülüyor = false;
            self.boyut = 400;
            return self.boyut;
        }
        if !self.küçülüyor && self.boyut > 800 {
            self.küçülüyor = true;
            self.boyut = 800;
            return self.boyut;
        }
        self.boyut = if self.küçülüyor {
            self.boyut.saturating_sub(10)
        } else {
            self.boyut.saturating_add(10)
        };
        self.boyut
    }
}

impl Default for BoyutSenkronAkışı {
    fn default() -> Self {
        Self::yeni()
    }
}

pub fn update_cursor_select_resize_kartı(
    boyut: u32,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    // Resmî 800×800 yüzeyde ölçülen uPlot çizim alanı 725×733 px'tir.
    let düzen = BoyutSenkronDüzeni::piksel_değerlerinden(
        725.0, 733.0, 200.0, 200.0, 100.0, 0.0, 100.0, 733.0, 363.0, 400.0,
    )
    .ok_or_else(|| UplotHatası::GeçersizKaynakVeri {
        varlık: "BoyutSenkronDüzeni",
        açıklama: "resmî uPlot seçim ve imleç ölçüleri geçersiz".to_string(),
    })?;
    let seçenekler = GrafikSeçenekleri::yeni(boyut, boyut)?
        .başlık("Maintain loc of cursor/select/hoverPts")
        .x_zaman(false)
        .boyut_senkronu(düzen)
        .etkileşimler(ortak_kart_etkileşimleri())
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

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut};

    #[test]
    fn kaynak_veri_boyut_akışı_ve_oransal_katmanlar_korunur() -> Result<(), UplotHatası> {
        let mut akış = BoyutSenkronAkışı::yeni();
        assert_eq!(akış.boyut(), 800);
        for _ in 0..41 {
            akış.ilerlet();
        }
        assert_eq!(akış.boyut(), 390);
        assert_eq!(akış.ilerlet(), 400);

        let (seçenekler, veri) = update_cursor_select_resize_kartı(800)?;
        assert_eq!(veri.x(), &[0.0, 1.0, 2.0]);
        assert_eq!(
            veri.seriler().first().map(Vec::as_slice),
            Some([Some(0.0), Some(1.0), Some(2.0)].as_slice())
        );
        let mut grafik = Grafik::yeni(seçenekler, veri)?;
        let büyük = grafik.çiz();
        let büyük_imleç_x = büyük.komutlar().iter().find_map(|komut| match komut {
            Komut::KesikliÇizgi {
                başlangıç,
                bitiş,
                renk,
                ..
            } if renk == "#607d8b" && (başlangıç.x - bitiş.x).abs() <= f32::EPSILON => {
                Some(başlangıç.x)
            }
            _ => None,
        });
        assert!(grafik.boyutu_ayarla(400, 400)?);
        let küçük = grafik.çiz();
        let küçük_imleç_x = küçük.komutlar().iter().find_map(|komut| match komut {
            Komut::KesikliÇizgi {
                başlangıç,
                bitiş,
                renk,
                ..
            } if renk == "#607d8b" && (başlangıç.x - bitiş.x).abs() <= f32::EPSILON => {
                Some(başlangıç.x)
            }
            _ => None,
        });
        let (büyük_sol, büyük_sağ, _, _) = grafik.çizim_alanı_boyutta(800, 800);
        let (küçük_sol, küçük_sağ, _, _) = grafik.çizim_alanı_boyutta(400, 400);
        let büyük_oran = büyük_imleç_x
            .map(|x| (x - büyük_sol) / (büyük_sağ - büyük_sol))
            .ok_or(UplotHatası::YetersizVeri { uzunluk: 0 })?;
        let küçük_oran = küçük_imleç_x
            .map(|x| (x - küçük_sol) / (küçük_sağ - küçük_sol))
            .ok_or(UplotHatası::YetersizVeri { uzunluk: 0 })?;
        assert!((büyük_oran - 200.0 / 725.0).abs() < 0.0001);
        assert!((küçük_oran - büyük_oran).abs() < 0.0001);
        Ok(())
    }
}
