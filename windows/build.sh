#!/bin/bash
set -e

# Script para generar el instalador de Windows (setup.exe) de UMP.
# UMP - Umbral Package Manager
# Requiere:
#   - Rust configurado para el target x86_64-pc-windows-gnu
#     (rustup target add x86_64-pc-windows-gnu)
#   - makensis  (https://nsis.sourceforge.net/) en el PATH
#
# Uso:
#   ./windows/build.sh
#
# Salida:
#   windows/ump-setup-<version>.exe

echo "╔══════════════════════════════════════════════╗"
echo "║  Build del instalador de Windows de UMP      ║"
echo "╚══════════════════════════════════════════════╝"
echo ""

TARGET="x86_64-pc-windows-gnu"

# 1. Verificar herramientas
if ! command -v cargo &> /dev/null; then
    echo "❌ Error: cargo no está instalado"
    exit 1
fi
if ! command -v makensis &> /dev/null; then
    echo "❌ Error: makensis no está instalado"
    echo "   Instálalo desde https://nsis.sourceforge.net/ o con:"
    echo "   sudo apt install nsis"
    exit 1
fi

# 2. Asegurar el target de Windows
echo "🎯 Asegurando target ${TARGET}..."
rustup target add ${TARGET}

# 3. Compilar en modo release para Windows
echo "📦 Compilando UMP (target: ${TARGET})..."
cargo build --release --target ${TARGET}

# Copiar el .exe a target/release para que el script NSIS lo encuentre
CROSS_BIN="target/${TARGET}/release"
mkdir -p target/release
cp "${CROSS_BIN}/ump.exe" target/release/ump.exe

# 4. Generar el instalador
VERSION=$(grep -m1 '^version' Cargo.toml | sed 's/.*= *"\(.*\)".*/\1/')
# VIProductVersion solo acepta X.X.X.X numérico (ej: 1.1.1.0).
BASE="${VERSION%%-*}"
BASE="${BASE%%+*}"
IFS='.' read -ra PARTS <<< "$BASE"
CLEAN=()
for p in "${PARTS[@]}"; do
  n="$(echo "$p" | tr -cd '0-9')"
  [ -z "$n" ] && n="0"
  CLEAN+=("$n")
done
while [ "${#CLEAN[@]}" -lt 4 ]; do CLEAN+=(0); done
VI_VERSION="$(IFS=.; echo "${CLEAN[*]:0:4}")"
echo "🚀 Generando instalador v${VERSION} (VI: ${VI_VERSION})..."
# -INPUTCHARSET UTF8: el .nsi está en UTF-8 con acentos en comentarios.
makensis -INPUTCHARSET UTF8 -DVERSION="${VERSION}" -DVI_VERSION="${VI_VERSION}" windows/ump.nsi

echo ""
echo "✅ Instalador generado: windows/ump-setup-${VERSION}.exe"
