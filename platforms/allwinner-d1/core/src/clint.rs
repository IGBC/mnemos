use d1_pac::CLINT;

/// Core Local Interruptor (ClINT) interface.
///
/// The implementation in the (single) C906 core of the D1 provides timer functionalities
/// that can be accessed from machine and supervisor mode.
pub struct Clint {
    clint: CLINT,
}

impl Clint {
    /// Create a new `Clint` from the [`CLINT`](d1_pac::CLINT) peripheral
    #[must_use]
    pub fn new(clint: CLINT) -> Self {
        Self { clint }
    }

    /// Release the underlying [`CLINT`](d1_pac::CLINT) peripheral
    #[must_use]
    pub fn release(self) -> CLINT {
        self.clint
    }

    /// Summon the clint peripheral
    ///
    /// # Safety
    ///
    /// This is intended for use in interrupt context. Care should be taken not to have
    /// multiple instances live at the same time that may race or cause other UB issues
    #[must_use]
    pub unsafe fn summon() -> Self {
        Self {
            clint: d1_pac::Peripherals::steal().CLINT,
        }
    }

    /// Get the (machine) time value.
    pub fn get_mtime(&self) -> usize {
        // Note that the CLINT of the C906 core does not implement
        // the `mtime` register and we need to get the time value
        // with a CSR, which the `riscv` crate implements for us.
        riscv::register::time::read()
    }

    /// Set the machine time comparator.
    ///
    /// When `mtime` >= this value, a (machine) interrupt
    /// will be generated (if configured properly).
    pub fn set_mtimecmp(&mut self, cmp: usize) {
        let cmph = (cmp >> 32) as u32;
        let cmpl = (cmp & 0xffff_ffff) as u32;
        unsafe {
            self.clint.mtimecmph.write(|w| w.bits(cmph));
            self.clint.mtimecmpl.write(|w| w.bits(cmpl));
        }
    }

    /// Reset the machine time comparator to its default value.
    pub fn reset_mtimecmp(&mut self) {
        unsafe {
            self.clint.mtimecmph.write(|w| w.bits(0xffff_ffff));
            self.clint.mtimecmpl.write(|w| w.bits(0xffff_ffff));
        }
    }
}
