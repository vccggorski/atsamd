#[doc = "Register `BUSYCH` reader"]
pub type R = crate::R<BusychSpec>;
#[doc = "Field `BUSYCH0` reader - Busy Channel 0"]
pub type Busych0R = crate::BitReader;
#[doc = "Field `BUSYCH1` reader - Busy Channel 1"]
pub type Busych1R = crate::BitReader;
#[doc = "Field `BUSYCH2` reader - Busy Channel 2"]
pub type Busych2R = crate::BitReader;
#[doc = "Field `BUSYCH3` reader - Busy Channel 3"]
pub type Busych3R = crate::BitReader;
#[doc = "Field `BUSYCH4` reader - Busy Channel 4"]
pub type Busych4R = crate::BitReader;
#[doc = "Field `BUSYCH5` reader - Busy Channel 5"]
pub type Busych5R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Busy Channel 0"]
    #[inline(always)]
    pub fn busych0(&self) -> Busych0R {
        Busych0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Busy Channel 1"]
    #[inline(always)]
    pub fn busych1(&self) -> Busych1R {
        Busych1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Busy Channel 2"]
    #[inline(always)]
    pub fn busych2(&self) -> Busych2R {
        Busych2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Busy Channel 3"]
    #[inline(always)]
    pub fn busych3(&self) -> Busych3R {
        Busych3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Busy Channel 4"]
    #[inline(always)]
    pub fn busych4(&self) -> Busych4R {
        Busych4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Busy Channel 5"]
    #[inline(always)]
    pub fn busych5(&self) -> Busych5R {
        Busych5R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Busy Channels\n\nYou can [`read`](crate::Reg::read) this register and get [`busych::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BusychSpec;
impl crate::RegisterSpec for BusychSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`busych::R`](R) reader structure"]
impl crate::Readable for BusychSpec {}
#[doc = "`reset()` method sets BUSYCH to value 0"]
impl crate::Resettable for BusychSpec {
    const RESET_VALUE: u32 = 0;
}
