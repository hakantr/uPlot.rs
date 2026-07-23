use super::{ortak_kart_etkileşimleri, veri_uretici::KanıtRastgele};
use crate::{
    Aralık, BantYönü, GrafikSeçenekleri, HizalıDeğer, HizalıVeri, SeriBandı, SeriSeçenekleri,
    UplotHatası,
};

pub const STACKED_SERIES_KANIT_TOHUMU: u32 = 0x57AC_CED5;

pub const STACKED_SERIES_KART_TANIM_ÖRNEĞİ: &str = r##"for örnek in StackedSeriesÖrneği::TÜMÜ {
    let (seçenekler, veri) = stacked_series_kartı(örnek)?;
    // Yığma, ham lejant değerleri, null/undefined, yüzde, grup ve bant
    // geometrisi Rust çekirdeğinde çözülür.
    let grafik = Grafik::yeni(seçenekler, veri)?;
}"##;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StackedSeriesÖrneği {
    Stacked1,
    Stacked2,
    BarsStacked,
    Interpolated,
    Unstacked,
    Stacked,
    UndefBoth,
    RedNull,
    GreenNull,
    BothNull,
    BothZero,
    NegYNull,
    NegYZero,
    NegativePercent,
    Groups,
    JoinedMixed,
}

impl StackedSeriesÖrneği {
    pub const TÜMÜ: [Self; 16] = [
        Self::Stacked1,
        Self::Stacked2,
        Self::BarsStacked,
        Self::Interpolated,
        Self::Unstacked,
        Self::Stacked,
        Self::UndefBoth,
        Self::RedNull,
        Self::GreenNull,
        Self::BothNull,
        Self::BothZero,
        Self::NegYNull,
        Self::NegYZero,
        Self::NegativePercent,
        Self::Groups,
        Self::JoinedMixed,
    ];

