#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
生成日语假名的 ASCII art
"""

from PIL import Image, ImageDraw, ImageFont
import sys
import os

# 平假名列表
HIRAGANA = [
    'あ', 'い', 'う', 'え', 'お',
    'か', 'き', 'く', 'け', 'こ',
    'さ', 'し', 'す', 'せ', 'そ',
    'た', 'ち', 'つ', 'て', 'と',
    'な', 'に', 'ぬ', 'ね', 'の',
    'は', 'ひ', 'ふ', 'へ', 'ほ',
    'ま', 'み', 'む', 'め', 'も',
    'や', 'ゆ', 'よ',
    'ら', 'り', 'る', 'れ', 'ろ',
    'わ', 'を', 'ん'
]

# 片假名列表
KATAKANA = [
    'ア', 'イ', 'ウ', 'エ', 'オ',
    'カ', 'キ', 'ク', 'ケ', 'コ',
    'サ', 'シ', 'ス', 'セ', 'ソ',
    'タ', 'チ', 'ツ', 'テ', 'ト',
    'ナ', 'ニ', 'ヌ', 'ネ', 'ノ',
    'ハ', 'ヒ', 'フ', 'ヘ', 'ホ',
    'マ', 'ミ', 'ム', 'メ', 'モ',
    'ヤ', 'ユ', 'ヨ',
    'ラ', 'リ', 'ル', 'レ', 'ロ',
    'ワ', 'ヲ', 'ン'
]

def generate_ascii_art(char, width=50, height=15):
    """
    生成单个假名的 ASCII art

    Args:
        char: 假名字符
        width: ASCII art 宽度
        height: ASCII art 高度

    Returns:
        ASCII art 字符串列表
    """
    # 创建大图像
    img_size = 1000
    img = Image.new("L", (img_size, img_size), 255)
    draw = ImageDraw.Draw(img)

    # 尝试不同的字体
    font_paths = [
        "C:\Windows\Fonts\msgothic.ttc",  # MS Gothic
        "C:\Windows\Fonts\msmincho.ttc",  # MS Mincho
        "C:\Windows\Fonts\meiryo.ttc",    # Meiryo
        "/System/Library/Fonts/ヒラギノ角ゴシック W3.ttc",  # macOS
        "/usr/share/fonts/truetype/takao-gothic/TakaoGothic.ttf",  # Linux
        "C:\Windows\Fonts\YuGothM.ttc",   # Yu Gothic Medium
    ]

    font = None
    for font_path in font_paths:
        try:
            font = ImageFont.truetype(font_path, 800)
            break
        except:
            continue

    if font is None:
        print(f"警告: 无法加载日文字体，使用默认字体", file=sys.stderr)
        font = ImageFont.load_default()

    # 获取文本边界框
    bbox = draw.textbbox((0, 0), char, font=font)
    text_width = bbox[2] - bbox[0]
    text_height = bbox[3] - bbox[1]

    # 居中绘制
    x = (img_size - text_width) // 2 - bbox[0]
    y = (img_size - text_height) // 2 - bbox[1]

    draw.text((x, y), char, font=font, fill=0)

    # 缩放到目标大小
    small = img.resize((width, height), Image.LANCZOS)

    # 转换为 ASCII art
    # 使用不同密度的字符 - 只使用纯ASCII字符以避免显示宽度问题
    chars = " .',:;-=+*#%@"

    lines = []
    for y in range(height):
        row = ""
        for x in range(width):
            g = small.getpixel((x, y))
            # 灰度值映射到字符
            idx = int((255 - g) / 255 * (len(chars) - 1))
            row += chars[idx]
        lines.append(row)

    return lines

def format_rust_code(char, lines, is_katakana=False):
    """
    格式化为 Rust 代码
    """
    char_type = "片假名" if is_katakana else "平假名"
    result = f'        "{char}" => vec![\n'
    for line in lines:
        # 转义 Rust 字符串中的特殊字符
        escaped = line.replace('\', '\\').replace('"', '\\"')
        result += f'            "{escaped}".to_string(),\n'
    result += '        ],'
    return result

def main():
    # Get the script directory
    script_dir = os.path.dirname(os.path.abspath(__file__))
    
    print("开始生成 ASCII art...")
    print("\n" + "="*60)
    print("平假名 (Hiragana)")
    print("="*60)

    hiragana_code = []
    for char in HIRAGANA:
        lines = generate_ascii_art(char)
        print(f"\n{char}:")
        for line in lines:
            print(line)
        hiragana_code.append(format_rust_code(char, lines, False))

    print("\n" + "="*60)
    print("片假名 (Katakana)")
    print("="*60)

    katakana_code = []
    for char in KATAKANA:
        lines = generate_ascii_art(char)
        print(f"\n{char}:")
        for line in lines:
            print(line)
        katakana_code.append(format_rust_code(char, lines, True))

    # 保存到scripts目录
    hiragana_path = os.path.join(script_dir, "hiragana_ascii_art.txt")
    katakana_path = os.path.join(script_dir, "katakana_ascii_art.txt")
    
    with open(hiragana_path, "w", encoding="utf-8") as f:
        f.write("\n".join(hiragana_code))

    with open(katakana_path, "w", encoding="utf-8") as f:
        f.write("\n".join(katakana_code))

    print("\n" + "="*60)
    print("完成！代码已保存到:")
    print(f"  - {hiragana_path}")
    print(f"  - {katakana_path}")
    print("="*60)

if __name__ == "__main__":
    main()
