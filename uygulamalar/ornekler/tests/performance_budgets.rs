#![allow(unexpected_cfgs)]

use std::{
    hint::black_box,
    time::{Duration, Instant},
};

use uplot_rs_gpui_ornekler::{
    GpuiGrafik, GpuiSvgKayıtAyarları, Grafik, LatencyHeatmapÖrneği, MultiBarsÖrneği, SineAkışı,
    SparseÖrneği, SyncCursorÖrneği, TekerlekEkseni, UplotHatası,
    diagnostics::{GpuiRetainedBoyaÖlçer, Komut},
    latency_heatmap_kartı, mass_spectrum_kartı, multi_bars_kartı, resize_kartı, sparse_kartı,
    sync_cursor_kartı,
};

const ÖLÇÜM_TURU: usize = 180;
const AĞIR_ÖLÇÜM_TURU: usize = 36;
const ISINMA_TURU: usize = 12;
const KARE_BÜTÇESİ: Duration = Duration::from_micros(16_700);
const P99_BÜTÇESİ: Duration = Duration::from_micros(33_400);
const RSS_ÜST_SINIRI_KIB: u64 = 512 * 1_024;
const RSS_BÜYÜME_SINIRI_KIB: u64 = 128 * 1_024;

#[derive(Debug, Clone, Copy)]
struct TahsisÖlçümü {
    sayı: u64,
    toplam_bayt: u64,
    azami_canlı_bayt: usize,
}

#[cfg(phase12_allocator)]
mod sayan_ayırıcı {
    use std::{
        alloc::{GlobalAlloc, Layout, System},
        sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering},
    };

    use super::TahsisÖlçümü;

    pub struct SayanAyırıcı;

    static ÖLÇÜM_ETKİN: AtomicBool = AtomicBool::new(false);
    static TAHSİS_SAYISI: AtomicU64 = AtomicU64::new(0);
    static TAHSİS_EDİLEN_BAYT: AtomicU64 = AtomicU64::new(0);
    static CANLI_BAYT: AtomicUsize = AtomicUsize::new(0);
    static AZAMİ_CANLI_BAYT: AtomicUsize = AtomicUsize::new(0);

    #[global_allocator]
    static AYIRICI: SayanAyırıcı = SayanAyırıcı;

    unsafe impl GlobalAlloc for SayanAyırıcı {
        unsafe fn alloc(&self, düzen: Layout) -> *mut u8 {
            let işaretçi = unsafe { System.alloc(düzen) };
            if !işaretçi.is_null() && ÖLÇÜM_ETKİN.load(Ordering::Relaxed) {
                tahsisi_kaydet(düzen.size());
            }
            işaretçi
        }

        unsafe fn alloc_zeroed(&self, düzen: Layout) -> *mut u8 {
            let işaretçi = unsafe { System.alloc_zeroed(düzen) };
            if !işaretçi.is_null() && ÖLÇÜM_ETKİN.load(Ordering::Relaxed) {
                tahsisi_kaydet(düzen.size());
            }
            işaretçi
        }

        unsafe fn dealloc(&self, işaretçi: *mut u8, düzen: Layout) {
            if ÖLÇÜM_ETKİN.load(Ordering::Relaxed) {
                canlı_baytı_azalt(düzen.size());
            }
            unsafe { System.dealloc(işaretçi, düzen) };
        }

        unsafe fn realloc(&self, işaretçi: *mut u8, eski: Layout, yeni_boyut: usize) -> *mut u8 {
            let yeni_işaretçi = unsafe { System.realloc(işaretçi, eski, yeni_boyut) };
            if !yeni_işaretçi.is_null() && ÖLÇÜM_ETKİN.load(Ordering::Relaxed) {
                canlı_baytı_azalt(eski.size());
                tahsisi_kaydet(yeni_boyut);
            }
            yeni_işaretçi
        }
    }

    fn tahsisi_kaydet(bayt: usize) {
        TAHSİS_SAYISI.fetch_add(1, Ordering::Relaxed);
        TAHSİS_EDİLEN_BAYT.fetch_add(bayt as u64, Ordering::Relaxed);
        let canlı = CANLI_BAYT.fetch_add(bayt, Ordering::Relaxed) + bayt;
        AZAMİ_CANLI_BAYT.fetch_max(canlı, Ordering::Relaxed);
    }

    fn canlı_baytı_azalt(bayt: usize) {
        let _ = CANLI_BAYT.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |mevcut| {
            Some(mevcut.saturating_sub(bayt))
        });
    }

    pub fn ölç<T>(işlem: impl FnOnce() -> T) -> (T, TahsisÖlçümü) {
        TAHSİS_SAYISI.store(0, Ordering::Relaxed);
        TAHSİS_EDİLEN_BAYT.store(0, Ordering::Relaxed);
        CANLI_BAYT.store(0, Ordering::Relaxed);
        AZAMİ_CANLI_BAYT.store(0, Ordering::Relaxed);
        ÖLÇÜM_ETKİN.store(true, Ordering::SeqCst);
        let sonuç = işlem();
        ÖLÇÜM_ETKİN.store(false, Ordering::SeqCst);
        (
            sonuç,
            TahsisÖlçümü {
                sayı: TAHSİS_SAYISI.load(Ordering::Relaxed),
                toplam_bayt: TAHSİS_EDİLEN_BAYT.load(Ordering::Relaxed),
                azami_canlı_bayt: AZAMİ_CANLI_BAYT.load(Ordering::Relaxed),
            },
        )
    }
}

