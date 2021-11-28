//! # DFLL48M - Digital Frequency Locked Loop
//!
//! Dfll is an internal 48 MHz oscillator that provides two different modes of
//! operation
//!
//! - [`Enabled`]`<`[`Dfll`]`<`[`OpenLoop`]`>>`: Dfll operates as a stand-alone,
//!   high-frequency oscillator (default)
//! - [`Enabled`]`<`[`Dfll`]`<`[`ClosedLoop`]`>>`: Dfll engages internal
//!   frequency tuner operating against the external reference clock signal to
//!   tune internally produced signal (e.g. drifting)
//!
//! Dfll in a default state is provided
//! - in a return value of [`crate::clock::v2::retrieve_clocks`]
use typenum::{U0, U1};

use crate::clock::v2::{types::Enabled, Source, SourceMarker};
use crate::time::{Hertz, U32Ext};
use crate::typelevel::{Counter, PrivateIncrement, Sealed};

use super::gclk::{DynGclkSourceId, Gclk0, GclkId, GclkSource, GclkSourceId};
use super::gclkio::NotGclkInput;
use super::pclk::{Dfll48, Pclk, PclkSourceId};

/// Token type required to construct a [`Dfll`] type instance.
///
/// From a [`atsamd_hal`][`crate`] external user perspective, it does not
/// contain any methods and serves only a token purpose.
///
/// Within a [`atsamd_hal`][`crate`], [`DfllToken`] struct is a low-level access
/// abstraction for HW register calls.
pub struct DfllToken {
    __: (),
}

impl DfllToken {
    /// Constructor
    ///
    /// Unsafe: There should always be only a single instance thereof. It can be
    /// retrieved upon disabling and freeing an [`Enabled`]`<`[`Dfll`]`>`
    /// instance returned from `crate::clock::v2::retrieve_clocks` method
    #[inline]
    pub(crate) unsafe fn new() -> Self {
        Self { __: () }
    }

    #[inline]
    fn oscctrl(&self) -> &crate::pac::oscctrl::RegisterBlock {
        unsafe { &*crate::pac::OSCCTRL::ptr() }
    }

    #[inline]
    fn dfllctrla(&self) -> &crate::pac::oscctrl::DFLLCTRLA {
        &self.oscctrl().dfllctrla
    }

    #[inline]
    fn dfllctrlb(&self) -> &crate::pac::oscctrl::DFLLCTRLB {
        &self.oscctrl().dfllctrlb
    }

    #[allow(dead_code)]
    #[inline]
    fn dfllval(&self) -> &crate::pac::oscctrl::DFLLVAL {
        &self.oscctrl().dfllval
    }

    #[inline]
    fn dfllmul(&self) -> &crate::pac::oscctrl::DFLLMUL {
        &self.oscctrl().dfllmul
    }

    #[inline]
    fn dfllsync(&self) -> &crate::pac::oscctrl::DFLLSYNC {
        &self.oscctrl().dfllsync
    }

    #[inline]
    fn wait_sync_enable(&self) {
        while self.dfllsync().read().enable().bit() {}
    }

    #[inline]
    fn wait_sync_dfllmul(&self) {
        while self.dfllsync().read().dfllmul().bit() {}
    }

    #[allow(dead_code)]
    #[inline]
    fn wait_sync_dfllval(&self) {
        while self.dfllsync().read().dfllval().bit() {}
    }

    #[inline]
    fn wait_sync_dfllctrlb(&self) {
        while self.dfllsync().read().dfllctrlb().bit() {}
    }

    #[inline]
    fn set_on_demand_mode(&mut self, value: bool) {
        self.dfllctrla().modify(|_, w| w.ondemand().bit(value));
    }

    #[inline]
    fn set_run_standby(&mut self, value: bool) {
        self.dfllctrla().modify(|_, w| w.runstdby().bit(value));
    }

    #[inline]
    fn enable(&mut self) {
        self.dfllctrla().modify(|_, w| w.enable().set_bit());
        self.wait_sync_enable();
    }

    #[inline]
    fn disable(&mut self) {
        self.dfllctrla().modify(|_, w| w.enable().clear_bit());
        self.wait_sync_enable();
    }

    #[inline]
    fn set_open_mode(&mut self) {
        self.dfllctrlb().modify(|_, w| w.mode().clear_bit());
        self.wait_sync_dfllctrlb();
    }

    #[inline]
    fn set_closed_mode(&mut self) {
        self.dfllctrlb().modify(|_, w| w.mode().set_bit());
        self.wait_sync_dfllctrlb();
    }

    #[inline]
    fn set_fine_maximum_step(&mut self, value: u8) {
        self.dfllmul()
            .modify(|_, w| unsafe { w.fstep().bits(value) });
        self.wait_sync_dfllmul();
    }

    #[inline]
    fn set_coarse_maximum_step(&mut self, value: u8) {
        self.dfllmul()
            .modify(|_, w| unsafe { w.cstep().bits(value) });
        self.wait_sync_dfllmul();
    }

