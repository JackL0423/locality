use array2::Array2;
use clap::Parser;
use csc411_image::{Read, Rgb, RgbImage, Write};
use std::time::Instant;
#[derive(Clone, Copy, Debug)]
pub enum Order {
    Row,
    Col,
    /// Tiled: visit the image in TILE x TILE squares, row-major inside each.
    Block,
}

/// Side length, in pixels, of the squares visited by `Order::Block`. Small
/// enough that a tile's source rows and destination rows all fit in L1 cache.
const TILE: usize = 32;

/// Transforms an image by applying a mapping function to each pixel's coordinates.
///
/// # Arguments
///
/// * `src` - The source image represented as a 2D array of `Rgb` pixels.
/// * `dst_w` - The width of the destination image.
/// * `dst_h` - The height of the destination image.
/// * `order` - The order in which to iterate over the source image (row-major,
/// column-major or tiled).
/// * `map` - A function that takes the coordinates of a pixel in the source image
/// and returns the coordinates of the corresponding pixel in the destination image.
///
/// # Returns
///
/// The transformed image as a 2D array of `Rgb` pixels.
///
/// # Examples
///
/// ```
/// use array2::Array2;
/// use csc411_image::Rgb;
///
/// let src = Array2::new(2, 2, Rgb { red: 255, green: 0, blue: 0 });
/// let dst = transform(&src, 2, 2, &Order::Row, |c, r| (c, r));
/// ```
pub fn transform(
    src: &Array2<Rgb>,
    dst_w: usize,
    dst_h: usize,
    order: &Order,
    map: impl Fn(usize, usize) -> (usize, usize),
) -> Array2<Rgb> {
    let blank = Rgb {
        red: 0,
        green: 0,
        blue: 0,
    };
    let (w, h) = (src.get_width(), src.get_height());
    let mut dst = Array2::new(dst_w, dst_h, blank);
    // Index the row-major storage directly: no per-pixel division to recover
    // coordinates and no Option-returning bounds checks.
    let s = src.as_slice();
    let d = dst.as_mut_slice();
    let mut copy = |c: usize, r: usize| {
        let (dc, dr) = map(c, r);
        d[dr * dst_w + dc] = s[r * w + c].clone();
    };
    match order {
        Order::Row => {
            for r in 0..h {
                for c in 0..w {
                    copy(c, r)
                }
            }
        }
        Order::Col => {
            for c in 0..w {
                for r in 0..h {
                    copy(c, r)
                }
            }
        }
        Order::Block => {
            // Each tile touches only TILE short runs of source rows and TILE
            // short runs of destination rows, so every cache line it loads is
            // still resident when its neighbouring pixels are used.
            for tr in (0..h).step_by(TILE) {
                for tc in (0..w).step_by(TILE) {
                    for r in tr..(tr + TILE).min(h) {
                        for c in tc..(tc + TILE).min(w) {
                            copy(c, r)
                        }
                    }
                }
            }
        }
    }
    dst
}

/// Rotates the source image 90 degrees clockwise and returns the resulting image.
///
/// # Arguments
/// * `src` - The source image represented as a 2D array of `Rgb` pixels
/// * `o` - The order in which to iterate over the source image (row-major, column-major or tiled)
/// # Returns
/// The rotated image as a 2D array of `Rgb` pixels
///
/// # Examples
/// ```
/// use array2::Array2;
/// use csc411_image::Rgb;
///
/// let src = Array2::new(2, 2, Rgb { red: 255, green: 0, blue: 0 });
/// let dst = rotate90(&src, &Order::Row);
/// ```
pub fn rotate90(src: &Array2<Rgb>, o: &Order) -> Array2<Rgb> {
    let (w, h) = (src.get_width(), src.get_height());
    transform(src, h, w, o, |i, j| (h - j - 1, i))
}

