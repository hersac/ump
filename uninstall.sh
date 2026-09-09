#!/bin/bash

# Script de desinstalación de UMP (Umbral Package Manager)

set -e

echo "╔════════════════════════════════════════╗"
echo "║   Desinstalador de UMP                 ║"
echo "╚════════════════════════════════════════╝"
echo ""

# Desinstalar
echo "🗑️  Desinstalando UMP..."
cargo uninstall ump 2>/dev/null || echo "ump no estaba instalado"

echo ""
echo "✓ UMP desinstalado correctamente"
echo ""
