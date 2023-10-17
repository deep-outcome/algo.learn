mod auxies;
mod lex_sort;
mod mem_eff_lex_sort;
mod poly_sort;

mod consts {
    pub const SIG_BIT_MASK: u32 = 0b_1000_0000;
    pub const EXP_MASK: u32 = 0b_0111_1111;
}

pub type FPoint = u32;
