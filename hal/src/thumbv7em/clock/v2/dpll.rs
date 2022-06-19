//! # Digital Phase Locked Loop (DPLL)
//!
//! [`Dpll`] allows user to multiply an input signal and supports a handful of
//! them. It maintains the output signal frequency by constant phase comparison
//! against the reference, input signal.
//!
//! There are two Dplls that are available
//!
//! - [`Enabled`]`<`[`Dpll`]`<`[`marker::Dpll0`]`, _>>`: [`Dpll0`]
//! - [`Enabled`]`<`[`Dpll`]`<`[`marker::Dpll1`]`, _>>`: [`Dpll1`]
//!
//! Each of them can operate in 3 different modes
//!
//! - [`Enabled`]`<`[`Dpll`]`<_, `[`PclkDriven`]`>`: `Gclk` as a reference clock
//! - [`Enabled`]`<`[`Dpll`]`<_, `[`XoscDriven`]`>`: `Xosc{0, 1}` as a reference
//!   clock
//! - [`Enabled`]`<`[`Dpll`]`<_, `[`Xosc32kDriven`]`>`: `Xosc32k` as a
//!   reference_clk
//!
//! These can be created by using an appropriate construction function:
//! - [`Dpll::from_pclk`]
//! - [`Dpll::from_xosc`]
//! - [`Dpll::from_xosc32k`]
//! and then enabled by [`Dpll::enable`] function call

use core::convert::Infallible;
use core::marker::PhantomData;

use paste::paste;
use seq_macro::seq;
use typenum::U0;

use crate::pac::oscctrl::dpll::{dpllstatus, dpllsyncbusy, DPLLCTRLA, DPLLCTRLB, DPLLRATIO};
use crate::pac::oscctrl::DPLL;

use crate::pac::oscctrl::dpll::dpllctrlb::REFCLK_A;

use crate::time::Hertz;
use crate::typelevel::{Counter, Decrement, Increment, Sealed};

use super::gclk::{GclkId, GclkSourceId};
use super::pclk::{Pclk, PclkId};
use super::xosc::{Xosc0Id, Xosc1Id};
use super::xosc32k::Xosc32kId;
use super::{Enabled, Source};

//==============================================================================
// DpllId
//==============================================================================

/// Type-level `enum` for DPLL identifiers
///
/// See the documentation on [type-level enums] for more details on the
/// pattern.
///
/// [type-level enums]: crate::typelevel#type-level-enum
pub trait DpllId: Sealed {
    /// Corresponding numeric index
    const NUM: usize;
}

/// Type-level variant representing the identity of DPLL0
///
/// This type is a member of several [type-level enums]. See the documentation
/// on [type-level enums] for more details on the pattern.
///
/// [type-level enums]: crate::typelevel#type-level-enum
pub enum Dpll0Id {}

impl Sealed for Dpll0Id {}

impl DpllId for Dpll0Id {
    const NUM: usize = 0;
}

/// Type-level variant representing the identity of DPLL1
///
/// This type is a member of several [type-level enums]. See the documentation
/// on [type-level enums] for more details on the pattern.
///
/// [type-level enums]: crate::typelevel#type-level-enum
pub enum Dpll1Id {}

impl Sealed for Dpll1Id {}

impl DpllId for Dpll1Id {
    const NUM: usize = 1;
}

//==============================================================================
// DynDpllSourceId
//==============================================================================

/// Value-level version of [`DpllSourceId`]
///
/// Indicates the clock source for a [`Dpll`]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum DynDpllSourceId {
    /// The DPLL is driven by a [`Pclk`]
    Pclk,
    /// The DPLL is driven by [`Xosc0`](super::xosc::Xosc0)
    Xosc0,
    /// The DPLL is driven by [`Xosc0`](super::xosc::Xosc1)
    Xosc1,
    /// The DPLL is driven by [`Xosc32k`](super::xosc32k::Xosc32k)
    Xosc32k,
}

