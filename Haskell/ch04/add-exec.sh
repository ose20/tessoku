#!/bin/bash

# package.yaml ファイルへの記述を追加する関数
add_entry() {
    local file="$1"
    local base_name="${file%.*}"
    local entry="\t$base_name:\n\t\tmain:\t\t$file"
    
    echo -e "$entry" >> test.yaml
}

# package.yaml ファイルの初期化
echo "executables:" > test.yaml

# 現在のディレクトリ内の Haskell ファイルに対して処理を実行
for file in *.hs; do
    add_entry "$file"
done

