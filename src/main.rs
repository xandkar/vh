use std::io::{Read, Write};

use anyhow::{anyhow, Result};
use clap::Parser;
use pixelate::Color;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(long = "fg-rgb", default_value = "0-0-0")]
    fg_rgb: String,

    #[arg(long = "bg-rgb", default_value = "255-255-255")]
    bg_rgb: String,

    #[arg(long = "fg-alpha", default_value = "255")]
    fg_alpha: u8,

    #[arg(long = "bg-alpha", default_value = "0")]
    bg_alpha: u8,

    /// in pixels
    #[arg(long = "block-size", default_value = "8")]
    block_size: usize,

    /// in blocks
    #[arg(long = "row-size", default_value = "8")]
    row_size: usize,
}

fn blockify<W: Write>(
    input: &[u8],
    output: W,
    fg: Color,
    bg: Color,
    block_size: usize,
    row_size: usize,
) -> Result<()> {
    // TODO Re-implement blockies.
    let gen = blockies::Classic {
        color: Some(fg),
        size: row_size,    // in blocks
        scale: block_size, // in pixels
        background_color: Some(bg),
    };
    gen.create_icon(output, input)
        .map_err(|e| anyhow!("blockies failure: {:?}", e))
}

fn color_parse(rgb_string: &str, alpha: u8) -> Result<Color> {
    let mut rgb: Vec<u8> = Vec::new();
    for num in rgb_string.split('-') {
        rgb.push(num.parse()?)
    }
    match rgb[..] {
        [r, g, b] => Ok(Color::Rgba(r, g, b, alpha)),
        _ => Err(anyhow!(
            "Invalid number of RGB components in: {:?}",
            rgb_string
        )),
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let fg = color_parse(&cli.fg_rgb, cli.fg_alpha)?;
    let bg = color_parse(&cli.bg_rgb, cli.bg_alpha)?;
    let input = {
        let mut buf = Vec::new();
        for byte_result in std::io::stdin().bytes() {
            buf.push(byte_result?)
        }
        buf
    };
    let mut output = std::io::stdout();
    blockify(
        &input[..],
        &mut output,
        fg,
        bg,
        cli.block_size,
        cli.row_size,
    )?;
    Ok(())
}
