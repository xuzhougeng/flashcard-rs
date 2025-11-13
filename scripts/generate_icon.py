"""
Quick icon generator for JP Desktop application
"""

from PIL import Image, ImageDraw, ImageFont
import os

def create_icon():
    # Create icons directory if not exists
    if not os.path.exists('icons'):
        os.makedirs('icons')

    # Create 512x512 icon
    size = 512
    img = Image.new('RGBA', (size, size), (102, 126, 234, 255))
    draw = ImageDraw.Draw(img)

    # Draw a circle background
    margin = 50
    draw.ellipse([margin, margin, size-margin, size-margin],
                 fill=(118, 75, 162, 255))

    # Draw JP text
    try:
        # Try to use a nice font
        font = ImageFont.truetype("arial.ttf", 180)
    except:
        try:
            font = ImageFont.truetype("C:\\Windows\\Fonts\\arial.ttf", 180)
        except:
            print("Using default font (text might be small)")
            font = ImageFont.load_default()

    text = "JP"
    bbox = draw.textbbox((0, 0), text, font=font)
    text_width = bbox[2] - bbox[0]
    text_height = bbox[3] - bbox[1]
    position = ((size - text_width) // 2, (size - text_height) // 2 - 20)

    draw.text(position, text, fill=(255, 255, 255, 255), font=font)

    # Save main icon
    img.save('icons/icon.png')
    print("Created: icons/icon.png (512x512)")

    # Create ICO file for Windows
    try:
        # ICO needs multiple sizes
        ico_sizes = [(256, 256), (128, 128), (64, 64), (48, 48), (32, 32), (16, 16)]
        ico_imgs = []
        for size in ico_sizes:
            ico_img = img.resize(size, Image.Resampling.LANCZOS)
            ico_imgs.append(ico_img)

        # Save as ICO
        ico_imgs[0].save('icons/icon.ico', format='ICO', sizes=ico_sizes)
        print("Created: icons/icon.ico (multi-size)")
    except Exception as e:
        print(f"Warning: Could not create ICO file: {e}")

    # Create smaller icons
    for icon_size in [256, 128, 64, 32]:
        small_img = img.resize((icon_size, icon_size), Image.Resampling.LANCZOS)
        small_img.save(f'icons/{icon_size}x{icon_size}.png')
        print(f"Created: icons/{icon_size}x{icon_size}.png")

    print("\nAll icons generated successfully!")
    print("\nYou can now build the desktop application:")
    print("  cargo build --bin jp-desktop --features desktop --release")

if __name__ == '__main__':
    try:
        create_icon()
    except ImportError:
        print("Error: Pillow (PIL) is required")
        print("\nInstall it with:")
        print("  pip install Pillow")
    except Exception as e:
        print(f"Error: {e}")