impl From<DynDpllSourceId> for REFCLK_A {
    fn from(source: DynDpllSourceId) -> Self {
        match source {
            DynDpllSourceId::Pclk => REFCLK_A::GCLK,
            DynDpllSourceId::Xosc0 => REFCLK_A::XOSC0,
            DynDpllSourceId::Xosc1 => REFCLK_A::XOSC1,
            DynDpllSourceId::Xosc32k => REFCLK_A::XOSC32,
        }
    }
}

//==============================================================================
// DpllSourceId
//==============================================================================

/// Type-level `enum` for DPLL sources
///
/// See the documentation on [type-level enums] for more details on the
/// pattern.
///
/// [type-level enums]: crate::typelevel#type-level-enum
pub trait DpllSourceId<D: DpllId> {
    /// Corresponding variant of [`DynDpllSourceId`]
    const DYN: DynDpllSourceId;
    /// Corresponding [`Pclk`] type if the DPLL source is a peripheral clock
    type Pclk;
}

impl<D: DpllId + PclkId, G: GclkId> DpllSourceId<D> for G {
    const DYN: DynDpllSourceId = DynDpllSourceId::Pclk;
    type Pclk = Pclk<D, G>;
}

seq!(N in 0..=1 {
    paste! {
        impl<D: DpllId> DpllSourceId<D> for [<Xosc N Id>] {
            const DYN: DynDpllSourceId = DynDpllSourceId::Xosc~N;
            type Pclk = ();
        }
    }
});

impl<D: DpllId> DpllSourceId<D> for Xosc32kId {
    const DYN: DynDpllSourceId = DynDpllSourceId::Xosc32k;
    type Pclk = ();
}

//==============================================================================
// DpllToken
//==============================================================================

/// Token type required to construct a [`Dpll`] type instance.
///
/// From an [`atsamd_hal`][`crate`] external user perspective, it does not
/// contain any methods and serves only a token purpose.
///
/// Within an [`atsamd_hal`][`crate`], [`DpllToken`] struct is a low-level
/// access abstraction for HW register calls.
pub struct DpllToken<D: DpllId> {
    dpll: PhantomData<D>,
}

impl<D: DpllId> DpllToken<D> {
    /// Constructor
    ///
    /// Unsafe: There should always be only a single instance thereof. It can be
    /// retrieved upon disabling and freeing an [`Enabled`]`<`[`Dpll`]`>`
    /// instance returned from `crate::clock::v2::retrieve_clocks` method
    #[inline]
    pub(super) unsafe fn new() -> Self {
        Self { dpll: PhantomData }
    }

    #[inline]
    fn dpll(&self) -> &DPLL {
        unsafe { &(*crate::pac::OSCCTRL::ptr()).dpll[D::NUM] }
    }

    #[inline]
    fn ctrla(&self) -> &DPLLCTRLA {
        &self.dpll().dpllctrla
    }

    #[inline]
    fn ctrlb(&self) -> &DPLLCTRLB {
        &self.dpll().dpllctrlb
    }

    #[inline]
    fn ratio(&self) -> &DPLLRATIO {
        &self.dpll().dpllratio
    }

    #[inline]
    fn syncbusy(&self) -> dpllsyncbusy::R {
        self.dpll().dpllsyncbusy.read()
    }

    #[inline]
    fn status(&self) -> dpllstatus::R {
        self.dpll().dpllstatus.read()
    }

    /// Set the loop division, see page 701 in the Datasheet
    ///
    /// Formula for calculating the frequency:
    /// f_clk_dpll = clk_src * (LDR + 1 + (LDRFRAC / 32))
    ///
    /// `int` is including the `+ 1`,
    /// 'frac` is the same as `LDRFRAC`
    ///
    /// Write to the divider must be write synchronized
    #[inline]
    fn set_loop_div(&mut self, int: u16, frac: u8) {
        self.ratio().write(|w| unsafe {
            w.ldr().bits((int - 1) & 0x1FFF);
            w.ldrfrac().bits(frac & 0x1F)
        });
        while self.syncbusy().dpllratio().bit_is_set() {}
    }

