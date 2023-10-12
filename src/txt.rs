use const_format::formatcp;
use image::{imageops::FilterType, io::Reader as ImageReader};
use rayon::prelude::*;
use std::{fs, path::Path};

const IMAGE_DIR: &str = "images";
const TXT_DIR: &str = "txts";

const SYMBOL: &str = "█";

pub fn generate_txt(force_rebuild: bool, width: u32, height: u32) {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(128)
        .build()
        .unwrap();

    fs::create_dir_all(TXT_DIR).ok();

    pool.install(|| {
        fs::read_dir(IMAGE_DIR)
            .unwrap()
            .par_bridge()
            .for_each(|entry| {
                let image_path = entry.unwrap().path();
                let txt_path = Path::new(image_path.file_name().unwrap()).with_extension("txt");
                let txt_path = Path::new(TXT_DIR).join(txt_path.with_extension("txt"));

                // print!("{} ", image_path.display());

                if txt_path.exists() && !force_rebuild {
                    return;
                }

                let image = ImageReader::open(&image_path).unwrap().decode().unwrap();
                let image = image.resize(width, height, FilterType::Lanczos3);

                let txt: Vec<_> = image
                    .as_rgb8()
                    .unwrap()
                    .pixels()
                    .enumerate()
                    .par_bridge()
                    .map(|(idx, pixel)| {
                        let (r, g, b) = (pixel[0] as f64, pixel[1] as f64, pixel[2] as f64);
                        let bright = r * 0.2989 + g * 0.5870 + b * 0.1140;

                        let (is_white, should_newline) =
                            (bright <= 128.0, idx % image.width() as usize == 0);

                        let symbol = match (is_white, should_newline) {
                            (true, true) => formatcp!("{SYMBOL}\n"),
                            (true, false) => formatcp!("{SYMBOL}"),
                            (false, true) => formatcp!(" \n"),
                            (false, false) => formatcp!(" "),
                        };

                        symbol.as_bytes()
                    })
                    .flatten()
                    .cloned()
                    .collect();

                fs::write(txt_path, txt).unwrap();
            });
    });
}