/// Rotates the source image 180 degrees and returns the resulting image.
/// # Arguments
/// * `src` - The source image represented as a 2D array of
/// `Rgb` pixels
/// * `o` - The order in which to iterate over the source image (row-major, column-major or tiled)
/// # Returns
/// The rotated image as a 2D array of `Rgb` pixels
/// # Examples
/// ```
/// use array2::Array2;
/// use csc411_image::Rgb;
/// let src = Array2::new(2, 2, Rgb { red: 255, green: 0, blue: 0 });
/// let dst = rotate180(&src, &Order::Row);
/// ```
pub fn rotate180(src: &Array2<Rgb>, o: &Order) -> Array2<Rgb> {
    let (w, h) = (src.get_width(), src.get_height());
    transform(src, w, h, o, |i, j| (w - i - 1, h - j - 1))
}

/// Rotates the source image 270 degrees clockwise and returns the resulting image.
/// # Arguments
/// * `src` - The source image represented as a 2D array of `Rgb` pixels
/// * `o` - The order in which to iterate over the source image (row-major, column-major or tiled)
/// # Returns
/// The rotated image as a 2D array of `Rgb` pixels
/// # Examples
/// ```
/// use array2::Array2;
/// use csc411_image::Rgb;
/// let src = Array2::new(2, 2, Rgb { red: 255, green: 0, blue: 0 });
/// let dst = rotate270(&src, &Order::Row);
/// ```
pub fn rotate270(src: &Array2<Rgb>, o: &Order) -> Array2<Rgb> {
    let (w, h) = (src.get_width(), src.get_height());
    transform(src, h, w, o, |i, j| (j, w - i - 1))
}

/// Transposes the source image and returns the resulting image.
/// # Arguments
/// * `src` - The source image represented as a 2D array of `Rgb` pixels
/// * `o` - The order in which to iterate over the source image
/// # Returns
/// The transposed image as a 2D array of `Rgb` pixels
/// # Examples
/// ```
/// use array2::Array2;
/// use csc411_image::Rgb;
/// let src = Array2::new(2, 2, Rgb { red: 255, green: 0, blue: 0 });
/// let dst = transpose(&src, &Order::Block);
/// ```
pub fn transpose(src: &Array2<Rgb>, o: &Order) -> Array2<Rgb> {
    let (w, h) = (src.get_width(), src.get_height());
    transform(src, h, w, o, |i, j| (j, i))
}

/// Flips the source image horizontally and returns the resulting image.
/// # Arguments
/// * `src` - The source image represented as a 2D array of `Rgb` pixels
/// # Returns
/// The flipped image as a 2D array of `Rgb` pixels
/// # Examples
/// ```
/// use array2::Array2;
/// use csc411_image::Rgb;
/// let src = Array2::new(2, 2, Rgb { red: 255, green: 0, blue: 0 });
/// let dst = flip_horizontally(&src);
/// ```
pub fn flip_horizontally(src: &Array2<Rgb>) -> Array2<Rgb> {
    let (w, h) = (src.get_width(), src.get_height());
    transform(src, w, h, &Order::Row, |i, j| (w - i - 1, j))
}

/// Flips the source image vertically and returns the resulting image.
/// # Arguments
/// * `src` - The source image represented as a 2D array of `Rgb` pixels
/// # Returns
/// The flipped image as a 2D array of `Rgb` pixels
/// # Examples
/// ```
/// use array2::Array2;
/// use csc411_image::Rgb;
/// let src = Array2::new(2, 2, Rgb { red: 255, green: 0, blue: 0 });
/// let dst = flip_vertically(&src);
/// ```
pub fn flip_vertically(src: &Array2<Rgb>) -> Array2<Rgb> {
    let (w, h) = (src.get_width(), src.get_height());
    transform(src, w, h, &Order::Row, |i, j| (i, h - j - 1))
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Input image; reads stdin when omitted
    input: Option<String>,

    #[clap(long = "flip", required = false)]
    flip: Option<String>,

    #[clap(short = 'r', long = "rotate")]
    rotate: Option<u32>,

    #[clap(long = "transpose")]
    transpose: bool,

    #[clap(long = "row-major", conflicts_with_all = ["col_major", "block_major"])]
    row_major: bool,

    #[clap(long = "col-major", conflicts_with_all = ["row_major", "block_major"])]
    col_major: bool,

    #[clap(long = "block-major", conflicts_with_all = ["row_major", "col_major"])]
    block_major: bool,
}

