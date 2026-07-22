import { createHash } from "node:crypto";
import { readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { basename, relative, resolve } from "node:path";

const kok = resolve(import.meta.dirname, "../..");
const demoKoku = resolve(kok, "../uPlot/demos");
const uplotKoku = resolve(kok, "../uPlot");

const sha256 = (içerik) => createHash("sha256").update(içerik).digest("hex");

function dosyalarıGez(dizin, kabul) {
  const sonuç = [];
  for (const ad of readdirSync(dizin).sort((a, b) => a.localeCompare(b, "en"))) {
    const yol = resolve(dizin, ad);
    if (statSync(yol).isDirectory()) {
      sonuç.push(...dosyalarıGez(yol, kabul));
    } else if (kabul(yol)) {
      sonuç.push(yol);
    }
  }
  return sonuç;
}

const fazlar = [
  {
    no: 0,
    ad: "Envanter ve ortak sözleşme",
    dosyalar: ["resize.html"],
  },
  {
    no: 1,
    ad: "Doğrusal yollar, alanlar ve veri boşlukları",
    dosyalar: [
      "align-data.html", "area-fill.html", "data-smoothing.html",
      "gradients.html", "high-low-bands.html", "line-paths.html",
      "missing-data.html", "no-data.html", "path-gap-clip.html",
      "pixel-align.html", "points.html", "scale-padding.html",
      "soft-minmax.html", "sparklines.html", "sparse.html",
      "stacked-series.html", "y-shifted-series.html",
    ],
  },
  {
    no: 2,
    ad: "Ölçekler, eksenler ve zaman",
    dosyalar: [
      "arcsinh-scales.html", "axis-autosize.html", "axis-control.html",
      "custom-scales.html", "dependent-scale.html", "grid-over-series.html",
      "log-scales.html", "log-scales2.html", "months-ru.html", "months.html",
      "nice-scale.html", "scales-dir-ori.html", "sync-y-zero.html",
      "time-periods.html", "timezones-dst.html", "y-scale-drag.html",
    ],
  },
  {
    no: 3,
    ad: "Özel çizim ve seri eklentileri",
    dosyalar: [
      "annotations.html", "axis-indicators.html", "bars-grouped-stacked.html",
      "bars-values-autosize.html", "box-whisker.html",
      "candlestick-ohlc.html", "draw-hooks.html", "latency-heatmap.html",
      "mass-spectrum.html", "measure-datums.html", "multi-bars.html",
      "scatter.html", "sparklines-bars.html", "svg-image.html",
      "thin-bars-stroke-fill.html", "timeline-discrete.html",
      "timeseries-discrete.html", "trendlines.html", "wind-direction.html",
    ],
  },
  {
    no: 4,
    ad: "Cursor, tooltip ve yakınlaştırma eklentileri",
    dosyalar: [
      "cursor-bind.html", "cursor-snap.html", "cursor-tooltip.html",
      "focus-cursor.html", "nearest-non-null.html", "tooltips-closest.html",
      "tooltips.html", "zoom-fetch.html", "zoom-ranger-grips.html",
      "zoom-ranger-xy.html", "zoom-ranger.html", "zoom-touch.html",
      "zoom-variations.html", "zoom-wheel.html",
    ],
  },
  {
    no: 5,
    ad: "Çoklu grafik, senkronizasyon ve canlı veri",
    dosyalar: [
      "add-del-series.html", "scroll-sync.html", "sine-stream.html",
      "stream-data.html", "sync-cursor.html",
      "update-cursor-select-resize.html",
    ],
  },
];

const fazHaritasi = new Map();
for (const faz of fazlar) {
  for (const dosya of faz.dosyalar) {
    if (fazHaritasi.has(dosya)) {
      throw new Error(`yinelenen faz ataması: ${dosya}`);
    }
    fazHaritasi.set(dosya, faz);
  }
}

const dosyalar = readdirSync(demoKoku)
  .filter((dosya) => dosya.endsWith(".html") && dosya !== "index.html")
  .sort((a, b) => a.localeCompare(b, "en"));

const eksikAtamalar = dosyalar.filter((dosya) => !fazHaritasi.has(dosya));
const olmayanDosyalar = [...fazHaritasi.keys()].filter(
  (dosya) => !dosyalar.includes(dosya),
);
if (eksikAtamalar.length > 0 || olmayanDosyalar.length > 0) {
  throw new Error(
    `faz kapsamı uyuşmuyor; eksik=${eksikAtamalar.join(",")}; olmayan=${olmayanDosyalar.join(",")}`,
  );
}

const htmlCoz = (metin) => metin
  .replaceAll("&amp;", "&")
  .replaceAll("&lt;", "<")
  .replaceAll("&gt;", ">")
  .replaceAll("&quot;", "\"")
  .replaceAll("&#39;", "'");

function veriVarlıkları(içerik, demoYolu) {
  const adaylar = new Set();
  for (const eşleşme of içerik.matchAll(/["'`](\.?\.?\/[^"'`]+|(?:data|lib)\/[^"'`]+)["'`]/g)) {
    const ham = eşleşme[1];
    if (ham.includes("${") || ham.includes(" + ")) {
      continue;
    }
    const yol = resolve(demoYolu, "..", ham);
    try {
      if (statSync(yol).isFile()) {
        adaylar.add(yol);
      }
    } catch {
      // Dinamik veya tarayıcı tarafından oluşturulan yollar ayrı yetenek olarak incelenir.
    }
  }
  return [...adaylar]
    .sort((a, b) => a.localeCompare(b, "en"))
    .map((yol) => {
      const içerik = readFileSync(yol);
      const göreli = relative(uplotKoku, yol);
      if (!göreli.startsWith("demos/data/") &&
          !göreli.startsWith("demos/lib/") &&
          !göreli.startsWith("bench/")) {
        return null;
      }
      return {
        yol: göreli,
        sha256: sha256(içerik),
      };
    })
    .filter((değer) => değer !== null);
}

const uygulanmışDemolar = new Set([
  "add-del-series.html", "align-data.html", "annotations.html", "arcsinh-scales.html",
  "area-fill.html", "axis-autosize.html", "axis-control.html", "axis-indicators.html",
  "bars-grouped-stacked.html", "bars-values-autosize.html", "box-whisker.html",
  "candlestick-ohlc.html", "cursor-bind.html", "cursor-snap.html", "cursor-tooltip.html",
  "custom-scales.html", "data-smoothing.html", "dependent-scale.html", "draw-hooks.html",
  "focus-cursor.html", "gradients.html",
  "missing-data.html", "months.html", "resize.html", "scale-padding.html", "zoom-touch.html",
  "zoom-wheel.html",
]);

const demolar = dosyalar.map((dosya, indeks) => {
  const yol = resolve(demoKoku, dosya);
  const içerik = readFileSync(yol, "utf8");
  const başlık = içerik.match(/<title>(.*?)<\/title>/is)?.[1]?.trim() ?? basename(dosya, ".html");
  const faz = fazHaritasi.get(dosya);
  const durum = uygulanmışDemolar.has(dosya) ? "uygulandı_kanıtlı" : "bekliyor";
  return {
    sıra: indeks + 1,
    id: basename(dosya, ".html"),
    başlık: htmlCoz(başlık),
    kaynak: `demos/${dosya}`,
    kaynak_sha256: sha256(içerik),
    veri_politikası: /Math\.random|randomWalk|randomSkewNormal/.test(içerik)
      ? "aynı_üreteç_ve_parametreler_sabit_kanıt_tohumu"
      : "kaynak_veri_aynen",
    veri_kaynakları: veriVarlıkları(içerik, yol),
    sahip_faz: faz.no,
    faz: faz.ad,
    durum,
  };
});

const çıktı = {
  şema_sürümü: 1,
  kaynak_commit: "0e5812c504430f5c804e0f993376d8999b26cc34",
  demo_sayısı: demolar.length,
  durum_özeti: {
    uygulandı_kanıtlı: demolar.filter((demo) => demo.durum === "uygulandı_kanıtlı").length,
    çekirdek_davranışı_port_edildi_kart_bekliyor: demolar.filter(
      (demo) => demo.durum === "çekirdek_davranışı_port_edildi_kart_bekliyor",
    ).length,
    bekliyor: demolar.filter((demo) => demo.durum === "bekliyor").length,
  },
  demolar,
};

writeFileSync(
  resolve(kok, "uyum/demo_envanteri.json"),
  `${JSON.stringify(çıktı, null, 2)}\n`,
);

const kaynakDosyaları = [
  ...dosyalarıGez(resolve(uplotKoku, "src"), (yol) => yol.endsWith(".js")),
  resolve(uplotKoku, "dist/uPlot.d.ts"),
];

const kaynaklar = kaynakDosyaları.map((yol) => {
  const içerik = readFileSync(yol, "utf8");
  return {
    yol: relative(uplotKoku, yol),
    sha256: sha256(içerik),
    satır_sayısı: içerik.split(/\r?\n/).length,
  };
});

const veriVarlığıYolları = [
  ...dosyalarıGez(resolve(uplotKoku, "demos/data"), () => true),
  ...dosyalarıGez(resolve(uplotKoku, "demos/lib"), (yol) => yol.endsWith(".js")),
  ...["bench/data.json", "bench/results.json"]
    .map((yol) => resolve(uplotKoku, yol))
    .filter((yol) => {
      try {
        return statSync(yol).isFile();
      } catch {
        return false;
      }
    }),
];
const tümVeriVarlıkları = veriVarlığıYolları.map((yol) => {
  const içerik = readFileSync(yol);
  return { yol: relative(uplotKoku, yol), sha256: sha256(içerik) };
});

const bildirim = readFileSync(resolve(uplotKoku, "dist/uPlot.d.ts"), "utf8");
const genelApi = [];
let kapsam = null;
let derinlik = 0;
for (const satır of bildirim.split(/\r?\n/)) {
  const başlangıç = satır.match(/^\s*(?:export\s+)?(?:declare\s+)?(?:class|interface)\s+([A-Za-z_$][\w$]*)/);
  if (başlangıç && kapsam === null) {
    kapsam = başlangıç[1];
    derinlik = 0;
  }
  if (kapsam !== null && derinlik === 1) {
    const üye = satır.match(/^\s*(?:readonly\s+|static\s+|get\s+)*([A-Za-z_$][\w$]*)\??\s*(?::|\()/);
    if (üye) {
      genelApi.push(`${kapsam}.${üye[1]}`);
    }
  }
  if (kapsam !== null) {
    derinlik += (satır.match(/{/g) ?? []).length;
    derinlik -= (satır.match(/}/g) ?? []).length;
    if (derinlik === 0 && satır.includes("}")) {
      kapsam = null;
    }
  }
}

const yolÜreteçleri = [];
for (const yol of kaynakDosyaları.filter((dosya) => dosya.includes("/src/paths/") && dosya.endsWith(".js"))) {
  const içerik = readFileSync(yol, "utf8");
  for (const eşleşme of içerik.matchAll(/export\s+(?:default\s+)?(?:function|const)\s+([A-Za-z_$][\w$]*)/g)) {
    yolÜreteçleri.push({
      sembol: eşleşme[1],
      kaynak: relative(uplotKoku, yol),
    });
  }
}

const uygulananApi = new Set([
  "Options.width", "Options.height", "Options.title", "Options.series",
  "Scale.time", "Scale.range", "Series.stroke", "Series.width",
]);
const kaynakEnvanteri = {
  şema_sürümü: 1,
  kaynak_commit: "0e5812c504430f5c804e0f993376d8999b26cc34",
  kaynaklar,
  veri_varlıkları: tümVeriVarlıkları,
  genel_api: [...new Set(genelApi)].sort((a, b) => a.localeCompare(b, "en")).map((sembol) => ({
    sembol,
    durum: uygulananApi.has(sembol) ? "kısmi_veya_uygulandı" : "araştırılacak",
  })),
  yerleşik_yol_üreteçleri: yolÜreteçleri,
};
writeFileSync(
  resolve(kok, "uyum/kaynak_envanteri.json"),
  `${JSON.stringify(kaynakEnvanteri, null, 2)}\n`,
);

process.stdout.write(
  `uPlot envanteri üretildi: ${demolar.length} demo, ${kaynaklar.length} kaynak, ${kaynakEnvanteri.genel_api.length} genel API üyesi\n`,
);
