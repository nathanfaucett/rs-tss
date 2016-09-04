

#[derive(Debug)]
#[repr(packed)]
pub struct Tss {
    pub reserved1: u32,
    pub sp0: usize,
    pub sp1: usize,
    pub sp2: usize,
    pub reserved2: u32,
    pub reserved3: u32,
    pub ist1: usize,
    pub ist2: usize,
    pub ist3: usize,
    pub ist4: usize,
    pub ist5: usize,
    pub ist6: usize,
    pub ist7: usize,
    pub reserved4: u32,
    pub reserved5: u32,
    pub reserved6: u16,
    pub iomap_base: u16,
}
