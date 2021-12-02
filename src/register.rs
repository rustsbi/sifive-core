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
    use crate::feature::Mask;

    /// Clear corresponding bits in feature register
    #[inline]
    pub unsafe fn clear_features(flags: Mask) {
        asm!("csrrc 0x7C1, {}", in(reg) flags.bits())
    }
}
