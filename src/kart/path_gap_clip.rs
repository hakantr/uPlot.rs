use std::sync::OnceLock;

use serde::Deserialize;

use super::ortak_kart_etkileşimleri;
use crate::{
    Aralık, BoşlukKipi, GrafikSeçenekleri, HizalıDeğer, HizalıVeri, SeriBandı, SeriSeçenekleri,
    UplotHatası, hizalı_verileri_birleştir,
};

const KAYNAK_JSON: &str = include_str!("veri/path_gap_clip.json");

pub const PATH_GAP_CLIP_KART_TANIM_ÖRNEĞİ: &str = r##"for örnek in PathGapClipÖrneği::TÜMÜ {
    let (seçenekler, veri) = path_gap_clip_kartı(örnek)?;
    // null/undefined ayrımı, boşluk kırpması, bant ve yol geometrisi
    // platform arayüzünden bağımsız olarak çekirdekte çözülür.
    let grafik = Grafik::yeni(seçenekler, veri)?;
}"##;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathGapClipÖrneği {
    VeriDışınaTaşanÖlçek,
    BantBoşlukları,
    BasamakSonra,
    BasamakÖnce,
    BirleşikBasamakSonra,
    BirleşikBasamakÖnce,
    GenişletilmişHizalama,
    SayısalHizalama,
    TekBoşlukÇıkışı,
    TekBoşlukGirişi,
    TekBoşluk3001,
    TekBoşluk4999,
    TekBoşluk5001,
    ÇiftBoşluk,
    Tanımsız,
}

impl PathGapClipÖrneği {
    pub const TÜMÜ: [Self; 15] = [
        Self::VeriDışınaTaşanÖlçek,
        Self::BantBoşlukları,
        Self::BasamakSonra,
        Self::BasamakÖnce,
        Self::BirleşikBasamakSonra,
        Self::BirleşikBasamakÖnce,
        Self::GenişletilmişHizalama,
        Self::SayısalHizalama,
        Self::TekBoşlukÇıkışı,
        Self::TekBoşlukGirişi,
        Self::TekBoşluk3001,
        Self::TekBoşluk4999,
        Self::TekBoşluk5001,
        Self::ÇiftBoşluk,
        Self::Tanımsız,
    ];

    pub const fn kimlik(self) -> &'static str {
        match self {
            Self::VeriDışınaTaşanÖlçek => "path-gap-clip-scale-range",
            Self::BantBoşlukları => "path-gap-clip-band-gaps",
            Self::BasamakSonra => "path-gap-clip-stepped-after",
            Self::BasamakÖnce => "path-gap-clip-stepped-before",
            Self::BirleşikBasamakSonra => "path-gap-clip-joined-stepped-after",
            Self::BirleşikBasamakÖnce => "path-gap-clip-joined-stepped-before",
            Self::GenişletilmişHizalama => "path-gap-clip-align-expand",
            Self::SayısalHizalama => "path-gap-clip-align-numeric",
            Self::TekBoşlukÇıkışı => "path-gap-clip-single-null-outro",
            Self::TekBoşlukGirişi => "path-gap-clip-single-null-intro",
            Self::TekBoşluk3001 => "path-gap-clip-single-null-3001",
            Self::TekBoşluk4999 => "path-gap-clip-single-null-4999",
            Self::TekBoşluk5001 => "path-gap-clip-single-null-5001",
            Self::ÇiftBoşluk => "path-gap-clip-double-null",
            Self::Tanımsız => "path-gap-clip-undefined",
        }
    }

    pub const fn başlık(self) -> &'static str {
        match self {
            Self::VeriDışınaTaşanÖlçek => "Scale range exceeds data range (zoom out)",
            Self::BantBoşlukları => "Gaps in a band",
            Self::BasamakSonra | Self::BirleşikBasamakSonra => "Gaps in stepped after",
            Self::BasamakÖnce | Self::BirleşikBasamakÖnce => "Gaps in stepped before",
            Self::GenişletilmişHizalama | Self::SayısalHizalama => {
                "Align & null-fill vs \"real\" null gaps"
            }
            Self::TekBoşlukÇıkışı => "Single-null pixel-outro bug test",
            Self::TekBoşlukGirişi
            | Self::TekBoşluk3001
            | Self::TekBoşluk4999
            | Self::TekBoşluk5001
            | Self::ÇiftBoşluk => "Single-null pixel-intro bug test",
            Self::Tanımsız => "Undefined",
        }
    }

    pub const fn nokta_sayısı(self) -> usize {
        match self {
            Self::VeriDışınaTaşanÖlçek => 304,
            Self::BantBoşlukları => 7,
            Self::BasamakSonra | Self::BasamakÖnce => 26,
            Self::BirleşikBasamakSonra | Self::BirleşikBasamakÖnce => 10,
            Self::GenişletilmişHizalama => 22,
            Self::SayısalHizalama => 13,
            Self::TekBoşlukÇıkışı | Self::TekBoşlukGirişi => 2_228,
            Self::TekBoşluk3001 | Self::TekBoşluk4999 | Self::TekBoşluk5001 | Self::ÇiftBoşluk => {
                10
            }
            Self::Tanımsız => 7,
        }
    }

    pub fn kimlikten(kimlik: &str) -> Option<Self> {
        Self::TÜMÜ
            .into_iter()
            .find(|örnek| örnek.kimlik() == kimlik)
    }
}

