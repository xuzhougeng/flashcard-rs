# 图标文件说明

桌面应用需要以下图标文件：

## 必需文件

1. **icon.png** (推荐 512x512 或 1024x1024 像素)
   - 应用主图标
   - 用于安装包、任务栏等
   - 格式：PNG，带透明背景

2. **32x32.png** (可选，系统会自动缩放)
   - 小图标版本
   - 用于托盘图标

## 临时解决方案

如果暂时没有图标，可以使用以下方法创建简单的占位符图标：

### 方法1：使用Python生成

```python
from PIL import Image, ImageDraw, ImageFont

# 创建 512x512 的图标
size = 512
img = Image.new('RGBA', (size, size), (102, 126, 234, 255))
draw = ImageDraw.Draw(img)

# 绘制简单的 JP 文字
try:
    font = ImageFont.truetype("arial.ttf", 200)
except:
    font = ImageFont.load_default()

text = "JP"
bbox = draw.textbbox((0, 0), text, font=font)
text_width = bbox[2] - bbox[0]
text_height = bbox[3] - bbox[1]
position = ((size - text_width) // 2, (size - text_height) // 2 - 50)

draw.text(position, text, fill=(255, 255, 255, 255), font=font)

img.save('icons/icon.png')
print("图标已生成: icons/icon.png")
```

### 方法2：在线生成

访问以下网站在线生成图标：
- https://www.favicon.cc/
- https://favicon.io/
- https://www.canva.com/

### 方法3：复制其他图标

暂时复制任意 PNG 图标到 `icons/` 目录，重命名为 `icon.png` 即可。

## 当前状态

✅ icons/ 目录已创建
⏳ 待添加图标文件

## 构建提示

即使没有图标文件，也可以尝试构建（可能会有警告但通常不会失败）：

```bash
cargo build --bin jp-desktop --features desktop --release
```
