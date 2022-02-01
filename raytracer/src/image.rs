use crate::Vec3;

pub struct Image {
    pub data: Vec<Vec3>,
    pub width: usize,
    pub height: usize,
}

impl Image {
    pub fn new(data: Vec<Vec3>, width: usize, height: usize) -> Self {
        assert_eq!(data.len(), width * height);
        Self {
            data,
            width,
            height,
        }
    }
    pub fn get_raw_data(&self) -> Vec<u8> {
        let mut output = vec![0; self.width * self.height * 4];
        for (i, pixel) in self.data.iter().enumerate() {
            output[i * 4 + 0] = (pixel.x * 255.).clamp(0., 255.) as u8;
            output[i * 4 + 1] = (pixel.y * 255.).clamp(0., 255.) as u8;
            output[i * 4 + 2] = (pixel.z * 255.).clamp(0., 255.) as u8;
            output[i * 4 + 3] = 255;
        }
        output
    }
}