#[derive(Debug, Deserialize)]
struct KaynakVeri {
    data0: Vec<Vec<Option<f64>>>,
    data3: Vec<Vec<Option<f64>>>,
}

fn kaynak_veri() -> Result<&'static KaynakVeri, UplotHatası> {
    static KAYNAK: OnceLock<Result<KaynakVeri, String>> = OnceLock::new();
    match KAYNAK.get_or_init(|| serde_json::from_str(KAYNAK_JSON).map_err(|hata| hata.to_string()))
    {
        Ok(kaynak) => Ok(kaynak),
        Err(açıklama) => Err(UplotHatası::GeçersizKaynakVeri {
            varlık: "src/kart/veri/path_gap_clip.json",
            açıklama: açıklama.clone(),
        }),
    }
}

pub fn path_gap_clip_kartı(
    örnek: PathGapClipÖrneği,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    match örnek {
        PathGapClipÖrneği::VeriDışınaTaşanÖlçek => ölçek_dışı_bant(),
        PathGapClipÖrneği::BantBoşlukları => boşluklu_bant(),
        PathGapClipÖrneği::BasamakSonra => doğrudan_basamak(true),
        PathGapClipÖrneği::BasamakÖnce => doğrudan_basamak(false),
        PathGapClipÖrneği::BirleşikBasamakSonra => birleşik_basamak(true),
        PathGapClipÖrneği::BirleşikBasamakÖnce => birleşik_basamak(false),
        PathGapClipÖrneği::GenişletilmişHizalama => genişletilmiş_hizalama(),
        PathGapClipÖrneği::SayısalHizalama => sayısal_hizalama(),
        PathGapClipÖrneği::TekBoşlukÇıkışı => yoğun_tek_boşluk(true),
        PathGapClipÖrneği::TekBoşlukGirişi => yoğun_tek_boşluk(false),
        PathGapClipÖrneği::TekBoşluk3001 => küçük_tek_boşluk(3.001, false),
        PathGapClipÖrneği::TekBoşluk4999 => küçük_tek_boşluk(4.999, false),
        PathGapClipÖrneği::TekBoşluk5001 => küçük_tek_boşluk(4.999, true),
        PathGapClipÖrneği::ÇiftBoşluk => çift_boşluk(),
        PathGapClipÖrneği::Tanımsız => tanımsız_veri(),
    }
}

fn temel(
    genişlik: u32,
    yükseklik: u32,
    başlık: &str,
    zaman: bool,
) -> Result<GrafikSeçenekleri, UplotHatası> {
    Ok(GrafikSeçenekleri::yeni(genişlik, yükseklik)?
        .başlık(başlık)
        .x_zaman(zaman)
        .etkileşimler(ortak_kart_etkileşimleri()))
}

