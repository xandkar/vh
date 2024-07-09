vh
===============================================================================

Visual hash CLI tool. Currently based only on eth blockies.

build and install
-------------------------------------------------------------------------------

    cargo install --git https://github.com/xandkar/vh.git

example
-------------------------------------------------------------------------------

Execute

    echo example | vh > example.png

then open the `example.png` and see: ![example](example.png)

background
-------------------------------------------------------------------------------

### Neal Krawetz's post

<https://www.hackerfactor.com/blog/?/archives/432-Looks-Like-It.html>

#### Average Hash

> 1. **Reduce size**. The fastest way to remove high frequencies and detail
>    is to shrink the image. In this case, shrink it to 8x8 so that there
>    are 64 total pixels. Don't bother keeping the aspect ratio, just crush
>    it down to fit an 8x8 square. This way, the hash will match any
>    variation of the image, regardless of scale or aspect ratio.
> 2. **Reduce color**. The tiny 8x8 picture is converted to a grayscale.
>    This changes the hash from 64 pixels (64 red, 64 green, and 64 blue) to
>    64 total colors.
> 3. **Average the colors**. Compute the mean value of the 64 colors.
> 4. **Compute the bits**. This is the fun part. Each bit is simply set
>    based on whether the color value is above or below the mean.
> 5. **Construct the hash**. Set the 64 bits into a 64-bit integer. The
>    order does not matter, just as long as you are consistent. (I set the
>    bits from left to right, top to bottom using big-endian.)

#### pHash

> 1. **Reduce size**. Like Average Hash, pHash starts with a small image.
>    However, the image is larger than 8x8; 32x32 is a good size. This is
>    really done to simplify the DCT computation and not because it is
>    needed to reduce the high frequencies.
> 2. **Reduce color**. The image is reduced to a grayscale just to further
>    simplify the number of computations.
> 3. **Compute the DCT**. The DCT separates the image into a collection of
>    frequencies and scalars. While JPEG uses an 8x8 DCT, this algorithm
>    uses a 32x32 DCT.
> 4. **Reduce the DCT**. While the DCT is 32x32, just keep the top-left 8x8.
>    Those represent the lowest frequencies in the picture.
> 5. **Compute the average value**. Like the Average Hash, compute the mean
>    DCT value (using only the 8x8 DCT low-frequency values and excluding
>    the first term since the DC coefficient can be significantly different
>    from the other values and will throw off the average). Thanks to David
>    Starkweather for the added information about pHash. He wrote: "the dct
>    hash is based on the low 2D DCT coefficients starting at the second
>    from lowest, leaving out the first DC term. This excludes completely
>    flat image information (i.e. solid colors) from being included in the
>    hash description."
> 6. **Further reduce the DCT**. This is the magic step. Set the 64 hash
>    bits to 0 or 1 depending on whether each of the 64 DCT values is above
>    or below the average value. The result doesn't tell us the actual low
>    frequencies; it just tells us the very-rough relative scale of the
>    frequencies to the mean. The result will not vary as long as the
>    overall structure of the image remains the same; this can survive gamma
>    and color histogram adjustments without a problem.
> 7. **Construct the hash**. Set the 64 bits into a 64-bit integer. The
>    order does not matter, just as long as you are consistent. To see what
>    this fingerprint looks like, simply set the values (this uses +255 and
>    -255 based on whether the bits are 1 or 0) and convert from the 32x32
>    DCT (with zeros for the high frequencies) back into the 32x32 image: ..
>    = 8a0303f6df3ec8cd At first glance, this might look like some random
>    blobs... but look closer. There is a dark ring around her head and the
>    dark horizontal line in the background (right side of the picture)
>    appears as a dark spot.
