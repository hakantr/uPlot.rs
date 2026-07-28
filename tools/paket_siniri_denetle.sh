#!/usr/bin/env bash
# Kütüphane paketi ile görselleştirme uygulamalarının sınırını doğrular.
#
# Kart listesi (katalog), örnek kartlar ve masaüstü/web kabukları kütüphaneyi
# göstermek içindir; `uplot-rs` paketinin parçası değildir. Bu ayrım bugün
# üç ayrı mekanizmayla duruyor ve üçü de sessizce bozulabilir:
#   1. `uplot-rs/Cargo.toml` içindeki `include` yalnız `/src/**` alıyor,
#   2. uygulama crate'leri `publish = false`,
#   3. `uplot-rs` hiçbir uygulama crate'ine bağımlı değil.
# Betik üçünü de sınar; native ve wasm için ayrım aynıdır çünkü sınır
# hedeften değil paket içeriğinden ve bağımlılık yönünden gelir.
set -euo pipefail

kok="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$kok"

hata=0
bildir() {
    printf '  ✗ %s\n' "$1" >&2
    hata=1
}

echo "1) Paket içeriğinde uygulama dosyası var mı"
paket_listesi="$(cargo package -p uplot-rs --list --allow-dirty)"
if sizinti="$(printf '%s\n' "$paket_listesi" | grep '^uygulamalar/' || true)"; [ -n "$sizinti" ]; then
    bildir "uplot-rs paketine uygulama dosyaları girmiş:"
    printf '%s\n' "$sizinti" | sed 's/^/      /' >&2
else
    echo "  ✓ paket yalnız kütüphane dosyalarını taşıyor"
fi

echo "2) Uygulama crate'leri yayınlanabilir mi"
for manifest in uygulamalar/*/Cargo.toml; do
    if grep -q '^publish = false' "$manifest"; then
        echo "  ✓ $manifest yayına kapalı"
    else
        bildir "$manifest içinde 'publish = false' yok"
    fi
done

echo "3) Kütüphane bir uygulama crate'ine bağımlı mı"
bagimliliklar="$(cargo tree -p uplot-rs --edges normal,build,dev --prefix none 2>/dev/null || true)"
if uygulama="$(printf '%s\n' "$bagimliliklar" | grep -E '^uplot-rs-(gpui-katalog|gpui-ornekler|chart-listesi|web)' || true)"; [ -n "$uygulama" ]; then
    bildir "uplot-rs bağımlılık ağacında uygulama crate'i var:"
    printf '%s\n' "$uygulama" | sed 's/^/      /' >&2
else
    echo "  ✓ bağımlılık ağacında uygulama crate'i yok"
fi

if [ "$hata" -ne 0 ]; then
    echo "Paket sınırı ihlal edildi." >&2
    exit 1
fi
echo "Paket sınırı korunuyor."
