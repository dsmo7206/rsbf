mod array;
mod resizable;

pub use array::ArrayData;
pub use resizable::ResizableData;

pub trait Data {
    fn get(&mut self, index: i32) -> &mut u8;
}
