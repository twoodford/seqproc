//! This provides methods for image sequence processing

extern crate image;
extern crate threadpool;
use image::DynamicImage;
use image::Pixel;
use threadpool::ThreadPool;
use std::fs;
use std::path::Path;
use std::sync::mpsc::channel;

pub struct ImageSequence {
    pub images: Vec<DynamicImage>,
}

pub fn load_seq_directory<'a>(dir: &Path) -> Result<ImageSequence, image::ImageError> {
    let paths = try!(fs::read_dir(dir));
    let mut imgs = Vec::with_capacity(paths.size_hint().0);
    print!("Load images:\n");
    for path in paths {
        let mypath = try!(path).path();
        match mypath.to_str() {
            Some(p) => print!("{}\n", p),
            None => print!(""),
        }
        let img = image::open(mypath);
        match img {
            Ok(v) => imgs.push(v),
            Err(e) => return Result::Err(e),
        }
    }
    Result::Ok(ImageSequence{ images: imgs })
}

/*pub fn load_seq_directory_parallel(dir: &Path, thpool: &ThreadPool) -> Result<ImageSequence, image::ImageError> {
    let paths = try!(fs::read_dir(dir));
    let (tx, rx) = channel();
    let (etx, erx) = channel();
    for path in paths {
        let tx = tx.clone();
        let path2 = try!(path).path();
        thpool.execute(move|| {
            let img = image::open(path2);
            match img {
                Ok(v) => tx.send(v),
                Err(e) => etx.send(e),
            }
        });
    }
    // This is a bit counterintuitive
    // If the error channel returns an error because everyone hung up w/o an error, we've been 
    // successful.  If it has been successful, an error occurred while opening an image, and we'll 
    // return the first one as soon as it comes
    match erx.recv() {
        Ok(v) => Err(v),
        Err(e) => Result::Ok(ImageSequence{ images: rx.iter().collect::<Vec<DynamicImage>>() })
    }
}*/


fn average_luma<I, P>(img: &I) -> f64
    where I: image::GenericImage<Pixel=P>,
          P: image::Pixel<Subpixel=u8> {
    let mut lsum: f64 = 0f64;
    for pix in img.pixels() {
        lsum += pix.2.to_luma()[0] as f64;
    }
    return lsum / (img.pixels().count() as f64);
}

pub fn get_seq_luma<'a>(imgseq: ImageSequence) -> Vec<f64> {
    imgseq.images.iter().map(|img| average_luma(img)).collect::<Vec<f64>>()
}
