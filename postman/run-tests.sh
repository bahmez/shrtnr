#!/bin/bash

# Script pour exécuter les tests Postman avec Newman
# Nécessite Newman: npm install -g newman

echo "🧪 Exécution des tests Postman pour Shrtnr API..."

# Vérifier si Newman est installé
if ! command -v newman &> /dev/null
then
    echo "❌ Newman n'est pas installé"
    echo "📦 Installation: npm install -g newman"
    exit 1
fi

# Vérifier si le serveur est en cours d'exécution
echo "🔍 Vérification du serveur..."
if ! curl -s http://localhost:3000 > /dev/null 2>&1; then
    echo "⚠️  Le serveur ne semble pas être en cours d'exécution"
    echo "🚀 Démarrez le serveur avec: cargo leptos watch"
    echo ""
    read -p "Voulez-vous continuer quand même ? (y/n) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]
    then
        exit 1
    fi
fi

# Définir le dossier de sortie
OUTPUT_DIR="postman/test-results"
mkdir -p "$OUTPUT_DIR"

# Date pour nommer les fichiers
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")

echo ""
echo "🏃 Lancement de Newman..."
echo ""

# 1) Auth: export d'environnement pour chaîner les suites
AUTH_HTML="$OUTPUT_DIR/auth_${TIMESTAMP}.html"
AUTH_JSON="$OUTPUT_DIR/auth_${TIMESTAMP}.json"
RUNTIME_ENV="$OUTPUT_DIR/dev.runtime.${TIMESTAMP}.json"

newman run postman/auth.collection.json \
  --environment postman/environments/development.postman_environment.json \
  --export-environment "$RUNTIME_ENV" \
  --reporters cli,html,json \
  --reporter-html-export "$AUTH_HTML" \
  --reporter-json-export "$AUTH_JSON" \
  --color on \
  --delay-request 100 \
  --timeout-request 5000

AUTH_CODE=$?

# 2) Workspaces (réutilise l'env runtime avec tokens/ids)
WS_HTML="$OUTPUT_DIR/workspaces_${TIMESTAMP}.html"
WS_JSON="$OUTPUT_DIR/workspaces_${TIMESTAMP}.json"

newman run postman/workspaces.collection.json \
  --environment "$RUNTIME_ENV" \
  --export-environment "$RUNTIME_ENV" \
  --reporters cli,html,json \
  --reporter-html-export "$WS_HTML" \
  --reporter-json-export "$WS_JSON" \
  --color on \
  --delay-request 100 \
  --timeout-request 5000

WS_CODE=$?

# 3) Stats
STATS_HTML="$OUTPUT_DIR/stats_${TIMESTAMP}.html"
STATS_JSON="$OUTPUT_DIR/stats_${TIMESTAMP}.json"

newman run postman/stats.collection.json \
  --environment "$RUNTIME_ENV" \
  --reporters cli,html,json \
  --reporter-html-export "$STATS_HTML" \
  --reporter-json-export "$STATS_JSON" \
  --color on \
  --delay-request 100 \
  --timeout-request 5000

STATS_CODE=$?

echo ""
echo "📦 Résumés rapports:"
echo "- Auth: $AUTH_HTML (code=$AUTH_CODE)"
echo "- Workspaces: $WS_HTML (code=$WS_CODE)"
echo "- Stats: $STATS_HTML (code=$STATS_CODE)"

EXIT_CODE=$(( AUTH_CODE | WS_CODE | STATS_CODE ))

echo ""
if [ $EXIT_CODE -eq 0 ]; then
  echo "✅ Tous les tests sont passés!"
else
  echo "❌ Certains tests ont échoué (code combiné: $EXIT_CODE)"
fi

echo ""
echo "📁 Fichiers générés:"
ls -lh "$OUTPUT_DIR" | tail -n +2 | grep "$TIMESTAMP"

exit $EXIT_CODE

