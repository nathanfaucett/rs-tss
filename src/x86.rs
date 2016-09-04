

#[derive(Debug)]
#[repr(packed)]
pub struct Tss {
    pub prev_tss: u32,
    pub sp0: usize,
    pub ss0: usize,
    pub sp1: usize,
    pub ss1: usize,
    pub sp2: usize,
    pub ss2: usize,
    pub cr3: usize,
    pub ip: usize,
    pub flags: usize,
    pub ax: usize,
    pub cx: usize,
    pub dx: usize,
    pub bx: usize,
    pub sp: usize,
    pub bp: usize,
    pub si: usize,
    pub di: usize,
    pub es: usize,
    pub cs: usize,
    pub ss: usize,
    pub ds: usize,
    pub fs: usize,
    pub gs: usize,
    pub ldt: usize,
    pub trap: u16,
    pub iomap_base: u16,
}
