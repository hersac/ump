# Script de instalación de UMP (Umbral Package Manager) para Windows
# Ejecutar con: PowerShell -ExecutionPolicy Bypass -File install.ps1

Write-Host "╔════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║   Instalador de UMP v1.0.3             ║" -ForegroundColor Cyan
Write-Host "║   Umbral Package Manager               ║" -ForegroundColor Cyan
Write-Host "╚════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

# Verificar que Rust esté instalado
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Error: Cargo no está instalado" -ForegroundColor Red
    Write-Host "Por favor instala Rust desde: https://rustup.rs/" -ForegroundColor Yellow
    exit 1
}

$rustVersion = cargo --version
Write-Host "✓ Rust encontrado: $rustVersion" -ForegroundColor Green
Write-Host ""

# Compilar en modo release
Write-Host "📦 Compilando UMP (esto puede tomar unos minutos)..." -ForegroundColor Yellow
cargo build --release

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Error al compilar" -ForegroundColor Red
    exit 1
}

Write-Host "✓ Compilación exitosa" -ForegroundColor Green
Write-Host ""

# Instalar globalmente
Write-Host "🚀 Instalando UMP globalmente..." -ForegroundColor Yellow
cargo install --path . --force

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Error al instalar" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "╔════════════════════════════════════════╗" -ForegroundColor Green
Write-Host "║   ✓ UMP instalado correctamente        ║" -ForegroundColor Green
Write-Host "╚════════════════════════════════════════╝" -ForegroundColor Green
Write-Host ""

# Configurar PATH automáticamente
Write-Host "🔧 Configurando PATH en las variables de entorno..." -ForegroundColor Yellow

$cargoPath = "$env:USERPROFILE\.cargo\bin"
$currentPath = [Environment]::GetEnvironmentVariable("Path", "User")

if ($currentPath -notlike "*$cargoPath*") {
    # Agregar al PATH del usuario de forma permanente
    $newPath = "$currentPath;$cargoPath"
    [Environment]::SetEnvironmentVariable("Path", $newPath, "User")

    Write-Host "✓ PATH agregado a las variables de entorno del usuario" -ForegroundColor Green

    # Actualizar PATH en la sesión actual
    $env:Path = "$env:Path;$cargoPath"
    Write-Host "✓ PATH actualizado en la sesión actual" -ForegroundColor Green
} else {
    Write-Host "✓ PATH ya está configurado en las variables de entorno" -ForegroundColor Green
}

Write-Host ""
Write-Host "Comandos disponibles:" -ForegroundColor Cyan
Write-Host "  ump init               - Inicializar proyecto existente" -ForegroundColor White
Write-Host "  ump create <nombre>    - Crear un proyecto nuevo" -ForegroundColor White
Write-Host "  ump add <paquetes...>  - Agregar dependencias" -ForegroundColor White
Write-Host "  ump remove <paquetes...> - Remover dependencias" -ForegroundColor White
Write-Host "  ump run <script>       - Ejecutar un script de umpkg.yml" -ForegroundColor White
Write-Host ""
Write-Host "Ejemplo:" -ForegroundColor Cyan
Write-Host "  ump create mi-super-app" -ForegroundColor White
Write-Host ""
Write-Host "Nota: El PATH está configurado para todas las nuevas ventanas de PowerShell/CMD." -ForegroundColor Yellow
Write-Host "      Para la ventana actual, los comandos ya están disponibles." -ForegroundColor Yellow
Write-Host ""
Write-Host "¡Disfruta gestionando paquetes de Umbral! 🎉" -ForegroundColor Cyan
Write-Host ""
