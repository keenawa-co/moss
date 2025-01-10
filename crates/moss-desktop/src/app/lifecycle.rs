#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LifecyclePhase {
    Starting = 1,
    Ready = 2,
    Restored = 3,
    Settled = 4,
}

impl From<u8> for LifecyclePhase {
    #[inline]
    fn from(value: u8) -> LifecyclePhase {
        unsafe { std::mem::transmute(value) }
    }
}

impl LifecyclePhase {
    #[inline]
    pub fn as_u8(self) -> u8 {
        self as u8
    }
}
