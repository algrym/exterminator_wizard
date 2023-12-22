import argparse
from PIL import Image, ImageDraw, ImageFont

def get_contrasting_color(color):
    """
    Returns a contrasting color (black or white) based on the luminance of the given color.

    Args:
    color (tuple): A tuple representing an RGB color.

    Returns:
    str: 'black' or 'white' depending on the luminance.
    """
    # Calculate the luminance of the background color using the standard formula
    luminance = (0.299 * color[0] + 0.587 * color[1] + 0.114 * color[2])
    return "white" if luminance < 128 else "black"

def label_sprite_sheet(image_path, tile_width, tile_height, scale, padding, offset_x, offset_y, hairline, output_path):
    """
    Labels each tile in a sprite sheet with its index number and optionally adds hairline dividers.

    Args:
    image_path (str): Path to the sprite sheet image file.
    tile_width (int): Width of each tile in pixels.
    tile_height (int): Height of each tile in pixels.
    scale (int): Scale factor for the image.
    padding (int): Padding around each tile in pixels.
    offset_x (int): Horizontal offset in pixels.
    offset_y (int): Vertical offset in pixels.
    hairline (bool): Flag to add hairline dividers between tiles.
    output_path (str): Path for the output labeled sprite sheet image.
    """
    # Load and scale the sprite sheet image
    sprite_sheet = Image.open(image_path).convert("RGB")
    scaled_width = sprite_sheet.width * scale
    scaled_height = sprite_sheet.height * scale
    sprite_sheet = sprite_sheet.resize((scaled_width, scaled_height), Image.NEAREST)

    draw = ImageDraw.Draw(sprite_sheet)
    font = ImageFont.load_default()

    # Determine the number of tiles in each row and column
    cols = (scaled_width - 2 * offset_x) // ((tile_width + padding) * scale)
    rows = (scaled_height - 2 * offset_y) // ((tile_height + padding) * scale)

    # Process each tile
    for row in range(rows):
        for col in range(cols):
            x = col * (tile_width + padding) * scale + offset_x
            y = row * (tile_height + padding) * scale + offset_y
            tile_area = (x, y, x + tile_width * scale, y + tile_height * scale)
            tile = sprite_sheet.crop(tile_area)
            avg_color = tile.resize((1, 1)).getpixel((0, 0))

            contrasting_color = get_contrasting_color(avg_color)

            # Draw hairline dividers if enabled
            if hairline:
                draw.line([(x, offset_y), (x, scaled_height - offset_y)], fill=contrasting_color)
                draw.line([(offset_x, y), (scaled_width - offset_x, y)], fill=contrasting_color)

            # Label the tile with its index
            index = row * cols + col
            draw.text((x + 5 * scale, y + 5 * scale), str(index), fill=contrasting_color, font=font)

    # Final set of dividers on the right and bottom edges
    if hairline:
        draw.line([(scaled_width - offset_x, offset_y), (scaled_width - offset_x, scaled_height - offset_y)], fill=contrasting_color)
        draw.line([(offset_x, scaled_height - offset_y), (scaled_width - offset_x, scaled_height - offset_y)], fill=contrasting_color)

    # Save the modified sprite sheet
    sprite_sheet.save(output_path)

def main():
    """Parses command line arguments and calls the label_sprite_sheet function."""
    parser = argparse.ArgumentParser(description='Label tiles in a sprite sheet with index numbers, scale, padding, offset, and optional hairline dividers with dynamic contrasting colors.')
    parser.add_argument('image_path', type=str, help='Path to the sprite sheet image file')
    parser.add_argument('tile_width', type=int, help='Width of each tile in pixels')
    parser.add_argument('tile_height', type=int, help='Height of each tile in pixels')
    parser.add_argument('--scale', type=int, default=1, help='Scale factor for the image (default: 1)')
    parser.add_argument('--padding', type=int, default=0, help='Padding around each tile in pixels (default: 0)')
    parser.add_argument('--offset_x', type=int, default=0, help='Horizontal offset in pixels (default: 0)')
    parser.add_argument('--offset_y', type=int, default=0, help='Vertical offset in pixels (default: 0)')
    parser.add_argument('--hairline', action='store_true', help='Add hairline dividers between tiles (default: False)')
    parser.add_argument('output_path', type=str, help='Path for the output labeled sprite sheet image')

    args = parser.parse_args()

    label_sprite_sheet(args.image_path, args.tile_width, args.tile_height, args.scale, args.padding, args.offset_x, args.offset_y, args.hairline, args.output_path)

if __name__ == "__main__":
    main()
