extern crate image;
extern crate num;
extern crate argparse;

use std::fs::File;
use std::path::Path;

use num::Complex;

use argparse::{ArgumentParser, Store};

fn main() {
    
    let mut max_iter = 256u16;
    let mut imgsize = 800;
    let mut scale = 4.0;
    let (mut tx, mut ty) = (0.0, 0.0);
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Generate a Julia fractal.");
        ap.refer(&mut max_iter).add_argument("iterations", Store,
                                             "The maximum number of iterations per pixel.");
        ap.refer(&mut imgsize).add_argument("size", Store, "The size of the image on on side.");
        ap.refer(&mut scale).add_argument("scale", Store, "\"Zoom factor\" for generation");
        ap.refer(&mut tx).add_argument("tx", Store, "Translation in the X dimension");
        ap.refer(&mut ty).add_argument("ty", Store, "Translation in the Y dimension");
        ap.parse_args_or_exit();
    }
    let (scalex, scaley) = (scale/imgsize as f32, scale/imgsize as f32);
 
    let mut imgbuf = image::ImageBuffer::new(imgsize,imgsize);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Center the image
        let cy = y as f32 * scaley - (scale * 0.5) + tx / 100.0;
        let cx = x as f32 * scalex - (scale * 0.5) + ty / 100.0;

        let mut z = Complex::new(cx, cy);
        let c = Complex::new(-0.4, 0.6);


        let mut i = 0;

        for t in 0..max_iter {
            if z.norm() > 2.0 {
                break
            }
            z = z * z + c;
            i = t;
        }

        *pixel = image::Luma([i as u8]);
    }

    let ref mut fout = File::create(&Path::new("fractal.png")).unwrap();

    let _ = image::ImageLuma8(imgbuf).save(fout, image::PNG);
}
