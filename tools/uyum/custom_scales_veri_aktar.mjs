import { readFileSync, writeFileSync } from "node:fs";
import vm from "node:vm";

const kaynakYolu = new URL("../../../uPlot/demos/custom-scales.html", import.meta.url);
const hedefYolu = new URL("../../src/kart/veri/custom_scales_kaynak.rs", import.meta.url);
const kaynak = readFileSync(kaynakYolu, "utf8");
const veriBaşlangıcı = kaynak.indexOf("let data =");
const noktaBaşlangıcı = kaynak.indexOf("let pointVals");
const kancaBaşlangıcı = kaynak.indexOf("const hooks");
if (veriBaşlangıcı < 0 || noktaBaşlangıcı < 0 || kancaBaşlangıcı < 0) {
  throw new Error("custom-scales.html veri blokları bulunamadı");
}
const bağlam = {};
vm.createContext(bağlam);
vm.runInContext(
  kaynak.slice(veriBaşlangıcı, noktaBaşlangıcı).replace("let data =", "data =") +
  kaynak.slice(noktaBaşlangıcı, kancaBaşlangıcı).replace("let pointVals =", "pointVals ="),
  bağlam,
);
const dizi = (ad, değerler) =>
  `pub const ${ad}: [f64; ${değerler.length}] = [\n${değerler.map(v => `    ${v},`).join("\n")}\n];\n`;
const çıktı = `// tools/uyum/custom_scales_veri_aktar.mjs tarafından resmî demodan üretilir.\n` +
  dizi("CUSTOM_X", bağlam.data[0]) + dizi("CUSTOM_UPPER_CI", bağlam.data[1]) +
  dizi("CUSTOM_LOWER_CI", bağlam.data[2]) + dizi("CUSTOM_WEIBULL", bağlam.data[3]) +
  dizi("CUSTOM_POINT_X", bağlam.pointVals[0]) + dizi("CUSTOM_POINT_Y", bağlam.pointVals[1]);
writeFileSync(hedefYolu, çıktı);