    /// Set the clock source.
    #[inline]
    fn set_source_clock(&mut self, source: DynDpllSourceId) {
        self.ctrlb()
            .modify(|_, w| w.refclk().variant(source.into()));
    }

    /// When source is a XOSC this has effect, ignored otherwise.
    #[inline]
    fn set_source_div(&mut self, div: u16) {
        self.ctrlb()
            .modify(|_, w| unsafe { w.div().bits(div & ((1 << 10) - 1)) });
    }

    /// Ignore the lock, CLK_DPLLn is always running.
    #[inline]
    fn set_lock_bypass(&mut self, bypass: bool) {
        self.ctrlb().modify(|_, w| w.lbypass().bit(bypass));
    }

    /// Wake up fast, output the clock directly without waiting for lock.
    #[inline]
    fn set_wake_up_fast(&mut self, wuf: bool) {
        self.ctrlb().modify(|_, w| w.wuf().bit(wuf));
    }

    #[inline]
    fn set_on_demand(&mut self, on_demand: bool) {
        self.ctrla().modify(|_, w| w.ondemand().bit(on_demand));
    }

    /// Check if [`Dpll`] clock is ready.
    #[inline]
    fn wait_until_ready(&self) -> nb::Result<(), Infallible> {
        if self.status().clkrdy().bit_is_clear() {
            Err(nb::Error::WouldBlock)
        } else {
            Ok(())
        }
    }

    /// Check if [`Dpll`] clock is locked.
    #[inline]
    fn wait_until_locked(&self) -> nb::Result<(), Infallible> {
        if self.status().lock().bit_is_clear() {
            Err(nb::Error::WouldBlock)
        } else {
            Ok(())
        }
    }

    /// Wait until register has been synchronized.
    #[inline]
    fn wait_until_enable_synced(&self) {
        while self.syncbusy().enable().bit_is_set() {}
    }

    /// Enable the [`Dpll`], ensure register write is synchronized.
    #[inline]
    fn enable(&mut self) {
        self.ctrla().modify(|_, w| w.enable().set_bit());
        self.wait_until_enable_synced();
    }

    /// Disable the [`Dpll`], ensure register write is synchronized.
    #[inline]
    fn disable(&mut self) {
        self.ctrla().modify(|_, w| w.enable().clear_bit());
        self.wait_until_enable_synced();
    }
}

//==============================================================================
// Dpll
//==============================================================================

/// Struct representing a [`Dpll`] abstraction
///
/// It is generic over:
/// - a numeric variant (available variants: [`marker::Dpll0`],
///   [`marker::Dpll1`])
/// - a mode of operation (available modes: [`PclkDriven`], [`XoscDriven`],
///   [`Xosc32kDriven`])
pub struct Dpll<D, I>
where
    D: DpllId,
    I: DpllSourceId<D>,
{
    token: DpllToken<D>,
    src_freq: Hertz,
    mult: u16,
    frac: u8,
    lock_bypass: bool,
    wake_up_fast: bool,
    on_demand: bool,
    pclk: I::Pclk,
    prediv: u16,
}

impl<D, G> Dpll<D, G>
where
    D: DpllId + PclkId,
    G: GclkId,
{
    /// Create a [`Dpll`] from a [`Pclk`]
    ///
    /// The corresponding [`Gclk`](super::gclk::Gclk) frequency must be between
    /// 32 kHz and 3.2 MHz.
    #[inline]
    pub fn from_pclk(token: DpllToken<D>, pclk: Pclk<D, G>) -> Self {
        let src_freq = pclk.freq();
        let (mult, frac) = (1, 0);
        Self {
            token,
            src_freq,
            mult,
            frac,
            lock_bypass: false,
            wake_up_fast: false,
            on_demand: true,
            pclk,
            prediv: 1,
        }
    }

    /// Deconstruct the [`Dpll`], release the token, and return the [`Pclk`]
    #[inline]
    pub fn free(self) -> (DpllToken<D>, Pclk<D, G>) {
        (self.token, self.pclk)
    }
}

