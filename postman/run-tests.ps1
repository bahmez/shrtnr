# Script PowerShell pour exécuter les tests Postman avec Newman
# Nécessite Newman: npm install -g newman

Write-Host "🧪 Exécution des tests Postman pour Shrtnr API..." -ForegroundColor Cyan

# Vérifier si Newman est installé
$newmanInstalled = Get-Command newman -ErrorAction SilentlyContinue
if (-not $newmanInstalled) {
    Write-Host "❌ Newman n'est pas installé" -ForegroundColor Red
    Write-Host "📦 Installation: npm install -g newman" -ForegroundColor Yellow
    exit 1
}

# Vérifier si le serveur est en cours d'exécution
Write-Host "🔍 Vérification du serveur..." -ForegroundColor Yellow
try {
    $response = Invoke-WebRequest -Uri "http://localhost:3000" -TimeoutSec 2 -UseBasicParsing -ErrorAction Stop
    Write-Host "✅ Serveur détecté" -ForegroundColor Green
} catch {
    Write-Host "⚠️  Le serveur ne semble pas être en cours d'exécution" -ForegroundColor Yellow
    Write-Host "🚀 Démarrez le serveur avec: cargo leptos watch" -ForegroundColor Cyan
    Write-Host ""
    $continue = Read-Host "Voulez-vous continuer quand même ? (y/n)"
    if ($continue -ne "y" -and $continue -ne "Y") {
        exit 1
    }
}

# Définir le dossier de sortie
$OUTPUT_DIR = "postman\test-results"
if (-not (Test-Path $OUTPUT_DIR)) {
    New-Item -ItemType Directory -Path $OUTPUT_DIR -Force | Out-Null
}

# Date pour nommer les fichiers
$TIMESTAMP = Get-Date -Format "yyyyMMdd_HHmmss"

Write-Host ""
Write-Host "🏃 Lancement de Newman..." -ForegroundColor Cyan
Write-Host ""

# Exécuter Newman avec la collection
$htmlReport = "$OUTPUT_DIR\report_${TIMESTAMP}.html"
$jsonReport = "$OUTPUT_DIR\report_${TIMESTAMP}.json"

newman run postman\auth.collection.json `
    --environment postman\environments\development.postman_environment.json `
    --reporters cli,html,json `
    --reporter-html-export $htmlReport `
    --reporter-json-export $jsonReport `
    --color on `
    --delay-request 100 `
    --timeout-request 5000

# Vérifier le code de sortie
$EXIT_CODE = $LASTEXITCODE

Write-Host ""
if ($EXIT_CODE -eq 0) {
    Write-Host "✅ Tous les tests sont passés!" -ForegroundColor Green
    Write-Host "📊 Rapport HTML: $htmlReport" -ForegroundColor Cyan
} else {
    Write-Host "❌ Certains tests ont échoué (code: $EXIT_CODE)" -ForegroundColor Red
    Write-Host "📊 Consultez le rapport pour plus de détails: $htmlReport" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "📁 Fichiers générés:" -ForegroundColor Cyan
Get-ChildItem -Path $OUTPUT_DIR | Where-Object { $_.Name -like "*${TIMESTAMP}*" } | Format-Table Name, Length, LastWriteTime -AutoSize

# Ouvrir le rapport HTML dans le navigateur par défaut
if ($EXIT_CODE -eq 0) {
    $open = Read-Host "Voulez-vous ouvrir le rapport HTML ? (y/n)"
    if ($open -eq "y" -or $open -eq "Y") {
        Start-Process $htmlReport
    }
}

exit $EXIT_CODE

