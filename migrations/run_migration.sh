#!/bin/bash

# Script pour exécuter les migrations SQL
# Pour SQLite:
# ./run_migration.sh

# Charger les variables d'environnement depuis .env si présent
if [ -f .env ]; then
    export $(cat .env | grep -v '#' | xargs)
fi

# Utiliser DATABASE_URL ou valeur par défaut
DB_URL=${DATABASE_URL:-"sqlite:shrtnr.db?mode=rwc"}

# Extraire le type de base de données
if [[ $DB_URL == sqlite:* ]]; then
    DB_FILE=${DB_URL#sqlite:}
    DB_FILE=${DB_FILE%\?*}
    echo "Exécution de la migration pour SQLite: $DB_FILE"
    sqlite3 "$DB_FILE" < migrations/001_create_tables.sql
    echo "Migration terminée!"
elif [[ $DB_URL == postgresql:* ]]; then
    echo "Exécution de la migration pour PostgreSQL"
    psql "$DB_URL" -f migrations/001_create_tables.sql
    echo "Migration terminée!"
else
    echo "Type de base de données non supporté: $DB_URL"
    exit 1
fi

