//! Low level access to SiFive RISC-V processor cores
//! 
//! This crate provides:
//! 
//! - Access to core SiFive CSRs like bpm and feature disable;
//! - Access to assemble instructions like CEASE, PAUSE and cache control instructions;
//! - High level wrapper for handling SiFive platform features.

#![feature(asm, asm_const)]
#![no_std]

pub mod asm;
pub mod register;
pub mod feature;