fn ölçek_dışı_bant() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let kaynak = kaynak_veri()?;
    let x = x_sütunu(&kaynak.data0)?;
    let mut seriler = kaynak
        .data0
        .iter()
        .skip(1)
        .take(2)
        .cloned()
        .collect::<Vec<_>>();
    for seri in &mut seriler {
        for indeks in 35..50 {
            if let Some(değer) = seri.get_mut(indeks) {
                *değer = None;
            }
        }
    }
    let seçenekler = temel(
        756,
        475,
        PathGapClipÖrneği::VeriDışınaTaşanÖlçek.başlık(),
        true,
    )?
    .x_aralığı(Aralık::yeni(1_577_425_500.0, 1_578_028_500.0)?)
    .seri(SeriSeçenekleri::yeni("Low").renk("green"))
    .seri(SeriSeçenekleri::yeni("High").renk("green"))
    .bant(SeriBandı::yeni(1, 0, "rgba(0, 255, 0, .2)"));
    Ok((seçenekler, HizalıVeri::yeni(x, seriler)?))
}

fn boşluklu_bant() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let x = vec![
        1_572_679_693.747,
        1_572_679_694.747,
        1_572_679_695.747,
        1_572_679_696.747,
        1_572_679_697.747,
        1_572_679_698.746,
        1_572_679_699.746,
    ];
    let seriler = vec![
        seçenekli(&[9.5, 10.5, 11.5, 0.0, 13.5, 14.5, 15.5], &[3]),
        seçenekli(&[10.5, 11.5, 12.5, 0.0, 14.5, 15.5, 16.5], &[3]),
        seçenekli(&[10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0], &[]),
    ];
    let seçenekler = temel(800, 400, PathGapClipÖrneği::BantBoşlukları.başlık(), true)?
        .seri(SeriSeçenekleri::yeni("Low").renk("red"))
        .seri(SeriSeçenekleri::yeni("High").renk("red"))
        .seri(SeriSeçenekleri::yeni("Avg").renk("green"))
        .bant(SeriBandı::yeni(1, 0, "rgba(0, 255, 0, .2)"));
    Ok((seçenekler, HizalıVeri::yeni(x, seriler)?))
}