impl<D> Dpll<D, Xosc32kId>
where
    D: DpllId,
{
    /// Create a [`Dpll`] from an [`Xosc32k`](super::xosc32k::Xosc32k)
    ///
    /// [`Increment`] the `Xosc32k` [`Enabled`] [`Counter`] to indicate it is
    /// being used by the `Dpll`
    #[inline]
    pub fn from_xosc32k<S>(token: DpllToken<D>, xosc32k: S) -> (Self, S::Inc)
    where
        S: Source<Id = Xosc32kId> + Increment,
    {
        let src_freq = xosc32k.freq();
        let (mult, frac) = (1, 0);

        let dpll = Self {
            token,
            src_freq,
            mult,
            frac,
            lock_bypass: false,
            wake_up_fast: false,
            on_demand: true,
            pclk: (),
            prediv: 1,
        };
        (dpll, xosc32k.inc())
    }

    /// Deconstruct the [`Dpll`], release the token, and [`Decrement`] the
    /// [`Xosc32k`](super::xosc32k::Xosc32k) [`Enabled`] [`Counter`]
    #[inline]
    pub fn free<S>(self, xosc32k: S) -> (DpllToken<D>, S::Dec)
    where
        S: Source<Id = Xosc32kId> + Decrement,
    {
        (self.token, xosc32k.dec())
    }
}

seq!(N in 0..=1 {
    paste! {
        impl<D: DpllId> Dpll<D, [<Xosc N Id>]> {
            /// Create a [`Dpll`] from an external oscillator
            ///
            /// After division by the clock divider (see [`RawPredivider`]), the
            /// input frequency must be between 32 kHz and 3.2 MHz.
            ///
            /// [`Increment`] the `Xosc` [`Enabled`] [`Counter`] to indicate it is
            /// being used by the `Dpll`
            #[inline]
            pub fn from_xosc~N<S>(
                token: DpllToken<D>,
                xosc: S,
            ) -> (Self, S::Inc)
            where
                S: Source<Id = [<Xosc N Id>]> + Increment,
            {
                let src_freq = xosc.freq();
                let (mult, frac) = (1, 0);

                let dpll = Self {
                    token,
                    src_freq,
                    mult,
                    frac,
                    lock_bypass: false,
                    wake_up_fast: false,
                    on_demand: true,
                    pclk: (),
                    prediv: 2,
                };
                (dpll, xosc.inc())
            }

            /// Set the raw predivider, see [`RawPredivider`]
            #[inline]
            pub fn set_prediv(mut self, prediv: u16) -> Self {
                if prediv < 2 || prediv > 4096 || prediv % 2 != 0 {
                    panic!("prediv must be an even number in the interval [2, 4096]");
                }
                self.prediv = prediv;
                self
            }

            /// Deconstruct the [`Dpll`], release the token, and [`Decrement`] the
            /// [`Xosc`](super::xosc::Xosc) [`Enabled`] [`Counter`]
            #[inline]
            pub fn free<S>(self, xosc: S) -> (DpllToken<D>, S::Dec)
            where
                S: Source<Id = [<Xosc N Id>]> + Decrement,
            {
                (self.token, xosc.dec())
            }
        }
    }
});

