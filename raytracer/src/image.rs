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
        self.data
            .iter()
            .map(|color| color.map(|b| (b * 255.).clamp(0., 255.) as u8))
            .map(|color| vek::Vec4::<u8>::new(color.x, color.y, color.z, 255))
            .map(|color| color.into_array())
            .flatten()
            .collect()
    }
}