    pub const fn kimlik(self) -> &'static str {
        match self {
            Self::Stacked1 => "stacked-series-stacked-1",
            Self::Stacked2 => "stacked-series-stacked-2",
            Self::BarsStacked => "stacked-series-bars",
            Self::Interpolated => "stacked-series-interpolated",
            Self::Unstacked => "stacked-series-unstacked",
            Self::Stacked => "stacked-series-stacked",
            Self::UndefBoth => "stacked-series-undef-both",
            Self::RedNull => "stacked-series-red-null",
            Self::GreenNull => "stacked-series-green-null",
            Self::BothNull => "stacked-series-both-null",
            Self::BothZero => "stacked-series-both-zero",
            Self::NegYNull => "stacked-series-negy-null",
            Self::NegYZero => "stacked-series-negy-zero",
            Self::NegativePercent => "stacked-series-negative-percent",
            Self::Groups => "stacked-series-groups",
            Self::JoinedMixed => "stacked-series-joined-mixed",
        }
    }

    pub const fn başlık(self) -> &'static str {
        match self {
            Self::Stacked1 => "Stacked 1",
            Self::Stacked2 => "Stacked 2",
            Self::BarsStacked => "Bars Stacked",
            Self::Interpolated => "Stacked / Interpolated (at magenta x=3)",
            Self::Unstacked => "unstacked",
            Self::Stacked => "stacked",
            Self::UndefBoth => "stacked, red=undef, green=undef",
            Self::RedNull => "stacked, red=null",
            Self::GreenNull => "stacked, green=null",
            Self::BothNull => "stacked, red=null, green=null",
            Self::BothZero => "stacked, red=0, green=0",
            Self::NegYNull => "unstacked, green negY, green=null, red=null",
            Self::NegYZero => "unstacked, green negY, green=0, red=0",
            Self::NegativePercent => "neg percent stacked",
            Self::Groups => "stacking groups",
            Self::JoinedMixed => "stacked joined/mixed",
        }
    }

    pub const fn boyut(self) -> (u32, u32) {
        match self {
            Self::Stacked1 | Self::Stacked2 => (800, 400),
            Self::BarsStacked | Self::Interpolated => (1_600, 400),
            _ => (400, 300),
        }
    }

    pub fn kimlikten(kimlik: &str) -> Option<Self> {
        Self::TÜMÜ
            .into_iter()
            .find(|örnek| örnek.kimlik() == kimlik)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Hücre {
    Değer(f64),
    Boş,
    Tanımsız,
}

impl Hücre {
    fn ham(self) -> Option<f64> {
        match self {
            Self::Değer(değer) => Some(değer),
            Self::Boş | Self::Tanımsız => None,
        }
    }

    fn hizalı(self) -> HizalıDeğer {
        match self {
            Self::Değer(değer) => HizalıDeğer::Değer(değer),
            Self::Boş => HizalıDeğer::Boş,
            Self::Tanımsız => HizalıDeğer::Tanımsız,
        }
    }
}

#[derive(Clone, Copy)]
struct Stil {
    etiket: &'static str,
    çizgi: &'static str,
    dolgu: &'static str,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum YığmaKipi {
    Yok,
    Normal,
    Yüzde,
}

struct YığmaGirdisi {
    değerler: Vec<Hücre>,
    neg_y: bool,
    kip: YığmaKipi,
    grup: &'static str,
}

struct YığmaÇıktısı {
    seriler: Vec<Vec<Hücre>>,
    bantlar: Vec<(usize, usize, BantYönü)>,
}

struct Grup {
    anahtar: String,
    birikim: Vec<f64>,
    seriler: Vec<usize>,
    yön: BantYönü,
}

struct Kartİçeriği {
    x: Vec<f64>,
    çizim: Vec<Vec<Hücre>>,
    ham: Vec<Vec<Option<f64>>>,
    stiller: Vec<Stil>,
    bantlar: Vec<(usize, usize, BantYönü)>,
    çubuklar: Vec<bool>,
    nokta_boyutu: Option<f32>,
    nokta_indeksleri: Vec<Option<Vec<usize>>>,
    görünür: Vec<bool>,
}

pub fn stacked_series_kartı(
    örnek: StackedSeriesÖrneği,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    stacked_series_kartı_görünür(örnek, &[])
}

pub fn stacked_series_kartı_görünür(
    örnek: StackedSeriesÖrneği,
    görünür: &[bool],
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let mut içerik = match örnek {
        StackedSeriesÖrneği::Stacked1 => klasik_yığılmış(false),
        StackedSeriesÖrneği::Stacked2 => klasik_yığılmış(true),
        StackedSeriesÖrneği::BarsStacked => yığılmış_çubuklar(),
        StackedSeriesÖrneği::Interpolated => interpolasyonlu(),
        StackedSeriesÖrneği::Unstacked => küçük_dörtlü(KüçükBoşluk::Yok, false),
        StackedSeriesÖrneği::Stacked => küçük_dörtlü(KüçükBoşluk::Yok, true),
        StackedSeriesÖrneği::UndefBoth => küçük_dörtlü(KüçükBoşluk::İkiTanımsız, true),
        StackedSeriesÖrneği::RedNull => küçük_dörtlü(KüçükBoşluk::KırmızıBoş, true),
        StackedSeriesÖrneği::GreenNull => küçük_dörtlü(KüçükBoşluk::YeşilBoş, true),
        StackedSeriesÖrneği::BothNull => küçük_dörtlü(KüçükBoşluk::İkiBoş, true),
        StackedSeriesÖrneği::BothZero => küçük_dörtlü(KüçükBoşluk::İkiSıfır, true),
        StackedSeriesÖrneği::NegYNull => negatif_y(false),
        StackedSeriesÖrneği::NegYZero => negatif_y(true),
        StackedSeriesÖrneği::NegativePercent => negatif_yüzde(),
        StackedSeriesÖrneği::Groups => yığma_grupları(),
        StackedSeriesÖrneği::JoinedMixed => birleşik_karma(),
    };
    görünürlüğü_uygula(örnek, &mut içerik, görünür);
    kartı_kur(örnek, içerik)
}

fn kartı_kur(
    örnek: StackedSeriesÖrneği,
    içerik: Kartİçeriği,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let çizim_en_çoğu = içerik
        .çizim
        .iter()
        .flat_map(|seri| seri.iter())
        .filter_map(|hücre| match hücre {
            Hücre::Değer(değer) => Some(*değer),
            Hücre::Boş | Hücre::Tanımsız => None,
        })
        .max_by(f64::total_cmp);
    let anlamlı = içerik
        .çizim
        .iter()
        .map(|seri| seri.iter().copied().map(Hücre::hizalı).collect())
        .collect();
    let veri = HizalıVeri::anlamlı(içerik.x, anlamlı)?;
    let (genişlik, yükseklik) = örnek.boyut();
    let mut seçenekler = GrafikSeçenekleri::yeni(genişlik, yükseklik)?
        .başlık(örnek.başlık())
        .x_zaman(false)
        .etkileşimler(ortak_kart_etkileşimleri());
    if matches!(
        örnek,
        StackedSeriesÖrneği::Stacked1
            | StackedSeriesÖrneği::Stacked2
            | StackedSeriesÖrneği::BarsStacked
            | StackedSeriesÖrneği::Interpolated
    ) && let Some(en_çok) = çizim_en_çoğu
    {
        seçenekler = seçenekler.y_aralığı(Aralık::uplot_sayısal(0.0, en_çok, 0.1, true)?);
    }
    for (indeks, stil) in içerik.stiller.iter().copied().enumerate() {
        let mut seri = SeriSeçenekleri::yeni(stil.etiket)
            .renk(stil.çizgi)
            .dolgu(stil.dolgu)
            .göster(içerik.görünür.get(indeks).copied().unwrap_or(true))
            .lejant_değerleri(
                içerik
                    .ham
                    .get(indeks)
                    .cloned()
                    .unwrap_or_else(|| vec![None; veri.uzunluk()]),
            );
        if içerik.çubuklar.get(indeks).copied().unwrap_or(false) {
            seri = seri
                .çubuk(true)
                .çubuk_boyutu(0.6, 100.0)
                .noktaları_göster(false)
                .çizgi_kalınlığı(2.0);
        }
        if let Some(boyut) = içerik.nokta_boyutu {
            seri = seri
                .noktaları_göster(true)
                .nokta_stili(boyut, 1.0, None::<String>);
        }
        if let Some(indeksler) = içerik.nokta_indeksleri.get(indeks).and_then(Clone::clone) {
            seri = seri.nokta_indeksleri(indeksler);
        }
        seçenekler = seçenekler.seri(seri);
    }
    for (üst, alt, yön) in içerik.bantlar {
        let dolgu = içerik
            .stiller
            .get(üst)
            .map_or("rgba(107,114,128,.3)", |stil| stil.dolgu);
        seçenekler = seçenekler.bant(SeriBandı::yeni(üst, alt, dolgu).yön(yön));
    }
    Ok((seçenekler, veri))
}

fn klasik_yığılmış(ters: bool) -> Kartİçeriği {
    let x = (1..=30).map(f64::from).collect::<Vec<_>>();
    let d1 = (0..30)
        .map(|indeks| {
            let değer = if indeks < 15 {
                20 + indeks
            } else if indeks == 15 {
                15
            } else if indeks == 16 {
                14
            } else if indeks == 24 {
                20
            } else if indeks == 28 {
                15
            } else if indeks == 29 {
                0
            } else {
                30
            };
            Some(değer as f64)
        })
        .collect::<Vec<_>>();
    let d2 = (0..30)
        .map(|indeks| Some(if indeks == 24 { 20.0 } else { 10.0 }))
        .collect();
    let d3 = vec![Some(10.0); 30];
    let d4 = vec![Some(5.0); 30];
    let d5 = vec![Some(5.0); 30];
    let mut ham = vec![d1, d2, d3, d4, d5];
    let mut stiller = klasik_stiller();
    if ters {
        ham.reverse();
        stiller.reverse();
    }
    basit_yığılmış_içerik(x, ham, stiller, vec![false; 5], None)
}

fn yığılmış_çubuklar() -> Kartİçeriği {
    let x = (0..=100).map(f64::from).collect::<Vec<_>>();
    let temel = vec![
        109, 117, 122, 104, 105, 117, 119, 121, 117, 121, 122, 129, 119, 113, 113, 121, 108, 108,
        100, 103, 113, 110, 107, 105, 99, 93, 87, 83, 91, 85, 81, 69, 76, 61, 63, 74, 76, 68, 55,
        61, 48, 39, 54, 44, 37, 30, 22, 33, 29, 21, 22, 43, 47, 33, 47, 28, 29, 31, 32, 35, 37, 25,
        -5, -14, -7, -14, -7, -18, -18, -18, -16, -41, -22, -30, -27, -30, -47, -49, -47, -42, -55,
        -34, -27, -22, -23, -34, -23, -32, -36, -47, -33, -32, -18, -23, -21, -33, -39, -21, -18,
        -27, -5,
    ]
    .into_iter()
    .map(|değer| Some(f64::from(değer)))
    .collect::<Vec<_>>();
    let mut rastgele = KanıtRastgele::yeni(STACKED_SERIES_KANIT_TOHUMU);
    let mut ham = vec![temel];
    for _ in 0..3 {
        let önceki = ham.last().cloned().unwrap_or_default();
        ham.push(
            önceki
                .into_iter()
                .map(|değer| {
                    değer.map(|değer| 100.0 + (değer + rastgele.sonraki() * 100.0 + 0.5).floor())
                })
                .collect(),
        );
    }
    if let Some(ilk) = ham.first_mut() {
        for indeks in 22..26 {
            if let Some(değer) = ilk.get_mut(indeks) {
                *değer = None;
            }
        }
    }
    basit_yığılmış_içerik(
        x,
        ham,
        vec![
            Stil::yeni("bars 1", "green", "rgba(0, 255, 0, 0.3)"),
            Stil::yeni("bars 2", "magenta", "rgba(255, 0, 255, 0.3)"),
            Stil::yeni("bars 3", "blue", "rgba(0, 0, 255, 0.3)"),
            Stil::yeni("bars 4", "red", "rgba(255, 0, 0, 0.3)"),
        ],
        vec![true; 4],
        None,
    )
}

fn interpolasyonlu() -> Kartİçeriği {
    let x = (0..=5).map(f64::from).collect::<Vec<_>>();
    let ham_hücreler = vec![
        vec![0., 1., 2., 3., 4., 5.]
            .into_iter()
            .map(Hücre::Değer)
            .collect::<Vec<_>>(),
        vec![
            Hücre::Değer(5.0),
            Hücre::Değer(4.0),
            Hücre::Değer(3.0),
            Hücre::Tanımsız,
            Hücre::Değer(1.0),
            Hücre::Değer(0.0),
        ],
    ];
    let ham = ham_hücreler
        .iter()
        .map(|seri| seri.iter().copied().map(Hücre::ham).collect())
        .collect::<Vec<_>>();
    let mut interpolasyon = ham_hücreler;
    if let Some(hücre) = interpolasyon.get_mut(1).and_then(|seri| seri.get_mut(3)) {
        *hücre = Hücre::Değer(2.0);
    }
    let çıktı = stack2(
        interpolasyon
            .into_iter()
            .map(|değerler| YığmaGirdisi::normal(değerler, "A"))
            .collect(),
    );
    Kartİçeriği {
        x,
        çizim: çıktı.seriler,
        ham,
        stiller: vec![
            Stil::yeni("A", "green", "rgba(0, 255, 0, 0.3)"),
            Stil::yeni("B", "magenta", "rgba(255, 0, 255, 0.3)"),
        ],
        bantlar: çıktı.bantlar,
        çubuklar: vec![false; 2],
        nokta_boyutu: Some(8.0),
        nokta_indeksleri: vec![None, Some(vec![0, 1, 2, 4, 5])],
        görünür: vec![true; 2],
    }
}

#[derive(Clone, Copy)]
enum KüçükBoşluk {
    Yok,
    İkiTanımsız,
    KırmızıBoş,
    YeşilBoş,
    İkiBoş,
    İkiSıfır,
}

fn küçük_dörtlü(boşluk: KüçükBoşluk, yığ: bool) -> Kartİçeriği {
    let mut seriler = vec![
        vec![Hücre::Değer(5.0); 5],
        vec![Hücre::Değer(-10.0); 5],
        vec![Hücre::Değer(10.0); 5],
        vec![Hücre::Değer(-5.0); 5],
    ];
    let yeşil = match boşluk {
        KüçükBoşluk::İkiTanımsız => Some(Hücre::Tanımsız),
        KüçükBoşluk::YeşilBoş | KüçükBoşluk::İkiBoş => Some(Hücre::Boş),
        KüçükBoşluk::İkiSıfır => Some(Hücre::Değer(0.0)),
        _ => None,
    };
    let kırmızı = match boşluk {
        KüçükBoşluk::İkiTanımsız => Some(Hücre::Tanımsız),
        KüçükBoşluk::KırmızıBoş | KüçükBoşluk::İkiBoş => Some(Hücre::Boş),
        KüçükBoşluk::İkiSıfır => Some(Hücre::Değer(0.0)),
        _ => None,
    };
    if let Some(hücre) = yeşil
        && let Some(hedef) = seriler.get_mut(1).and_then(|seri| seri.get_mut(2))
    {
        *hedef = hücre;
    }
    if let Some(hücre) = kırmızı
        && let Some(hedef) = seriler.get_mut(3).and_then(|seri| seri.get_mut(2))
    {
        *hedef = hücre;
    }
    let ham = ham_değerler(&seriler);
    let çıktı = if yığ {
        stack2(
            seriler
                .into_iter()
                .map(|değerler| YığmaGirdisi::normal(değerler, "A"))
                .collect(),
        )
    } else {
        YığmaÇıktısı {
            seriler,
            bantlar: Vec::new(),
        }
    };
    Kartİçeriği::yeni(
        (0..=4).map(f64::from).collect(),
        çıktı,
        ham,
        küçük_stiller(),
    )
}

fn negatif_y(sıfır: bool) -> Kartİçeriği {
    let boş = if sıfır {
        Hücre::Değer(0.0)
    } else {
        Hücre::Boş
    };
    let seriler = vec![
        vec![
            Hücre::Değer(5.0),
            Hücre::Değer(5.0),
            Hücre::Değer(5.0),
            boş,
            Hücre::Değer(5.0),
            Hücre::Değer(5.0),
        ],
        vec![
            Hücre::Değer(10.0),
            Hücre::Değer(10.0),
            boş,
            Hücre::Değer(10.0),
            Hücre::Değer(10.0),
            Hücre::Değer(10.0),
        ],
    ];
    let ham = ham_değerler(&seriler);
    let çıktı = stack2(vec![
        YığmaGirdisi {
            değerler: seriler.first().cloned().unwrap_or_default(),
            neg_y: true,
            kip: YığmaKipi::Normal,
            grup: "A",
        },
        YığmaGirdisi::normal(seriler.get(1).cloned().unwrap_or_default(), "A"),
    ]);
    Kartİçeriği::yeni(
        (0..=5).map(f64::from).collect(),
        çıktı,
        ham,
        vec![
            Stil::yeni("", "green", "rgba(0, 255, 0, 0.3)"),
            Stil::yeni("", "red", "rgba(255, 0, 0, 0.3)"),
        ],
    )
}

fn negatif_yüzde() -> Kartİçeriği {
    let seriler = vec![
        sayılar(&[5., 5., 5., 5., 5.]),
        sayılar(&[-25., -8., -1., -3., -10.]),
        sayılar(&[10., 35., 100., 10., 10.]),
        sayılar(&[-5., -5., -5., -5., -5.]),
    ];
    let ham = ham_değerler(&seriler);
    let çıktı = stack2(
        seriler
            .into_iter()
            .map(|değerler| YığmaGirdisi {
                değerler,
                neg_y: false,
                kip: YığmaKipi::Yüzde,
                grup: "A",
            })
            .collect(),
    );
    Kartİçeriği::yeni(
        (0..=4).map(f64::from).collect(),
        çıktı,
        ham,
        küçük_stiller(),
    )
}

fn yığma_grupları() -> Kartİçeriği {
    let seriler = vec![
        sayılar(&[5., 5., 5., 5., 5.]),
        sayılar(&[25., 8., 1., 3., 10.]),
        sayılar(&[10., 35., 50., 10., 10.]),
        sayılar(&[5., 5., 5., 5., 5.]),
    ];
    let ham = ham_değerler(&seriler);
    let çıktı = stack2(
        seriler
            .into_iter()
            .enumerate()
            .map(|(indeks, değerler)| {
                YığmaGirdisi::normal(değerler, if indeks == 0 || indeks == 3 { "A" } else { "B" })
            })
            .collect(),
    );
    Kartİçeriği::yeni(
        (0..=4).map(f64::from).collect(),
        çıktı,
        ham,
        küçük_stiller(),
    )
}

fn birleşik_karma() -> Kartİçeriği {
    let x = vec![0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 4.0];
    let m = Hücre::Tanımsız;
    let seriler = vec![
        vec![
            Hücre::Değer(5.0),
            m,
            Hücre::Değer(5.0),
            m,
            Hücre::Değer(5.0),
            m,
            Hücre::Değer(5.0),
            Hücre::Değer(5.0),
        ],
        vec![
            m,
            Hücre::Değer(1.0),
            m,
            Hücre::Değer(2.0),
            m,
            Hücre::Değer(3.0),
            m,
            m,
        ],
        vec![
            m,
            Hücre::Değer(6.0),
            m,
            Hücre::Değer(7.0),
            m,
            Hücre::Değer(8.0),
            m,
            m,
        ],
    ];
    let ham = ham_değerler(&seriler);
    let çıktı = stack2(vec![
        YığmaGirdisi {
            değerler: seriler.first().cloned().unwrap_or_default(),
            neg_y: false,
            kip: YığmaKipi::Yok,
            grup: "A",
        },
        YığmaGirdisi::normal(seriler.get(1).cloned().unwrap_or_default(), "A"),
        YığmaGirdisi::normal(seriler.get(2).cloned().unwrap_or_default(), "A"),
    ]);
    let mut içerik = Kartİçeriği::yeni(
        x,
        çıktı,
        ham,
        vec![
            Stil::yeni("", "blue", "rgba(0, 0, 255, 0.3)"),
            Stil::yeni("", "green", "rgba(0, 255, 0, 0.3)"),
            Stil::yeni("", "orange", "rgba(255, 165, 0, 0.4)"),
        ],
    );
    içerik.çubuklar = vec![false, true, true];
    içerik
}

fn basit_yığılmış_içerik(
    x: Vec<f64>,
    ham: Vec<Vec<Option<f64>>>,
    stiller: Vec<Stil>,
    çubuklar: Vec<bool>,
    nokta_boyutu: Option<f32>,
) -> Kartİçeriği {
    let mut birikim = vec![0.0; x.len()];
    let mut çizim = Vec::with_capacity(ham.len());
    for seri in &ham {
        çizim.push(
            seri.iter()
                .enumerate()
                .map(|(indeks, değer)| {
                    let katkı = değer.unwrap_or(0.0);
                    if let Some(toplam) = birikim.get_mut(indeks) {
                        *toplam += katkı;
                        Hücre::Değer(*toplam)
                    } else {
                        Hücre::Değer(katkı)
                    }
                })
                .collect(),
        );
    }
    let bantlar = (1..çizim.len())
        .map(|indeks| (indeks, indeks - 1, BantYönü::EnAza))
        .collect();
    let seri_sayısı = çizim.len();
    Kartİçeriği {
        x,
        çizim,
        ham,
        stiller,
        bantlar,
        çubuklar,
        nokta_boyutu,
        nokta_indeksleri: vec![None; seri_sayısı],
        görünür: vec![true; seri_sayısı],
    }
}

fn stack2(girdiler: Vec<YığmaGirdisi>) -> YığmaÇıktısı {
    let uzunluk = girdiler.first().map_or(0, |girdi| girdi.değerler.len());
    let mut gruplar = Vec::<Grup>::new();
    let mut anahtarlar = vec![None::<String>; girdiler.len()];
    let mut veri = vec![Vec::<Hücre>::new(); girdiler.len()];
    for (seri_indeksi, girdi) in girdiler.iter().enumerate() {
        let mut değerler = girdi.değerler.clone();
        if girdi.neg_y {
            for hücre in &mut değerler {
                if let Hücre::Değer(değer) = hücre {
                    *değer *= -1.0;
                }
            }
        }
        if girdi.kip == YığmaKipi::Yok {
            if let Some(hedef) = veri.get_mut(seri_indeksi) {
                *hedef = değerler;
            }
            continue;
        }
        let pozitif = değerler
            .iter()
            .any(|hücre| matches!(hücre, Hücre::Değer(değer) if *değer > 0.0));
        let kip = match girdi.kip {
            YığmaKipi::Normal => "normal",
            YığmaKipi::Yüzde => "percent",
            YığmaKipi::Yok => "none",
        };
        let anahtar = format!("{kip}:{}:{}", girdi.grup, if pozitif { '+' } else { '-' });
        let grup_indeksi = gruplar
            .iter()
            .position(|grup| grup.anahtar == anahtar)
            .unwrap_or_else(|| {
                gruplar.push(Grup {
                    anahtar: anahtar.clone(),
                    birikim: vec![0.0; uzunluk],
                    seriler: Vec::new(),
                    yön: if pozitif {
                        BantYönü::EnAza
                    } else {
                        BantYönü::EnÇoğa
                    },
                });
                gruplar.len().saturating_sub(1)
            });
        if let Some(anahtar_hedefi) = anahtarlar.get_mut(seri_indeksi) {
            *anahtar_hedefi = Some(anahtar);
        }
        let Some(grup) = gruplar.get_mut(grup_indeksi) else {
            continue;
        };
        grup.seriler.insert(0, seri_indeksi);
        let yığılmış = değerler
            .into_iter()
            .enumerate()
            .map(|(indeks, hücre)| match hücre {
                Hücre::Değer(değer) => {
                    if let Some(toplam) = grup.birikim.get_mut(indeks) {
                        *toplam += değer;
                        Hücre::Değer(*toplam)
                    } else {
                        Hücre::Değer(değer)
                    }
                }
                Hücre::Boş => Hücre::Boş,
                Hücre::Tanımsız => Hücre::Tanımsız,
            })
            .collect();
        if let Some(hedef) = veri.get_mut(seri_indeksi) {
            *hedef = yığılmış;
        }
    }
    for (seri_indeksi, girdi) in girdiler.iter().enumerate() {
        if girdi.kip != YığmaKipi::Yüzde {
            continue;
        }
        let Some(anahtar) = anahtarlar.get(seri_indeksi).and_then(Option::as_ref) else {
            continue;
        };
        let Some(grup) = gruplar.iter().find(|grup| &grup.anahtar == anahtar) else {
            continue;
        };
        let işaret = if grup.yön == BantYönü::EnAza {
            1.0
        } else {
            -1.0
        };
        let Some(seri) = veri.get_mut(seri_indeksi) else {
            continue;
        };
        for (indeks, hücre) in seri.iter_mut().enumerate() {
            let Hücre::Değer(değer) = hücre else {
                continue;
            };
            let toplam = grup.birikim.get(indeks).copied().unwrap_or(0.0);
            if toplam.abs() > f64::EPSILON {
                *değer = işaret * (*değer / toplam);
            }
        }
    }
    let bantlar = gruplar
        .iter()
        .flat_map(|grup| {
            grup.seriler
                .windows(2)
                .filter_map(|çift| çift.first().zip(çift.get(1)))
                .map(|(üst, alt)| (*üst, *alt, grup.yön))
        })
        .collect();
    YığmaÇıktısı {
        seriler: veri,
        bantlar,
    }
}

fn ham_değerler(seriler: &[Vec<Hücre>]) -> Vec<Vec<Option<f64>>> {
    seriler
        .iter()
        .map(|seri| seri.iter().copied().map(Hücre::ham).collect())
        .collect()
}

fn sayılar(değerler: &[f64]) -> Vec<Hücre> {
    değerler.iter().copied().map(Hücre::Değer).collect()
}

fn klasik_stiller() -> Vec<Stil> {
    vec![
        Stil::yeni("", "purple", "rgba(165, 55, 253, 0.4)"),
        Stil::yeni("", "orange", "rgba(255, 165, 0, 0.4)"),
        Stil::yeni("", "blue", "rgba(0,0,255,0.3)"),
        Stil::yeni("", "green", "rgba(0,255,0,0.3)"),
        Stil::yeni("", "red", "rgba(255,0,0,0.3)"),
    ]
}

fn küçük_stiller() -> Vec<Stil> {
    vec![
        Stil::yeni("", "blue", "rgba(0, 0, 255, 0.3)"),
        Stil::yeni("", "green", "rgba(0, 255, 0, 0.3)"),
        Stil::yeni("", "orange", "rgba(255, 165, 0, 0.4)"),
        Stil::yeni("", "red", "rgba(255, 0, 0, 0.3)"),
    ]
}

impl Stil {
    const fn yeni(etiket: &'static str, çizgi: &'static str, dolgu: &'static str) -> Self {
        Self {
            etiket,
            çizgi,
            dolgu,
        }
    }
}

impl YığmaGirdisi {
    fn normal(değerler: Vec<Hücre>, grup: &'static str) -> Self {
        Self {
            değerler,
            neg_y: false,
            kip: YığmaKipi::Normal,
            grup,
        }
    }
}

impl Kartİçeriği {
    fn yeni(
        x: Vec<f64>,
        çıktı: YığmaÇıktısı,
        ham: Vec<Vec<Option<f64>>>,
        stiller: Vec<Stil>,
    ) -> Self {
        let seri_sayısı = çıktı.seriler.len();
        Self {
            x,
            çizim: çıktı.seriler,
            ham,
            stiller,
            bantlar: çıktı.bantlar,
            çubuklar: vec![false; seri_sayısı],
            nokta_boyutu: None,
            nokta_indeksleri: vec![None; seri_sayısı],
            görünür: vec![true; seri_sayısı],
        }
    }
}

fn görünürlüğü_uygula(
    örnek: StackedSeriesÖrneği,
    içerik: &mut Kartİçeriği,
    görünür: &[bool],
) {
    içerik.görünür = (0..içerik.stiller.len())
        .map(|indeks| görünür.get(indeks).copied().unwrap_or(true))
        .collect();
    if !matches!(
        örnek,
        StackedSeriesÖrneği::Stacked1
            | StackedSeriesÖrneği::Stacked2
            | StackedSeriesÖrneği::BarsStacked
            | StackedSeriesÖrneği::Interpolated
    ) {
        return;
    }
    let mut birikim = vec![0.0; içerik.x.len()];
    let mut çizim = Vec::with_capacity(içerik.ham.len());
    let mut görünür_indeksler = Vec::new();
    for (seri_indeksi, ham) in içerik.ham.iter().enumerate() {
        let etkin = içerik.görünür.get(seri_indeksi).copied().unwrap_or(true);
        if etkin {
            görünür_indeksler.push(seri_indeksi);
        }
        çizim.push(
            ham.iter()
                .enumerate()
                .map(|(indeks, değer)| {
                    let katkı = if örnek == StackedSeriesÖrneği::Interpolated
                        && seri_indeksi == 1
                        && indeks == 3
                    {
                        2.0
                    } else {
                        değer.unwrap_or(0.0)
                    };
                    if etkin {
                        if let Some(toplam) = birikim.get_mut(indeks) {
                            *toplam += katkı;
                            Hücre::Değer(*toplam)
                        } else {
                            Hücre::Değer(katkı)
                        }
                    } else {
                        değer.map_or(Hücre::Boş, Hücre::Değer)
                    }
                })
                .collect(),
        );
    }
    içerik.çizim = çizim;
    içerik.bantlar = görünür_indeksler
        .windows(2)
        .filter_map(|çift| çift.first().zip(çift.get(1)))
        .map(|(alt, üst)| (*üst, *alt, BantYönü::EnAza))
        .collect();
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut};

    #[test]
    fn on_altı_kaynak_yüzeyi_boyut_ve_geometriyle_çizilir() -> Result<(), UplotHatası> {
        for örnek in StackedSeriesÖrneği::TÜMÜ {
            let (seçenekler, veri) = stacked_series_kartı(örnek)?;
            assert_eq!(
                (seçenekler.genişlik, seçenekler.yükseklik),
                örnek.boyut(),
                "{}",
                örnek.kimlik()
            );
            if matches!(
                örnek,
                StackedSeriesÖrneği::Stacked1
                    | StackedSeriesÖrneği::Stacked2
                    | StackedSeriesÖrneği::BarsStacked
                    | StackedSeriesÖrneği::Interpolated
            ) {
                assert_eq!(seçenekler.y_aralığı.map(|aralık| aralık.en_az), Some(0.0));
            }
            assert!(!veri.x().is_empty(), "{}", örnek.kimlik());
            let sahne = Grafik::yeni(seçenekler, veri)?.çiz();
            assert!(
                sahne
                    .komutlar()
                    .iter()
                    .any(|komut| matches!(komut, Komut::Yol { .. } | Komut::Alan { .. })),
                "{}",
                örnek.kimlik()
            );
        }
        Ok(())
    }

    #[test]
    fn klasik_iki_yüzey_aynı_ham_veriyi_ters_sırada_gösterir() -> Result<(), UplotHatası> {
        let (bir, _) = stacked_series_kartı(StackedSeriesÖrneği::Stacked1)?;
        let (iki, _) = stacked_series_kartı(StackedSeriesÖrneği::Stacked2)?;
        let bir_ham = bir
            .seriler
            .iter()
            .filter_map(|seri| seri.lejant_değerleri.clone())
            .collect::<Vec<_>>();
        let mut iki_ham = iki
            .seriler
            .iter()
            .filter_map(|seri| seri.lejant_değerleri.clone())
            .collect::<Vec<_>>();
        iki_ham.reverse();
        assert_eq!(bir_ham, iki_ham);
        Ok(())
    }

    #[test]
    fn null_tanımsız_ve_sıfır_ayrı_anlamları_korur() -> Result<(), UplotHatası> {
        let (_, tanımsız) = stacked_series_kartı(StackedSeriesÖrneği::UndefBoth)?;
        let (_, boş) = stacked_series_kartı(StackedSeriesÖrneği::BothNull)?;
        let (_, sıfır) = stacked_series_kartı(StackedSeriesÖrneği::BothZero)?;
        assert!(tanımsız.hizalama_eksiği_mi(1, 2));
        assert!(!boş.hizalama_eksiği_mi(1, 2));
        assert_eq!(
            sıfır
                .seriler()
                .get(1)
                .and_then(|seri| seri.get(2))
                .copied()
                .flatten(),
            Some(0.0)
        );
        Ok(())
    }

    #[test]
    fn interpolasyon_çizilir_ama_ham_lejant_ve_nokta_filtresi_korunur() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = stacked_series_kartı(StackedSeriesÖrneği::Interpolated)?;
        assert_eq!(
            veri.seriler()
                .get(1)
                .and_then(|seri| seri.get(3))
                .copied()
                .flatten(),
            Some(5.0)
        );
        let ikinci = seçenekler.seriler.get(1);
        assert_eq!(
            ikinci
                .and_then(|seri| seri.lejant_değerleri.as_ref())
                .and_then(|seri| seri.get(3))
                .copied()
                .flatten(),
            None
        );
        assert!(
            ikinci
                .and_then(|seri| seri.nokta_indeksleri.as_ref())
                .is_some_and(|indeksler| !indeksler.contains(&3))
        );
        Ok(())
    }

    #[test]
    fn yüzde_ve_gruplar_kaynak_yığma_sınırlarını_korur() -> Result<(), UplotHatası> {
        let (yüzde_ayarları, yüzde_veri) =
            stacked_series_kartı(StackedSeriesÖrneği::NegativePercent)?;
        assert_eq!(yüzde_ayarları.bantlar.len(), 2);
        assert!(
            yüzde_veri
                .seriler()
                .iter()
                .flat_map(|seri| seri.iter().flatten())
                .all(|değer| (-1.0..=1.0).contains(değer))
        );
        let (grup_ayarları, _) = stacked_series_kartı(StackedSeriesÖrneği::Groups)?;
        assert_eq!(grup_ayarları.bantlar.len(), 2);
        Ok(())
    }

    #[test]
    fn kaynak_set_series_görünürlüğü_kalan_serileri_yeniden_yığar() -> Result<(), UplotHatası> {
        let (_, tümü) = stacked_series_kartı(StackedSeriesÖrneği::Stacked1)?;
        let (ayarlar, gizli) = stacked_series_kartı_görünür(
            StackedSeriesÖrneği::Stacked1,
            &[true, false, true, true, true],
        )?;
        assert!(ayarlar.seriler.get(1).is_some_and(|seri| !seri.göster));
        assert_eq!(ayarlar.bantlar.len(), 3);
        let tüm_tepe = tümü
            .seriler()
            .last()
            .and_then(|seri| seri.first())
            .copied()
            .flatten();
        let gizli_tepe = gizli
            .seriler()
            .last()
            .and_then(|seri| seri.first())
            .copied()
            .flatten();
        assert_eq!(tüm_tepe, Some(50.0));
        assert_eq!(gizli_tepe, Some(40.0));
        Ok(())
    }
}