#[cfg(phase12_allocator)]
fn tahsisleri_ölç<T>(işlem: impl FnOnce() -> T) -> (T, TahsisÖlçümü) {
    sayan_ayırıcı::ölç(işlem)
}

#[cfg(not(phase12_allocator))]
fn tahsisleri_ölç<T>(işlem: impl FnOnce() -> T) -> (T, TahsisÖlçümü) {
    (
        işlem(),
        TahsisÖlçümü {
            sayı: 0,
            toplam_bayt: 0,
            azami_canlı_bayt: 0,
        },
    )
}

#[derive(Debug, Clone, Copy)]
struct SüreDağılımı {
    p50: Duration,
    p95: Duration,
    p99: Duration,
    azami: Duration,
}

#[derive(Debug)]
struct AğırSenaryoÖlçümü {
    ad: &'static str,
    ilk_çizim: Duration,
    yeniden_çizim: SüreDağılımı,
    zoom: SüreDağılımı,
    komut: usize,
    geometri: usize,
    tahsis: TahsisÖlçümü,
}

fn süre_dağılımı(mut ölçümler: Vec<Duration>) -> SüreDağılımı {
    ölçümler.sort_unstable();
    let yüzdelik = |yüzde: usize| {
        let son = ölçümler.len().saturating_sub(1);
        let indeks = son.saturating_mul(yüzde).div_ceil(100);
        ölçümler.get(indeks).copied().unwrap_or_default()
    };
    SüreDağılımı {
        p50: yüzdelik(50),
        p95: yüzdelik(95),
        p99: yüzdelik(99),
        azami: ölçümler.last().copied().unwrap_or_default(),
    }
}

fn geometri_öğesi_sayısı(komut: &Komut) -> usize {
    match komut {
        Komut::ArkaPlan { .. } => 1,
        Komut::Çizgi { .. } | Komut::KesikliÇizgi { .. } => 2,
        Komut::Yol { parçalar, .. }
        | Komut::GradyanYol { parçalar, .. }
        | Komut::KesikliYol { parçalar, .. } => parçalar.iter().map(Vec::len).sum(),
        Komut::Alan { çokgenler, .. } | Komut::GradyanAlan { çokgenler, .. } => {
            çokgenler.iter().map(Vec::len).sum()
        }
        Komut::Daire { .. }
        | Komut::Dikdörtgen { .. }
        | Komut::YuvarlatılmışDikdörtgen { .. }
        | Komut::Metin { .. }
        | Komut::DöndürülmüşMetin { .. } => 1,
        Komut::Daireler { merkezler, .. } => merkezler.len(),
        Komut::DeğişkenDaireler { daireler, .. } => daireler.len(),
    }
}

