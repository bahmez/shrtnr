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

# 1) Auth avec export d'environnement pour chaîner
$authHtml = "$OUTPUT_DIR\auth_${TIMESTAMP}.html"
$authJson = "$OUTPUT_DIR\auth_${TIMESTAMP}.json"
$runtimeEnv = "$OUTPUT_DIR\dev.runtime.${TIMESTAMP}.json"

newman run postman\auth.collection.json `
  --environment postman\environments\development.postman_environment.json `
  --export-environment $runtimeEnv `
  --reporters cli,html,json `
  --reporter-html-export $authHtml `
  --reporter-json-export $authJson `
  --color on `
  --delay-request 100 `
  --timeout-request 5000

$authCode = $LASTEXITCODE

# 2) Workspaces
$wsHtml = "$OUTPUT_DIR\workspaces_${TIMESTAMP}.html"
$wsJson = "$OUTPUT_DIR\workspaces_${TIMESTAMP}.json"

newman run postman\workspaces.collection.json `
  --environment $runtimeEnv `
  --reporters cli,html,json `
  --reporter-html-export $wsHtml `
  --reporter-json-export $wsJson `
  --color on `
  --delay-request 100 `
  --timeout-request 5000

$wsCode = $LASTEXITCODE

# 3) Stats
$statsHtml = "$OUTPUT_DIR\stats_${TIMESTAMP}.html"
$statsJson = "$OUTPUT_DIR\stats_${TIMESTAMP}.json"

newman run postman\stats.collection.json `
  --environment $runtimeEnv `
  --reporters cli,html,json `
  --reporter-html-export $statsHtml `
  --reporter-json-export $statsJson `
  --color on `
  --delay-request 100 `
  --timeout-request 5000

$statsCode = $LASTEXITCODE

$exitCode = ($authCode -bor $wsCode -bor $statsCode)

Write-Host ""; Write-Host "📦 Résumés rapports:" -ForegroundColor Cyan
Write-Host ("- Auth: {0} (code={1})" -f $authHtml, $authCode)
Write-Host ("- Workspaces: {0} (code={1})" -f $wsHtml, $wsCode)
Write-Host ("- Stats: {0} (code={1})" -f $statsHtml, $statsCode)

Write-Host ""; Write-Host "📁 Fichiers générés:" -ForegroundColor Cyan
Get-ChildItem -Path $OUTPUT_DIR | Where-Object { $_.Name -like "*${TIMESTAMP}*" } | Format-Table Name, Length, LastWriteTime -AutoSize

exit $exitCode

