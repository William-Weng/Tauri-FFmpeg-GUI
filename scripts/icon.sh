#!/bin/bash

# 設定源圖片路徑（請將您的原始圖片放在這裡，建議使用 1024x1024 的 PNG 檔案）
SOURCE_IMAGE="$1"

# 檢查是否提供了源圖片參數
# -z 測試字串長度是否為 0（空字串）
if [ -z "$SOURCE_IMAGE" ]; then
    echo "使用方式: $0 <source_image.png>"
    echo "請提供一個高解析度的 PNG 圖片（建議 1024x1024）"
    exit 1
fi

# 檢查源圖片文件是否存在
# -f 測試檔案是否存在且為一般檔案（不是目錄或裝置檔案）
if [ ! -f "$SOURCE_IMAGE" ]; then
    echo "錯誤: 找不到源圖片 $SOURCE_IMAGE"
    exit 1
fi

# -d：測試是否為目錄
# -r：測試檔案是否可讀
# -w：測試檔案是否可寫
# -x：測試檔案是否可執行
# -s：測試檔案是否不為空（大小大於 0）

# 獲取源圖片的完整路徑和目錄
SOURCE_IMAGE_FULL_PATH=$(realpath "$SOURCE_IMAGE")
SOURCE_DIR=$(dirname "$SOURCE_IMAGE_FULL_PATH")
ICONS_DIR="$SOURCE_DIR/icons"

echo "源圖片: $SOURCE_IMAGE_FULL_PATH"
echo "輸出目錄: $ICONS_DIR"

# 檢查 ImageMagick 是否安裝
if ! command -v magick &> /dev/null; then
    echo "錯誤: 找不到 ImageMagick。請使用 brew install imagemagick 安裝"
    exit 1
fi

# 確保輸出目錄存在
mkdir -p "$ICONS_DIR"

# 生成 Windows/Linux PNG 圖示
magick "$SOURCE_IMAGE" -resize 32x32 "$ICONS_DIR/32x32.png"
magick "$SOURCE_IMAGE" -resize 128x128 "$ICONS_DIR/128x128.png"
magick "$SOURCE_IMAGE" -resize 256x256 "$ICONS_DIR/128x128@2x.png"

# 生成 Windows ICO
magick "$SOURCE_IMAGE" -define icon:auto-resize=256,128,96,64,48,32,16 "$ICONS_DIR/icon.ico"

# 生成 macOS ICNS
mkdir -p "$ICONS_DIR/iconset.iconset"
magick "$SOURCE_IMAGE" -resize 16x16 "$ICONS_DIR/iconset.iconset/icon_16x16.png"
magick "$SOURCE_IMAGE" -resize 32x32 "$ICONS_DIR/iconset.iconset/icon_16x16@2x.png"
magick "$SOURCE_IMAGE" -resize 32x32 "$ICONS_DIR/iconset.iconset/icon_32x32.png"
magick "$SOURCE_IMAGE" -resize 64x64 "$ICONS_DIR/iconset.iconset/icon_32x32@2x.png"
magick "$SOURCE_IMAGE" -resize 128x128 "$ICONS_DIR/iconset.iconset/icon_128x128.png"
magick "$SOURCE_IMAGE" -resize 256x256 "$ICONS_DIR/iconset.iconset/icon_128x128@2x.png"
magick "$SOURCE_IMAGE" -resize 256x256 "$ICONS_DIR/iconset.iconset/icon_256x256.png"
magick "$SOURCE_IMAGE" -resize 512x512 "$ICONS_DIR/iconset.iconset/icon_256x256@2x.png"
magick "$SOURCE_IMAGE" -resize 512x512 "$ICONS_DIR/iconset.iconset/icon_512x512.png"
magick "$SOURCE_IMAGE" -resize 1024x1024 "$ICONS_DIR/iconset.iconset/icon_512x512@2x.png"

# 使用 iconutil 生成 .icns 文件 (僅在 macOS 上可用)
if [ "$(uname)" == "Darwin" ]; then
    if ! command -v iconutil &> /dev/null; then
        echo "警告: 找不到 iconutil 命令，無法生成 .icns 文件"
    else
        iconutil -c icns "$ICONS_DIR/iconset.iconset" -o "$ICONS_DIR/icon.icns"
        rm -rf "$ICONS_DIR/iconset.iconset"
    fi
else
    echo "警告: 無法在非 macOS 系統上生成 .icns 文件"
fi

echo "圖示生成完成！"
echo "請檢查 $ICONS_DIR 目錄中的圖示文件"
