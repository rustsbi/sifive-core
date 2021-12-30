//! Low level access to SiFive RISC-V processor cores
//!
//! This crate provides:
//!
//! - Access to core SiFive CSRs like bpm and feature disable;
//! - Access to assemble instructions like CEASE and cache control instructions;
//! - High level wrapper for handling SiFive platform features.
#![no_std]

pub mod asm;
#[doc(hidden)] // hide by now, API has not been decided yet
pub mod feature;
pub mod register;
