use std::io::{Read, Write};

use anyhow::{anyhow, Result};

fn blockify<W: Write>(input: &[u8], output: W) -> Result<()> {
    // TODO Re-implement blockies.
    let gen = blockies::Classic {
        color: Some(pixelate::BLACK),
        size: 8,
        scale: 8,
        background_color: None,
    };
    gen.create_icon(output, input).map_err(|e| anyhow!("blockies failure: {:?}", e))
}

fn main() -> Result<()> {
    let input = {
        let mut buf = Vec::new();
        for byte_result in std::io::stdin().bytes() {
            buf.push(byte_result?)
        }
        buf
    };
    let mut output = std::io::stdout();
    blockify(&input[..], &mut output)?;
    Ok(())
}
