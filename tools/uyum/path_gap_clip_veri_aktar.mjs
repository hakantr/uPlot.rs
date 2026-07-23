import { readFileSync, writeFileSync } from "node:fs";
import vm from "node:vm";

const kaynakYolu = new URL("../../../uPlot/demos/path-gap-clip.html", import.meta.url);
const hedefYolu = new URL("../../src/kart/veri/path_gap_clip.json", import.meta.url);
const kaynak = readFileSync(kaynakYolu, "utf8");

function blok(değişken, bitişİşareti) {
  const başlangıç = kaynak.indexOf(değişken);
  const bitiş = kaynak.indexOf(bitişİşareti, başlangıç);
  if (başlangıç < 0 || bitiş < 0) {
    throw new Error(`${değişken} kaynak veri bloğu bulunamadı`);
  }
  return kaynak.slice(başlangıç, bitiş);
}

const bağlam = {};
vm.createContext(bağlam);
vm.runInContext(
  blok("const data0 =", "// add gap").replace("const data0 =", "this.data0 ="),
  bağlam,
);
vm.runInContext(
  blok("let data3 =", "//\tdata3[0]").replace("let data3 =", "this.data3 ="),
  bağlam,
);

const çıktı = {
  source: "demos/path-gap-clip.html",
  source_commit: "0e5812c504430f5c804e0f993376d8999b26cc34",
  data0: bağlam.data0,
  data3: bağlam.data3,
};

writeFileSync(hedefYolu, `${JSON.stringify(çıktı)}\n`);
