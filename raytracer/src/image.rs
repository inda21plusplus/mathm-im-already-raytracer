use crate::Vec4;

pub struct Image {
    pub data: Vec<Vec4>,
    pub width: usize,
    pub heigth: usize,
}

impl Image {
    pub fn new(data: Vec<Vec4>, width: usize, heigth: usize) -> Self {
        assert_eq!(data.len(), width * heigth);
        Self {
            data,
            width,
            heigth,
        }
    }
    pub fn get_raw_data(&self) -> Vec<u8> {
        self.data
            .iter()
            .map(|color| color.map(|b| (b * 255.).clamp(0., 255.) as u8))
            .map(|color| color.into_array())
            .flatten()
            .collect()
    }
}
