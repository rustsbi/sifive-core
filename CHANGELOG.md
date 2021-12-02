# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.2] - 2021-12-02
### Added
- Explanation of current state of PAUSE instruction
- Clearify that feature disable must run on M mode 

### Modified
- Set CEASE to be unsafe function
- Rearrange documents

### Removed
- PAUSE instruction, it will be in `core::hint::spin_loop`

## [0.0.1] - 2021-12-02
### Added
- Assembly instruction CEASE, PAUSE, CFLUSH.D.L1 and CDISCARD.D.L1
- CSR register mbpm and mfeature
- Feature enable API and platform feature mask
- Documentations, hardware notes for instructions and registers

[Unreleased]: https://github.com/olivierlacan/keep-a-changelog/compare/v0.0.2...HEAD
[0.0.2]: https://github.com/luojia65/sifive-core/releases/tag/v0.0.2