fn resize_ölç(
    nokta_sayısı: usize,
) -> Result<(SüreDağılımı, usize, usize, TahsisÖlçümü), UplotHatası> {
    for _ in 0..ISINMA_TURU {
        let (seçenekler, veri) = resize_kartı(nokta_sayısı)?;
        black_box(Grafik::yeni(seçenekler, veri)?.çiz());
    }

    let mut süreler = Vec::with_capacity(ÖLÇÜM_TURU);
    let (sayılar_sonucu, tahsis) = tahsisleri_ölç(|| {
        let mut son_sayılar = (0, 0);
        for _ in 0..ÖLÇÜM_TURU {
            let başlangıç = Instant::now();
            let (seçenekler, veri) = resize_kartı(nokta_sayısı)?;
            let sahne = Grafik::yeni(seçenekler, veri)?.çiz();
            süreler.push(başlangıç.elapsed());
            son_sayılar = (
                sahne.komutlar().len(),
                sahne.komutlar().iter().map(geometri_öğesi_sayısı).sum(),
            );
            black_box(sahne);
        }
        Ok::<_, UplotHatası>(son_sayılar)
    });
    let sayılar = sayılar_sonucu?;
    Ok((süre_dağılımı(süreler), sayılar.0, sayılar.1, tahsis))
}

fn sine_akışı_ölç() -> Result<(SüreDağılımı, usize, usize, TahsisÖlçümü), UplotHatası> {
    let mut akış = SineAkışı::kanıt()?;
    let (seçenekler, veri) = akış.kartı()?;
    let mut grafik = Grafik::yeni(seçenekler, veri)?;

    for _ in 0..ISINMA_TURU {
        grafik.canlı_veriyi_ayarla(akış.ilerlet()?)?;
        black_box(grafik.çiz());
    }

    let mut süreler = Vec::with_capacity(ÖLÇÜM_TURU);
    let (sayılar_sonucu, tahsis) = tahsisleri_ölç(|| {
        let mut son_sayılar = (0, 0);
        for _ in 0..ÖLÇÜM_TURU {
            let başlangıç = Instant::now();
            grafik.canlı_veriyi_ayarla(akış.ilerlet()?)?;
            let sahne = grafik.çiz();
            süreler.push(başlangıç.elapsed());
            son_sayılar = (
                sahne.komutlar().len(),
                sahne.komutlar().iter().map(geometri_öğesi_sayısı).sum(),
            );
            black_box(sahne);
        }
        Ok::<_, UplotHatası>(son_sayılar)
    });
    let sayılar = sayılar_sonucu?;
    Ok((süre_dağılımı(süreler), sayılar.0, sayılar.1, tahsis))
}

fn ağır_senaryoyu_ölç(
    ad: &'static str,
    kart: impl Fn() -> Result<
        (
            uplot_rs_gpui_ornekler::GrafikSeçenekleri,
            uplot_rs_gpui_ornekler::HizalıVeri,
        ),
        UplotHatası,
    >,
) -> Result<AğırSenaryoÖlçümü, UplotHatası> {
    let (seçenekler, veri) = kart()?;
    let mut grafik = Grafik::yeni(seçenekler, veri)?;

    let başlangıç = Instant::now();
    black_box(grafik.çiz());
    let ilk_çizim = başlangıç.elapsed();

    let mut yeniden_çizimler = Vec::with_capacity(AĞIR_ÖLÇÜM_TURU);
    let (sayılar_sonucu, tahsis) = tahsisleri_ölç(|| {
        let mut son_sayılar = (0, 0);
        for _ in 0..AĞIR_ÖLÇÜM_TURU {
            let başlangıç = Instant::now();
            let sahne = grafik.çiz();
            yeniden_çizimler.push(başlangıç.elapsed());
            son_sayılar = (
                sahne.komutlar().len(),
                sahne.komutlar().iter().map(geometri_öğesi_sayısı).sum(),
            );
            black_box(sahne);
        }
        Ok::<_, UplotHatası>(son_sayılar)
    });
    let (komut, geometri) = sayılar_sonucu?;

    let mut zoomlar = Vec::with_capacity(AĞIR_ÖLÇÜM_TURU);
    for tur in 0..AĞIR_ÖLÇÜM_TURU {
        if tur % 2 == 0 {
            let _ = grafik.tekerlek_eksende(0.5, 0.5, -1.0, false, TekerlekEkseni::İkisi)?;
        } else {
            let _ = grafik.tam_görünüm();
        }
        let başlangıç = Instant::now();
        black_box(grafik.çiz());
        zoomlar.push(başlangıç.elapsed());
    }

    Ok(AğırSenaryoÖlçümü {
        ad,
        ilk_çizim,
        yeniden_çizim: süre_dağılımı(yeniden_çizimler),
        zoom: süre_dağılımı(zoomlar),
        komut,
        geometri,
        tahsis,
    })
}

