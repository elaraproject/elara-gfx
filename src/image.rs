use std::path::Path;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use png;
use png::ColorType::*;

#[derive(Debug)]
pub struct PixelArray {
    pub width: usize,
    pub height: usize,
    pub channels: usize,
    pub data: Vec<u8>
}

impl PixelArray {
    pub fn load_png<T: AsRef<Path>>(path: T) -> std::io::Result<PixelArray> {
        let mut decoder = png::Decoder::new(File::open(path)?);
        decoder.set_transformations(png::Transformations::normalize_to_color8());
        let mut reader = decoder.read_info()?;
        let mut img_data = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut img_data)?;
    
        let (data, channels) = match info.color_type {
            Rgb => (img_data, 3),
            Rgba => (img_data, 4),
            Grayscale => (
                {
                    let mut vec = Vec::with_capacity(img_data.len() * 3);
                    for g in &img_data {
                        vec.extend([g, g, g].iter().cloned())
                    }
                    vec
                }, 3),
            GrayscaleAlpha => (
                {
                    let mut vec = Vec::with_capacity(img_data.len() * 3);
                    for ga in img_data.chunks(2) {
                        let g = ga[0];
                        let a = ga[1];
                        vec.extend([g, g, g, a].iter().cloned())
                    }
                    vec
                }, 4),
            _ => unreachable!("uncovered color type"),
        };
        
        Ok(PixelArray {
            width: info.width as usize,
            height: info.height as usize,
            channels,
            data: data
        })
    }

    pub fn new(width: usize, height: usize) -> PixelArray {
        PixelArray { 
            width: width,
            height: height,
            channels: 4, 
            data: vec![0; width * height * 4] }
    }

    pub fn set_data(&mut self, data: &[u8]) {
        self.data = data.to_vec();
    }
    
    pub fn len(&self) -> usize {
        self.data.len()
    }
    
    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }
    
    // Not working rn
    pub fn flipv(&mut self) {
        for x in 0..(self.width * 4) {
            for y in 0..(self.height * 4) {
                self.data[x * self.width + y] = self.data[x * self.width + (self.height - 1 - y)];
            }
        }
    }
    
    // pub fn data_as_ptr(&self) -> 

    pub fn write_ppm(&self) -> String {
        let mut data_str = String::new();
        for i in (0..self.data.len()).step_by(self.channels as usize) {
            for j in 0..(self.channels - 1) {
                data_str += &(self.data[i + j].to_string() + " ");
            }
        }
        format!("P3\n# Created by elara-gfx\n{} {}\n255\n{}", self.width, self.height, data_str)
    }

    pub fn save_as_ppm(&self, path: PathBuf) -> std::io::Result<()> {
        let mut output = File::create(path)?;
        write!(output, "{}", self.write_ppm())?;
        Ok(())
    }
}
