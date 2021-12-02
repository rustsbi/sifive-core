//! SiFive platform features
use crate::register::mfeature;

bitflags::bitflags! {
    /// Mask SiFive platform features
    pub struct Mask: usize {
        /// Disable data cache clock gating
        const DCACHE_CLOCK_GATING = 1 << 0;
        /// Disable instruction cache clock gating
        const ICACHE_CLOCK_GATING = 1 << 1;
        /// Disable pipeline clock gating
        const PIPELINE_CLOCK_GATING = 1 << 2;
        /// Disable speculative instruction cache refill
        const SPECULATIVE_ICACHE_REFILL = 1 << 3;
        /// Suppress corrupt signal on GrantData messages
        const CORRUPT_SIGNAL_GRANTDATA = 1 << 9;
        /// Disable short forward branch optimization
        const SHORT_FORWARD_BRANCH_OPTIMIZE = 1 << 16;
        /// Disable instruction cache next-line prefetcher
        const ICACHE_NEXT_LINE_PREFETCH = 1 << 17;
    }
}

/// Enable features on bootloading
///
/// Must run on M mode.
#[inline]
pub unsafe fn enable(flags: Mask) {
    mfeature::clear_features(flags)
}
