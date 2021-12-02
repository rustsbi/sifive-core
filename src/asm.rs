//! Assembly instructions

const RS1_SHIFT: usize = 15;
const XREG_A0: usize = 10;

/// CEASE, core halt instruction
/// 
/// This function will never return and will immediately cease the current hart.
/// 
/// # Privilege mode perimissions
/// 
/// This is a privileged instruction and it's only available in M-mode.
/// 
/// # Hardware implmenetaion
///
/// After retiring CEASE, hart will not retire another instruction until reset.
/// 
/// Instigates power-down sequence, which will eventually raise the `cease_from_tile_N` signal 
/// to the outside of the Core Complex, indicating that it is safe to power down.
/// 
/// CEASE has no effect on System Bus Access.
/// 
/// Debug `haltreq` will not work after a CEASE instruction has retired.
#[inline]
pub unsafe fn cease() -> ! {
    asm!(".word 0x30500073", options(noreturn))
}

/// PAUSE, spin-wait loop idle instruction
/// 
/// This instruction may be used for more efficient idling in spin-wait loops.
/// 
/// PAUSE is a FENCE instruction with predecessor set W and null successor set. 
/// Therefore, PAUSE is a HINT instruction that executes as a no-op on all RISC-V implementations.
/// 
/// # Hardware implmenetaion
/// 
/// This instruction causes a stall of up to 32 cycles or until a cache eviction occurs, whichever comes first.
#[inline]
pub fn pause() {
    unsafe { 
        asm!(".word 0x0100000F")
    }
}

/// CFLUSH.D.L1 x0, L1 data cache full-cache flush instruction
/// 
/// This instruction writes back and invalidates all lines in the L1 data cache.
/// 
/// # Hardware implmenetaion
/// 
/// Implemented as state machine in L1 data cache, for cores with data caches.
/// 
/// # Privilege mode perimissions
/// 
/// Only available in M-mode.
/// 
/// # Platform support
/// 
/// CFLUSH.D.L1 full-cache flush instruction is supported by all SiFive® Performance™ cores,
/// all SiFive® Intelligence™ cores, and SiFive® Essential™ U7, U5, S7 and E7 cores.
/// 
// supported by U74, U74-MC, U54, U54-MC, S76, S76-MC, E76 and E76-MC
// *not* supported by S54, S51, S21, E34, E31, E24, E21 and E20 cores
// 
/// If this instruction is not supported by current platform, an illegal-instruction exception is raised.
#[inline]
pub fn cflush_d_l1_all() {
    unsafe { 
        asm!(".word 0xFC000073")
    }
}

/// CFLUSH.D.L1 rs1, L1 data cache flush virtual address instruction
/// 
/// This instruction writes back and invalidates the L1 data cache line containing
/// the virtual address in integer register rs1.
/// 
/// # Hardware implmenetaion
/// 
/// Implemented as state machine in L1 data cache, for cores with data caches.
/// 
/// # Privilege mode perimissions
/// 
/// Only available in M-mode.
/// 
/// # Exceptions
/// 
/// If the effective privilege mode does not have write permissions to the address in rs1, then
/// a store access or store page-fault exception is raised.
/// 
/// If the address in rs1 is in an uncacheable region with write permissions, the instruction has
/// no effect but raises no exceptions.
/// 
/// Note that if the PMP scheme write-protects only part of a cache line, then using a value for
/// rs1 in the write-protected region will cause an exception, whereas using a value for rs1 in
/// the write-permitted region will write back the entire cache line.
/// 
/// # Platform support
/// 
/// CFLUSH.D.L1 with rs1 ≠ x0 is supported by SiFive® Performance™ P550, P550-MC, 
/// SiFive® Essential™ S76, S76-MC, E76 and E76-MC cores.
/// 
// CFLUSH.D.L1 with rs1 ≠ x0 is *not* supported on SiFive® Performance™ P270, P270-MC, 
// SiFive® Intelligence™ X280, X280-MC,
// SiFive® Essential™ U74, U74-MC, U54, U54-MC, S54, S51, S21, E34, E31, E24, E21 and E20 cores.
// 
/// If this instruction is not supported by current platform, an illegal-instruction exception is raised.
#[inline]
pub fn cflush_d_l1_va(va: usize) {
    unsafe { 
        asm!(".word {}", const 0xFC000073 + (XREG_A0 << RS1_SHIFT), in("a0") va)
    }
}

/// CDISCARD.D.L1 x0, L1 data cache full-cache invalidate instruction
/// 
/// This instruction invalidates, but does not write back, all lines in the L1 data cache.
/// Dirty data within the cache is lost.
/// 
/// # Hardware implmenetaion
/// 
/// Implemented as state machine in L1 data cache, for cores with data caches.
/// 
/// # Privilege mode perimissions
/// 
/// Only available in M-mode.
/// 
/// # Platform support
/// 
/// CDISCARD.D.L1 full-cache invalidate instruction is supported by all SiFive® Performance™ cores,
/// all SiFive® Intelligence™ cores, and SiFive® Essential™ U7, U5, S7 and E7 cores.
/// 
// supported by U74, U74-MC, U54, U54-MC, S76, S76-MC, E76 and E76-MC
// *not* supported by S54, S51, S21, E34, E31, E24, E21 and E20 cores
// 
/// If this instruction is not supported by current platform, an illegal-instruction exception is raised.
#[inline]
pub fn cdiscard_d_l1_all() {
    unsafe { 
        asm!(".word 0xFC200073")
    }
}

/// CDISCARD.D.L1 rs1, L1 data cache invalidate virtual address instruction
/// 
/// This instruction invalidates, but does not write back, the L1 data cache line containing 
/// the virtual address in integer parameter `va`. 
/// Dirty data within the cache line is lost.
/// 
/// # Hardware implmenetaion
/// 
/// Implemented as state machine in L1 data cache, for cores with data caches.
/// 
/// # Privilege mode perimissions
/// 
/// Only available in M-mode.
/// 
/// # Exceptions
/// 
/// If the effective privilege mode does not have write permissions to the address in rs1, then
/// a store access or store page-fault exception is raised.
/// 
/// If the address in rs1 is in an uncacheable region with write permissions, the instruction has
/// no effect but raises no exceptions.
/// 
/// Note that if the PMP scheme write-protects only part of a cache line, then using a value for
/// rs1 in the write-protected region will cause an exception, whereas using a value for rs1 in
/// the write-permitted region will invalidate and discard the entire cache line.
/// 
/// # Platform support
/// 
/// CDISCARD.D.L1 virtual address cache invalidate instruction is supported by all SiFive® Performance™ cores,
/// all SiFive® Intelligence™ cores, and SiFive® Essential™ U7, U5, S7 and E7 cores.
///
/// If this instruction is not supported by current platform, an illegal-instruction exception is raised.
#[inline]
pub fn cdiscard_d_l1_va(va: usize) {
    unsafe { 
        asm!(".word {}", const 0xFC200073 + (XREG_A0 << RS1_SHIFT), in("a0") va)
    }
}