impl<D, I> Dpll<D, I>
where
    D: DpllId,
    I: DpllSourceId<D>,
{
    /// Set the [`Dpll`] loop divider
    ///
    /// Calculated as
    ///
    /// ```text
    /// f_clk_dpll = clk_src * (int + (frac / 32))
    /// ```
    ///
    /// The `+ 1` in the datasheet is not forgotten, it is handled by the
    /// underlying register write function
    ///
    /// Example 1:
    /// ```text
    /// clk_src = 2 MHz
    /// int = 50
    /// frac = 0
    ///
    /// 2 * 50 = 100 MHz
    /// ```
    /// Example 2:
    /// ```text
    /// clk_src = 32 kHz
    /// int = 3000
    /// frac = 24
    ///
    /// 0.032 * (3000 +  24/32) = 96.024 MHz
    /// ```
    #[inline]
    pub fn set_loop_div(mut self, int: u16, frac: u8) -> Self {
        self.mult = int;
        self.frac = frac;
        self
    }

    /// Set to ignore the phase-lock, CLK_DPLL is always running regardless of
    /// lock status
    #[inline]
    pub fn set_lock_bypass(mut self, bypass: bool) -> Self {
        self.lock_bypass = bypass;
        self
    }

    /// Set to skip waiting for [`Dpll`] lock before outputting clock
    #[inline]
    pub fn set_wake_up_fast(mut self, wuf: bool) -> Self {
        self.wake_up_fast = wuf;
        self
    }

    #[inline]
    pub fn set_on_demand(mut self, on_demand: bool) -> Self {
        self.on_demand = on_demand;
        self
    }

    #[inline]
    fn input_freq(&self) -> u32 {
        self.src_freq.0 / self.prediv as u32
    }

    #[inline]
    fn output_freq(&self) -> u32 {
        self.input_freq() * (self.mult as u32 + self.frac as u32 / 32)
    }

    /// Return the frequency of the [`Dpll`]
    #[inline]
    pub fn freq(&self) -> Hertz {
        Hertz(self.output_freq())
    }

    /// Enables [`Dpll`] and performs assertions in local configuration
    ///
    /// - Performs HW register writes
    #[inline]
    pub fn enable(mut self) -> EnabledDpll<D, I> {
        if !(32_000..=3_200_000).contains(&self.input_freq()) {
            panic!("Invalid DPLL input frequency");
        }
        if !(96_000_000..=200_000_000).contains(&self.output_freq()) {
            panic!("Invalid DPLL output frequency");
        }
        self.token.set_source_clock(I::DYN);
        match I::DYN {
            DynDpllSourceId::Xosc0 | DynDpllSourceId::Xosc1 => {
                self.token.set_source_div(self.prediv / 2 - 1)
            }
            _ => {}
        }
        // Set the loop divider ratio and other settings
        self.token.set_loop_div(self.mult, self.frac);
        self.token.set_lock_bypass(self.lock_bypass);
        self.token.set_wake_up_fast(self.wake_up_fast);
        self.token.set_on_demand(self.on_demand);
        self.token.enable();
        Enabled::new(self)
    }
}

/// Alias of [`Dpll`]`<`[`marker::Dpll0`]`, _>`
pub type Dpll0<M> = Dpll<Dpll0Id, M>;

/// Alias of [`Dpll`]`<`[`marker::Dpll1`]`, _>`
pub type Dpll1<M> = Dpll<Dpll1Id, M>;

pub type EnabledDpll<D, I, N = U0> = Enabled<Dpll<D, I>, N>;

pub type EnabledDpll0<I, N = U0> = EnabledDpll<Dpll0Id, I, N>;

pub type EnabledDpll1<I, N = U0> = EnabledDpll<Dpll1Id, I, N>;

impl<D, I> EnabledDpll<D, I>
where
    D: DpllId,
    I: DpllSourceId<D>,
{
    /// Disable the [`Dpll`]
    #[inline]
    pub fn disable(mut self) -> Dpll<D, I> {
        self.0.token.disable();
        self.0
    }
}

impl<D, I, N> EnabledDpll<D, I, N>
where
    D: DpllId,
    I: DpllSourceId<D>,
    N: Counter,
{
    /// Check if [`Dpll`] has achieved lock
    #[inline]
    pub fn wait_until_locked(&self) -> nb::Result<(), Infallible> {
        self.0.token.wait_until_locked()
    }

    /// Check if [`Dpll`] is ready
    #[inline]
    pub fn wait_until_ready(&self) -> nb::Result<(), Infallible> {
        self.0.token.wait_until_ready()
    }
}

//==============================================================================
// Source
//==============================================================================

impl<D, I, N> Source for EnabledDpll<D, I, N>
where
    D: DpllId + GclkSourceId,
    I: DpllSourceId<D>,
    N: Counter,
{
    type Id = D;

    #[inline]
    fn freq(&self) -> Hertz {
        self.0.freq()
    }
}
