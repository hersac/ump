# Script de desinstalación de UMP (Umbral Package Manager) para Windows
# Ejecutar con: PowerShell -ExecutionPolicy Bypass -File uninstall.ps1

Write-Host "╔════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║   Desinstalador de UMP                 ║" -ForegroundColor Cyan
Write-Host "╚════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

# Desinstalar
Write-Host "🗑️  Desinstalando UMP..." -ForegroundColor Yellow

cargo uninstall ump 2>$null
if ($LASTEXITCODE -ne 0) {
    Write-Host "  ump no estaba instalado" -ForegroundColor Gray
}

Write-Host ""
Write-Host "✓ UMP desinstalado correctamente" -ForegroundColor Green
Write-Host ""