fn doğrudan_basamak(sonra: bool) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let vals = [
        HizalıDeğer::Boş,
        HizalıDeğer::Boş,
        HizalıDeğer::Boş,
        HizalıDeğer::Değer(1.0),
        HizalıDeğer::Değer(1.0),
        HizalıDeğer::Değer(1.0),
        HizalıDeğer::Değer(1.0),
        HizalıDeğer::Değer(1.0),
        HizalıDeğer::Boş,
        HizalıDeğer::Boş,
        HizalıDeğer::Değer(2.0),
        HizalıDeğer::Değer(2.0),
        HizalıDeğer::Boş,
        HizalıDeğer::Değer(1.0),
        HizalıDeğer::Tanımsız,
        HizalıDeğer::Tanımsız,
        HizalıDeğer::Boş,
        HizalıDeğer::Değer(1.0),
        HizalıDeğer::Değer(1.0),
        HizalıDeğer::Değer(1.0),
        HizalıDeğer::Değer(1.0),
        HizalıDeğer::Değer(2.0),
        HizalıDeğer::Değer(2.0),
        HizalıDeğer::Değer(2.0),
        HizalıDeğer::Tanımsız,
        HizalıDeğer::Tanımsız,
    ];
    let seriler = [0.0, 0.1, 0.2]
        .into_iter()
        .map(|ek| {
            vals.iter()
                .map(|değer| match değer {
                    HizalıDeğer::Değer(sayı) => HizalıDeğer::Değer(*sayı + ek),
                    diğer => *diğer,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let ilk = SeriSeçenekleri::yeni(if sonra { "step after" } else { "step before" })
        .renk(if sonra { "blue" } else { "red" })
        .dolgu(if sonra {
            "rgba(0,0,255,0.3)"
        } else {
            "rgba(255,0,0,0.3)"
        })
        .çizgi_kalınlığı(2.0);
    let ilk = if sonra {
        ilk.basamak_sonra()
    } else {
        ilk.basamak_önce()
    };
    let örnek = if sonra {
        PathGapClipÖrneği::BasamakSonra
    } else {
        PathGapClipÖrneği::BasamakÖnce
    };
    let seçenekler = temel(800, 400, örnek.başlık(), false)?
        .seri(ilk)
        .seri(
            SeriSeçenekleri::yeni("linear")
                .renk(if sonra { "red" } else { "blue" })
                .çizgi_kalınlığı(2.0),
        )
        .seri(
            SeriSeçenekleri::yeni("spline")
                .renk("orange")
                .çizgi_kalınlığı(2.0)
                .eğri(),
        );
    Ok((
        seçenekler,
        HizalıVeri::anlamlı((0..26).map(f64::from).collect(), seriler)?,
    ))
}

fn birleşik_basamak(sonra: bool) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let tablolar = birleşik_kaynak_tablolar()?;
    let veri = hizalı_verileri_birleştir(&tablolar, None)?;
    let basamak = SeriSeçenekleri::yeni("on/off")
        .renk("blue")
        .dolgu("rgba(0,0,255,0.3)")
        .çizgi_kalınlığı(2.0);
    let basamak = if sonra {
        basamak.basamak_sonra()
    } else {
        basamak.basamak_önce()
    };
    let aralık = SeriSeçenekleri::yeni("[0..5]")
        .renk("red")
        .dolgu("rgba(255,0,0,0.3)")
        .çizgi_kalınlığı(2.0);
    let aralık = if sonra {
        aralık.basamak_sonra()
    } else {
        aralık.basamak_önce()
    };
    let örnek = if sonra {
        PathGapClipÖrneği::BirleşikBasamakSonra
    } else {
        PathGapClipÖrneği::BirleşikBasamakÖnce
    };
    let seçenekler = temel(800, 400, örnek.başlık(), false)?
        .seri(basamak)
        .seri(aralık)
        .seri(
            SeriSeçenekleri::yeni("dataset03")
                .renk("green")
                .çizgi_kalınlığı(2.0),
        )
        .seri(
            SeriSeçenekleri::yeni("dataset04")
                .renk("yellow")
                .çizgi_kalınlığı(2.0),
        )
        .seri(
            SeriSeçenekleri::yeni("dataset05 sin")
                .renk("black")
                .çizgi_kalınlığı(2.0),
        );
    Ok((seçenekler, veri))
}

fn birleşik_kaynak_tablolar() -> Result<Vec<HizalıVeri>, UplotHatası> {
    let derece = std::f64::consts::PI / 180.0;
    Ok(vec![
        HizalıVeri::yeni(vec![0.0, 4.0, 6.0], vec![tam(&[1.0, 0.0, 1.0])])?,
        HizalıVeri::yeni(
            (0..10).map(f64::from).collect(),
            vec![seçenekli(
                &[0.0, 0.0, 3.0, 4.0, 5.0, 0.0, 3.0, 2.0, 1.0, 0.0],
                &[0, 1, 5],
            )],
        )?,
        HizalıVeri::yeni(vec![1.0, 8.0], vec![tam(&[4.9, 1.55])])?,
        HizalıVeri::yeni(
            vec![0.0, 5.0, 9.0],
            vec![seçenekli(&[1.11, 0.0, 4.44], &[1])],
        )?,
        HizalıVeri::yeni(
            vec![0.0, 1.0, 3.0, 4.0, 5.0, 7.0, 8.0, 9.0],
            vec![seçenekli(
                &[
                    0.0,
                    (10.0 * derece).sin() * 5.0,
                    (30.0 * derece).sin() * 5.0,
                    0.0,
                    (50.0 * derece).sin() * 5.0,
                    (70.0 * derece).sin() * 5.0,
                    (80.0 * derece).sin() * 5.0,
                    (90.0 * derece).sin() * 5.0,
                ],
                &[3],
            )],
        )?,
    ])
}

fn genişletilmiş_hizalama() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let tablolar = vec![
        HizalıVeri::yeni(
            vec![
                1_607_676_419_481.0,
                1_607_680_019_481.0,
                1_607_683_619_481.0,
                1_607_687_219_481.0,
                1_607_690_819_481.0,
                1_607_694_419_481.0,
                1_607_698_019_481.0,
                1_607_698_019_482.0,
            ],
            vec![tam(&[10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 90.0])],
        )?,
        HizalıVeri::yeni(
            vec![
                1_607_676_419_481.0,
                1_607_677_962_338.0,
                1_607_679_505_195.0,
                1_607_681_048_052.0,
                1_607_682_590_909.0,
                1_607_684_133_766.0,
                1_607_685_676_623.0,
                1_607_687_219_480.0,
                1_607_688_762_337.0,
                1_607_690_305_194.0,
                1_607_691_848_051.0,
                1_607_693_390_908.0,
                1_607_694_933_765.0,
                1_607_696_476_622.0,
                1_607_698_019_479.0,
            ],
            vec![seçenekli(
                &[
                    1.0, 0.0, 40.0, 0.0, 90.0, 0.0, 0.0, 100.0, 0.0, 0.0, 100.0, 0.0, 0.0, 80.0,
                    0.0,
                ],
                &[1, 3, 5, 6, 8, 9, 11, 12, 14],
            )],
        )?,
    ];
    let kipler = vec![vec![BoşlukKipi::Genişlet], vec![BoşlukKipi::Genişlet]];
    let veri = hizalı_verileri_birleştir(&tablolar, Some(&kipler))?;
    let seçenekler = temel(
        1_200,
        600,
        PathGapClipÖrneği::GenişletilmişHizalama.başlık(),
        true,
    )?
    .x_zaman_milisaniye(true)
    .seri(
        SeriSeçenekleri::yeni("")
            .renk("red")
            .dolgu("rgba(255,0,0,0.1)"),
    )
    .seri(
        SeriSeçenekleri::yeni("")
            .renk("green")
            .dolgu("rgba(0,255,0,0.1)"),
    );
    Ok((seçenekler, veri))
}

fn sayısal_hizalama() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let tablolar = vec![
        HizalıVeri::yeni(
            vec![3.0, 5.0, 6.0, 7.0, 20.0],
            vec![tam(&[2.0, 3.0, 4.0, 10.0, 5.0])],
        )?,
        HizalıVeri::yeni(
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 17.0],
            vec![seçenekli(&[7.0, 2.0, 1.0, 0.0, 6.0, 13.0], &[3])],
        )?,
        HizalıVeri::yeni(
            vec![9.0, 14.0, 15.0, 16.0],
            vec![seçenekli(&[9.0, 5.0, 0.0, 1.0], &[2])],
        )?,
    ];
    let veri = hizalı_verileri_birleştir(&tablolar, None)?;
    let seçenekler = temel(
        1_200,
        600,
        PathGapClipÖrneği::SayısalHizalama.başlık(),
        false,
    )?
    .seri(
        SeriSeçenekleri::yeni("")
            .renk("red")
            .dolgu("rgba(255,0,0,0.1)"),
    )
    .seri(
        SeriSeçenekleri::yeni("")
            .renk("green")
            .dolgu("rgba(0,255,0,0.1)"),
    )
    .seri(
        SeriSeçenekleri::yeni("")
            .renk("blue")
            .dolgu("rgba(0,0,255,0.1)"),
    );
    Ok((seçenekler, veri))
}

