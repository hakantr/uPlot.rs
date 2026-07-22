# Ortak kart port davranışları

Bu belge, `Resize` portu görsel ve etkileşimli olarak doğrulanırken belirlenen
ortak davranışları listeler. Normatif ve makine-okunur sözleşme
[`uyum/ortak_davranis_sozlesmesi.json`](uyum/ortak_davranis_sozlesmesi.json)
dosyasındadır.

Her yeni kart `uyum/demo_manifesti.json` içinde sözleşme sürümünü ve **bütün**
davranışlar için kararını kaydetmek zorundadır. Karar değerleri:

- `devralındı`: ortak çekirdek/yüzey davranışı kartta kullanılır;
- `kartta_etkin`: isteğe bağlı davranış kart tanımında açıkça açılmıştır;
- `kartta_kapalı`: yetenek çekirdekte vardır fakat kaynak kart için kapalıdır;
- `uygulanamaz`: grafik türüne uygulanamaz.

Son iki karar açıklamalı bir gerekçe gerektirir. `npm --prefix tools/uyum run
denetle`, eksik veya bilinmeyen davranışı, sözleşme sürümü uyuşmazlığını,
gerekçesiz istisnayı, kayıp kanıt dosyasını ve kart kaynağında
`ortak_kart_etkileşimleri` profilinin uygulanmamasını hata sayar. Bu komut
CI'da her değişiklikte çalıştığı için sözleşme kaydı ve gerçek profil çağrısı
yapılmadan yeni kart eklenemez.

## Davranış listesi

1. Resmî demonun sabit verisi, üretici algoritması, seri sırası ve kaynakta sabitse boyutu korunur.
2. Veri, ölçek, yakınlaştırma, taşıma ve diğer kart davranışları çekirdekte çözülür; kataloglar bunları yeniden yazmaz.
3. GPUI ve WASM aynı Rust kart tanımını ve aynı sahne semantiğini kullanır.
4. Grafik yüzey boyutuna duyarlı yeniden çizilir.
5. Seri yolları ve dolguları çizim dikdörtgeninin dört sınırında kırpılır.
6. Eksen ve ızgara yakınlaştırma/taşıma sonrasında görünür aralıktan yeniden hesaplanır; sıfır hizası korunur.
7. Kesikli cursor çizgileri veri noktaları arasında atlamaz, gerçek fare konumunu kesintisiz izler.
8. Lejant ortak X konumundaki bütün seri değerlerini canlı gösterir.
9. Kaynak nokta gösteriyorsa normal noktalar boş, hover noktası seri rengiyle dolu çizilir.
10. Sürükleyerek X seçimi/yakınlaştırma kart seçeneğiyle yönetilir.
11. Çift tıklamayla tam veri görünümüne dönüş kart seçeneğiyle yönetilir.
12. Resmî wheel eklentisi açıldığında fare odaklı X/Y yakınlaştırma ve veri sınırı korunur.
13. Magic Mouse/trackpad piksel akışı ile klasik mouse tekerleği otomatik ve ayrı normalize edilir.
14. Resmî touch eklentisi açıldığında iki parmakla yakınlaştırma ve tek parmakla taşıma çalışır.
15. Yakın görünümde boşluk + sol sürükleme X/Y taşıma sağlar; boşluğa grafiğe
    girmeden önce veya sonra basılması davranışı değiştirmez ve ayrıca açılması gerekmez.
16. İstenirse hareket bazlı görünüm geçmişi ve geri alma sağlanır.
17. Kataloglar görünür `Geri` ve `Tam görünüm` kontrolleri sunar.
18. Bilgi satırı başlık altında/çizim üstünde, Rust tanımı kart altında ve varsayılan kapalıdır.
19. Geçersiz durumlar panic yerine tipli `UplotHatası` ile bildirilir.

## Yeni kart kabul kapısı

Bir kart ancak kaynak/veri hash'i, sözleşmedeki 19 karar, sayısal test, GPUI ve
WASM yüzey kaydı, statik görünüm kanıtı ve varsa etkileşim kanıtı tamamlandığında
`uygulandı_kanıtlı` durumuna getirilebilir. Ortak davranış için yeni kod
gerekiyorsa önce çekirdeğe eklenir; kart veya katalog içinde özel bir kopya
oluşturulmaz.
