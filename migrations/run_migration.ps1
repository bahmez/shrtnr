# Script PowerShell pour exécuter les migrations SQL
# Usage: .\migrations\run_migration.ps1

# Charger les variables d'environnement depuis .env si présent
if (Test-Path .env) {
    Get-Content .env | ForEach-Object {
        if ($_ -match '^([^#][^=]+)=(.*)$') {
            [Environment]::SetEnvironmentVariable($matches[1], $matches[2], "Process")
        }
    }
}

# Utiliser DATABASE_URL ou valeur par défaut
$DATABASE_URL = if ($env:DATABASE_URL) { $env:DATABASE_URL } else { "sqlite:shrtnr.db?mode=rwc" }

Write-Host "DATABASE_URL: $DATABASE_URL"

# Extraire le type de base de données
if ($DATABASE_URL -match '^sqlite:(.+?)(\?.*)?$') {
    $dbFile = $matches[1]
    Write-Host "Exécution de la migration pour SQLite: $dbFile"
    
    # Créer le fichier de base de données s'il n'existe pas
    if (!(Test-Path $dbFile)) {
        New-Item -Path $dbFile -ItemType File -Force | Out-Null
    }
    
    # Exécuter la migration
    Get-Content migrations\001_create_tables.sql | sqlite3 $dbFile
    Write-Host "Migration terminée!"
}
elseif ($DATABASE_URL -match '^postgresql:') {
    Write-Host "Exécution de la migration pour PostgreSQL"
    Get-Content migrations\001_create_tables.sql | psql $DATABASE_URL
    Write-Host "Migration terminée!"
}
else {
    Write-Host "Type de base de données non supporté: $DATABASE_URL"
    exit 1
}

