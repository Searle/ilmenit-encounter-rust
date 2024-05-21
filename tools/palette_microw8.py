from PIL import Image

# Load the image
image_path = "./Microw8pal.png"
image = Image.open(image_path)

# Dimensions of the palette
palette_width, palette_height = image.size
rows, cols = 16, 16
square_width = 45
square_height = 45


# Function to get the color of the square at (row, col) from bottom-right alignment
def get_color(row, col):
    # Calculate the coordinates of the top-left pixel of the square
    left = palette_width - (cols - col) * square_width
    top = palette_height - (rows - row) * square_height
    # Get the color of the pixel in the middle of the square
    mid_x = left + square_width // 3
    mid_y = top + square_height // 3
    return image.getpixel((mid_x, mid_y))


# Extract all colors
colors = [get_color(row, col) for row in range(rows) for col in range(cols)]

# Prepare the colors in Rust format
rust_colors = [
    f"Color::get_color({hex((((((r << 8) + g) << 8) + b) << 8) + 0xff)})"
    for r, g, b, _a in colors
]

# Display the Rust array
rust_code = (
    """
use raylib::prelude::*;
pub fn make_palette() -> Box<[Color; 256]> { Box::new([
"""
    + "    "
    + ",\n    ".join(rust_colors)
    + "\n])\n}\n"
)
print(rust_code)
