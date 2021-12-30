//! Platform specific SiFive CSRs

/// Branch prediction mode register
///
/// This SiFive custom extension adds an M-mode CSR to control the current branch prediction mode, bpm at CSR 0x7C0.
///
/// Depending on platform, the Core Complex’s branch prediction system may include a Return Address Stack (RAS),
/// a Branch Target Buffer (BTB), and a Branch History Table (BHT).
///
/// While branch predictors are essential to achieve high performance in pipelined processors, they can also cause
/// undesirable timing variability for hard real-time systems. The bpm register provides a means to customize
/// the branch predictor behavior to trade average performance for a more predictable execution time
pub mod mbpm {
    use core::arch::asm;
    use bit_field::BitField;
    /// Branch prediction mode register
    #[derive(Clone, Copy, Debug)]
    #[repr(transparent)]
    pub struct Mbpm {
        bits: usize,
    }
    impl Mbpm {
        /// Branch-Direction Prediction. Determines the value returned by the BHT component of the branch prediction system.
        ///
        /// A zero value indicates dynamic direction prediction, and a non-zero value indicates static-taken direction prediction.
        ///
        /// The BTB is cleared on any write to bdp, and the RAS is unaffected by writes to bdp.
        #[inline]
        pub fn bdp(&self) -> bool {
            self.bits.get_bit(0)
        }
    }
    /// Reads the register
    #[inline]
    pub fn read() -> Mbpm {
        let bits: usize;
        unsafe { asm!("csrr {}, 0x7C0", out(reg) bits) };
        Mbpm { bits }
    }
    /// Set mode to dynamic direction prediction.
    #[inline]
    pub unsafe fn clear_bdp() {
        asm!("csrrci 0x7C0, 0")
    }
    /// Set mode to static-taken direction prediction.
    #[inline]
    pub unsafe fn set_bdp() {
        asm!("csrrsi 0x7C0, 0")
    }
}

#[doc(hidden)] // hide by now, API has not been decided yet
/// Feature disable register
///
/// The SiFive custom M-mode Feature Disable CSR is provided to enable or disable certain
/// microarchitectural features.
///
/// A feature is fully enabled when the associated bit is zero. If a particular core does not support
/// the disabling of a feature, the corresponding bit is hardwired to zero.
///
/// On reset, all implemented bits are set to 1, disabling all features. The bootloader is responsible
/// for turning on all required features and can simply write zero to turn on the maximal set of features.
/// SiFive’s Freedom Metal bootloader handles turning on these features; when using a custom bootloader,
/// clearing the Feature Disable CSR must be implemented.
///
/// Note that arbitrary toggling of the Feature Disable CSR bits is neither recommended nor supported;
/// they are only intended to be set from 1 to 0. A particular Feature Disable CSR bit is only to be
/// used in a very limited number of situations, as detailed in the Example Usage entry in tables
/// on documentation of each core.
pub mod mfeature {
    use core::arch::asm;
    use crate::feature::Mask;

    /// Clear corresponding bits in feature register
    #[inline]
    pub unsafe fn clear_features(flags: Mask) {
        asm!("csrrc 0x7C1, {}", in(reg) flags.bits())
    }
}

/// Rnmi scratch register
///
/// The mnscratch CSR holds a 64-bit read-write register, which enables the NMI trap handler
/// to save and restore the context that was interrupted.
pub mod mnscratch {
    use core::arch::asm;
    /// Reads the `mnscratch` register
    #[inline]
    pub fn read() -> usize {
        let ans: usize;
        unsafe { asm!("csrr {}, 0x351", out(reg) ans) };
        ans
    }
    /// Writes the `mnscratch` register
    #[inline]
    pub unsafe fn write(data: usize) {
        asm!("csrw 0x351, {}", in(reg) data)
    }
}

/// Rnmi exception program counter register
///
/// The mnepc CSR is a 64-bit read-write register, which, on entry to the NMI trap handler,
/// holds the PC of the instruction that took the interrupt. The lowest bit of mnepc is
/// hardwired to zero.
pub mod mnepc {
    use core::arch::asm;
    /// Reads the `mnepc` register
    #[inline]
    pub fn read() -> usize {
        let ans: usize;
        unsafe { asm!("csrr {}, 0x351", out(reg) ans) };
        ans
    }
}

/// Rnmi cause register
///
/// The mncause CSR holds the reason for the NMI, with bit 63 set to 1, and the NMI cause
/// encoded in the least-significant bits, or zero if NMI causes are not supported.
///
/// The lower bits of mncause, defined as the exception_code, are as follows:
///
/// | mncause | NMI Cause | Function |
/// |:--------|:----------|:---------|
/// | 1 | *Reserved* | *Reserved* |
/// | 2 | RNMI input pin | External `rnmi_N` input |
/// | 3 | Bus error | RNMI caused by BEU |
pub mod mncause {
    use core::arch::asm;
    /// NMI causes
    #[repr(usize)]
    pub enum Nmi {
        RnmiInput = 2,
        BusError = 3,
    }

    /// Check if NMI cause is supported
    #[inline]
    pub fn is_supported() -> bool {
        let ans: usize;
        unsafe { asm!("csrr {}, 0x352", out(reg) ans) };
        ans != 0
    }

    /// Reads the NMI cause, or None if not supported
    #[inline]
    pub fn exception_code() -> Option<Nmi> {
        let ans: usize;
        unsafe { asm!("csrr {}, 0x352", out(reg) ans) };
        match ans {
            2 => Some(Nmi::RnmiInput),
            3 => Some(Nmi::BusError),
            _ => None,
        }
    }
}

/// Rnmi status register
///
/// The mnstatus CSR holds a two-bit field, which, on entry to the trap handler,
/// holds the privilege mode of the interrupted context encoded in the same manner
/// as mstatus.mpp.
pub mod mnstatus {}
