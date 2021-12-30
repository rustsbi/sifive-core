//! Assembly instructions
//!
//! # Absence of PAUSE instruction
//!
//! SiFive cores may have PAUSE instruction implemented, which is also defined in RISC-V extension
//! Zihintpause. This extension is adapted to [`core::hint::spin_loop()`] function in Rust core crate,
//! and thus not implemented separately in platform specific assembly instruction module.
//!
//! On hardware implementation of SiFive platform, PAUSE instruction causes a stall of
//! up to 32 cycles or until a cache eviction occurs, whichever comes first.
//!
//! [`core::hint::spin_loop()`]: https://doc.rust-lang.org/stable/core/hint/fn.spin_loop.html
use core::arch::asm;

/// CEASE, core halt instruction
///
/// This function will never return and will immediately cease the current hart.
///
/// # Unsafety
///
/// Calling this function is unsafe, because the resource this function uses
/// is not freed (i.e. `fn drop` in Drop trait is not called) after current hart ceased.
/// Caller must ensure that all resources are freed before invoking CEASE instruction.
///
/// # Privilege mode permissions
///
/// This is a privileged instruction and it's only available in M-mode.
///
/// # Example
///
/// ```no_run
/// #[panic_handler]
/// fn machine_panic(info: &PanicInfo) -> ! {
///     print_stack_trace_and_information(info);
///     unsafe { sifive_core::asm::cease() }
/// }
/// ```
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
    // opcode: 0x30500073
    asm!(".insn i 0x73, 0, x0, x0, 0x305", options(noreturn))
}

/// CFLUSH.D.L1 x0, L1 data cache full-cache flush instruction
///
/// This instruction writes back and invalidates all lines in the L1 data cache.
///
/// # Privilege mode permissions
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
///
/// # Hardware implmenetaion
///
/// Implemented as state machine in L1 data cache, for cores with data caches.
#[inline]
pub fn cflush_d_l1_all() {
    // opcode: 0xFC000073
    unsafe { asm!(".insn i 0x73, 0, x0, x0, -0x40") }
}

/// CFLUSH.D.L1 rs1, L1 data cache flush virtual address instruction
///
/// This instruction writes back and invalidates the L1 data cache line containing
/// the virtual address in integer register rs1.
///
/// # Privilege mode permissions
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
///
/// # Hardware implmenetaion
///
/// Implemented as state machine in L1 data cache, for cores with data caches.
#[inline]
pub fn cflush_d_l1_va(va: usize) {
    // opcode: 0xFC000073 + (rs1 << 15)
    unsafe { asm!(".insn i 0x73, 0, x0, {}, -0x40", in(reg) va) }
}

/// CDISCARD.D.L1 x0, L1 data cache full-cache invalidate instruction
///
/// This instruction invalidates, but does not write back, all lines in the L1 data cache.
/// Dirty data within the cache is lost.
///
/// # Privilege mode permissions
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
///
/// # Hardware implmenetaion
///
/// Implemented as state machine in L1 data cache, for cores with data caches.
#[inline]
pub fn cdiscard_d_l1_all() {
    // opcode: 0xFC200073
    unsafe { asm!(".insn i 0x73, 0, x0, x0, -0x3E") }
}

/// CDISCARD.D.L1 rs1, L1 data cache invalidate virtual address instruction
///
/// This instruction invalidates, but does not write back, the L1 data cache line containing
/// the virtual address in integer parameter `va`.
/// Dirty data within the cache line is lost.
///
/// # Privilege mode permissions
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
///
/// # Hardware implmenetaion
///
/// Implemented as state machine in L1 data cache, for cores with data caches.
#[inline]
pub fn cdiscard_d_l1_va(va: usize) {
    // opcode: 0xFC200073 + (rs1 << 15)
    unsafe { asm!(".insn i 0x73, 0, x0, {}, -0x3E", in(reg) va) }
}

/// MNRET, non-maskable interrupt return instruction
///
/// This M-mode only instruction uses the values in `mnepc` and `mnstatus` to return
/// to the program counter and privileged mode of the interrupted context, respectively.
/// This instruction also sets the internal `rnmie` state bits.
#[inline]
pub unsafe fn mnret() -> ! {
    // opcode: 0x70200073
    asm!(".insn i 0x73, 0, x0, x0, 0x702", options(noreturn))
}