#[cfg(target_os = "linux")]
fn linux_rss_kib() -> Option<u64> {
    std::fs::read_to_string("/proc/self/status")
        .ok()?
        .lines()
        .find_map(|satır| {
            let değer = satır.strip_prefix("VmRSS:")?.trim();
            değer.split_ascii_whitespace().next()?.parse().ok()
        })
}

#[cfg(not(target_os = "linux"))]
fn linux_rss_kib() -> Option<u64> {
    None
}

fn süre_bütçesini_doğrula(ad: &str, dağılım: SüreDağılımı) {
    assert!(
        dağılım.p50 <= KARE_BÜTÇESİ / 2,
        "{ad} p50 {:?}, bütçe {:?}; tüm dağılım: {dağılım:?}",
        dağılım.p50,
        KARE_BÜTÇESİ / 2
    );
    assert!(
        dağılım.p95 <= KARE_BÜTÇESİ,
        "{ad} p95 {:?}, bütçe {KARE_BÜTÇESİ:?}; tüm dağılım: {dağılım:?}",
        dağılım.p95
    );
    assert!(
        dağılım.p99 <= P99_BÜTÇESİ,
        "{ad} p99 {:?}, bütçe {P99_BÜTÇESİ:?}; tüm dağılım: {dağılım:?}",
        dağılım.p99
    );
    assert!(
        dağılım.azami <= Duration::from_millis(100),
        "{ad} azami süre {:?}; tüm dağılım: {dağılım:?}",
        dağılım.azami
    );
}

