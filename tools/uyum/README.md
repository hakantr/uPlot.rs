# Uyum araçları

`npm run denetle`, kardeş `../uPlot` deposunun kilitli commit ve sürümde
olduğunu; kart kaynak hash'lerini, manifest yollarını ve API matrisi kaynak
hash'ini doğrular. Harici npm bağımlılığı yoktur.

Kaynak veri varlıkları mekanik ve yeniden üretilebilir biçimde aktarılır:

- `node tools/uyum/path_gap_clip_veri_aktar.mjs`
- `node tools/uyum/points_veri_aktar.mjs`
- `node tools/uyum/custom_scales_veri_aktar.mjs`
- `node tools/uyum/data_smoothing_veri_aktar.mjs`
