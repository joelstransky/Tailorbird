import math
from PIL import Image, ImageDraw

def create_tailorbird_icon(size=256):
    img = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    
    # Scale factor from standard 24x24 viewBox to icon canvas
    # Leave padding around the bird
    pad = size * 0.08
    corner = size * 0.22
    
    # Draw dark rounded background container
    bg_color = (15, 23, 42, 255) # #0F172A
    border_color = (56, 189, 248, 180) # #38BDF8 glow border
    
    # Background rounded rect
    draw.rounded_rectangle(
        [pad, pad, size - pad, size - pad],
        radius=corner,
        fill=bg_color,
        outline=border_color,
        width=max(1, int(size * 0.03))
    )
    
    # Bird coordinates mapped from 24x24 viewBox to inner area
    # inner area: [inner_pad, inner_pad, size - inner_pad, size - inner_pad]
    inner_pad = size * 0.22
    inner_w = size - inner_pad * 2
    
    def transform(x, y):
        # x, y in 0..24
        tx = inner_pad + (x / 24.0) * inner_w
        ty = inner_pad + (y / 24.0) * inner_w
        return (tx, ty)

    stroke_w = max(2, int(size * 0.065))
    accent_cyan = (56, 189, 248, 255) # #38BDF8
    accent_indigo = (129, 140, 248, 255) # #818CF8
    white = (255, 255, 255, 255)
    
    # Draw bird body & head curve
    # Points approximating: M3.4 18H12a8 8 0 0 0 8-8V7a4 4 0 0 0-7.28-2.3L2 18
    # Let's draw high quality smooth curves
    # Head & beak: (20, 7) to (22, 7.5) to (20, 8)
    p_beak = [transform(20, 7), transform(22.5, 7.5), transform(20, 8)]
    draw.line(p_beak, fill=accent_cyan, width=stroke_w, joint="round")
    
    # Eye: dot at (16, 7)
    eye_pos = transform(16, 7)
    eye_r = max(2.0, size * 0.03)
    draw.ellipse([eye_pos[0]-eye_r, eye_pos[1]-eye_r, eye_pos[0]+eye_r, eye_pos[1]+eye_r], fill=white)
    
    # Legs: M10 18v3 and M14 17.75V21
    draw.line([transform(10, 18), transform(10, 21.5)], fill=accent_indigo, width=stroke_w)
    draw.line([transform(14, 17.75), transform(14, 21.5)], fill=accent_indigo, width=stroke_w)
    
    # Wing arc: M7 18a6 6 0 0 0 3.84-10.61
    # Generate points along arc
    wing_pts = []
    for deg in range(180, 310, 10):
        rad = math.radians(deg)
        # Center ~ (10.8, 14.5), r ~ 4.2
        wx = 10.5 + 4.8 * math.cos(rad)
        wy = 14.5 + 4.8 * math.sin(rad)
        wing_pts.append(transform(wx, wy))
    if len(wing_pts) > 1:
        draw.line(wing_pts, fill=accent_cyan, width=stroke_w, joint="round")
        
    # Main body: Tail (2, 18) -> bottom belly (12, 18) -> breast curve (20, 10) -> head top (17, 5) -> back to tail (2, 18)
    body_pts = [
        transform(2.5, 18),
        transform(7, 18),
        transform(12, 18),
        transform(16.5, 16),
        transform(19.5, 12),
        transform(20, 8),
        transform(19, 6),
        transform(16.5, 4.8),
        transform(13, 5.5),
        transform(8, 11),
        transform(2.5, 18),
    ]
    draw.line(body_pts, fill=accent_indigo, width=stroke_w, joint="round")
    
    return img

def main():
    sizes = [16, 24, 32, 48, 64, 128, 256]
    images = {}
    for s in sizes:
        # Create at 4x and downsample with Lanczos for beautiful antialiasing
        large = create_tailorbird_icon(s * 4)
        images[s] = large.resize((s, s), Image.Resampling.LANCZOS)
    
    # Save 256x256 PNG
    images[256].save("src/assets/icon.png", "PNG")
    print("Saved src/assets/icon.png")
    
    # Save multi-size Windows ICO
    images[256].save(
        "src/assets/icon.ico",
        format="ICO",
        sizes=[(s, s) for s in sizes]
    )
    print("Saved src/assets/icon.ico")
    
    # Save 32x32 raw RGBA bytes for Tao WindowBuilder::with_window_icon
    img_32 = images[32]
    rgba_bytes = img_32.tobytes("raw", "RGBA")
    with open("src/assets/icon_32.rgba", "wb") as f:
        f.write(rgba_bytes)
    print(f"Saved src/assets/icon_32.rgba ({len(rgba_bytes)} bytes)")

if __name__ == "__main__":
    main()