fn main() {
    // parse command line arguments
    let args = Args::parse();
    // read image file (or stdin) into an image object
    let img = RgbImage::read(args.input.as_deref()).expect("Failed to read image file");
    // traversal order chosen on the command line, if any
    let chosen = if args.row_major {
        Some(Order::Row)
    } else if args.col_major {
        Some(Order::Col)
    } else if args.block_major {
        Some(Order::Block)
    } else {
        None
    };
    // Without a flag, use the most cache-friendly order for each transform:
    // tiled when rows turn into columns (rotate 90/270, transpose), row-major
    // when rows stay rows (rotate 180), since both sides are then in order.
    let order = chosen.unwrap_or(Order::Block);
    let order180 = chosen.unwrap_or(Order::Row);

    // image pixels are always stored row-major
    let mut arr = Array2::from_row_major(img.width as usize, img.height as usize, img.pixels);

    // apply transformations based on command line arguments
    let now = Instant::now();
    if let Some(rotate) = args.rotate {
        match rotate {
            0 => {}
            90 => {
                arr = rotate90(&arr, &order);
            }
            180 => {
                arr = rotate180(&arr, &order180);
            }
            270 => {
                arr = rotate270(&arr, &order);
            }
            _ => {
                eprintln!("Invalid rotation argument: {}", rotate);
                std::process::exit(1);
            }
        }
    }
    if let Some(flip) = args.flip {
        match flip.as_str() {
            "horizontal" => {
                arr = flip_horizontally(&arr);
            }
            "vertical" => {
                arr = flip_vertically(&arr);
            }
            _ => {
                eprintln!("Invalid flip argument: {}", flip);
                std::process::exit(1);
            }
        }
    }
    if args.transpose {
        arr = transpose(&arr, &order);
    }
    let elapsed = now.elapsed();
    // timing goes to stderr so stdout holds only the image
    eprintln!(
        "Image transformation took {:.2} ms",
        elapsed.as_secs_f64() * 1000.0
    );

    // write the transformed image to stdout
    let out = RgbImage {
        width: arr.get_width() as u32,
        height: arr.get_height() as u32,
        denominator: img.denominator,
        pixels: arr.into_row_major(),
    };
    out.write(None).expect("Failed to write image");
}

#[cfg(test)]
mod tests {
    use super::*;

    // w x h image whose pixels are numbered 0.. in row-major order. Every
    // channel encodes the number so we can tell whether whole pixels moved.
    fn grid(w: usize, h: usize) -> Array2<Rgb> {
        let data = (0..(w * h) as u16)
            .map(|v| Rgb {
                red: v,
                green: v + 100,
                blue: v + 200,
            })
            .collect();
        Array2::from_row_major(w, h, data)
    }

    // Pixel numbers in row-major order; also checks green/blue stayed with red.
    fn ids(a: &Array2<Rgb>) -> Vec<u16> {
        a.iter_row_major()
            .map(|p| {
                assert_eq!(p.green, p.red + 100);
                assert_eq!(p.blue, p.red + 200);
                p.red
            })
            .collect()
    }

    fn dims(a: &Array2<Rgb>) -> (usize, usize) {
        (a.get_width(), a.get_height())
    }

    // Test grid used throughout (3 wide x 2 tall):
    // 0 1 2
    // 3 4 5

    #[test]
    fn transform_identity_preserves_image() {
        let g = grid(3, 2);
        for o in [Order::Row, Order::Col, Order::Block] {
            let d = transform(&g, 3, 2, &o, |c, r| (c, r));
            assert_eq!(dims(&d), (3, 2));
            assert_eq!(ids(&d), vec![0, 1, 2, 3, 4, 5]);
        }
    }

