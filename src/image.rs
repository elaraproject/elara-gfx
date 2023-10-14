use std::{path::Path, io::Read};
use std::fs::File;
use std::io::Write;
use png;
use png::ColorType::*;
use std::ops::{Index, IndexMut};

#[derive(Clone, Debug)]
pub struct PixelArray {
    pub width: usize,
    pub height: usize,
    pub data: Vec<RGBA>
}

#[derive(Clone, Debug, Copy)]
pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

impl RGBA {
	pub fn new(r: u8, g: u8, b: u8, a: u8) -> RGBA {
		RGBA { r, g, b, a }
	}

	pub fn black() -> RGBA {
		RGBA { r: 0, g: 0, b: 0, a: 1 }
	}

	pub fn white() -> RGBA {
		RGBA { r: 255, g: 255, b: 255, a: 1 }
	}
}

#[derive(Debug)]
pub enum ImageError {
	JPEGError(jpeg_decoder::Error),
	IoError(std::io::Error),
	PNGError(png::DecodingError)
}

impl std::fmt::Display for ImageError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			ImageError::JPEGError(ref desc) => write!(f, "JPEG processing error: {}", desc),
			ImageError::IoError(ref err) => err.fmt(f)
		}
	}
}

impl std::error::Error for ImageError {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match *self {
			ImageError::JPEGError(ref err) => Some(err),
			ImageError::IoError(ref err) => Some(err)
		}
	}
}

impl From<std::io::Error> for ImageError {
	fn from(err: std::io::Error) -> ImageError {
		ImageError::IoError(err)
	}
}

impl From<jpeg_decoder::Error> for ImageError {
	fn from(err: jpeg_decoder::Error) -> ImageError {
		ImageError::JPEGError(err)
	}
}

impl From<png::DecodingError> for ImageError {
	fn from(err: png::DecodingError) -> ImageError {
		ImageError::PNGError(err)
	}
}

impl PixelArray {
    pub fn load_png_from_path<T: AsRef<Path>>(path: T) -> Result<PixelArray, ImageError> {
        Self::load_png(File::open(path)?)
    }

    pub fn load_jpg_from_path<T: AsRef<Path>>(path: T) -> Result<PixelArray, ImageError> {
    	Self::load_jpg(File::open(path)?)
    }

    pub fn load_jpg<R: Read>(r: R) -> Result<PixelArray, ImageError> {
    	let mut decoder = jpeg_decoder::Decoder::new(r);
    	let data = decoder.decode()?;
    	let metadata = decoder.info().unwrap();

    	let mut pixels = Vec::new();
        for i in (0..data.len()).step_by(3) {
            let pixel = RGBA { r: data[i], g: data[i + 1], b: data[i + 2], a: 1 };
            pixels.push(pixel);
        }

        Ok(PixelArray {
        	width: metadata.width as usize,
        	height: metadata.height as usize,
        	data: pixels
        })
    }

    pub fn load_png<R: Read>(r: R) -> Result<PixelArray, ImageError> {
        let mut decoder = png::Decoder::new(r);
        decoder.set_transformations(png::Transformations::normalize_to_color8());
        let mut reader = decoder.read_info()?;
        let mut img_data = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut img_data)?;
    
        // Convert RGB and Grayscale to RGBA by adding 1 for alpha channel
        let data = match info.color_type {
            Rgb => {
                let mut vec = Vec::with_capacity(img_data.len() * 3);
                for rgb in img_data.chunks(3) {
                    let r = rgb[0];
                    let g = rgb[1];
                    let b = rgb[2];
                    vec.extend([r, g, b, 1].iter().cloned())
                }
                vec
            }
            Rgba => img_data,
            Grayscale =>
                {
                    let mut vec = Vec::with_capacity(img_data.len() * 3);
                    for g in &img_data {
                        vec.extend([g, g, g, &1].iter().cloned())
                    }
                    vec
                },
            GrayscaleAlpha =>
                {
                    let mut vec = Vec::with_capacity(img_data.len() * 3);
                    for ga in img_data.chunks(2) {
                        let g = ga[0];
                        let a = ga[1];
                        vec.extend([g, g, g, a].iter().cloned())
                    }
                    vec
                },
            _ => unreachable!("uncovered color type"),
        };
        
        let mut pixels = Vec::new();
        for i in (0..data.len()).step_by(4) {
            let pixel = RGBA { r: data[i], g: data[i + 1], b: data[i + 2], a: data[i + 3] };
            pixels.push(pixel);
        }
        Ok(PixelArray {
            width: info.width as usize,
            height: info.height as usize,
            data: pixels
        })
    }

    pub fn new(width: usize, height: usize) -> PixelArray {
        PixelArray { 
            width,
            height,
            data: vec![RGBA::black(); width * height] }
    }
    
    pub fn len(&self) -> usize {
        self.data.len()
    }
    
    pub fn data(&self) -> Vec<u8> {
        let mut flattened_data = Vec::new();
        for i in 0..self.height {
            for j in 0..self.width {
                let pixel = self[[i, j]];
                let r = pixel.r;
                let g = pixel.g;
                let b = pixel.b;
                let a = pixel.a;
                flattened_data.push(r);
                flattened_data.push(g);
                flattened_data.push(b);
                flattened_data.push(a);
            }
        }
        flattened_data
    }
    
    pub fn flipv(&mut self) {
        let mut new_data = self.clone();
        for i in 0..self.height {
            for j in 0..self.width {
                new_data[[i, j]] = self[[(self.height - 1 - i), j]];
            }
        }
        self.data = new_data.data;
    }

	// User-facing function - it is highly recommended to use
	// set() to change the value of a particular pixel as it
	// uses the conventional (x, y) order instead of the reversed
	// (row, column) - that is (y, x) - order used by Index
    pub fn set(&mut self, x: usize, y: usize, color: RGBA) {
    	self[[y, x]] = color;
    }
    
    // TODO: add more methods from https://github.com/ankitaS11/pyImageEdits
    
    pub fn write_ppm(&self) -> String {
        let mut data_str = String::new();
        let flattened_data = self.data();
        for i in (0..flattened_data.len()).step_by(4) {
            for j in 0..3 {
                data_str += &(flattened_data[i + j].to_string() + " ");
            }
        }
        format!("P3\n# Created by elara-gfx\n{} {}\n255\n{}", self.width, self.height, data_str)
    }

    pub fn save_as_ppm<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let mut output = File::create(path)?;
        write!(output, "{}", self.write_ppm())?;
        Ok(())
    }
}

impl Index<[usize; 2]> for PixelArray {
    type Output = RGBA;
    
    fn index(&self, index: [usize; 2]) -> &Self::Output {
        let [i, j] = index;
        if i >= self.height || j >= self.width {
            panic!("Image index [{}, {}] out of bounds, dimensions [{}, {}]", i, j, self.height, self.width);
        }
        
        &self.data[(self.width * i) + j]
    }
}

impl IndexMut<[usize; 2]> for PixelArray {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        let [i, j] = index;
        if i >= self.height || j >= self.width {
            panic!("Image index [{}, {}] out of bounds, dimensions [{}, {}", i, j, self.height, self.width);
        }
        
        &mut self.data[(self.width * i) + j]
    }
}
