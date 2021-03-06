#[doc = "Reader of register CFG"]
pub type R = crate::R<u32, super::CFG>;
#[doc = "Writer for register CFG"]
pub type W = crate::W<u32, super::CFG>;
#[doc = "Register CFG `reset()`'s with value 0x0700"]
impl crate::ResetValue for super::CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0700
    }
}
#[doc = "CRC Mismatch Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMI_A {
    #[doc = "0: CRC Mismatch Interrupt is disabled"]
    VALUE1 = 0,
    #[doc = "1: CRC Mismatch Interrupt is enabled"]
    VALUE2 = 1,
}
impl From<CMI_A> for bool {
    #[inline(always)]
    fn from(variant: CMI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CMI`"]
pub type CMI_R = crate::R<bool, CMI_A>;
impl CMI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMI_A {
        match self.bits {
            false => CMI_A::VALUE1,
            true => CMI_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMI_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMI_A::VALUE2
    }
}
#[doc = "Write proxy for field `CMI`"]
pub struct CMI_W<'a> {
    w: &'a mut W,
}
impl<'a> CMI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CRC Mismatch Interrupt is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMI_A::VALUE1)
    }
    #[doc = "CRC Mismatch Interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMI_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Configuration Error Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEI_A {
    #[doc = "0: Configuration Error Interrupt is disabled"]
    VALUE1 = 0,
    #[doc = "1: Configuration Error Interrupt is enabled"]
    VALUE2 = 1,
}
impl From<CEI_A> for bool {
    #[inline(always)]
    fn from(variant: CEI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CEI`"]
pub type CEI_R = crate::R<bool, CEI_A>;
impl CEI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEI_A {
        match self.bits {
            false => CEI_A::VALUE1,
            true => CEI_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CEI_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CEI_A::VALUE2
    }
}
#[doc = "Write proxy for field `CEI`"]
pub struct CEI_W<'a> {
    w: &'a mut W,
}
impl<'a> CEI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Configuration Error Interrupt is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEI_A::VALUE1)
    }
    #[doc = "Configuration Error Interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEI_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Length Error Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEI_A {
    #[doc = "0: Length Error Interrupt is disabled"]
    VALUE1 = 0,
    #[doc = "1: Length Error Interrupt is enabled"]
    VALUE2 = 1,
}
impl From<LEI_A> for bool {
    #[inline(always)]
    fn from(variant: LEI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LEI`"]
pub type LEI_R = crate::R<bool, LEI_A>;
impl LEI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEI_A {
        match self.bits {
            false => LEI_A::VALUE1,
            true => LEI_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LEI_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LEI_A::VALUE2
    }
}
#[doc = "Write proxy for field `LEI`"]
pub struct LEI_W<'a> {
    w: &'a mut W,
}
impl<'a> LEI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Length Error Interrupt is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LEI_A::VALUE1)
    }
    #[doc = "Length Error Interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LEI_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Bus Error Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BEI_A {
    #[doc = "0: Bus Error Interrupt is disabled"]
    VALUE1 = 0,
    #[doc = "1: Bus Error Interrupt is enabled"]
    VALUE2 = 1,
}
impl From<BEI_A> for bool {
    #[inline(always)]
    fn from(variant: BEI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BEI`"]
pub type BEI_R = crate::R<bool, BEI_A>;
impl BEI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BEI_A {
        match self.bits {
            false => BEI_A::VALUE1,
            true => BEI_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BEI_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BEI_A::VALUE2
    }
}
#[doc = "Write proxy for field `BEI`"]
pub struct BEI_W<'a> {
    w: &'a mut W,
}
impl<'a> BEI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BEI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bus Error Interrupt is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BEI_A::VALUE1)
    }
    #[doc = "Bus Error Interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BEI_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "CRC Check Comparison\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCE_A {
    #[doc = "0: CRC check comparison at the end of a message is disabled"]
    VALUE1 = 0,
    #[doc = "1: CRC check comparison at the end of a message is enabled"]
    VALUE2 = 1,
}
impl From<CCE_A> for bool {
    #[inline(always)]
    fn from(variant: CCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CCE`"]
pub type CCE_R = crate::R<bool, CCE_A>;
impl CCE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCE_A {
        match self.bits {
            false => CCE_A::VALUE1,
            true => CCE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CCE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CCE_A::VALUE2
    }
}
#[doc = "Write proxy for field `CCE`"]
pub struct CCE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CRC check comparison at the end of a message is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CCE_A::VALUE1)
    }
    #[doc = "CRC check comparison at the end of a message is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CCE_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Automatic Length Reload\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALR_A {
    #[doc = "0: Disables automatic reload of the LENGTH field."]
    VALUE1 = 0,
    #[doc = "1: Enables automatic reload of the LENGTH field at the end of a message."]
    VALUE2 = 1,
}
impl From<ALR_A> for bool {
    #[inline(always)]
    fn from(variant: ALR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ALR`"]
pub type ALR_R = crate::R<bool, ALR_A>;
impl ALR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALR_A {
        match self.bits {
            false => ALR_A::VALUE1,
            true => ALR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ALR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ALR_A::VALUE2
    }
}
#[doc = "Write proxy for field `ALR`"]
pub struct ALR_W<'a> {
    w: &'a mut W,
}
impl<'a> ALR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables automatic reload of the LENGTH field."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ALR_A::VALUE1)
    }
    #[doc = "Enables automatic reload of the LENGTH field at the end of a message."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ALR_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "IR Byte Wise Reflection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFIN_A {
    #[doc = "0: IR Byte Wise Reflection is disabled"]
    VALUE1 = 0,
    #[doc = "1: IR Byte Wise Reflection is enabled"]
    VALUE2 = 1,
}
impl From<REFIN_A> for bool {
    #[inline(always)]
    fn from(variant: REFIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REFIN`"]
pub type REFIN_R = crate::R<bool, REFIN_A>;
impl REFIN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFIN_A {
        match self.bits {
            false => REFIN_A::VALUE1,
            true => REFIN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == REFIN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REFIN_A::VALUE2
    }
}
#[doc = "Write proxy for field `REFIN`"]
pub struct REFIN_W<'a> {
    w: &'a mut W,
}
impl<'a> REFIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFIN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "IR Byte Wise Reflection is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REFIN_A::VALUE1)
    }
    #[doc = "IR Byte Wise Reflection is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REFIN_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "CRC 32-Bit Wise Reflection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFOUT_A {
    #[doc = "0: CRC 32-bit wise is disabled"]
    VALUE1 = 0,
    #[doc = "1: CRC 32-bit wise is enabled"]
    VALUE2 = 1,
}
impl From<REFOUT_A> for bool {
    #[inline(always)]
    fn from(variant: REFOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REFOUT`"]
pub type REFOUT_R = crate::R<bool, REFOUT_A>;
impl REFOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFOUT_A {
        match self.bits {
            false => REFOUT_A::VALUE1,
            true => REFOUT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == REFOUT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REFOUT_A::VALUE2
    }
}
#[doc = "Write proxy for field `REFOUT`"]
pub struct REFOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> REFOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFOUT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CRC 32-bit wise is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REFOUT_A::VALUE1)
    }
    #[doc = "CRC 32-bit wise is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REFOUT_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Selects the value to be xored with the final CRC\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XSEL_A {
    #[doc = "0: 0x00000000"]
    VALUE1 = 0,
    #[doc = "1: 0xFFFFFFFF"]
    VALUE2 = 1,
}
impl From<XSEL_A> for bool {
    #[inline(always)]
    fn from(variant: XSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `XSEL`"]
pub type XSEL_R = crate::R<bool, XSEL_A>;
impl XSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XSEL_A {
        match self.bits {
            false => XSEL_A::VALUE1,
            true => XSEL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == XSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == XSEL_A::VALUE2
    }
}
#[doc = "Write proxy for field `XSEL`"]
pub struct XSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> XSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "0x00000000"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(XSEL_A::VALUE1)
    }
    #[doc = "0xFFFFFFFF"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(XSEL_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - CRC Mismatch Interrupt"]
    #[inline(always)]
    pub fn cmi(&self) -> CMI_R {
        CMI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Configuration Error Interrupt"]
    #[inline(always)]
    pub fn cei(&self) -> CEI_R {
        CEI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Length Error Interrupt"]
    #[inline(always)]
    pub fn lei(&self) -> LEI_R {
        LEI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Bus Error Interrupt"]
    #[inline(always)]
    pub fn bei(&self) -> BEI_R {
        BEI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CRC Check Comparison"]
    #[inline(always)]
    pub fn cce(&self) -> CCE_R {
        CCE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Automatic Length Reload"]
    #[inline(always)]
    pub fn alr(&self) -> ALR_R {
        ALR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - IR Byte Wise Reflection"]
    #[inline(always)]
    pub fn refin(&self) -> REFIN_R {
        REFIN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CRC 32-Bit Wise Reflection"]
    #[inline(always)]
    pub fn refout(&self) -> REFOUT_R {
        REFOUT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Selects the value to be xored with the final CRC"]
    #[inline(always)]
    pub fn xsel(&self) -> XSEL_R {
        XSEL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CRC Mismatch Interrupt"]
    #[inline(always)]
    pub fn cmi(&mut self) -> CMI_W {
        CMI_W { w: self }
    }
    #[doc = "Bit 1 - Configuration Error Interrupt"]
    #[inline(always)]
    pub fn cei(&mut self) -> CEI_W {
        CEI_W { w: self }
    }
    #[doc = "Bit 2 - Length Error Interrupt"]
    #[inline(always)]
    pub fn lei(&mut self) -> LEI_W {
        LEI_W { w: self }
    }
    #[doc = "Bit 3 - Bus Error Interrupt"]
    #[inline(always)]
    pub fn bei(&mut self) -> BEI_W {
        BEI_W { w: self }
    }
    #[doc = "Bit 4 - CRC Check Comparison"]
    #[inline(always)]
    pub fn cce(&mut self) -> CCE_W {
        CCE_W { w: self }
    }
    #[doc = "Bit 5 - Automatic Length Reload"]
    #[inline(always)]
    pub fn alr(&mut self) -> ALR_W {
        ALR_W { w: self }
    }
    #[doc = "Bit 8 - IR Byte Wise Reflection"]
    #[inline(always)]
    pub fn refin(&mut self) -> REFIN_W {
        REFIN_W { w: self }
    }
    #[doc = "Bit 9 - CRC 32-Bit Wise Reflection"]
    #[inline(always)]
    pub fn refout(&mut self) -> REFOUT_W {
        REFOUT_W { w: self }
    }
    #[doc = "Bit 10 - Selects the value to be xored with the final CRC"]
    #[inline(always)]
    pub fn xsel(&mut self) -> XSEL_W {
        XSEL_W { w: self }
    }
}
