pub struct ArrayData([u8; 10000]);

impl ArrayData {
    pub fn new() -> Self {
        Self([0; 10000])
    }
}

impl Default for ArrayData {
    fn default() -> Self {
        Self::new()
    }
}

impl super::Data for ArrayData {
    fn get(&mut self, index: i32) -> &mut u8 {
        unsafe { self.0.get_unchecked_mut(index as usize) }
    }
}