    #[inline]
    fn set_multiplication_factor(&mut self, value: u16) {
        self.dfllmul().modify(|_, w| unsafe { w.mul().bits(value) });
        self.wait_sync_dfllmul();
    }
}

type MultiplicationFactor = u16;
type CoarseMaximumStep = u8;
type FineMaximumStep = u8;

/// Trait generalizing over the concept of [`Dfll`] operation mode. Implemented
/// by structs representing specific modes
pub trait LoopMode: Sealed {
    /// Method encapsulating all mode specific HW calls
    fn enable(&self, token: &mut DfllToken);
}

/// Struct representing an open loop mode of [`Dfll`] operation
///
/// It is used as a generic parameter allowing to create specialized
/// implementations blocks for [`Enabled`]`<`[`Dfll`]`<`[`OpenLoop`]`>>` and
/// [`Dfll`]`<`[`OpenLoop`]`>` structs
pub struct OpenLoop {
    __: (),
}

impl LoopMode for OpenLoop {
    #[inline]
    fn enable(&self, token: &mut DfllToken) {
        token.set_open_mode();
    }
}
impl Sealed for OpenLoop {}
/// Struct representing a closed loop mode of [`Dfll`] operation
///
/// It is generic over the source of an associated peripheral clock/channel
/// ([`Pclk`]`<`[`Dfll48`]`, T>`)
///
/// It is used as a generic parameter allowing to create specialized
/// implementations blocks for [`Enabled`]`<`[`Dfll`]`<`[`ClosedLoop<T>`]`>>`
/// and [`Dfll`]`<`[`ClosedLoop<T>`]`>` structs
pub struct ClosedLoop<T: PclkSourceId> {
    reference_clk: Pclk<Dfll48, T>,
    coarse_maximum_step: CoarseMaximumStep,
    fine_maximum_step: FineMaximumStep,
}
impl<T: PclkSourceId> Sealed for ClosedLoop<T> {}
impl<T: PclkSourceId> LoopMode for ClosedLoop<T> {
    #[inline]
    fn enable(&self, token: &mut DfllToken) {
        token.set_fine_maximum_step(self.fine_maximum_step);
        token.set_coarse_maximum_step(self.coarse_maximum_step);
        token.set_closed_mode();
    }
}

/// Struct representing a [`Dfll`] abstraction
///
/// It is generic over the supported modes of operation
pub struct Dfll<TMode: LoopMode> {
    token: DfllToken,
    freq: Hertz,
    mode: TMode,
    multiplication_factor: MultiplicationFactor,
    run_standby: bool,
    on_demand_mode: bool,
}

impl<TMode: LoopMode> Dfll<TMode> {
    /// Returns the frequency of the [`Dfll`]
    #[inline]
    pub fn freq(&self) -> Hertz {
        Hertz(self.freq.0 * self.multiplication_factor as u32)
    }

    /// Controls the clock source behaviour during standby
    ///
    /// See Datasheet c. 28.6.4.1
    #[inline]
    pub fn set_run_standby(&mut self, value: bool) {
        self.run_standby = value;
    }

    /// Controls the on demand functionality of the clock source
    ///
    /// Only starts the clock source when a peripheral uses it. If cleared the
    /// clock will be always active
    ///
    /// See Datasheet c. 13.5 for general information; 28.6.4.1 for [`Dfll`]
    /// specific details
    #[inline]
    pub fn set_on_demand_mode(&mut self, value: bool) {
        self.on_demand_mode = value;
    }

    /// Enabling a [`Dfll`] modifies hardware to match the configuration stored
    /// within
    #[inline]
    pub fn enable(mut self) -> Enabled<Self, U0> {
        self.mode.enable(&mut self.token);
        self.token.set_on_demand_mode(self.on_demand_mode);
        self.token.set_run_standby(self.run_standby);
        self.token
            .set_multiplication_factor(self.multiplication_factor);
        self.token.enable();
        Enabled::new(self)
    }
}

impl Dfll<OpenLoop> {
    /// Constructs a builder of [`Dfll`] in an [`OpenLoop`]. To affect the
    /// hardware, it requires an additional call to [`Dfll::enable`]
    #[inline]
    pub fn in_open_mode(token: DfllToken) -> Dfll<OpenLoop> {
        Self {
            token,
            freq: 48.mhz().into(),
            mode: OpenLoop { __: () },
            multiplication_factor: 1_u16,
            run_standby: false,
            on_demand_mode: true,
        }
    }

    /// Release the resources
    #[inline]
    pub fn free(self) -> DfllToken {
        self.token
    }
}

impl<T: PclkSourceId> Dfll<ClosedLoop<T>> {
    /// Constructs a builder of [`Dfll`] in a [`ClosedLoop`]. To affect the
    /// hardware, it requires an additional call to [`Dfll::enable`]
    #[inline]
    pub fn in_closed_mode(
        token: DfllToken,
        reference_clk: Pclk<Dfll48, T>,
        multiplication_factor: MultiplicationFactor,
        coarse_maximum_step: CoarseMaximumStep,
        fine_maximum_step: FineMaximumStep,
    ) -> Dfll<ClosedLoop<T>> {
        Self {
            token,
            freq: reference_clk.freq(),
            mode: ClosedLoop {
                reference_clk,
                coarse_maximum_step,
                fine_maximum_step,
            },
            multiplication_factor,
            run_standby: false,
            on_demand_mode: true,
        }
    }
    /// Set a multiplication factor for an input frequency
    ///
    /// Consult datasheet regarding what kind of set of parameters is acceptable
    /// (c. 28.6.4.1, Closed-Loop Operation). Otherwise, [`Dfll`] behavior might
    /// be incorrect
    #[inline]
    pub fn set_multiplication_factor(&mut self, multiplication_factor: MultiplicationFactor) {
        self.multiplication_factor = multiplication_factor;
    }

