import { createHash } from "node:crypto";
import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

const kok = resolve(import.meta.dirname, "../..");
const kaynak = resolve(kok, "../uPlot");
const beklenenCommit = "0e5812c504430f5c804e0f993376d8999b26cc34";
const beklenenSurum = "1.6.32";

function hata(mesaj) {
  process.stderr.write(`uyum denetimi başarısız: ${mesaj}\n`);
  process.exitCode = 1;
}

function sha256(yol) {
  return createHash("sha256").update(readFileSync(yol)).digest("hex");
}

const commit = execFileSync("git", ["rev-parse", "HEAD"], {
  cwd: kaynak,
  encoding: "utf8",
}).trim();
if (commit !== beklenenCommit) {
  hata(`uPlot commit ${commit}; beklenen ${beklenenCommit}`);
}

const paket = JSON.parse(readFileSync(resolve(kaynak, "package.json"), "utf8"));
if (paket.version !== beklenenSurum) {
  hata(`uPlot sürümü ${paket.version}; beklenen ${beklenenSurum}`);
}

const manifest = JSON.parse(
  readFileSync(resolve(kok, "uyum/demo_manifesti.json"), "utf8"),
);
const matris = JSON.parse(
  readFileSync(resolve(kok, "uyum/api_matrisi.json"), "utf8"),
);
const demoEnvanteri = JSON.parse(
  readFileSync(resolve(kok, "uyum/demo_envanteri.json"), "utf8"),
);
const kaynakEnvanteri = JSON.parse(
  readFileSync(resolve(kok, "uyum/kaynak_envanteri.json"), "utf8"),
);
const davranışSözleşmesi = JSON.parse(
  readFileSync(resolve(kok, "uyum/ortak_davranis_sozlesmesi.json"), "utf8"),
);

const davranışKimlikleri = new Set();
for (const davranış of davranışSözleşmesi.davranışlar) {
  if (davranışKimlikleri.has(davranış.id)) {
    hata(`yinelenen ortak davranış kimliği: ${davranış.id}`);
  }
  davranışKimlikleri.add(davranış.id);
  if (!davranış.açıklama || !davranış.tür || !davranış.başlık) {
    hata(`eksik ortak davranış tanımı: ${davranış.id}`);
  }
  for (const kanıtDosyası of davranış.kanıt_dosyaları ?? []) {
    readFileSync(resolve(kok, kanıtDosyası));
  }
}
if (davranışKimlikleri.size < 19) {
  hata(`ortak davranış sözleşmesi beklenenden küçük: ${davranışKimlikleri.size}`);
}
const izinliKararlar = new Set(davranışSözleşmesi.izinli_kararlar);

if (demoEnvanteri.demo_sayısı !== 73 || demoEnvanteri.demolar.length !== 73) {
  hata(`demo envanteri 73 kayıt içermiyor: ${demoEnvanteri.demolar.length}`);
}
for (const demo of demoEnvanteri.demolar) {
  if (sha256(resolve(kaynak, demo.kaynak)) !== demo.kaynak_sha256) {
    hata(`demo envanteri kaynak hash'i değişti: ${demo.kaynak}`);
  }
  for (const veri of demo.veri_kaynakları) {
    if (sha256(resolve(kaynak, veri.yol)) !== veri.sha256) {
      hata(`demo veri hash'i değişti: ${veri.yol}`);
    }
  }
}

if (kaynakEnvanteri.genel_api.length < 250) {
  hata(`genel API envanteri beklenenden küçük: ${kaynakEnvanteri.genel_api.length}`);
}
for (const dosya of kaynakEnvanteri.kaynaklar) {
  if (sha256(resolve(kaynak, dosya.yol)) !== dosya.sha256) {
    hata(`uPlot kaynak hash'i değişti: ${dosya.yol}`);
  }
}
for (const dosya of kaynakEnvanteri.veri_varlıkları) {
  if (sha256(resolve(kaynak, dosya.yol)) !== dosya.sha256) {
    hata(`uPlot veri varlığı hash'i değişti: ${dosya.yol}`);
  }
}

const kimlikler = new Set();
for (const kart of manifest.kartlar) {
  if (kimlikler.has(kart.id)) {
    hata(`yinelenen kart kimliği: ${kart.id}`);
  }
  kimlikler.add(kart.id);
  const kartKaynagi = resolve(kaynak, kart.kaynak);
  if (sha256(kartKaynagi) !== kart.kaynak_sha256) {
    hata(`${kart.id} kaynak hash'i değişti`);
  }
  for (const ekKaynak of kart.ek_kaynaklar ?? []) {
    if (sha256(resolve(kaynak, ekKaynak.yol)) !== ekKaynak.sha256) {
      hata(`${kart.id} ek kaynak hash'i değişti: ${ekKaynak.yol}`);
    }
  }
  for (const yerelYol of [
    kart.örnek,
    kart.masaüstü_örneği,
    kart.wasm_örneği,
    kart.senaryo,
  ]) {
    readFileSync(resolve(kok, yerelYol));
  }

  const kartSözleşmesi = kart.ortak_davranış_sözleşmesi;
  if (kartSözleşmesi?.sürüm !== davranışSözleşmesi.şema_sürümü) {
    hata(`${kart.id} ortak davranış sözleşmesi sürümü eksik veya güncel değil`);
    continue;
  }
  const [profilDosyası, profilSembolü] =
    kartSözleşmesi.uygulama?.split("#") ?? [];
  if (!profilDosyası || profilSembolü !== "ortak_kart_etkileşimleri") {
    hata(`${kart.id} ortak davranış profili uygulama kaydı eksik`);
  } else if (!readFileSync(resolve(kok, profilDosyası), "utf8").includes(profilSembolü)) {
    hata(`${kart.id} ortak davranış profili kart kaynağında uygulanmamış`);
  }
  const kararlar = kartSözleşmesi.kararlar ?? {};
  const gerekçeler = kartSözleşmesi.gerekçeler ?? {};
  for (const davranışKimliği of davranışKimlikleri) {
    const karar = kararlar[davranışKimliği];
    if (!izinliKararlar.has(karar)) {
      hata(`${kart.id} davranış kararı eksik/geçersiz: ${davranışKimliği}`);
      continue;
    }
    if (
      (karar === "kartta_kapalı" || karar === "uygulanamaz") &&
      !gerekçeler[davranışKimliği]?.trim()
    ) {
      hata(`${kart.id} davranış gerekçesi eksik: ${davranışKimliği}`);
    }
  }
  for (const davranışKimliği of Object.keys(kararlar)) {
    if (!davranışKimlikleri.has(davranışKimliği)) {
      hata(`${kart.id} bilinmeyen davranış kararı: ${davranışKimliği}`);
    }
  }
  for (const davranışKimliği of Object.keys(gerekçeler)) {
    if (!davranışKimlikleri.has(davranışKimliği)) {
      hata(`${kart.id} bilinmeyen davranış gerekçesi: ${davranışKimliği}`);
    }
  }
}

const tipHash = sha256(resolve(kaynak, "dist/uPlot.d.ts"));
if (tipHash !== matris.kaynak_sha256) {
  hata("uPlot.d.ts hash'i API matrisiyle eşleşmiyor");
}

if (process.exitCode !== 1) {
  process.stdout.write(
    `uyum denetimi geçti: ${manifest.kartlar.length} kart, ${matris.satırlar.length} API satırı, ${davranışKimlikleri.size} ortak davranış\n`,
  );
}
