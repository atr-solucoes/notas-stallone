#!/bin/bash

set -e

BOOKS=("financeiro" "sinir" "administrativo")
GERENCIAL_DIR="books/gerencial"
GERENCIAL_SRC="$GERENCIAL_DIR/src"

echo "🧹 Limpando módulos e builds antigos..."
for AREA in "${BOOKS[@]}"; do
    rm -rf "$GERENCIAL_SRC/$AREA"
    rm -rf "books/$AREA/book" # Limpa build anterior da área
done
rm -rf "$GERENCIAL_DIR/book" # Limpa build anterior do gerencial

# Validar e Resetar Sumário Gerencial
if [ ! -f "$GERENCIAL_SRC/index.md" ]; then
    echo "⚠️ Erro: index.md não encontrado em $GERENCIAL_SRC"
    exit 1
fi

echo "📝 Resetando Sumário Gerencial..."
cat <<EOF > "$GERENCIAL_SRC/SUMMARY.md"
# Sumário Gerencial

- [🌐 Panorama Geral](index.md)

EOF

# Processar Áreas
for AREA in "${BOOKS[@]}"; do
    SRC_PATH="books/$AREA/src"
    
    if [ -d "$SRC_PATH" ] && [ -f "$SRC_PATH/SUMMARY.md" ]; then
        echo "📦 Acoplando: $AREA"
        
        # Cria subpasta no gerencial e copia conteúdo
        mkdir -p "$GERENCIAL_SRC/$AREA"
        cp -r "$SRC_PATH/"* "$GERENCIAL_SRC/$AREA/"
        
        # Injeta título da área no sumário mestre
        echo -e "\n# ${AREA^^}" >> "$GERENCIAL_SRC/SUMMARY.md"
        
        # Ajusta os links do SUMMARY.md da área para o contexto do Gerencial
        sed -E "s|\((.*\.md)\)|($AREA/\1)|g" "$SRC_PATH/SUMMARY.md" | grep "\[" >> "$GERENCIAL_SRC/SUMMARY.md"
        
        # Build individual da área
        echo "🛠️  Build individual: $AREA"
        mdbook build "books/$AREA"
    else
        echo "⏭️  Aviso: Pasta $AREA ignorada (vazia ou sem SUMMARY.md)"
    fi
done

# Build Final
echo "🏗️  Build final: Gerencial"
if [ -f "$GERENCIAL_DIR/book.toml" ]; then
    mdbook build "$GERENCIAL_DIR"
    echo "✅ Processo concluído com sucesso!"
else
    echo "❌ Erro: book.toml do Gerencial não encontrado!"
    exit 1
fi