fn yoğun_tek_boşluk(ters: bool) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let kaynak = kaynak_veri()?;
    let mut x = x_sütunu(&kaynak.data3)?;
    if let Some(değer) = x.get_mut(899) {
        *değer = if ters {
            1_578_812_550.0
        } else {
            1_578_812_608.0
        };
    }
    let y = kaynak
        .data3
        .get(1)
        .cloned()
        .ok_or(UplotHatası::YetersizVeri { uzunluk: 1 })?;
    let örnek = if ters {
        PathGapClipÖrneği::TekBoşlukÇıkışı
    } else {
        PathGapClipÖrneği::TekBoşlukGirişi
    };
    let seçenekler = temel(300, 475, örnek.başlık(), true)?
        .x_aralığı(Aralık::yeni(1_578_812_057.0, 1_578_812_879.0)?)
        .x_ters_yön(ters)
        .seri(
            SeriSeçenekleri::yeni("Low")
                .renk("red")
                .dolgu("rgba(255, 0, 0, .2)"),
        );
    Ok((seçenekler, HizalıVeri::yeni(x, vec![y])?))
}

fn küçük_tek_boşluk(
    boş_x: f64,
    ek_5001: bool,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let x = if ek_5001 {
        vec![0.0, 1.0, 2.0, 3.0, boş_x, 5.0, 5.001, 7.0, 8.0, 9.0]
    } else {
        vec![0.0, 1.0, 2.0, 3.0, boş_x, 5.0, 6.0, 7.0, 8.0, 9.0]
    };
    küçük_boşluk_kartı(x, &[4])
}

