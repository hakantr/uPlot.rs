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
}

const tipHash = sha256(resolve(kaynak, "dist/uPlot.d.ts"));
if (tipHash !== matris.kaynak_sha256) {
  hata("uPlot.d.ts hash'i API matrisiyle eşleşmiyor");
}

if (process.exitCode !== 1) {
  process.stdout.write(
    `uyum denetimi geçti: ${manifest.kartlar.length} kart, ${matris.satırlar.length} API satırı\n`,
  );
}
