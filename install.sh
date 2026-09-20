#!/bin/bash

# Script de instalación de UMP (Umbral Package Manager)
# Este script instala UMP globalmente en el sistema

set -e

echo "╔════════════════════════════════════════╗"
echo "║   Instalador de UMP v1.1.0             ║"
echo "║   Umbral Package Manager               ║"
echo "╚════════════════════════════════════════╝"
echo ""

# Verificar que Rust esté instalado
if ! command -v cargo &> /dev/null; then
    echo "❌ Error: Cargo no está instalado"
    echo "Por favor instala Rust desde: https://rustup.rs/"
    exit 1
fi

echo "✓ Rust encontrado: $(rustc --version)"
echo ""

# Compilar en modo release
echo "📦 Compilando UMP (esto puede tomar unos minutos)..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "❌ Error al compilar"
    exit 1
fi

echo "✓ Compilación exitosa"
echo ""

# Instalar globalmente
echo "🚀 Instalando UMP globalmente..."
cargo install --path . --force

if [ $? -ne 0 ]; then
    echo "❌ Error al instalar"
    exit 1
fi

echo ""
echo "╔════════════════════════════════════════╗"
echo "║   ✓ UMP instalado correctamente        ║"
echo "╚════════════════════════════════════════╝"
echo ""

# Configurar PATH automáticamente
echo "🔧 Configurando PATH en ~/.bashrc..."

# Verificar si ~/.bashrc existe, si no, crearlo
if [ ! -f "$HOME/.bashrc" ]; then
    echo "📝 Creando ~/.bashrc..."
    touch "$HOME/.bashrc"
fi

# Verificar si la ruta de cargo ya está en .bashrc
if ! grep -q 'export PATH="$HOME/.cargo/bin:$PATH"' "$HOME/.bashrc"; then
    echo "" >> "$HOME/.bashrc"
    echo "# Agregado por el instalador de UMP" >> "$HOME/.bashrc"
    echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> "$HOME/.bashrc"
    echo "✓ PATH agregado a ~/.bashrc"
else
    echo "✓ PATH ya configurado en ~/.bashrc"
fi

# Aplicar cambios en la sesión actual
export PATH="$HOME/.cargo/bin:$PATH"
echo "✓ PATH actualizado en la sesión actual"

echo ""
echo "Comandos disponibles:"
echo "  ump init               - Inicializar proyecto existente"
echo "  ump create <nombre>    - Crear un proyecto nuevo"
echo "  ump add <paquetes...>  - Agregar dependencias"
echo "  ump remove <paquetes...> - Remover dependencias"
echo "  ump run <script>       - Ejecutar un script de umpkg.yml"
echo ""
echo "Ejemplo:"
echo "  ump create mi-super-app"
echo ""
echo "Nota: Para nuevas terminales, el PATH se cargará automáticamente."
echo "Para la terminal actual, ejecuta: source ~/.bashrc"
echo ""
echo "¡Disfruta gestionando paquetes de Umbral! 🎉"
