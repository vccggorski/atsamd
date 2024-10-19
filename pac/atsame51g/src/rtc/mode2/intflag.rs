#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `PER0` reader - Periodic Interval 0"]
pub type Per0R = crate::BitReader;
#[doc = "Field `PER0` writer - Periodic Interval 0"]
pub type Per0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PER1` reader - Periodic Interval 1"]
pub type Per1R = crate::BitReader;
#[doc = "Field `PER1` writer - Periodic Interval 1"]
pub type Per1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PER2` reader - Periodic Interval 2"]
pub type Per2R = crate::BitReader;
#[doc = "Field `PER2` writer - Periodic Interval 2"]
pub type Per2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PER3` reader - Periodic Interval 3"]
pub type Per3R = crate::BitReader;
#[doc = "Field `PER3` writer - Periodic Interval 3"]
pub type Per3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PER4` reader - Periodic Interval 4"]
pub type Per4R = crate::BitReader;
#[doc = "Field `PER4` writer - Periodic Interval 4"]
pub type Per4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PER5` reader - Periodic Interval 5"]
pub type Per5R = crate::BitReader;
#[doc = "Field `PER5` writer - Periodic Interval 5"]
pub type Per5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PER6` reader - Periodic Interval 6"]
pub type Per6R = crate::BitReader;
#[doc = "Field `PER6` writer - Periodic Interval 6"]
pub type Per6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PER7` reader - Periodic Interval 7"]
pub type Per7R = crate::BitReader;
#[doc = "Field `PER7` writer - Periodic Interval 7"]
pub type Per7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALARM0` reader - Alarm 0"]
pub type Alarm0R = crate::BitReader;
#[doc = "Field `ALARM0` writer - Alarm 0"]
pub type Alarm0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALARM1` reader - Alarm 1"]
pub type Alarm1R = crate::BitReader;
#[doc = "Field `ALARM1` writer - Alarm 1"]
pub type Alarm1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMPER` reader - Tamper"]
pub type TamperR = crate::BitReader;
#[doc = "Field `TAMPER` writer - Tamper"]
pub type TamperW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF` reader - Overflow"]
pub type OvfR = crate::BitReader;
#[doc = "Field `OVF` writer - Overflow"]
pub type OvfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Periodic Interval 0"]
    #[inline(always)]
    pub fn per0(&self) -> Per0R {
        Per0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Periodic Interval 1"]
    #[inline(always)]
    pub fn per1(&self) -> Per1R {
        Per1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Periodic Interval 2"]
    #[inline(always)]
    pub fn per2(&self) -> Per2R {
        Per2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Periodic Interval 3"]
    #[inline(always)]
    pub fn per3(&self) -> Per3R {
        Per3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Periodic Interval 4"]
    #[inline(always)]
    pub fn per4(&self) -> Per4R {
        Per4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Periodic Interval 5"]
    #[inline(always)]
    pub fn per5(&self) -> Per5R {
        Per5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Periodic Interval 6"]
    #[inline(always)]
    pub fn per6(&self) -> Per6R {
        Per6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Periodic Interval 7"]
    #[inline(always)]
    pub fn per7(&self) -> Per7R {
        Per7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Alarm 0"]
    #[inline(always)]
    pub fn alarm0(&self) -> Alarm0R {
        Alarm0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Alarm 1"]
    #[inline(always)]
    pub fn alarm1(&self) -> Alarm1R {
        Alarm1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - Tamper"]
    #[inline(always)]
    pub fn tamper(&self) -> TamperR {
        TamperR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Overflow"]
    #[inline(always)]
    pub fn ovf(&self) -> OvfR {
        OvfR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Periodic Interval 0"]
    #[inline(always)]
    #[must_use]
    pub fn per0(&mut self) -> Per0W<IntflagSpec> {
        Per0W::new(self, 0)
    }
    #[doc = "Bit 1 - Periodic Interval 1"]
    #[inline(always)]
    #[must_use]
    pub fn per1(&mut self) -> Per1W<IntflagSpec> {
        Per1W::new(self, 1)
    }
    #[doc = "Bit 2 - Periodic Interval 2"]
    #[inline(always)]
    #[must_use]
    pub fn per2(&mut self) -> Per2W<IntflagSpec> {
        Per2W::new(self, 2)
    }
    #[doc = "Bit 3 - Periodic Interval 3"]
    #[inline(always)]
    #[must_use]
    pub fn per3(&mut self) -> Per3W<IntflagSpec> {
        Per3W::new(self, 3)
    }
    #[doc = "Bit 4 - Periodic Interval 4"]
    #[inline(always)]
    #[must_use]
    pub fn per4(&mut self) -> Per4W<IntflagSpec> {
        Per4W::new(self, 4)
    }
    #[doc = "Bit 5 - Periodic Interval 5"]
    #[inline(always)]
    #[must_use]
    pub fn per5(&mut self) -> Per5W<IntflagSpec> {
        Per5W::new(self, 5)
    }
    #[doc = "Bit 6 - Periodic Interval 6"]
    #[inline(always)]
    #[must_use]
    pub fn per6(&mut self) -> Per6W<IntflagSpec> {
        Per6W::new(self, 6)
    }
    #[doc = "Bit 7 - Periodic Interval 7"]
    #[inline(always)]
    #[must_use]
    pub fn per7(&mut self) -> Per7W<IntflagSpec> {
        Per7W::new(self, 7)
    }
    #[doc = "Bit 8 - Alarm 0"]
    #[inline(always)]
    #[must_use]
    pub fn alarm0(&mut self) -> Alarm0W<IntflagSpec> {
        Alarm0W::new(self, 8)
    }
    #[doc = "Bit 9 - Alarm 1"]
    #[inline(always)]
    #[must_use]
    pub fn alarm1(&mut self) -> Alarm1W<IntflagSpec> {
        Alarm1W::new(self, 9)
    }
    #[doc = "Bit 14 - Tamper"]
    #[inline(always)]
    #[must_use]
    pub fn tamper(&mut self) -> TamperW<IntflagSpec> {
        TamperW::new(self, 14)
    }
    #[doc = "Bit 15 - Overflow"]
    #[inline(always)]
    #[must_use]
    pub fn ovf(&mut self) -> OvfW<IntflagSpec> {
        OvfW::new(self, 15)
    }
}
#[doc = "MODE2 Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagSpec;
impl crate::RegisterSpec for IntflagSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`intflag::R`](R) reader structure"]
impl crate::Readable for IntflagSpec {}
#[doc = "`write(|w| ..)` method takes [`intflag::W`](W) writer structure"]
impl crate::Writable for IntflagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for IntflagSpec {
    const RESET_VALUE: u16 = 0;
}