fn çift_boşluk() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    küçük_boşluk_kartı(
        vec![0.0, 1.0, 2.0, 3.0, 4.999, 5.0, 5.001, 7.0, 8.0, 9.0],
        &[4, 6],
    )
}

fn küçük_boşluk_kartı(
    x: Vec<f64>,
    boşluklar: &[usize],
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let seçenekler = temel(500, 250, PathGapClipÖrneği::TekBoşlukGirişi.başlık(), false)?.seri(
        SeriSeçenekleri::yeni("Low")
            .renk("red")
            .dolgu("rgba(255, 0, 0, .2)"),
    );
    Ok((
        seçenekler,
        HizalıVeri::yeni(x, vec![seçenekli(&[1.0; 10], boşluklar)])?,
    ))
}

fn tanımsız_veri() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let x = vec![
        42_000.0, 71_999.0, 77_999.0, 78_000.0, 79_800.0, 92_400.0, 98_400.0,
    ];
    let y = vec![
        HizalıDeğer::Değer(10.0),
        HizalıDeğer::Değer(20.0),
        HizalıDeğer::Boş,
        HizalıDeğer::Değer(30.0),
        HizalıDeğer::Tanımsız,
        HizalıDeğer::Değer(40.0),
        HizalıDeğer::Tanımsız,
    ];
    let seçenekler = temel(600, 300, PathGapClipÖrneği::Tanımsız.başlık(), false)?
        .seri(SeriSeçenekleri::yeni("").renk("red"));
    Ok((seçenekler, HizalıVeri::anlamlı(x, vec![y])?))
}

fn x_sütunu(sütunlar: &[Vec<Option<f64>>]) -> Result<Vec<f64>, UplotHatası> {
    let x = sütunlar
        .first()
        .ok_or(UplotHatası::YetersizVeri { uzunluk: 0 })?;
    x.iter()
        .enumerate()
        .map(|(indeks, değer)| değer.ok_or(UplotHatası::SonluOlmayanX { indeks }))
        .collect()
}

fn tam(değerler: &[f64]) -> Vec<Option<f64>> {
    değerler.iter().copied().map(Some).collect()
}