#[test]
fn phase_12_release_performans_kapıları() -> Result<(), UplotHatası> {
    if cfg!(debug_assertions) {
        return Ok(());
    }

    let başlangıç_rss = linux_rss_kib();
    let (resize_100, resize_100_komut, resize_100_geometri, resize_100_tahsis) = resize_ölç(100)?;
    let (resize_1000, resize_1000_komut, resize_1000_geometri, resize_1000_tahsis) =
        resize_ölç(1_000)?;
    let (sine, sine_komut, sine_geometri, sine_tahsis) = sine_akışı_ölç()?;
    let ağır_senaryolar = [
        ağır_senaryoyu_ölç("Multi Bars", || {
            multi_bars_kartı(MultiBarsÖrneği::KitaplıklarDikey)
        })?,
        ağır_senaryoyu_ölç("Latency Heatmap ~35K", || {
            latency_heatmap_kartı(LatencyHeatmapÖrneği::Ham, 5.0, 0.0)
        })?,
        ağır_senaryoyu_ölç("Latency Heatmap ~20K", || {
            latency_heatmap_kartı(LatencyHeatmapÖrneği::Kovalanmış, 5.0, 0.0)
        })?,
        ağır_senaryoyu_ölç("Mass Spectrum 41.986", mass_spectrum_kartı)?,
        ağır_senaryoyu_ölç("Sparse 13.608", || {
            sparse_kartı(SparseÖrneği::YerleşikDoğrusal)
        })?,
        ağır_senaryoyu_ölç("Sync Cursor CPU", || {
            sync_cursor_kartı(SyncCursorÖrneği::Cpu)
        })?,
    ];
    let bitiş_rss = linux_rss_kib();

    süre_bütçesini_doğrula("Resize 100", resize_100);
    süre_bütçesini_doğrula("Resize 1000", resize_1000);
    süre_bütçesini_doğrula("Sine setData + scene", sine);
    for ölçüm in &ağır_senaryolar {
        assert!(
            ölçüm.ilk_çizim <= Duration::from_millis(100),
            "{} ilk çizim {:?}",
            ölçüm.ad,
            ölçüm.ilk_çizim
        );
        süre_bütçesini_doğrula(ölçüm.ad, ölçüm.yeniden_çizim);
        süre_bütçesini_doğrula(ölçüm.ad, ölçüm.zoom);
        assert!(ölçüm.komut <= 2_048, "{ölçüm:?}");
        assert!(ölçüm.geometri <= 250_000, "{ölçüm:?}");
        if cfg!(phase12_allocator) {
            assert!(
                ölçüm.tahsis.azami_canlı_bayt <= 128 * 1_024 * 1_024,
                "{ölçüm:?}"
            );
            assert!(
                ölçüm.tahsis.toplam_bayt <= 2 * 1_024 * 1_024 * 1_024,
                "{ölçüm:?}"
            );
        }
    }

    assert_eq!(resize_100_komut, resize_1000_komut);
    assert!(resize_100_komut <= 64, "{resize_100_komut}");
    assert!(resize_100_geometri <= 512, "{resize_100_geometri}");
    assert!(resize_1000_geometri <= 2_048, "{resize_1000_geometri}");
    assert!(sine_komut <= 128, "{sine_komut}");
    assert!(sine_geometri <= 8_192, "{sine_geometri}");

    if cfg!(phase12_allocator) {
        let resize_100_tur_tahsis = resize_100_tahsis.sayı / ÖLÇÜM_TURU as u64;
        let resize_1000_tur_tahsis = resize_1000_tahsis.sayı / ÖLÇÜM_TURU as u64;
        let sine_tur_tahsis = sine_tahsis.sayı / ÖLÇÜM_TURU as u64;
        assert!(resize_100_tur_tahsis <= 512, "{resize_100_tahsis:?}");
        assert!(resize_1000_tur_tahsis <= 512, "{resize_1000_tahsis:?}");
        // 600 × 6 sütunlu kayan pencere her tikte yeni hizalı veri sahibini
        // kurar. 640/tur sınırı mevcut ~543 tahsise %18 regress payı bırakır.
        assert!(sine_tur_tahsis <= 640, "{sine_tahsis:?}");
        assert!(
            resize_1000_tahsis.azami_canlı_bayt <= 32 * 1_024 * 1_024,
            "{resize_1000_tahsis:?}"
        );
        assert!(
            sine_tahsis.azami_canlı_bayt <= 64 * 1_024 * 1_024,
            "{sine_tahsis:?}"
        );
        assert!(
            resize_1000_tahsis.toplam_bayt <= 512 * 1_024 * 1_024,
            "{resize_1000_tahsis:?}"
        );
        assert!(
            sine_tahsis.toplam_bayt <= 768 * 1_024 * 1_024,
            "{sine_tahsis:?}"
        );
    }

    if let Some(rss) = bitiş_rss {
        assert!(rss <= RSS_ÜST_SINIRI_KIB, "VmRSS {rss} KiB");
        if let Some(başlangıç) = başlangıç_rss {
            assert!(
                rss.saturating_sub(başlangıç) <= RSS_BÜYÜME_SINIRI_KIB,
                "VmRSS {başlangıç} KiB -> {rss} KiB"
            );
        }
    }

    let (svg_seçenekler, svg_veri) = resize_kartı(1_000)?;
    let svg_grafik = GpuiGrafik::yeni(Grafik::yeni(svg_seçenekler, svg_veri)?);
    let svg_başlangıç = Instant::now();
    let svg = svg_grafik.svg_kaydı(GpuiSvgKayıtAyarları::yeni(1_200, 600)?);
    let svg_süresi = svg_başlangıç.elapsed();
    assert!(svg_süresi <= Duration::from_millis(100), "{svg_süresi:?}");
    assert!(svg.byte_değeri().len() <= 2 * 1_024 * 1_024);

    let mut retained_ölçer = GpuiRetainedBoyaÖlçer::yeni();
    black_box(retained_ölçer.tur());
    let (_, retained_tahsis) = tahsisleri_ölç(|| {
        for _ in 0..1_000 {
            black_box(retained_ölçer.tur());
        }
    });
    if cfg!(phase12_allocator) {
        assert_eq!(retained_tahsis.sayı, 0, "{retained_tahsis:?}");
        assert_eq!(retained_tahsis.toplam_bayt, 0, "{retained_tahsis:?}");
        assert_eq!(retained_tahsis.azami_canlı_bayt, 0, "{retained_tahsis:?}");
    }

    eprintln!(
        "Phase 12 | resize100={resize_100:?} alloc={resize_100_tahsis:?} | \
         resize1000={resize_1000:?} alloc={resize_1000_tahsis:?} | \
         sine={sine:?} alloc={sine_tahsis:?} | rss={başlangıç_rss:?}->{bitiş_rss:?}"
    );
    for ölçüm in &ağır_senaryolar {
        eprintln!("Phase 12 heavy | {ölçüm:?}");
    }
    eprintln!(
        "Phase 12 SVG | süre={svg_süresi:?} boyut={} bayt",
        svg.byte_değeri().len()
    );
    eprintln!("Phase 12 retained 1000 tur | alloc={retained_tahsis:?}");
    Ok(())
}