    /// Set a maximum step size allowed during a process a frequency tuning for
    /// a coarse parameter.
    ///
    /// Consult datasheet regarding what kind of set of parameters is acceptable
    /// (c. 28.6.4.1, Closed-Loop Operation). Otherwise, [`Dfll`] behavior might
    /// be incorrect
    #[inline]
    pub fn set_coarse_maximum_step(&mut self, coarse_maximum_step: CoarseMaximumStep) {
        self.mode.coarse_maximum_step = coarse_maximum_step;
    }

    /// Set a maximum step size allowed during a process a frequency tuning for
    /// a fine parameter.
    ///
    /// Consult datasheet regarding what kind of set of parameters is acceptable
    /// (c. 28.6.4.1, Closed-Loop Operation). Otherwise, [`Dfll`] behavior might
    /// be incorrect
    #[inline]
    pub fn set_fine_maximum_step(&mut self, fine_maximum_step: FineMaximumStep) {
        self.mode.fine_maximum_step = fine_maximum_step;
    }

    /// Release the resources
    #[inline]
    pub fn free(self) -> (DfllToken, Pclk<Dfll48, T>) {
        (self.token, self.mode.reference_clk)
    }
}

impl<TMode: LoopMode> Enabled<Dfll<TMode>, U0> {
    /// Disable the [`Dfll`]
    #[inline]
    pub fn disable(mut self) -> Dfll<TMode> {
        self.0.token.disable();
        self.0
    }
}

impl Enabled<Dfll<OpenLoop>, U1> {
    /// Special, helper method allowing to change a mode of [`Dfll`] operation
    /// in place. It is implemented only for an enabled [`Dfll`] having a single
    /// user (which is a [`Gclk0`]). Without it, it becomes unwieldy to change a
    /// mode of a [`Dfll`] actively used by [`Gclk0`], which is a very common
    /// scenario
    #[inline]
    pub fn to_closed_mode<T: PclkSourceId>(
        self,
        gclk0: Enabled<Gclk0<DfllId>, U1>,
        reference_clk: Pclk<Dfll48, T>,
        multiplication_factor: MultiplicationFactor,
        coarse_maximum_step: CoarseMaximumStep,
        fine_maximum_step: FineMaximumStep,
    ) -> (Enabled<Dfll<ClosedLoop<T>>, U1>, Enabled<Gclk0<DfllId>, U1>) {
        let token = self.0.free();
        let dfll = Dfll::in_closed_mode(
            token,
            reference_clk,
            multiplication_factor,
            coarse_maximum_step,
            fine_maximum_step,
        );
        (dfll.enable().inc(), gclk0)
    }
}

impl<T: PclkSourceId> Enabled<Dfll<ClosedLoop<T>>, U1> {
    /// Special, helper method allowing to change a mode of [`Dfll`] operation
    /// in place. It is implemented only for an enabled [`Dfll`] having a single
    /// user (which is a [`Gclk0`]). Without it, it becomes unwieldy to change a
    /// mode of a [`Dfll`] actively used by [`Gclk0`], which is a very common
    /// scenario
    #[inline]
    pub fn to_open_mode(
        self,
        gclk0: Enabled<Gclk0<DfllId>, U1>,
    ) -> (
        Enabled<Dfll<OpenLoop>, U1>,
        Enabled<Gclk0<DfllId>, U1>,
        Pclk<Dfll48, T>,
    ) {
        let (token, pclk) = self.0.free();
        let dfll = Dfll::in_open_mode(token);
        (dfll.enable().inc(), gclk0, pclk)
    }
}

//==============================================================================
// GclkSource
//==============================================================================

impl<G: GclkId, T: LoopMode, N: Counter> GclkSource<G> for Enabled<Dfll<T>, N> {
    type Type = DfllId;
}

impl<T: LoopMode, N: Counter> Source for Enabled<Dfll<T>, N> {
    #[inline]
    fn freq(&self) -> Hertz {
        self.0.freq()
    }
}

/// Type-level variant representing the identity of the DFLL clock
///
/// This type is a member of several [type-level enums]. See the documentation
/// on [type-level enums] for more details on the pattern.
///
/// [type-level enums]: crate::typelevel#type-level-enum
pub enum DfllId {}

impl Sealed for DfllId {}

impl GclkSourceId for DfllId {
    const DYN: DynGclkSourceId = DynGclkSourceId::DFLL;
}

impl NotGclkInput for DfllId {}

impl SourceMarker for DfllId {}
