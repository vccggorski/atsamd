#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `DRE` reader - Data Register Empty Interrupt"]
pub type DreR = crate::BitReader;
#[doc = "Field `TXC` reader - Transmit Complete Interrupt"]
pub type TxcR = crate::BitReader;
#[doc = "Field `TXC` writer - Transmit Complete Interrupt"]
pub type TxcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXC` reader - Receive Complete Interrupt"]
pub type RxcR = crate::BitReader;
#[doc = "Field `RXS` writer - Receive Start Interrupt"]
pub type RxsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSIC` reader - Clear To Send Input Change Interrupt"]
pub type CtsicR = crate::BitReader;
#[doc = "Field `CTSIC` writer - Clear To Send Input Change Interrupt"]
pub type CtsicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBRK` reader - Break Received Interrupt"]
pub type RxbrkR = crate::BitReader;
#[doc = "Field `RXBRK` writer - Break Received Interrupt"]
pub type RxbrkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERROR` reader - Combined Error Interrupt"]
pub type ErrorR = crate::BitReader;
#[doc = "Field `ERROR` writer - Combined Error Interrupt"]
pub type ErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Data Register Empty Interrupt"]
    #[inline(always)]
    pub fn dre(&self) -> DreR {
        DreR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Complete Interrupt"]
    #[inline(always)]
    pub fn txc(&self) -> TxcR {
        TxcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive Complete Interrupt"]
    #[inline(always)]
    pub fn rxc(&self) -> RxcR {
        RxcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Clear To Send Input Change Interrupt"]
    #[inline(always)]
    pub fn ctsic(&self) -> CtsicR {
        CtsicR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Break Received Interrupt"]
    #[inline(always)]
    pub fn rxbrk(&self) -> RxbrkR {
        RxbrkR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Combined Error Interrupt"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Transmit Complete Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn txc(&mut self) -> TxcW<IntflagSpec> {
        TxcW::new(self, 1)
    }
    #[doc = "Bit 3 - Receive Start Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxs(&mut self) -> RxsW<IntflagSpec> {
        RxsW::new(self, 3)
    }
    #[doc = "Bit 4 - Clear To Send Input Change Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ctsic(&mut self) -> CtsicW<IntflagSpec> {
        CtsicW::new(self, 4)
    }
    #[doc = "Bit 5 - Break Received Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxbrk(&mut self) -> RxbrkW<IntflagSpec> {
        RxbrkW::new(self, 5)
    }
    #[doc = "Bit 7 - Combined Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ErrorW<IntflagSpec> {
        ErrorW::new(self, 7)
    }
}
#[doc = "USART Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagSpec;
impl crate::RegisterSpec for IntflagSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intflag::R`](R) reader structure"]
impl crate::Readable for IntflagSpec {}
#[doc = "`write(|w| ..)` method takes [`intflag::W`](W) writer structure"]
impl crate::Writable for IntflagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for IntflagSpec {
    const RESET_VALUE: u8 = 0;
}