    #[test]
    fn transform_passes_col_row_to_map() {
        // Record every (col, row) the map sees; each source pixel exactly once.
        let g = grid(3, 2);
        for o in [Order::Row, Order::Col, Order::Block] {
            let seen = std::cell::RefCell::new(Vec::new());
            transform(&g, 3, 2, &o, |c, r| {
                seen.borrow_mut().push((c, r));
                (c, r)
            });
            let mut seen = seen.into_inner();
            seen.sort();
            assert_eq!(seen, vec![(0, 0), (0, 1), (1, 0), (1, 1), (2, 0), (2, 1)]);
        }
    }

    #[test]
    fn transform_uses_destination_dimensions() {
        // Map a 3x2 source into a 6x1 strip, column = row-major index.
        let g = grid(3, 2);
        let d = transform(&g, 6, 1, &Order::Row, |c, r| (r * 3 + c, 0));
        assert_eq!(dims(&d), (6, 1));
        assert_eq!(ids(&d), vec![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn transform_unwritten_pixels_are_black() {
        // Destination is bigger than the source; extra pixels stay blank.
        let g = grid(1, 1);
        let d = transform(&g, 2, 1, &Order::Row, |c, r| (c, r));
        let reds: Vec<u16> = d.iter_row_major().map(|p| p.red).collect();
        assert_eq!(reds, vec![0, 0]);
        let last = d.at(0, 1).unwrap();
        assert_eq!((last.green, last.blue), (0, 0));
    }

    #[test]
    fn rotate90_works() {
        // 3 0
        // 4 1
        // 5 2
        let g = grid(3, 2);
        for o in [Order::Row, Order::Col, Order::Block] {
            let d = rotate90(&g, &o);
            assert_eq!(dims(&d), (2, 3));
            assert_eq!(ids(&d), vec![3, 0, 4, 1, 5, 2]);
        }
    }

    #[test]
    fn rotate180_works() {
        // 5 4 3
        // 2 1 0
        let g = grid(3, 2);
        for o in [Order::Row, Order::Col, Order::Block] {
            let d = rotate180(&g, &o);
            assert_eq!(dims(&d), (3, 2));
            assert_eq!(ids(&d), vec![5, 4, 3, 2, 1, 0]);
        }
    }

    #[test]
    fn rotate90_and_180_row_and_col_major_agree() {
        // Large enough that row-major and column-major visit pixels in very
        // different orders; the output must be identical either way.
        let g = grid(7, 5);
        let r90 = ids(&rotate90(&g, &Order::Row));
        let c90 = ids(&rotate90(&g, &Order::Col));
        let r180 = ids(&rotate180(&g, &Order::Row));
        let c180 = ids(&rotate180(&g, &Order::Col));
        assert_eq!(r90, c90);
        assert_eq!(r180, c180);

        // Source pixel at (row 0, col 0) is id 0; (row 4, col 6) is id 34.
        // 90: top-left -> top-right, bottom-right -> bottom-left.
        let d = rotate90(&g, &Order::Col);
        assert_eq!(dims(&d), (5, 7));
        assert_eq!(d.at(0, 4).unwrap().red, 0);
        assert_eq!(d.at(6, 0).unwrap().red, 34);
        // 180: top-left <-> bottom-right.
        let d = rotate180(&g, &Order::Col);
        assert_eq!(dims(&d), (7, 5));
        assert_eq!(d.at(4, 6).unwrap().red, 0);
        assert_eq!(d.at(0, 0).unwrap().red, 34);
    }

    #[test]
    fn rotate270_works() {
        // 2 5
        // 1 4
        // 0 3
        let g = grid(3, 2);
        for o in [Order::Row, Order::Col, Order::Block] {
            let d = rotate270(&g, &o);
            assert_eq!(dims(&d), (2, 3));
            assert_eq!(ids(&d), vec![2, 5, 1, 4, 0, 3]);
        }
    }

    #[test]
    fn transpose_works() {
        // 0 3
        // 1 4
        // 2 5
        let d = transpose(&grid(3, 2), &Order::Block);
        assert_eq!(dims(&d), (2, 3));
        assert_eq!(ids(&d), vec![0, 3, 1, 4, 2, 5]);
    }

    #[test]
    fn flip_horizontally_works() {
        // 2 1 0
        // 5 4 3
        let d = flip_horizontally(&grid(3, 2));
        assert_eq!(dims(&d), (3, 2));
        assert_eq!(ids(&d), vec![2, 1, 0, 5, 4, 3]);
    }

    #[test]
    fn flip_vertically_works() {
        // 3 4 5
        // 0 1 2
        let d = flip_vertically(&grid(3, 2));
        assert_eq!(dims(&d), (3, 2));
        assert_eq!(ids(&d), vec![3, 4, 5, 0, 1, 2]);
    }

    #[test]
    fn all_orders_agree_across_partial_tiles() {
        // Dimensions are not multiples of TILE, so the last tile in each
        // direction is partial.
        let g = grid(TILE * 2 + 5, TILE + 3);
        let expect90 = ids(&rotate90(&g, &Order::Row));
        let expect270 = ids(&rotate270(&g, &Order::Row));
        let expect180 = ids(&rotate180(&g, &Order::Row));
        let expect_t = ids(&transpose(&g, &Order::Row));
        for o in [Order::Col, Order::Block] {
            assert_eq!(ids(&rotate90(&g, &o)), expect90);
            assert_eq!(ids(&rotate180(&g, &o)), expect180);
            assert_eq!(ids(&rotate270(&g, &o)), expect270);
            assert_eq!(ids(&transpose(&g, &o)), expect_t);
        }
        // Spot-check one pixel: source (col 0, row 0) lands top-right after 90.
        let d = rotate90(&g, &Order::Block);
        assert_eq!(d.at(0, TILE + 2).unwrap().red, 0);
    }

    #[test]
    fn rotations_compose_to_identity() {
        let g = grid(4, 3);
        let orig = ids(&g);
        for o in [Order::Row, Order::Col, Order::Block] {
            let mut d = g.clone();
            for _ in 0..4 {
                d = rotate90(&d, &o);
            }
            assert_eq!(ids(&d), orig);
            assert_eq!(ids(&rotate270(&rotate90(&g, &o), &o)), orig);
            assert_eq!(ids(&rotate90(&rotate90(&g, &o), &o)), ids(&rotate180(&g, &o)));
            assert_eq!(ids(&rotate180(&rotate180(&g, &o), &o)), orig);
        }
    }

    #[test]
    fn flips_and_transpose_are_their_own_inverse() {
        let g = grid(4, 3);
        let orig = ids(&g);
        assert_eq!(ids(&flip_horizontally(&flip_horizontally(&g))), orig);
        assert_eq!(ids(&flip_vertically(&flip_vertically(&g))), orig);
        assert_eq!(ids(&transpose(&transpose(&g, &Order::Row), &Order::Block)), orig);
        // Both flips together are a 180 rotation.
        assert_eq!(
            ids(&flip_vertically(&flip_horizontally(&g))),
            ids(&rotate180(&g, &Order::Row))
        );
    }

    #[test]
    fn single_pixel_and_single_row() {
        let one = grid(1, 1);
        for d in [
            rotate90(&one, &Order::Row),
            rotate180(&one, &Order::Col),
            rotate270(&one, &Order::Col),
            transpose(&one, &Order::Block),
            flip_horizontally(&one),
            flip_vertically(&one),
        ] {
            assert_eq!(dims(&d), (1, 1));
            assert_eq!(ids(&d), vec![0]);
        }

        let row = grid(4, 1);
        assert_eq!(ids(&rotate90(&row, &Order::Row)), vec![0, 1, 2, 3]);
        assert_eq!(dims(&rotate90(&row, &Order::Row)), (1, 4));
        assert_eq!(ids(&flip_horizontally(&row)), vec![3, 2, 1, 0]);
        assert_eq!(ids(&flip_vertically(&row)), vec![0, 1, 2, 3]);
    }
}
