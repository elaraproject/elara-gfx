// Basic functionality to support OpenGL rendering to a surface

#[derive(Debug)]
pub struct PixelArray {
    width: i32,
    height: i32,
    channels: i32,
    data: Vec<u8>
}

impl PixelArray {
    pub fn new(width: i32, height: i32) -> PixelArray {
        PixelArray { 
            width: width,
            height: height,
            channels: 4, 
            data: Vec::new() }
    }

    pub fn add_data(&mut self, data: &[u8]) {
        self.data = data.to_vec();
    }

    pub fn write_ppm(&self) -> String {
        let mut data_str = String::new();
        for i in (0..self.data.len()).step_by(4) {
            for j in 0..3 {
                data_str += &(self.data[i + j].to_string() + " ");
            }
        }
        format!("P3\n# Created by elara-gfx\n{} {}\n255\n{}", self.width, self.height, data_str)
    }

    pub fn save_as_ppm(&self, path: PathBuf) -> GfxResult {
        let mut output = File::create(path)?;
        write!(output, "{}", self.write_ppm())?;
        Ok(())
    }
}