fn seçenekli(değerler: &[f64], boşluklar: &[usize]) -> Vec<Option<f64>> {
    değerler
        .iter()
        .copied()
        .enumerate()
        .map(|(indeks, değer)| (!boşluklar.contains(&indeks)).then_some(değer))
        .collect()
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut};

    #[test]
    fn resmi_on_bes_yuzey_kaynak_nokta_sayilarini_korur() -> Result<(), UplotHatası> {
        for örnek in PathGapClipÖrneği::TÜMÜ {
            let (seçenekler, veri) = path_gap_clip_kartı(örnek)?;
            assert_eq!(seçenekler.başlık, örnek.başlık());
            assert_eq!(veri.uzunluk(), örnek.nokta_sayısı());
        }
        Ok(())
    }

    #[test]
    fn null_gercek_bosluk_undefined_ise_hizalama_eksigidir() -> Result<(), UplotHatası> {
        let (_, veri) = path_gap_clip_kartı(PathGapClipÖrneği::Tanımsız)?;
        assert!(!veri.hizalama_eksiği_mi(0, 2));
        assert!(veri.hizalama_eksiği_mi(0, 4));
        assert!(veri.hizalama_eksiği_mi(0, 6));
        let grafik = Grafik::yeni(path_gap_clip_kartı(PathGapClipÖrneği::Tanımsız)?.0, veri)?;
        let sahne = grafik.çiz();
        let yol_parçaları = sahne
            .komutlar()
            .iter()
            .filter_map(|komut| match komut {
                Komut::Yol {
                    parçalar, renk, ..
                } if renk == "red" => Some(parçalar),
                _ => None,
            })
            .next();
        assert_eq!(yol_parçaları.map(Vec::len), Some(2));
        assert!(
            yol_parçaları.is_some_and(|parçalar| parçalar.iter().all(|parça| parça.len() == 2))
        );
        Ok(())
    }

    #[test]
    fn ters_x_olcegi_konum_ve_yakin_nokta_yonunu_birlikte_cevirir() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = path_gap_clip_kartı(PathGapClipÖrneği::TekBoşlukÇıkışı)?;
        assert!(seçenekler.x_ters_yön);
        let grafik = Grafik::yeni(seçenekler, veri)?;
        let aralık = grafik.görünür_x_aralığı();
        assert_eq!(grafik.x_konum_oranı(aralık.en_az), Some(1.0));
        assert_eq!(grafik.x_konum_oranı(aralık.en_çok), Some(0.0));
        let sol = grafik.en_yakın_nokta(0.0, 0).map(|nokta| nokta.0);
        let sağ = grafik.en_yakın_nokta(1.0, 0).map(|nokta| nokta.0);
        assert!(sol.zip(sağ).is_some_and(|(sol, sağ)| sol > sağ));
        Ok(())
    }

    #[test]
    fn bant_ve_dolgu_cokgenleri_cizim_sinirinda_kalir() -> Result<(), UplotHatası> {
        for örnek in [
            PathGapClipÖrneği::VeriDışınaTaşanÖlçek,
            PathGapClipÖrneği::BantBoşlukları,
            PathGapClipÖrneği::TekBoşlukÇıkışı,
        ] {
            let (seçenekler, veri) = path_gap_clip_kartı(örnek)?;
            let genişlik = seçenekler.genişlik;
            let yükseklik = seçenekler.yükseklik;
            let grafik = Grafik::yeni(seçenekler, veri)?;
            let (sol, sağ, üst, alt) = grafik.çizim_alanı_boyutta(genişlik, yükseklik);
            let sahne = grafik.çiz();
            for komut in sahne.komutlar() {
                if let Komut::Alan { çokgenler, .. } = komut {
                    assert!(çokgenler.iter().flatten().all(|nokta| {
                        nokta.x.is_finite()
                            && nokta.y.is_finite()
                            && (sol..=sağ).contains(&nokta.x)
                            && (üst..=alt).contains(&nokta.y)
                    }));
                }
            }
        }
        Ok(())
    }

    #[test]
    fn span_gaps_kaynak_animasyonunda_bant_ve_cizgi_birlikte_koprulenir() -> Result<(), UplotHatası>
    {
        let (seçenekler, veri) = path_gap_clip_kartı(PathGapClipÖrneği::BantBoşlukları)?;
        let mut grafik = Grafik::yeni(seçenekler, veri)?;
        let önce = bant_çokgen_sayısı(&grafik.çiz());
        assert!(grafik.boşlukları_birleştir_ayarla(true));
        let sonra = bant_çokgen_sayısı(&grafik.çiz());
        assert!(sonra > önce);
        Ok(())
    }

    fn bant_çokgen_sayısı(sahne: &crate::Sahne) -> usize {
        sahne
            .komutlar()
            .iter()
            .filter_map(|komut| match komut {
                Komut::Alan { çokgenler, dolgu } if dolgu == "rgba(0, 255, 0, .2)" => {
                    Some(çokgenler.len())
                }
                _ => None,
            })
            .sum()
    }
}
