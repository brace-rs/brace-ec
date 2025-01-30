use bytemuck::TransparentWrapper;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, TransparentWrapper)]
#[repr(transparent)]
pub struct Nil([(); 0]);

impl Nil {
    pub fn new() -> Self {
        Self([])
    }

    pub fn r#ref() -> &'static Self {
        Self::wrap_ref(&[])
    }

    pub fn r#mut() -> &'static mut Self {
        Self::wrap_mut(&mut [])
    }
}
