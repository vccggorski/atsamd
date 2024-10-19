#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Field `PER0` reader - Periodic Interval 0 Interrupt Enable"]
pub type Per0R = crate::BitReader;
#[doc = "Field `PER0` writer - Periodic Interval 0 Interrupt Enable"]
pub type Per0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PER1` reader - Periodic Interval 1 Interrupt Enable"]
pub type Per1R = crate::BitReader;
#[doc = "Field `PER1` writer - Periodic Interval 1 Interrupt Enable"]
pub type Per1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PER2` reader - Periodic Interval 2 Interrupt Enable"]
pub type Per2R = crate::BitReader;
#[doc = "Field `PER2` writer - Periodic Interval 2 Interrupt Enable"]
pub type Per2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PER3` reader - Periodic Interval 3 Interrupt Enable"]
pub type Per3R = crate::BitReader;
#[doc = "Field `PER3` writer - Periodic Interval 3 Interrupt Enable"]
pub type Per3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PER4` reader - Periodic Interval 4 Interrupt Enable"]
pub type Per4R = crate::BitReader;
#[doc = "Field `PER4` writer - Periodic Interval 4 Interrupt Enable"]
pub type Per4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PER5` reader - Periodic Interval 5 Interrupt Enable"]
pub type Per5R = crate::BitReader;
#[doc = "Field `PER5` writer - Periodic Interval 5 Interrupt Enable"]
pub type Per5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PER6` reader - Periodic Interval 6 Interrupt Enable"]
pub type Per6R = crate::BitReader;
#[doc = "Field `PER6` writer - Periodic Interval 6 Interrupt Enable"]
pub type Per6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PER7` reader - Periodic Interval 7 Interrupt Enable"]
pub type Per7R = crate::BitReader;
#[doc = "Field `PER7` writer - Periodic Interval 7 Interrupt Enable"]
pub type Per7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP0` reader - Compare 0 Interrupt Enable"]
pub type Cmp0R = crate::BitReader;
#[doc = "Field `CMP0` writer - Compare 0 Interrupt Enable"]
pub type Cmp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1` reader - Compare 1 Interrupt Enable"]
pub type Cmp1R = crate::BitReader;
#[doc = "Field `CMP1` writer - Compare 1 Interrupt Enable"]
pub type Cmp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMPER` reader - Tamper Enable"]
pub type TamperR = crate::BitReader;
#[doc = "Field `TAMPER` writer - Tamper Enable"]
pub type TamperW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF` reader - Overflow Interrupt Enable"]
pub type OvfR = crate::BitReader;
#[doc = "Field `OVF` writer - Overflow Interrupt Enable"]
pub type OvfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Periodic Interval 0 Interrupt Enable"]
    #[inline(always)]
    pub fn per0(&self) -> Per0R {
        Per0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Periodic Interval 1 Interrupt Enable"]
    #[inline(always)]
    pub fn per1(&self) -> Per1R {
        Per1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Periodic Interval 2 Interrupt Enable"]
    #[inline(always)]
    pub fn per2(&self) -> Per2R {
        Per2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Periodic Interval 3 Interrupt Enable"]
    #[inline(always)]
    pub fn per3(&self) -> Per3R {
        Per3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Periodic Interval 4 Interrupt Enable"]
    #[inline(always)]
    pub fn per4(&self) -> Per4R {
        Per4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Periodic Interval 5 Interrupt Enable"]
    #[inline(always)]
    pub fn per5(&self) -> Per5R {
        Per5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Periodic Interval 6 Interrupt Enable"]
    #[inline(always)]
    pub fn per6(&self) -> Per6R {
        Per6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Periodic Interval 7 Interrupt Enable"]
    #[inline(always)]
    pub fn per7(&self) -> Per7R {
        Per7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Compare 0 Interrupt Enable"]
    #[inline(always)]
    pub fn cmp0(&self) -> Cmp0R {
        Cmp0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Compare 1 Interrupt Enable"]
    #[inline(always)]
    pub fn cmp1(&self) -> Cmp1R {
        Cmp1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - Tamper Enable"]
    #[inline(always)]
    pub fn tamper(&self) -> TamperR {
        TamperR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn ovf(&self) -> OvfR {
        OvfR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Periodic Interval 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn per0(&mut self) -> Per0W<IntenclrSpec> {
        Per0W::new(self, 0)
    }
    #[doc = "Bit 1 - Periodic Interval 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn per1(&mut self) -> Per1W<IntenclrSpec> {
        Per1W::new(self, 1)
    }
    #[doc = "Bit 2 - Periodic Interval 2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn per2(&mut self) -> Per2W<IntenclrSpec> {
        Per2W::new(self, 2)
    }
    #[doc = "Bit 3 - Periodic Interval 3 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn per3(&mut self) -> Per3W<IntenclrSpec> {
        Per3W::new(self, 3)
    }
    #[doc = "Bit 4 - Periodic Interval 4 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn per4(&mut self) -> Per4W<IntenclrSpec> {
        Per4W::new(self, 4)
    }
    #[doc = "Bit 5 - Periodic Interval 5 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn per5(&mut self) -> Per5W<IntenclrSpec> {
        Per5W::new(self, 5)
    }
    #[doc = "Bit 6 - Periodic Interval 6 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn per6(&mut self) -> Per6W<IntenclrSpec> {
        Per6W::new(self, 6)
    }
    #[doc = "Bit 7 - Periodic Interval 7 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn per7(&mut self) -> Per7W<IntenclrSpec> {
        Per7W::new(self, 7)
    }
    #[doc = "Bit 8 - Compare 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0(&mut self) -> Cmp0W<IntenclrSpec> {
        Cmp0W::new(self, 8)
    }
    #[doc = "Bit 9 - Compare 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1(&mut self) -> Cmp1W<IntenclrSpec> {
        Cmp1W::new(self, 9)
    }
    #[doc = "Bit 14 - Tamper Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tamper(&mut self) -> TamperW<IntenclrSpec> {
        TamperW::new(self, 14)
    }
    #[doc = "Bit 15 - Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovf(&mut self) -> OvfW<IntenclrSpec> {
        OvfW::new(self, 15)
    }
}
#[doc = "MODE0 Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {
    const RESET_VALUE: u16 = 0;
}
