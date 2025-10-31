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

# Exécuter Newman avec la collection
newman run postman/auth.collection.json \
    --environment postman/environments/development.postman_environment.json \
    --reporters cli,html,json \
    --reporter-html-export "$OUTPUT_DIR/report_${TIMESTAMP}.html" \
    --reporter-json-export "$OUTPUT_DIR/report_${TIMESTAMP}.json" \
    --color on \
    --delay-request 100 \
    --timeout-request 5000

# Vérifier le code de sortie
EXIT_CODE=$?

echo ""
if [ $EXIT_CODE -eq 0 ]; then
    echo "✅ Tous les tests sont passés!"
    echo "📊 Rapport HTML: $OUTPUT_DIR/report_${TIMESTAMP}.html"
else
    echo "❌ Certains tests ont échoué (code: $EXIT_CODE)"
    echo "📊 Consultez le rapport pour plus de détails: $OUTPUT_DIR/report_${TIMESTAMP}.html"
fi

echo ""
echo "📁 Fichiers générés:"
ls -lh "$OUTPUT_DIR" | tail -n +2 | grep "$TIMESTAMP"

exit $EXIT_CODE

