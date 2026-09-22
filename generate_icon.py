import os
from PIL import Image

def main():
    src_path = "src/assets/logo-256x256.png"
    if not os.path.exists(src_path):
        print(f"Error: {src_path} not found.")
        return

    base_img = Image.open(src_path).convert("RGBA")
    
    # 1. src/assets/icon.png
    base_img.save("src/assets/icon.png", "PNG")
    print("Saved src/assets/icon.png")

    # 2. docs/images/tailorbird_icon.png
    os.makedirs("docs/images", exist_ok=True)
    base_img.save("docs/images/tailorbird_icon.png", "PNG")
    print("Saved docs/images/tailorbird_icon.png")

    # 3. Multi-size Windows ICO for executable and shortcuts
    ico_sizes = [(16, 16), (24, 24), (32, 32), (48, 48), (64, 64), (128, 128), (256, 256)]
    base_img.save("src/assets/icon.ico", format="ICO", sizes=ico_sizes)
    print("Saved src/assets/icon.ico")

    # 4. Web favicon.ico
    fav_sizes = [(16, 16), (32, 32), (48, 48)]
    base_img.save("docs/favicon.ico", format="ICO", sizes=fav_sizes)
    print("Saved docs/favicon.ico")

    # 5. Raw 32x32 RGBA buffer for Tao WindowBuilder::with_window_icon
    img_32 = base_img.resize((32, 32), Image.Resampling.LANCZOS)
    rgba_32_bytes = img_32.tobytes("raw", "RGBA")
    with open("src/assets/icon_32.rgba", "wb") as f:
        f.write(rgba_32_bytes)
    print(f"Saved src/assets/icon_32.rgba ({len(rgba_32_bytes)} bytes)")

    # 6. Web favicons and touch icons
    sizes = {
        "docs/images/favicon-16x16.png": (16, 16),
        "docs/images/favicon-32x32.png": (32, 32),
        "docs/images/apple-touch-icon.png": (180, 180),
        "docs/images/icon-192.png": (192, 192),
        "docs/images/tailorbird_icon_128.png": (128, 128),
    }

    for out_path, s in sizes.items():
        resized = base_img.resize(s, Image.Resampling.LANCZOS)
        resized.save(out_path, "PNG")
        print(f"Saved {out_path} ({s[0]}x{s[1]})")

    print("All icons successfully generated from logo-256x256.png.")

if __name__ == "__main__":
    main()
