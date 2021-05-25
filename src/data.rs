pub trait Data {
    fn get(&mut self, index: i32) -> &mut u8;
}

const RESIZABLE_DATA_EXTEND_SIZE: usize = 1024;

pub struct ResizableData {
    pos: Vec<u8>,
    neg: Vec<u8>,
}

impl ResizableData {
    pub fn new() -> Self {
        Self {
            pos: vec![],
            neg: vec![],
        }
    }
}

impl Data for ResizableData {
    fn get(&mut self, index: i32) -> &mut u8 {
        let (v, index) = if index >= 0 {
            (&mut self.pos, index as usize)
        } else {
            (&mut self.neg, (-index) as usize - 1)
        };

        if index >= v.len() {
            v.resize(index + RESIZABLE_DATA_EXTEND_SIZE, 0);
        }

        &mut v[index]
    }
}
