#![doc = "Peripheral access API for XMC4400 microcontrollers (generated using svd2rust v0.14.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.14.0/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r" Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 6;
#[cfg(feature = "rt")]
extern "C" {
    fn SCU_0();
    fn ERU0_0();
    fn ERU0_1();
    fn ERU0_2();
    fn ERU0_3();
    fn ERU1_0();
    fn ERU1_1();
    fn ERU1_2();
    fn ERU1_3();
    fn PMU0_0();
    fn VADC0_C0_0();
    fn VADC0_C0_1();
    fn VADC0_C0_2();
    fn VADC0_C0_3();
    fn VADC0_G0_0();
    fn VADC0_G0_1();
    fn VADC0_G0_2();
    fn VADC0_G0_3();
    fn VADC0_G1_0();
    fn VADC0_G1_1();
    fn VADC0_G1_2();
    fn VADC0_G1_3();
    fn VADC0_G2_0();
    fn VADC0_G2_1();
    fn VADC0_G2_2();
    fn VADC0_G2_3();
    fn VADC0_G3_0();
    fn VADC0_G3_1();
    fn VADC0_G3_2();
    fn VADC0_G3_3();
    fn DSD0_M_0();
    fn DSD0_M_1();
    fn DSD0_M_2();
    fn DSD0_M_3();
    fn DSD0_A_4();
    fn DSD0_A_5();
    fn DSD0_A_6();
    fn DSD0_A_7();
    fn DAC0_0();
    fn DAC0_1();
    fn CCU40_0();
    fn CCU40_1();
    fn CCU40_2();
    fn CCU40_3();
    fn CCU41_0();
    fn CCU41_1();
    fn CCU41_2();
    fn CCU41_3();
    fn CCU42_0();
    fn CCU42_1();
    fn CCU42_2();
    fn CCU42_3();
    fn CCU43_0();
    fn CCU43_1();
    fn CCU43_2();
    fn CCU43_3();
    fn CCU80_0();
    fn CCU80_1();
    fn CCU80_2();
    fn CCU80_3();
    fn CCU81_0();
    fn CCU81_1();
    fn CCU81_2();
    fn CCU81_3();
    fn POSIF0_0();
    fn POSIF0_1();
    fn POSIF1_0();
    fn POSIF1_1();
    fn HRPWM_0();
    fn HRPWM_1();
    fn HRPWM_2();
    fn HRPWM_3();
    fn CAN0_0();
    fn CAN0_1();
    fn CAN0_2();
    fn CAN0_3();
    fn CAN0_4();
    fn CAN0_5();
    fn CAN0_6();
    fn CAN0_7();
    fn USIC0_0();
    fn USIC0_1();
    fn USIC0_2();
    fn USIC0_3();
    fn USIC0_4();
    fn USIC0_5();
    fn USIC1_0();
    fn USIC1_1();
    fn USIC1_2();
    fn USIC1_3();
    fn USIC1_4();
    fn USIC1_5();
    fn LEDTS0_0();
    fn FCE0_0();
    fn GPDMA0_0();
    fn USB0_0();
    fn ETH0_0();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 109] = [
    Vector { _handler: SCU_0 },
    Vector { _handler: ERU0_0 },
    Vector { _handler: ERU0_1 },
    Vector { _handler: ERU0_2 },
    Vector { _handler: ERU0_3 },
    Vector { _handler: ERU1_0 },
    Vector { _handler: ERU1_1 },
    Vector { _handler: ERU1_2 },
    Vector { _handler: ERU1_3 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: PMU0_0 },
    Vector { _reserved: 0 },
    Vector { _handler: VADC0_C0_0 },
    Vector { _handler: VADC0_C0_1 },
    Vector { _handler: VADC0_C0_2 },
    Vector { _handler: VADC0_C0_3 },
    Vector { _handler: VADC0_G0_0 },
    Vector { _handler: VADC0_G0_1 },
    Vector { _handler: VADC0_G0_2 },
    Vector { _handler: VADC0_G0_3 },
    Vector { _handler: VADC0_G1_0 },
    Vector { _handler: VADC0_G1_1 },
    Vector { _handler: VADC0_G1_2 },
    Vector { _handler: VADC0_G1_3 },
    Vector { _handler: VADC0_G2_0 },
    Vector { _handler: VADC0_G2_1 },
    Vector { _handler: VADC0_G2_2 },
    Vector { _handler: VADC0_G2_3 },
    Vector { _handler: VADC0_G3_0 },
    Vector { _handler: VADC0_G3_1 },
    Vector { _handler: VADC0_G3_2 },
    Vector { _handler: VADC0_G3_3 },
    Vector { _handler: DSD0_M_0 },
    Vector { _handler: DSD0_M_1 },
    Vector { _handler: DSD0_M_2 },
    Vector { _handler: DSD0_M_3 },
    Vector { _handler: DSD0_A_4 },
    Vector { _handler: DSD0_A_5 },
    Vector { _handler: DSD0_A_6 },
    Vector { _handler: DSD0_A_7 },
    Vector { _handler: DAC0_0 },
    Vector { _handler: DAC0_1 },
    Vector { _handler: CCU40_0 },
    Vector { _handler: CCU40_1 },
    Vector { _handler: CCU40_2 },
    Vector { _handler: CCU40_3 },
    Vector { _handler: CCU41_0 },
    Vector { _handler: CCU41_1 },
    Vector { _handler: CCU41_2 },
    Vector { _handler: CCU41_3 },
    Vector { _handler: CCU42_0 },
    Vector { _handler: CCU42_1 },
    Vector { _handler: CCU42_2 },
    Vector { _handler: CCU42_3 },
    Vector { _handler: CCU43_0 },
    Vector { _handler: CCU43_1 },
    Vector { _handler: CCU43_2 },
    Vector { _handler: CCU43_3 },
    Vector { _handler: CCU80_0 },
    Vector { _handler: CCU80_1 },
    Vector { _handler: CCU80_2 },
    Vector { _handler: CCU80_3 },
    Vector { _handler: CCU81_0 },
    Vector { _handler: CCU81_1 },
    Vector { _handler: CCU81_2 },
    Vector { _handler: CCU81_3 },
    Vector { _handler: POSIF0_0 },
    Vector { _handler: POSIF0_1 },
    Vector { _handler: POSIF1_0 },
    Vector { _handler: POSIF1_1 },
    Vector { _handler: HRPWM_0 },
    Vector { _handler: HRPWM_1 },
    Vector { _handler: HRPWM_2 },
    Vector { _handler: HRPWM_3 },
    Vector { _handler: CAN0_0 },
    Vector { _handler: CAN0_1 },
    Vector { _handler: CAN0_2 },
    Vector { _handler: CAN0_3 },
    Vector { _handler: CAN0_4 },
    Vector { _handler: CAN0_5 },
    Vector { _handler: CAN0_6 },
    Vector { _handler: CAN0_7 },
    Vector { _handler: USIC0_0 },
    Vector { _handler: USIC0_1 },
    Vector { _handler: USIC0_2 },
    Vector { _handler: USIC0_3 },
    Vector { _handler: USIC0_4 },
    Vector { _handler: USIC0_5 },
    Vector { _handler: USIC1_0 },
    Vector { _handler: USIC1_1 },
    Vector { _handler: USIC1_2 },
    Vector { _handler: USIC1_3 },
    Vector { _handler: USIC1_4 },
    Vector { _handler: USIC1_5 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: LEDTS0_0 },
    Vector { _reserved: 0 },
    Vector { _handler: FCE0_0 },
    Vector { _handler: GPDMA0_0 },
    Vector { _reserved: 0 },
    Vector { _handler: USB0_0 },
    Vector { _handler: ETH0_0 },
];
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "0 - System Control"]
    SCU_0,
    #[doc = "1 - External Request Unit 0"]
    ERU0_0,
    #[doc = "2 - External Request Unit 0"]
    ERU0_1,
    #[doc = "3 - External Request Unit 0"]
    ERU0_2,
    #[doc = "4 - External Request Unit 0"]
    ERU0_3,
    #[doc = "5 - External Request Unit 1"]
    ERU1_0,
    #[doc = "6 - External Request Unit 1"]
    ERU1_1,
    #[doc = "7 - External Request Unit 1"]
    ERU1_2,
    #[doc = "8 - External Request Unit 1"]
    ERU1_3,
    #[doc = "12 - Program Management Unit"]
    PMU0_0,
    #[doc = "14 - Analog to Digital Converter Common Block 0"]
    VADC0_C0_0,
    #[doc = "15 - Analog to Digital Converter Common Block 0"]
    VADC0_C0_1,
    #[doc = "16 - Analog to Digital Converter Common Block 0"]
    VADC0_C0_2,
    #[doc = "17 - Analog to Digital Converter Common Block 0"]
    VADC0_C0_3,
    #[doc = "18 - Analog to Digital Converter Group 0"]
    VADC0_G0_0,
    #[doc = "19 - Analog to Digital Converter Group 0"]
    VADC0_G0_1,
    #[doc = "20 - Analog to Digital Converter Group 0"]
    VADC0_G0_2,
    #[doc = "21 - Analog to Digital Converter Group 0"]
    VADC0_G0_3,
    #[doc = "22 - Analog to Digital Converter Group 1"]
    VADC0_G1_0,
    #[doc = "23 - Analog to Digital Converter Group 1"]
    VADC0_G1_1,
    #[doc = "24 - Analog to Digital Converter Group 1"]
    VADC0_G1_2,
    #[doc = "25 - Analog to Digital Converter Group 1"]
    VADC0_G1_3,
    #[doc = "26 - Analog to Digital Converter Group 2"]
    VADC0_G2_0,
    #[doc = "27 - Analog to Digital Converter Group 2"]
    VADC0_G2_1,
    #[doc = "28 - Analog to Digital Converter Group 2"]
    VADC0_G2_2,
    #[doc = "29 - Analog to Digital Converter Group 2"]
    VADC0_G2_3,
    #[doc = "30 - Analog to Digital Converter Group 3"]
    VADC0_G3_0,
    #[doc = "31 - Analog to Digital Converter Group 3"]
    VADC0_G3_1,
    #[doc = "32 - Analog to Digital Converter Group 3"]
    VADC0_G3_2,
    #[doc = "33 - Analog to Digital Converter Group 3"]
    VADC0_G3_3,
    #[doc = "34 - Delta Sigma Demodulator Main"]
    DSD0_M_0,
    #[doc = "35 - Delta Sigma Demodulator Main"]
    DSD0_M_1,
    #[doc = "36 - Delta Sigma Demodulator Main"]
    DSD0_M_2,
    #[doc = "37 - Delta Sigma Demodulator Main"]
    DSD0_M_3,
    #[doc = "38 - Delta Sigma Demodulator Auxiliary"]
    DSD0_A_4,
    #[doc = "39 - Delta Sigma Demodulator Auxiliary"]
    DSD0_A_5,
    #[doc = "40 - Delta Sigma Demodulator Auxiliary"]
    DSD0_A_6,
    #[doc = "41 - Delta Sigma Demodulator Auxiliary"]
    DSD0_A_7,
    #[doc = "42 - Digital to Analog Converter"]
    DAC0_0,
    #[doc = "43 - Digital to Analog Converter"]
    DAC0_1,
    #[doc = "44 - Capture Compare Unit 4 (Module 0)"]
    CCU40_0,
    #[doc = "45 - Capture Compare Unit 4 (Module 0)"]
    CCU40_1,
    #[doc = "46 - Capture Compare Unit 4 (Module 0)"]
    CCU40_2,
    #[doc = "47 - Capture Compare Unit 4 (Module 0)"]
    CCU40_3,
    #[doc = "48 - Capture Compare Unit 4 (Module 1)"]
    CCU41_0,
    #[doc = "49 - Capture Compare Unit 4 (Module 1)"]
    CCU41_1,
    #[doc = "50 - Capture Compare Unit 4 (Module 1)"]
    CCU41_2,
    #[doc = "51 - Capture Compare Unit 4 (Module 1)"]
    CCU41_3,
    #[doc = "52 - Capture Compare Unit 4 (Module 2)"]
    CCU42_0,
    #[doc = "53 - Capture Compare Unit 4 (Module 2)"]
    CCU42_1,
    #[doc = "54 - Capture Compare Unit 4 (Module 2)"]
    CCU42_2,
    #[doc = "55 - Capture Compare Unit 4 (Module 2)"]
    CCU42_3,
    #[doc = "56 - Capture Compare Unit 4 (Module 3)"]
    CCU43_0,
    #[doc = "57 - Capture Compare Unit 4 (Module 3)"]
    CCU43_1,
    #[doc = "58 - Capture Compare Unit 4 (Module 3)"]
    CCU43_2,
    #[doc = "59 - Capture Compare Unit 4 (Module 3)"]
    CCU43_3,
    #[doc = "60 - Capture Compare Unit 8 (Module 0)"]
    CCU80_0,
    #[doc = "61 - Capture Compare Unit 8 (Module 0)"]
    CCU80_1,
    #[doc = "62 - Capture Compare Unit 8 (Module 0)"]
    CCU80_2,
    #[doc = "63 - Capture Compare Unit 8 (Module 0)"]
    CCU80_3,
    #[doc = "64 - Capture Compare Unit 8 (Module 1)"]
    CCU81_0,
    #[doc = "65 - Capture Compare Unit 8 (Module 1)"]
    CCU81_1,
    #[doc = "66 - Capture Compare Unit 8 (Module 1)"]
    CCU81_2,
    #[doc = "67 - Capture Compare Unit 8 (Module 1)"]
    CCU81_3,
    #[doc = "68 - Position Interface (Module 0)"]
    POSIF0_0,
    #[doc = "69 - Position Interface (Module 0)"]
    POSIF0_1,
    #[doc = "70 - Position Interface (Module 1)"]
    POSIF1_0,
    #[doc = "71 - Position Interface (Module 1)"]
    POSIF1_1,
    #[doc = "72 - High Resolution Pulse Width Modulation (Module 0)"]
    HRPWM_0,
    #[doc = "73 - High Resolution Pulse Width Modulation (Module 0)"]
    HRPWM_1,
    #[doc = "74 - High Resolution Pulse Width Modulation (Module 0)"]
    HRPWM_2,
    #[doc = "75 - High Resolution Pulse Width Modulation (Module 0)"]
    HRPWM_3,
    #[doc = "76 - MultiCAN"]
    CAN0_0,
    #[doc = "77 - MultiCAN"]
    CAN0_1,
    #[doc = "78 - MultiCAN"]
    CAN0_2,
    #[doc = "79 - MultiCAN"]
    CAN0_3,
    #[doc = "80 - MultiCAN"]
    CAN0_4,
    #[doc = "81 - MultiCAN"]
    CAN0_5,
    #[doc = "82 - MultiCAN"]
    CAN0_6,
    #[doc = "83 - MultiCAN"]
    CAN0_7,
    #[doc = "84 - Universal Serial Interface Channel (Module 0)"]
    USIC0_0,
    #[doc = "85 - Universal Serial Interface Channel (Module 0)"]
    USIC0_1,
    #[doc = "86 - Universal Serial Interface Channel (Module 0)"]
    USIC0_2,
    #[doc = "87 - Universal Serial Interface Channel (Module 0)"]
    USIC0_3,
    #[doc = "88 - Universal Serial Interface Channel (Module 0)"]
    USIC0_4,
    #[doc = "89 - Universal Serial Interface Channel (Module 0)"]
    USIC0_5,
    #[doc = "90 - Universal Serial Interface Channel (Module 1)"]
    USIC1_0,
    #[doc = "91 - Universal Serial Interface Channel (Module 1)"]
    USIC1_1,
    #[doc = "92 - Universal Serial Interface Channel (Module 1)"]
    USIC1_2,
    #[doc = "93 - Universal Serial Interface Channel (Module 1)"]
    USIC1_3,
    #[doc = "94 - Universal Serial Interface Channel (Module 1)"]
    USIC1_4,
    #[doc = "95 - Universal Serial Interface Channel (Module 1)"]
    USIC1_5,
    #[doc = "102 - LED and Touch Sense Control Unit (Module 0)"]
    LEDTS0_0,
    #[doc = "104 - Flexible CRC Engine"]
    FCE0_0,
    #[doc = "105 - General Purpose DMA Unit 0"]
    GPDMA0_0,
    #[doc = "107 - Universal Serial Bus (Module 0)"]
    USB0_0,
    #[doc = "108 - Ethernet (Module 0)"]
    ETH0_0,
}
unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::SCU_0 => 0,
            Interrupt::ERU0_0 => 1,
            Interrupt::ERU0_1 => 2,
            Interrupt::ERU0_2 => 3,
            Interrupt::ERU0_3 => 4,
            Interrupt::ERU1_0 => 5,
            Interrupt::ERU1_1 => 6,
            Interrupt::ERU1_2 => 7,
            Interrupt::ERU1_3 => 8,
            Interrupt::PMU0_0 => 12,
            Interrupt::VADC0_C0_0 => 14,
            Interrupt::VADC0_C0_1 => 15,
            Interrupt::VADC0_C0_2 => 16,
            Interrupt::VADC0_C0_3 => 17,
            Interrupt::VADC0_G0_0 => 18,
            Interrupt::VADC0_G0_1 => 19,
            Interrupt::VADC0_G0_2 => 20,
            Interrupt::VADC0_G0_3 => 21,
            Interrupt::VADC0_G1_0 => 22,
            Interrupt::VADC0_G1_1 => 23,
            Interrupt::VADC0_G1_2 => 24,
            Interrupt::VADC0_G1_3 => 25,
            Interrupt::VADC0_G2_0 => 26,
            Interrupt::VADC0_G2_1 => 27,
            Interrupt::VADC0_G2_2 => 28,
            Interrupt::VADC0_G2_3 => 29,
            Interrupt::VADC0_G3_0 => 30,
            Interrupt::VADC0_G3_1 => 31,
            Interrupt::VADC0_G3_2 => 32,
            Interrupt::VADC0_G3_3 => 33,
            Interrupt::DSD0_M_0 => 34,
            Interrupt::DSD0_M_1 => 35,
            Interrupt::DSD0_M_2 => 36,
            Interrupt::DSD0_M_3 => 37,
            Interrupt::DSD0_A_4 => 38,
            Interrupt::DSD0_A_5 => 39,
            Interrupt::DSD0_A_6 => 40,
            Interrupt::DSD0_A_7 => 41,
            Interrupt::DAC0_0 => 42,
            Interrupt::DAC0_1 => 43,
            Interrupt::CCU40_0 => 44,
            Interrupt::CCU40_1 => 45,
            Interrupt::CCU40_2 => 46,
            Interrupt::CCU40_3 => 47,
            Interrupt::CCU41_0 => 48,
            Interrupt::CCU41_1 => 49,
            Interrupt::CCU41_2 => 50,
            Interrupt::CCU41_3 => 51,
            Interrupt::CCU42_0 => 52,
            Interrupt::CCU42_1 => 53,
            Interrupt::CCU42_2 => 54,
            Interrupt::CCU42_3 => 55,
            Interrupt::CCU43_0 => 56,
            Interrupt::CCU43_1 => 57,
            Interrupt::CCU43_2 => 58,
            Interrupt::CCU43_3 => 59,
            Interrupt::CCU80_0 => 60,
            Interrupt::CCU80_1 => 61,
            Interrupt::CCU80_2 => 62,
            Interrupt::CCU80_3 => 63,
            Interrupt::CCU81_0 => 64,
            Interrupt::CCU81_1 => 65,
            Interrupt::CCU81_2 => 66,
            Interrupt::CCU81_3 => 67,
            Interrupt::POSIF0_0 => 68,
            Interrupt::POSIF0_1 => 69,
            Interrupt::POSIF1_0 => 70,
            Interrupt::POSIF1_1 => 71,
            Interrupt::HRPWM_0 => 72,
            Interrupt::HRPWM_1 => 73,
            Interrupt::HRPWM_2 => 74,
            Interrupt::HRPWM_3 => 75,
            Interrupt::CAN0_0 => 76,
            Interrupt::CAN0_1 => 77,
            Interrupt::CAN0_2 => 78,
            Interrupt::CAN0_3 => 79,
            Interrupt::CAN0_4 => 80,
            Interrupt::CAN0_5 => 81,
            Interrupt::CAN0_6 => 82,
            Interrupt::CAN0_7 => 83,
            Interrupt::USIC0_0 => 84,
            Interrupt::USIC0_1 => 85,
            Interrupt::USIC0_2 => 86,
            Interrupt::USIC0_3 => 87,
            Interrupt::USIC0_4 => 88,
            Interrupt::USIC0_5 => 89,
            Interrupt::USIC1_0 => 90,
            Interrupt::USIC1_1 => 91,
            Interrupt::USIC1_2 => 92,
            Interrupt::USIC1_3 => 93,
            Interrupt::USIC1_4 => 94,
            Interrupt::USIC1_5 => 95,
            Interrupt::LEDTS0_0 => 102,
            Interrupt::FCE0_0 => 104,
            Interrupt::GPDMA0_0 => 105,
            Interrupt::USB0_0 => 107,
            Interrupt::ETH0_0 => 108,
        }
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[doc = "Cortex-M4 Private Peripheral Block"]
pub struct PPB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PPB {}
impl PPB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ppb::RegisterBlock {
        3758153728 as *const _
    }
}
impl Deref for PPB {
    type Target = ppb::RegisterBlock;
    fn deref(&self) -> &ppb::RegisterBlock {
        unsafe { &*PPB::ptr() }
    }
}
#[doc = "Cortex-M4 Private Peripheral Block"]
pub mod ppb;
#[doc = "DMA Line Router"]
pub struct DLR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DLR {}
impl DLR {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dlr::RegisterBlock {
        1342195968 as *const _
    }
}
impl Deref for DLR {
    type Target = dlr::RegisterBlock;
    fn deref(&self) -> &dlr::RegisterBlock {
        unsafe { &*DLR::ptr() }
    }
}
#[doc = "DMA Line Router"]
pub mod dlr;
#[doc = "Event Request Unit 0"]
pub struct ERU0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ERU0 {}
impl ERU0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const eru0::RegisterBlock {
        1342195712 as *const _
    }
}
impl Deref for ERU0 {
    type Target = eru0::RegisterBlock;
    fn deref(&self) -> &eru0::RegisterBlock {
        unsafe { &*ERU0::ptr() }
    }
}
#[doc = "Event Request Unit 0"]
pub mod eru0;
#[doc = "Event Request Unit 1"]
pub struct ERU1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ERU1 {}
impl ERU1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const eru0::RegisterBlock {
        1074020352 as *const _
    }
}
impl Deref for ERU1 {
    type Target = eru0::RegisterBlock;
    fn deref(&self) -> &eru0::RegisterBlock {
        unsafe { &*ERU1::ptr() }
    }
}
#[doc = "General Purpose DMA Unit 0"]
pub struct GPDMA0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPDMA0 {}
impl GPDMA0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpdma0::RegisterBlock {
        1342259904 as *const _
    }
}
impl Deref for GPDMA0 {
    type Target = gpdma0::RegisterBlock;
    fn deref(&self) -> &gpdma0::RegisterBlock {
        unsafe { &*GPDMA0::ptr() }
    }
}
#[doc = "General Purpose DMA Unit 0"]
pub mod gpdma0;
#[doc = "General Purpose DMA Unit 0"]
pub struct GPDMA0_CH0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPDMA0_CH0 {}
impl GPDMA0_CH0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpdma0_ch0::RegisterBlock {
        1342259200 as *const _
    }
}
impl Deref for GPDMA0_CH0 {
    type Target = gpdma0_ch0::RegisterBlock;
    fn deref(&self) -> &gpdma0_ch0::RegisterBlock {
        unsafe { &*GPDMA0_CH0::ptr() }
    }
}
#[doc = "General Purpose DMA Unit 0"]
pub mod gpdma0_ch0;
#[doc = "General Purpose DMA Unit 0"]
pub struct GPDMA0_CH1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPDMA0_CH1 {}
impl GPDMA0_CH1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpdma0_ch0::RegisterBlock {
        1342259288 as *const _
    }
}
impl Deref for GPDMA0_CH1 {
    type Target = gpdma0_ch0::RegisterBlock;
    fn deref(&self) -> &gpdma0_ch0::RegisterBlock {
        unsafe { &*GPDMA0_CH1::ptr() }
    }
}
#[doc = "General Purpose DMA Unit 0"]
pub struct GPDMA0_CH2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPDMA0_CH2 {}
impl GPDMA0_CH2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpdma0_ch2::RegisterBlock {
        1342259376 as *const _
    }
}
impl Deref for GPDMA0_CH2 {
    type Target = gpdma0_ch2::RegisterBlock;
    fn deref(&self) -> &gpdma0_ch2::RegisterBlock {
        unsafe { &*GPDMA0_CH2::ptr() }
    }
}
#[doc = "General Purpose DMA Unit 0"]
pub mod gpdma0_ch2;
#[doc = "General Purpose DMA Unit 0"]
pub struct GPDMA0_CH3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPDMA0_CH3 {}
impl GPDMA0_CH3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpdma0_ch2::RegisterBlock {
        1342259464 as *const _
    }
}
impl Deref for GPDMA0_CH3 {
    type Target = gpdma0_ch2::RegisterBlock;
    fn deref(&self) -> &gpdma0_ch2::RegisterBlock {
        unsafe { &*GPDMA0_CH3::ptr() }
    }
}
#[doc = "General Purpose DMA Unit 0"]
pub struct GPDMA0_CH4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPDMA0_CH4 {}
impl GPDMA0_CH4 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpdma0_ch2::RegisterBlock {
        1342259552 as *const _
    }
}
impl Deref for GPDMA0_CH4 {
    type Target = gpdma0_ch2::RegisterBlock;
    fn deref(&self) -> &gpdma0_ch2::RegisterBlock {
        unsafe { &*GPDMA0_CH4::ptr() }
    }
}
#[doc = "General Purpose DMA Unit 0"]
pub struct GPDMA0_CH5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPDMA0_CH5 {}
impl GPDMA0_CH5 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpdma0_ch2::RegisterBlock {
        1342259640 as *const _
    }
}
impl Deref for GPDMA0_CH5 {
    type Target = gpdma0_ch2::RegisterBlock;
    fn deref(&self) -> &gpdma0_ch2::RegisterBlock {
        unsafe { &*GPDMA0_CH5::ptr() }
    }
}
#[doc = "General Purpose DMA Unit 0"]
pub struct GPDMA0_CH6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPDMA0_CH6 {}
impl GPDMA0_CH6 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpdma0_ch2::RegisterBlock {
        1342259728 as *const _
    }
}
impl Deref for GPDMA0_CH6 {
    type Target = gpdma0_ch2::RegisterBlock;
    fn deref(&self) -> &gpdma0_ch2::RegisterBlock {
        unsafe { &*GPDMA0_CH6::ptr() }
    }
}
#[doc = "General Purpose DMA Unit 0"]
pub struct GPDMA0_CH7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPDMA0_CH7 {}
impl GPDMA0_CH7 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpdma0_ch2::RegisterBlock {
        1342259816 as *const _
    }
}
impl Deref for GPDMA0_CH7 {
    type Target = gpdma0_ch2::RegisterBlock;
    fn deref(&self) -> &gpdma0_ch2::RegisterBlock {
        unsafe { &*GPDMA0_CH7::ptr() }
    }
}
#[doc = "Flexible CRC Engine"]
pub struct FCE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FCE {}
impl FCE {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const fce::RegisterBlock {
        1342308352 as *const _
    }
}
impl Deref for FCE {
    type Target = fce::RegisterBlock;
    fn deref(&self) -> &fce::RegisterBlock {
        unsafe { &*FCE::ptr() }
    }
}
#[doc = "Flexible CRC Engine"]
pub mod fce;
#[doc = "Flexible CRC Engine"]
pub struct FCE_KE0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FCE_KE0 {}
impl FCE_KE0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const fce_ke0::RegisterBlock {
        1342308384 as *const _
    }
}
impl Deref for FCE_KE0 {
    type Target = fce_ke0::RegisterBlock;
    fn deref(&self) -> &fce_ke0::RegisterBlock {
        unsafe { &*FCE_KE0::ptr() }
    }
}
#[doc = "Flexible CRC Engine"]
pub mod fce_ke0;
#[doc = "Flexible CRC Engine"]
pub struct FCE_KE1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FCE_KE1 {}
impl FCE_KE1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const fce_ke0::RegisterBlock {
        1342308416 as *const _
    }
}
impl Deref for FCE_KE1 {
    type Target = fce_ke0::RegisterBlock;
    fn deref(&self) -> &fce_ke0::RegisterBlock {
        unsafe { &*FCE_KE1::ptr() }
    }
}
#[doc = "Flexible CRC Engine"]
pub struct FCE_KE2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FCE_KE2 {}
impl FCE_KE2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const fce_ke0::RegisterBlock {
        1342308448 as *const _
    }
}
impl Deref for FCE_KE2 {
    type Target = fce_ke0::RegisterBlock;
    fn deref(&self) -> &fce_ke0::RegisterBlock {
        unsafe { &*FCE_KE2::ptr() }
    }
}
#[doc = "Flexible CRC Engine"]
pub struct FCE_KE3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FCE_KE3 {}
impl FCE_KE3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const fce_ke0::RegisterBlock {
        1342308480 as *const _
    }
}
impl Deref for FCE_KE3 {
    type Target = fce_ke0::RegisterBlock;
    fn deref(&self) -> &fce_ke0::RegisterBlock {
        unsafe { &*FCE_KE3::ptr() }
    }
}
#[doc = "Peripheral Bridge AHB 0"]
pub struct PBA0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PBA0 {}
impl PBA0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pba0::RegisterBlock {
        1073741824 as *const _
    }
}
impl Deref for PBA0 {
    type Target = pba0::RegisterBlock;
    fn deref(&self) -> &pba0::RegisterBlock {
        unsafe { &*PBA0::ptr() }
    }
}
#[doc = "Peripheral Bridge AHB 0"]
pub mod pba0;
#[doc = "Peripheral Bridge AHB 1"]
pub struct PBA1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PBA1 {}
impl PBA1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pba0::RegisterBlock {
        1207959552 as *const _
    }
}
impl Deref for PBA1 {
    type Target = pba0::RegisterBlock;
    fn deref(&self) -> &pba0::RegisterBlock {
        unsafe { &*PBA1::ptr() }
    }
}
#[doc = "Flash Memory Controller"]
pub struct FLASH0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASH0 {}
impl FLASH0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const flash0::RegisterBlock {
        1476399104 as *const _
    }
}
impl Deref for FLASH0 {
    type Target = flash0::RegisterBlock;
    fn deref(&self) -> &flash0::RegisterBlock {
        unsafe { &*FLASH0::ptr() }
    }
}
#[doc = "Flash Memory Controller"]
pub mod flash0;
#[doc = "Prefetch Unit"]
pub struct PREF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PREF {}
impl PREF {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pref::RegisterBlock {
        1476411392 as *const _
    }
}
impl Deref for PREF {
    type Target = pref::RegisterBlock;
    fn deref(&self) -> &pref::RegisterBlock {
        unsafe { &*PREF::ptr() }
    }
}
#[doc = "Prefetch Unit"]
pub mod pref;
#[doc = "Program Management Unit"]
pub struct PMU0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMU0 {}
impl PMU0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pmu0::RegisterBlock {
        1476396296 as *const _
    }
}
impl Deref for PMU0 {
    type Target = pmu0::RegisterBlock;
    fn deref(&self) -> &pmu0::RegisterBlock {
        unsafe { &*PMU0::ptr() }
    }
}
#[doc = "Program Management Unit"]
pub mod pmu0;
#[doc = "Watch Dog Timer"]
pub struct WDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT {}
impl WDT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wdt::RegisterBlock {
        1342210048 as *const _
    }
}
impl Deref for WDT {
    type Target = wdt::RegisterBlock;
    fn deref(&self) -> &wdt::RegisterBlock {
        unsafe { &*WDT::ptr() }
    }
}
#[doc = "Watch Dog Timer"]
pub mod wdt;
#[doc = "Real Time Clock"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtc::RegisterBlock {
        1342196224 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    fn deref(&self) -> &rtc::RegisterBlock {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Real Time Clock"]
pub mod rtc;
#[doc = "System Control Unit"]
pub struct SCU_CLK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCU_CLK {}
impl SCU_CLK {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const scu_clk::RegisterBlock {
        1342195200 as *const _
    }
}
impl Deref for SCU_CLK {
    type Target = scu_clk::RegisterBlock;
    fn deref(&self) -> &scu_clk::RegisterBlock {
        unsafe { &*SCU_CLK::ptr() }
    }
}
#[doc = "System Control Unit"]
pub mod scu_clk;
#[doc = "System Control Unit"]
pub struct SCU_OSC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCU_OSC {}
impl SCU_OSC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const scu_osc::RegisterBlock {
        1342195456 as *const _
    }
}
impl Deref for SCU_OSC {
    type Target = scu_osc::RegisterBlock;
    fn deref(&self) -> &scu_osc::RegisterBlock {
        unsafe { &*SCU_OSC::ptr() }
    }
}
#[doc = "System Control Unit"]
pub mod scu_osc;
#[doc = "System Control Unit"]
pub struct SCU_PLL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCU_PLL {}
impl SCU_PLL {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const scu_pll::RegisterBlock {
        1342195472 as *const _
    }
}
impl Deref for SCU_PLL {
    type Target = scu_pll::RegisterBlock;
    fn deref(&self) -> &scu_pll::RegisterBlock {
        unsafe { &*SCU_PLL::ptr() }
    }
}
#[doc = "System Control Unit"]
pub mod scu_pll;
#[doc = "System Control Unit"]
pub struct SCU_GENERAL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCU_GENERAL {}
impl SCU_GENERAL {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const scu_general::RegisterBlock {
        1342193664 as *const _
    }
}
impl Deref for SCU_GENERAL {
    type Target = scu_general::RegisterBlock;
    fn deref(&self) -> &scu_general::RegisterBlock {
        unsafe { &*SCU_GENERAL::ptr() }
    }
}
#[doc = "System Control Unit"]
pub mod scu_general;
#[doc = "System Control Unit"]
pub struct SCU_INTERRUPT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCU_INTERRUPT {}
impl SCU_INTERRUPT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const scu_interrupt::RegisterBlock {
        1342193780 as *const _
    }
}
impl Deref for SCU_INTERRUPT {
    type Target = scu_interrupt::RegisterBlock;
    fn deref(&self) -> &scu_interrupt::RegisterBlock {
        unsafe { &*SCU_INTERRUPT::ptr() }
    }
}
#[doc = "System Control Unit"]
pub mod scu_interrupt;
#[doc = "System Control Unit"]
pub struct SCU_PARITY {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCU_PARITY {}
impl SCU_PARITY {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const scu_parity::RegisterBlock {
        1342193980 as *const _
    }
}
impl Deref for SCU_PARITY {
    type Target = scu_parity::RegisterBlock;
    fn deref(&self) -> &scu_parity::RegisterBlock {
        unsafe { &*SCU_PARITY::ptr() }
    }
}
#[doc = "System Control Unit"]
pub mod scu_parity;
#[doc = "System Control Unit"]
pub struct SCU_TRAP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCU_TRAP {}
impl SCU_TRAP {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const scu_trap::RegisterBlock {
        1342194016 as *const _
    }
}
impl Deref for SCU_TRAP {
    type Target = scu_trap::RegisterBlock;
    fn deref(&self) -> &scu_trap::RegisterBlock {
        unsafe { &*SCU_TRAP::ptr() }
    }
}
#[doc = "System Control Unit"]
pub mod scu_trap;
#[doc = "System Control Unit"]
pub struct SCU_HIBERNATE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCU_HIBERNATE {}
impl SCU_HIBERNATE {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const scu_hibernate::RegisterBlock {
        1342194432 as *const _
    }
}
impl Deref for SCU_HIBERNATE {
    type Target = scu_hibernate::RegisterBlock;
    fn deref(&self) -> &scu_hibernate::RegisterBlock {
        unsafe { &*SCU_HIBERNATE::ptr() }
    }
}
#[doc = "System Control Unit"]
pub mod scu_hibernate;
#[doc = "System Control Unit"]
pub struct SCU_POWER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCU_POWER {}
impl SCU_POWER {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const scu_power::RegisterBlock {
        1342194176 as *const _
    }
}
impl Deref for SCU_POWER {
    type Target = scu_power::RegisterBlock;
    fn deref(&self) -> &scu_power::RegisterBlock {
        unsafe { &*SCU_POWER::ptr() }
    }
}
#[doc = "System Control Unit"]
pub mod scu_power;
#[doc = "System Control Unit"]
pub struct SCU_RESET {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCU_RESET {}
impl SCU_RESET {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const scu_reset::RegisterBlock {
        1342194688 as *const _
    }
}
impl Deref for SCU_RESET {
    type Target = scu_reset::RegisterBlock;
    fn deref(&self) -> &scu_reset::RegisterBlock {
        unsafe { &*SCU_RESET::ptr() }
    }
}
#[doc = "System Control Unit"]
pub mod scu_reset;
#[doc = "LED and Touch Sense Unit 0"]
pub struct LEDTS0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LEDTS0 {}
impl LEDTS0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ledts0::RegisterBlock {
        1208025088 as *const _
    }
}
impl Deref for LEDTS0 {
    type Target = ledts0::RegisterBlock;
    fn deref(&self) -> &ledts0::RegisterBlock {
        unsafe { &*LEDTS0::ptr() }
    }
}
#[doc = "LED and Touch Sense Unit 0"]
pub mod ledts0;
#[doc = "Ethernet Control Register"]
pub struct ETH0_CON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ETH0_CON {}
impl ETH0_CON {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const eth0_con::RegisterBlock {
        1342193728 as *const _
    }
}
impl Deref for ETH0_CON {
    type Target = eth0_con::RegisterBlock;
    fn deref(&self) -> &eth0_con::RegisterBlock {
        unsafe { &*ETH0_CON::ptr() }
    }
}
#[doc = "Ethernet Control Register"]
pub mod eth0_con;
#[doc = "Ethernet Unit 0"]
pub struct ETH0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ETH0 {}
impl ETH0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const eth0::RegisterBlock {
        1342226432 as *const _
    }
}
impl Deref for ETH0 {
    type Target = eth0::RegisterBlock;
    fn deref(&self) -> &eth0::RegisterBlock {
        unsafe { &*ETH0::ptr() }
    }
}
#[doc = "Ethernet Unit 0"]
pub mod eth0;
#[doc = "Universal Serial Bus"]
pub struct USB0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0 {}
impl USB0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usb0::RegisterBlock {
        1342439424 as *const _
    }
}
impl Deref for USB0 {
    type Target = usb0::RegisterBlock;
    fn deref(&self) -> &usb0::RegisterBlock {
        unsafe { &*USB0::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub mod usb0;
#[doc = "Universal Serial Bus"]
pub struct USB0_EP0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_EP0 {}
impl USB0_EP0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usb0_ep0::RegisterBlock {
        1342441728 as *const _
    }
}
impl Deref for USB0_EP0 {
    type Target = usb0_ep0::RegisterBlock;
    fn deref(&self) -> &usb0_ep0::RegisterBlock {
        unsafe { &*USB0_EP0::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub mod usb0_ep0;
#[doc = "Universal Serial Bus"]
pub struct USB0_EP1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_EP1 {}
impl USB0_EP1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usb0_ep1::RegisterBlock {
        1342441760 as *const _
    }
}
impl Deref for USB0_EP1 {
    type Target = usb0_ep1::RegisterBlock;
    fn deref(&self) -> &usb0_ep1::RegisterBlock {
        unsafe { &*USB0_EP1::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub mod usb0_ep1;
#[doc = "Universal Serial Bus"]
pub struct USB0_EP2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_EP2 {}
impl USB0_EP2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usb0_ep1::RegisterBlock {
        1342441792 as *const _
    }
}
impl Deref for USB0_EP2 {
    type Target = usb0_ep1::RegisterBlock;
    fn deref(&self) -> &usb0_ep1::RegisterBlock {
        unsafe { &*USB0_EP2::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub struct USB0_EP3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_EP3 {}
impl USB0_EP3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usb0_ep1::RegisterBlock {
        1342441824 as *const _
    }
}
impl Deref for USB0_EP3 {
    type Target = usb0_ep1::RegisterBlock;
    fn deref(&self) -> &usb0_ep1::RegisterBlock {
        unsafe { &*USB0_EP3::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub struct USB0_EP4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_EP4 {}
impl USB0_EP4 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usb0_ep1::RegisterBlock {
        1342441856 as *const _
    }
}
impl Deref for USB0_EP4 {
    type Target = usb0_ep1::RegisterBlock;
    fn deref(&self) -> &usb0_ep1::RegisterBlock {
        unsafe { &*USB0_EP4::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub struct USB0_EP5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_EP5 {}
impl USB0_EP5 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usb0_ep1::RegisterBlock {
        1342441888 as *const _
    }
}
impl Deref for USB0_EP5 {
    type Target = usb0_ep1::RegisterBlock;
    fn deref(&self) -> &usb0_ep1::RegisterBlock {
        unsafe { &*USB0_EP5::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub struct USB0_EP6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_EP6 {}
impl USB0_EP6 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usb0_ep1::RegisterBlock {
        1342441920 as *const _
    }
}
impl Deref for USB0_EP6 {
    type Target = usb0_ep1::RegisterBlock;
    fn deref(&self) -> &usb0_ep1::RegisterBlock {
        unsafe { &*USB0_EP6::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub struct USB0_CH0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_CH0 {}
impl USB0_CH0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usb0_ch0::RegisterBlock {
        1342440704 as *const _
    }
}
impl Deref for USB0_CH0 {
    type Target = usb0_ch0::RegisterBlock;
    fn deref(&self) -> &usb0_ch0::RegisterBlock {
        unsafe { &*USB0_CH0::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub mod usb0_ch0;
#[doc = "Universal Serial Bus"]
pub struct USB0_CH1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_CH1 {}
impl USB0_CH1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usb0_ch0::RegisterBlock {
        1342440736 as *const _
    }
}
impl Deref for USB0_CH1 {
    type Target = usb0_ch0::RegisterBlock;
    fn deref(&self) -> &usb0_ch0::RegisterBlock {
        unsafe { &*USB0_CH1::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub struct USB0_CH2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_CH2 {}
impl USB0_CH2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usb0_ch0::RegisterBlock {
        1342440768 as *const _
    }
}
impl Deref for USB0_CH2 {
    type Target = usb0_ch0::RegisterBlock;
    fn deref(&self) -> &usb0_ch0::RegisterBlock {
        unsafe { &*USB0_CH2::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub struct USB0_CH3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_CH3 {}
impl USB0_CH3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usb0_ch0::RegisterBlock {
        1342440800 as *const _
    }
}
impl Deref for USB0_CH3 {
    type Target = usb0_ch0::RegisterBlock;
    fn deref(&self) -> &usb0_ch0::RegisterBlock {
        unsafe { &*USB0_CH3::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub struct USB0_CH4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_CH4 {}
impl USB0_CH4 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usb0_ch0::RegisterBlock {
        1342440832 as *const _
    }
}
impl Deref for USB0_CH4 {
    type Target = usb0_ch0::RegisterBlock;
    fn deref(&self) -> &usb0_ch0::RegisterBlock {
        unsafe { &*USB0_CH4::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub struct USB0_CH5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_CH5 {}
impl USB0_CH5 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usb0_ch0::RegisterBlock {
        1342440864 as *const _
    }
}
impl Deref for USB0_CH5 {
    type Target = usb0_ch0::RegisterBlock;
    fn deref(&self) -> &usb0_ch0::RegisterBlock {
        unsafe { &*USB0_CH5::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub struct USB0_CH6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_CH6 {}
impl USB0_CH6 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usb0_ch0::RegisterBlock {
        1342440896 as *const _
    }
}
impl Deref for USB0_CH6 {
    type Target = usb0_ch0::RegisterBlock;
    fn deref(&self) -> &usb0_ch0::RegisterBlock {
        unsafe { &*USB0_CH6::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub struct USB0_CH7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_CH7 {}
impl USB0_CH7 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usb0_ch0::RegisterBlock {
        1342440928 as *const _
    }
}
impl Deref for USB0_CH7 {
    type Target = usb0_ch0::RegisterBlock;
    fn deref(&self) -> &usb0_ch0::RegisterBlock {
        unsafe { &*USB0_CH7::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub struct USB0_CH8 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_CH8 {}
impl USB0_CH8 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usb0_ch0::RegisterBlock {
        1342440960 as *const _
    }
}
impl Deref for USB0_CH8 {
    type Target = usb0_ch0::RegisterBlock;
    fn deref(&self) -> &usb0_ch0::RegisterBlock {
        unsafe { &*USB0_CH8::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub struct USB0_CH9 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_CH9 {}
impl USB0_CH9 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usb0_ch0::RegisterBlock {
        1342440992 as *const _
    }
}
impl Deref for USB0_CH9 {
    type Target = usb0_ch0::RegisterBlock;
    fn deref(&self) -> &usb0_ch0::RegisterBlock {
        unsafe { &*USB0_CH9::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub struct USB0_CH10 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_CH10 {}
impl USB0_CH10 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usb0_ch0::RegisterBlock {
        1342441024 as *const _
    }
}
impl Deref for USB0_CH10 {
    type Target = usb0_ch0::RegisterBlock;
    fn deref(&self) -> &usb0_ch0::RegisterBlock {
        unsafe { &*USB0_CH10::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub struct USB0_CH11 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_CH11 {}
impl USB0_CH11 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usb0_ch0::RegisterBlock {
        1342441056 as *const _
    }
}
impl Deref for USB0_CH11 {
    type Target = usb0_ch0::RegisterBlock;
    fn deref(&self) -> &usb0_ch0::RegisterBlock {
        unsafe { &*USB0_CH11::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub struct USB0_CH12 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_CH12 {}
impl USB0_CH12 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usb0_ch0::RegisterBlock {
        1342441088 as *const _
    }
}
impl Deref for USB0_CH12 {
    type Target = usb0_ch0::RegisterBlock;
    fn deref(&self) -> &usb0_ch0::RegisterBlock {
        unsafe { &*USB0_CH12::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub struct USB0_CH13 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0_CH13 {}
impl USB0_CH13 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usb0_ch0::RegisterBlock {
        1342441120 as *const _
    }
}
impl Deref for USB0_CH13 {
    type Target = usb0_ch0::RegisterBlock;
    fn deref(&self) -> &usb0_ch0::RegisterBlock {
        unsafe { &*USB0_CH13::ptr() }
    }
}
#[doc = "Universal Serial Interface Controller 0"]
pub struct USIC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USIC0 {}
impl USIC0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usic0::RegisterBlock {
        1073938440 as *const _
    }
}
impl Deref for USIC0 {
    type Target = usic0::RegisterBlock;
    fn deref(&self) -> &usic0::RegisterBlock {
        unsafe { &*USIC0::ptr() }
    }
}
#[doc = "Universal Serial Interface Controller 0"]
pub mod usic0;
#[doc = "Universal Serial Interface Controller 1"]
pub struct USIC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USIC1 {}
impl USIC1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usic0::RegisterBlock {
        1208090632 as *const _
    }
}
impl Deref for USIC1 {
    type Target = usic0::RegisterBlock;
    fn deref(&self) -> &usic0::RegisterBlock {
        unsafe { &*USIC1::ptr() }
    }
}
#[doc = "Universal Serial Interface Controller 0"]
pub struct USIC0_CH0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USIC0_CH0 {}
impl USIC0_CH0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usic0_ch0::RegisterBlock {
        1073938432 as *const _
    }
}
impl Deref for USIC0_CH0 {
    type Target = usic0_ch0::RegisterBlock;
    fn deref(&self) -> &usic0_ch0::RegisterBlock {
        unsafe { &*USIC0_CH0::ptr() }
    }
}
#[doc = "Universal Serial Interface Controller 0"]
pub mod usic0_ch0;
#[doc = "Universal Serial Interface Controller 0"]
pub struct USIC0_CH1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USIC0_CH1 {}
impl USIC0_CH1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usic0_ch0::RegisterBlock {
        1073938944 as *const _
    }
}
impl Deref for USIC0_CH1 {
    type Target = usic0_ch0::RegisterBlock;
    fn deref(&self) -> &usic0_ch0::RegisterBlock {
        unsafe { &*USIC0_CH1::ptr() }
    }
}
#[doc = "Universal Serial Interface Controller 1"]
pub struct USIC1_CH0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USIC1_CH0 {}
impl USIC1_CH0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usic0_ch0::RegisterBlock {
        1208090624 as *const _
    }
}
impl Deref for USIC1_CH0 {
    type Target = usic0_ch0::RegisterBlock;
    fn deref(&self) -> &usic0_ch0::RegisterBlock {
        unsafe { &*USIC1_CH0::ptr() }
    }
}
#[doc = "Universal Serial Interface Controller 1"]
pub struct USIC1_CH1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USIC1_CH1 {}
impl USIC1_CH1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usic0_ch0::RegisterBlock {
        1208091136 as *const _
    }
}
impl Deref for USIC1_CH1 {
    type Target = usic0_ch0::RegisterBlock;
    fn deref(&self) -> &usic0_ch0::RegisterBlock {
        unsafe { &*USIC1_CH1::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN {}
impl CAN {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can::RegisterBlock {
        1208041472 as *const _
    }
}
impl Deref for CAN {
    type Target = can::RegisterBlock;
    fn deref(&self) -> &can::RegisterBlock {
        unsafe { &*CAN::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub mod can;
#[doc = "Controller Area Networks"]
pub struct CAN_NODE0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_NODE0 {}
impl CAN_NODE0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_node0::RegisterBlock {
        1208041984 as *const _
    }
}
impl Deref for CAN_NODE0 {
    type Target = can_node0::RegisterBlock;
    fn deref(&self) -> &can_node0::RegisterBlock {
        unsafe { &*CAN_NODE0::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub mod can_node0;
#[doc = "Controller Area Networks"]
pub struct CAN_NODE1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_NODE1 {}
impl CAN_NODE1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_node0::RegisterBlock {
        1208042240 as *const _
    }
}
impl Deref for CAN_NODE1 {
    type Target = can_node0::RegisterBlock;
    fn deref(&self) -> &can_node0::RegisterBlock {
        unsafe { &*CAN_NODE1::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO0 {}
impl CAN_MO0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208045568 as *const _
    }
}
impl Deref for CAN_MO0 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO0::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub mod can_mo0;
#[doc = "Controller Area Networks"]
pub struct CAN_MO1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO1 {}
impl CAN_MO1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208045600 as *const _
    }
}
impl Deref for CAN_MO1 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO1::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO2 {}
impl CAN_MO2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208045632 as *const _
    }
}
impl Deref for CAN_MO2 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO2::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO3 {}
impl CAN_MO3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208045664 as *const _
    }
}
impl Deref for CAN_MO3 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO3::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO4 {}
impl CAN_MO4 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208045696 as *const _
    }
}
impl Deref for CAN_MO4 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO4::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO5 {}
impl CAN_MO5 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208045728 as *const _
    }
}
impl Deref for CAN_MO5 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO5::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO6 {}
impl CAN_MO6 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208045760 as *const _
    }
}
impl Deref for CAN_MO6 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO6::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO7 {}
impl CAN_MO7 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208045792 as *const _
    }
}
impl Deref for CAN_MO7 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO7::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO8 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO8 {}
impl CAN_MO8 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208045824 as *const _
    }
}
impl Deref for CAN_MO8 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO8::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO9 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO9 {}
impl CAN_MO9 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208045856 as *const _
    }
}
impl Deref for CAN_MO9 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO9::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO10 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO10 {}
impl CAN_MO10 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208045888 as *const _
    }
}
impl Deref for CAN_MO10 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO10::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO11 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO11 {}
impl CAN_MO11 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208045920 as *const _
    }
}
impl Deref for CAN_MO11 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO11::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO12 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO12 {}
impl CAN_MO12 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208045952 as *const _
    }
}
impl Deref for CAN_MO12 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO12::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO13 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO13 {}
impl CAN_MO13 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208045984 as *const _
    }
}
impl Deref for CAN_MO13 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO13::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO14 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO14 {}
impl CAN_MO14 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208046016 as *const _
    }
}
impl Deref for CAN_MO14 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO14::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO15 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO15 {}
impl CAN_MO15 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208046048 as *const _
    }
}
impl Deref for CAN_MO15 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO15::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO16 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO16 {}
impl CAN_MO16 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208046080 as *const _
    }
}
impl Deref for CAN_MO16 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO16::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO17 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO17 {}
impl CAN_MO17 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208046112 as *const _
    }
}
impl Deref for CAN_MO17 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO17::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO18 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO18 {}
impl CAN_MO18 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208046144 as *const _
    }
}
impl Deref for CAN_MO18 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO18::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO19 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO19 {}
impl CAN_MO19 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208046176 as *const _
    }
}
impl Deref for CAN_MO19 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO19::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO20 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO20 {}
impl CAN_MO20 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208046208 as *const _
    }
}
impl Deref for CAN_MO20 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO20::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO21 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO21 {}
impl CAN_MO21 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208046240 as *const _
    }
}
impl Deref for CAN_MO21 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO21::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO22 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO22 {}
impl CAN_MO22 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208046272 as *const _
    }
}
impl Deref for CAN_MO22 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO22::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO23 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO23 {}
impl CAN_MO23 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208046304 as *const _
    }
}
impl Deref for CAN_MO23 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO23::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO24 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO24 {}
impl CAN_MO24 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208046336 as *const _
    }
}
impl Deref for CAN_MO24 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO24::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO25 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO25 {}
impl CAN_MO25 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208046368 as *const _
    }
}
impl Deref for CAN_MO25 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO25::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO26 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO26 {}
impl CAN_MO26 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208046400 as *const _
    }
}
impl Deref for CAN_MO26 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO26::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO27 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO27 {}
impl CAN_MO27 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208046432 as *const _
    }
}
impl Deref for CAN_MO27 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO27::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO28 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO28 {}
impl CAN_MO28 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208046464 as *const _
    }
}
impl Deref for CAN_MO28 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO28::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO29 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO29 {}
impl CAN_MO29 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208046496 as *const _
    }
}
impl Deref for CAN_MO29 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO29::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO30 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO30 {}
impl CAN_MO30 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208046528 as *const _
    }
}
impl Deref for CAN_MO30 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO30::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO31 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO31 {}
impl CAN_MO31 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208046560 as *const _
    }
}
impl Deref for CAN_MO31 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO31::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO32 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO32 {}
impl CAN_MO32 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208046592 as *const _
    }
}
impl Deref for CAN_MO32 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO32::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO33 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO33 {}
impl CAN_MO33 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208046624 as *const _
    }
}
impl Deref for CAN_MO33 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO33::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO34 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO34 {}
impl CAN_MO34 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208046656 as *const _
    }
}
impl Deref for CAN_MO34 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO34::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO35 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO35 {}
impl CAN_MO35 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208046688 as *const _
    }
}
impl Deref for CAN_MO35 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO35::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO36 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO36 {}
impl CAN_MO36 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208046720 as *const _
    }
}
impl Deref for CAN_MO36 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO36::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO37 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO37 {}
impl CAN_MO37 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208046752 as *const _
    }
}
impl Deref for CAN_MO37 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO37::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO38 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO38 {}
impl CAN_MO38 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208046784 as *const _
    }
}
impl Deref for CAN_MO38 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO38::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO39 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO39 {}
impl CAN_MO39 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208046816 as *const _
    }
}
impl Deref for CAN_MO39 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO39::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO40 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO40 {}
impl CAN_MO40 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208046848 as *const _
    }
}
impl Deref for CAN_MO40 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO40::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO41 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO41 {}
impl CAN_MO41 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208046880 as *const _
    }
}
impl Deref for CAN_MO41 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO41::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO42 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO42 {}
impl CAN_MO42 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208046912 as *const _
    }
}
impl Deref for CAN_MO42 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO42::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO43 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO43 {}
impl CAN_MO43 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208046944 as *const _
    }
}
impl Deref for CAN_MO43 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO43::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO44 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO44 {}
impl CAN_MO44 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208046976 as *const _
    }
}
impl Deref for CAN_MO44 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO44::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO45 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO45 {}
impl CAN_MO45 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208047008 as *const _
    }
}
impl Deref for CAN_MO45 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO45::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO46 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO46 {}
impl CAN_MO46 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208047040 as *const _
    }
}
impl Deref for CAN_MO46 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO46::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO47 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO47 {}
impl CAN_MO47 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208047072 as *const _
    }
}
impl Deref for CAN_MO47 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO47::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO48 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO48 {}
impl CAN_MO48 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208047104 as *const _
    }
}
impl Deref for CAN_MO48 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO48::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO49 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO49 {}
impl CAN_MO49 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208047136 as *const _
    }
}
impl Deref for CAN_MO49 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO49::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO50 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO50 {}
impl CAN_MO50 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208047168 as *const _
    }
}
impl Deref for CAN_MO50 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO50::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO51 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO51 {}
impl CAN_MO51 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208047200 as *const _
    }
}
impl Deref for CAN_MO51 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO51::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO52 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO52 {}
impl CAN_MO52 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208047232 as *const _
    }
}
impl Deref for CAN_MO52 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO52::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO53 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO53 {}
impl CAN_MO53 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208047264 as *const _
    }
}
impl Deref for CAN_MO53 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO53::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO54 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO54 {}
impl CAN_MO54 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208047296 as *const _
    }
}
impl Deref for CAN_MO54 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO54::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO55 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO55 {}
impl CAN_MO55 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208047328 as *const _
    }
}
impl Deref for CAN_MO55 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO55::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO56 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO56 {}
impl CAN_MO56 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208047360 as *const _
    }
}
impl Deref for CAN_MO56 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO56::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO57 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO57 {}
impl CAN_MO57 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208047392 as *const _
    }
}
impl Deref for CAN_MO57 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO57::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO58 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO58 {}
impl CAN_MO58 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208047424 as *const _
    }
}
impl Deref for CAN_MO58 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO58::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO59 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO59 {}
impl CAN_MO59 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208047456 as *const _
    }
}
impl Deref for CAN_MO59 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO59::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO60 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO60 {}
impl CAN_MO60 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208047488 as *const _
    }
}
impl Deref for CAN_MO60 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO60::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO61 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO61 {}
impl CAN_MO61 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208047520 as *const _
    }
}
impl Deref for CAN_MO61 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO61::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO62 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO62 {}
impl CAN_MO62 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208047552 as *const _
    }
}
impl Deref for CAN_MO62 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO62::ptr() }
    }
}
#[doc = "Controller Area Networks"]
pub struct CAN_MO63 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_MO63 {}
impl CAN_MO63 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can_mo0::RegisterBlock {
        1208047584 as *const _
    }
}
impl Deref for CAN_MO63 {
    type Target = can_mo0::RegisterBlock;
    fn deref(&self) -> &can_mo0::RegisterBlock {
        unsafe { &*CAN_MO63::ptr() }
    }
}
#[doc = "Analog to Digital Converter"]
pub struct VADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VADC {}
impl VADC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const vadc::RegisterBlock {
        1073758208 as *const _
    }
}
impl Deref for VADC {
    type Target = vadc::RegisterBlock;
    fn deref(&self) -> &vadc::RegisterBlock {
        unsafe { &*VADC::ptr() }
    }
}
#[doc = "Analog to Digital Converter"]
pub mod vadc;
#[doc = "Analog to Digital Converter"]
pub struct VADC_G0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VADC_G0 {}
impl VADC_G0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const vadc_g0::RegisterBlock {
        1073759232 as *const _
    }
}
impl Deref for VADC_G0 {
    type Target = vadc_g0::RegisterBlock;
    fn deref(&self) -> &vadc_g0::RegisterBlock {
        unsafe { &*VADC_G0::ptr() }
    }
}
#[doc = "Analog to Digital Converter"]
pub mod vadc_g0;
#[doc = "Analog to Digital Converter"]
pub struct VADC_G1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VADC_G1 {}
impl VADC_G1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const vadc_g0::RegisterBlock {
        1073760256 as *const _
    }
}
impl Deref for VADC_G1 {
    type Target = vadc_g0::RegisterBlock;
    fn deref(&self) -> &vadc_g0::RegisterBlock {
        unsafe { &*VADC_G1::ptr() }
    }
}
#[doc = "Analog to Digital Converter"]
pub struct VADC_G2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VADC_G2 {}
impl VADC_G2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const vadc_g0::RegisterBlock {
        1073761280 as *const _
    }
}
impl Deref for VADC_G2 {
    type Target = vadc_g0::RegisterBlock;
    fn deref(&self) -> &vadc_g0::RegisterBlock {
        unsafe { &*VADC_G2::ptr() }
    }
}
#[doc = "Analog to Digital Converter"]
pub struct VADC_G3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VADC_G3 {}
impl VADC_G3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const vadc_g0::RegisterBlock {
        1073762304 as *const _
    }
}
impl Deref for VADC_G3 {
    type Target = vadc_g0::RegisterBlock;
    fn deref(&self) -> &vadc_g0::RegisterBlock {
        unsafe { &*VADC_G3::ptr() }
    }
}
#[doc = "Delta Sigma Demodulator"]
pub struct DSD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DSD {}
impl DSD {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dsd::RegisterBlock {
        1073774592 as *const _
    }
}
impl Deref for DSD {
    type Target = dsd::RegisterBlock;
    fn deref(&self) -> &dsd::RegisterBlock {
        unsafe { &*DSD::ptr() }
    }
}
#[doc = "Delta Sigma Demodulator"]
pub mod dsd;
#[doc = "Delta Sigma Demodulator"]
pub struct DSD_CH0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DSD_CH0 {}
impl DSD_CH0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dsd_ch0::RegisterBlock {
        1073774848 as *const _
    }
}
impl Deref for DSD_CH0 {
    type Target = dsd_ch0::RegisterBlock;
    fn deref(&self) -> &dsd_ch0::RegisterBlock {
        unsafe { &*DSD_CH0::ptr() }
    }
}
#[doc = "Delta Sigma Demodulator"]
pub mod dsd_ch0;
#[doc = "Delta Sigma Demodulator"]
pub struct DSD_CH1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DSD_CH1 {}
impl DSD_CH1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dsd_ch0::RegisterBlock {
        1073775104 as *const _
    }
}
impl Deref for DSD_CH1 {
    type Target = dsd_ch0::RegisterBlock;
    fn deref(&self) -> &dsd_ch0::RegisterBlock {
        unsafe { &*DSD_CH1::ptr() }
    }
}
#[doc = "Delta Sigma Demodulator"]
pub struct DSD_CH2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DSD_CH2 {}
impl DSD_CH2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dsd_ch0::RegisterBlock {
        1073775360 as *const _
    }
}
impl Deref for DSD_CH2 {
    type Target = dsd_ch0::RegisterBlock;
    fn deref(&self) -> &dsd_ch0::RegisterBlock {
        unsafe { &*DSD_CH2::ptr() }
    }
}
#[doc = "Delta Sigma Demodulator"]
pub struct DSD_CH3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DSD_CH3 {}
impl DSD_CH3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dsd_ch0::RegisterBlock {
        1073775616 as *const _
    }
}
impl Deref for DSD_CH3 {
    type Target = dsd_ch0::RegisterBlock;
    fn deref(&self) -> &dsd_ch0::RegisterBlock {
        unsafe { &*DSD_CH3::ptr() }
    }
}
#[doc = "Digital to Analog Converter"]
pub struct DAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DAC {}
impl DAC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dac::RegisterBlock {
        1208057856 as *const _
    }
}
impl Deref for DAC {
    type Target = dac::RegisterBlock;
    fn deref(&self) -> &dac::RegisterBlock {
        unsafe { &*DAC::ptr() }
    }
}
#[doc = "Digital to Analog Converter"]
pub mod dac;
#[doc = "Capture Compare Unit 4 - Unit 0"]
pub struct CCU40 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU40 {}
impl CCU40 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ccu40::RegisterBlock {
        1073790976 as *const _
    }
}
impl Deref for CCU40 {
    type Target = ccu40::RegisterBlock;
    fn deref(&self) -> &ccu40::RegisterBlock {
        unsafe { &*CCU40::ptr() }
    }
}
#[doc = "Capture Compare Unit 4 - Unit 0"]
pub mod ccu40;
#[doc = "Capture Compare Unit 4 - Unit 1"]
pub struct CCU41 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU41 {}
impl CCU41 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ccu40::RegisterBlock {
        1073807360 as *const _
    }
}
impl Deref for CCU41 {
    type Target = ccu40::RegisterBlock;
    fn deref(&self) -> &ccu40::RegisterBlock {
        unsafe { &*CCU41::ptr() }
    }
}
#[doc = "Capture Compare Unit 4 - Unit 2"]
pub struct CCU42 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU42 {}
impl CCU42 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ccu40::RegisterBlock {
        1073823744 as *const _
    }
}
impl Deref for CCU42 {
    type Target = ccu40::RegisterBlock;
    fn deref(&self) -> &ccu40::RegisterBlock {
        unsafe { &*CCU42::ptr() }
    }
}
#[doc = "Capture Compare Unit 4 - Unit 3"]
pub struct CCU43 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU43 {}
impl CCU43 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ccu40::RegisterBlock {
        1207975936 as *const _
    }
}
impl Deref for CCU43 {
    type Target = ccu40::RegisterBlock;
    fn deref(&self) -> &ccu40::RegisterBlock {
        unsafe { &*CCU43::ptr() }
    }
}
#[doc = "Capture Compare Unit 4 - Unit 0"]
pub struct CCU40_CC40 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU40_CC40 {}
impl CCU40_CC40 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ccu40_cc40::RegisterBlock {
        1073791232 as *const _
    }
}
impl Deref for CCU40_CC40 {
    type Target = ccu40_cc40::RegisterBlock;
    fn deref(&self) -> &ccu40_cc40::RegisterBlock {
        unsafe { &*CCU40_CC40::ptr() }
    }
}
#[doc = "Capture Compare Unit 4 - Unit 0"]
pub mod ccu40_cc40;
#[doc = "Capture Compare Unit 4 - Unit 0"]
pub struct CCU40_CC41 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU40_CC41 {}
impl CCU40_CC41 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ccu40_cc40::RegisterBlock {
        1073791488 as *const _
    }
}
impl Deref for CCU40_CC41 {
    type Target = ccu40_cc40::RegisterBlock;
    fn deref(&self) -> &ccu40_cc40::RegisterBlock {
        unsafe { &*CCU40_CC41::ptr() }
    }
}
#[doc = "Capture Compare Unit 4 - Unit 0"]
pub struct CCU40_CC42 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU40_CC42 {}
impl CCU40_CC42 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ccu40_cc40::RegisterBlock {
        1073791744 as *const _
    }
}
impl Deref for CCU40_CC42 {
    type Target = ccu40_cc40::RegisterBlock;
    fn deref(&self) -> &ccu40_cc40::RegisterBlock {
        unsafe { &*CCU40_CC42::ptr() }
    }
}
#[doc = "Capture Compare Unit 4 - Unit 0"]
pub struct CCU40_CC43 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU40_CC43 {}
impl CCU40_CC43 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ccu40_cc40::RegisterBlock {
        1073792000 as *const _
    }
}
impl Deref for CCU40_CC43 {
    type Target = ccu40_cc40::RegisterBlock;
    fn deref(&self) -> &ccu40_cc40::RegisterBlock {
        unsafe { &*CCU40_CC43::ptr() }
    }
}
#[doc = "Capture Compare Unit 4 - Unit 1"]
pub struct CCU41_CC40 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU41_CC40 {}
impl CCU41_CC40 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ccu40_cc40::RegisterBlock {
        1073807616 as *const _
    }
}
impl Deref for CCU41_CC40 {
    type Target = ccu40_cc40::RegisterBlock;
    fn deref(&self) -> &ccu40_cc40::RegisterBlock {
        unsafe { &*CCU41_CC40::ptr() }
    }
}
#[doc = "Capture Compare Unit 4 - Unit 1"]
pub struct CCU41_CC41 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU41_CC41 {}
impl CCU41_CC41 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ccu40_cc40::RegisterBlock {
        1073807872 as *const _
    }
}
impl Deref for CCU41_CC41 {
    type Target = ccu40_cc40::RegisterBlock;
    fn deref(&self) -> &ccu40_cc40::RegisterBlock {
        unsafe { &*CCU41_CC41::ptr() }
    }
}
#[doc = "Capture Compare Unit 4 - Unit 1"]
pub struct CCU41_CC42 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU41_CC42 {}
impl CCU41_CC42 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ccu40_cc40::RegisterBlock {
        1073808128 as *const _
    }
}
impl Deref for CCU41_CC42 {
    type Target = ccu40_cc40::RegisterBlock;
    fn deref(&self) -> &ccu40_cc40::RegisterBlock {
        unsafe { &*CCU41_CC42::ptr() }
    }
}
#[doc = "Capture Compare Unit 4 - Unit 1"]
pub struct CCU41_CC43 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU41_CC43 {}
impl CCU41_CC43 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ccu40_cc40::RegisterBlock {
        1073808384 as *const _
    }
}
impl Deref for CCU41_CC43 {
    type Target = ccu40_cc40::RegisterBlock;
    fn deref(&self) -> &ccu40_cc40::RegisterBlock {
        unsafe { &*CCU41_CC43::ptr() }
    }
}
#[doc = "Capture Compare Unit 4 - Unit 2"]
pub struct CCU42_CC40 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU42_CC40 {}
impl CCU42_CC40 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ccu40_cc40::RegisterBlock {
        1073824000 as *const _
    }
}
impl Deref for CCU42_CC40 {
    type Target = ccu40_cc40::RegisterBlock;
    fn deref(&self) -> &ccu40_cc40::RegisterBlock {
        unsafe { &*CCU42_CC40::ptr() }
    }
}
#[doc = "Capture Compare Unit 4 - Unit 2"]
pub struct CCU42_CC41 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU42_CC41 {}
impl CCU42_CC41 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ccu40_cc40::RegisterBlock {
        1073824256 as *const _
    }
}
impl Deref for CCU42_CC41 {
    type Target = ccu40_cc40::RegisterBlock;
    fn deref(&self) -> &ccu40_cc40::RegisterBlock {
        unsafe { &*CCU42_CC41::ptr() }
    }
}
#[doc = "Capture Compare Unit 4 - Unit 2"]
pub struct CCU42_CC42 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU42_CC42 {}
impl CCU42_CC42 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ccu40_cc40::RegisterBlock {
        1073824512 as *const _
    }
}
impl Deref for CCU42_CC42 {
    type Target = ccu40_cc40::RegisterBlock;
    fn deref(&self) -> &ccu40_cc40::RegisterBlock {
        unsafe { &*CCU42_CC42::ptr() }
    }
}
#[doc = "Capture Compare Unit 4 - Unit 2"]
pub struct CCU42_CC43 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU42_CC43 {}
impl CCU42_CC43 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ccu40_cc40::RegisterBlock {
        1073824768 as *const _
    }
}
impl Deref for CCU42_CC43 {
    type Target = ccu40_cc40::RegisterBlock;
    fn deref(&self) -> &ccu40_cc40::RegisterBlock {
        unsafe { &*CCU42_CC43::ptr() }
    }
}
#[doc = "Capture Compare Unit 4 - Unit 3"]
pub struct CCU43_CC40 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU43_CC40 {}
impl CCU43_CC40 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ccu40_cc40::RegisterBlock {
        1207976192 as *const _
    }
}
impl Deref for CCU43_CC40 {
    type Target = ccu40_cc40::RegisterBlock;
    fn deref(&self) -> &ccu40_cc40::RegisterBlock {
        unsafe { &*CCU43_CC40::ptr() }
    }
}
#[doc = "Capture Compare Unit 4 - Unit 3"]
pub struct CCU43_CC41 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU43_CC41 {}
impl CCU43_CC41 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ccu40_cc40::RegisterBlock {
        1207976448 as *const _
    }
}
impl Deref for CCU43_CC41 {
    type Target = ccu40_cc40::RegisterBlock;
    fn deref(&self) -> &ccu40_cc40::RegisterBlock {
        unsafe { &*CCU43_CC41::ptr() }
    }
}
#[doc = "Capture Compare Unit 4 - Unit 3"]
pub struct CCU43_CC42 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU43_CC42 {}
impl CCU43_CC42 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ccu40_cc40::RegisterBlock {
        1207976704 as *const _
    }
}
impl Deref for CCU43_CC42 {
    type Target = ccu40_cc40::RegisterBlock;
    fn deref(&self) -> &ccu40_cc40::RegisterBlock {
        unsafe { &*CCU43_CC42::ptr() }
    }
}
#[doc = "Capture Compare Unit 4 - Unit 3"]
pub struct CCU43_CC43 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU43_CC43 {}
impl CCU43_CC43 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ccu40_cc40::RegisterBlock {
        1207976960 as *const _
    }
}
impl Deref for CCU43_CC43 {
    type Target = ccu40_cc40::RegisterBlock;
    fn deref(&self) -> &ccu40_cc40::RegisterBlock {
        unsafe { &*CCU43_CC43::ptr() }
    }
}
#[doc = "Capture Compare Unit 8 - Unit 0"]
pub struct CCU80 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU80 {}
impl CCU80 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ccu80::RegisterBlock {
        1073872896 as *const _
    }
}
impl Deref for CCU80 {
    type Target = ccu80::RegisterBlock;
    fn deref(&self) -> &ccu80::RegisterBlock {
        unsafe { &*CCU80::ptr() }
    }
}
#[doc = "Capture Compare Unit 8 - Unit 0"]
pub mod ccu80;
#[doc = "Capture Compare Unit 8 - Unit 1"]
pub struct CCU81 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU81 {}
impl CCU81 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ccu80::RegisterBlock {
        1073889280 as *const _
    }
}
impl Deref for CCU81 {
    type Target = ccu80::RegisterBlock;
    fn deref(&self) -> &ccu80::RegisterBlock {
        unsafe { &*CCU81::ptr() }
    }
}
#[doc = "Capture Compare Unit 8 - Unit 0"]
pub struct CCU80_CC80 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU80_CC80 {}
impl CCU80_CC80 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ccu80_cc80::RegisterBlock {
        1073873152 as *const _
    }
}
impl Deref for CCU80_CC80 {
    type Target = ccu80_cc80::RegisterBlock;
    fn deref(&self) -> &ccu80_cc80::RegisterBlock {
        unsafe { &*CCU80_CC80::ptr() }
    }
}
#[doc = "Capture Compare Unit 8 - Unit 0"]
pub mod ccu80_cc80;
#[doc = "Capture Compare Unit 8 - Unit 0"]
pub struct CCU80_CC81 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU80_CC81 {}
impl CCU80_CC81 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ccu80_cc80::RegisterBlock {
        1073873408 as *const _
    }
}
impl Deref for CCU80_CC81 {
    type Target = ccu80_cc80::RegisterBlock;
    fn deref(&self) -> &ccu80_cc80::RegisterBlock {
        unsafe { &*CCU80_CC81::ptr() }
    }
}
#[doc = "Capture Compare Unit 8 - Unit 0"]
pub struct CCU80_CC82 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU80_CC82 {}
impl CCU80_CC82 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ccu80_cc80::RegisterBlock {
        1073873664 as *const _
    }
}
impl Deref for CCU80_CC82 {
    type Target = ccu80_cc80::RegisterBlock;
    fn deref(&self) -> &ccu80_cc80::RegisterBlock {
        unsafe { &*CCU80_CC82::ptr() }
    }
}
#[doc = "Capture Compare Unit 8 - Unit 0"]
pub struct CCU80_CC83 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU80_CC83 {}
impl CCU80_CC83 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ccu80_cc80::RegisterBlock {
        1073873920 as *const _
    }
}
impl Deref for CCU80_CC83 {
    type Target = ccu80_cc80::RegisterBlock;
    fn deref(&self) -> &ccu80_cc80::RegisterBlock {
        unsafe { &*CCU80_CC83::ptr() }
    }
}
#[doc = "Capture Compare Unit 8 - Unit 1"]
pub struct CCU81_CC80 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU81_CC80 {}
impl CCU81_CC80 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ccu80_cc80::RegisterBlock {
        1073889536 as *const _
    }
}
impl Deref for CCU81_CC80 {
    type Target = ccu80_cc80::RegisterBlock;
    fn deref(&self) -> &ccu80_cc80::RegisterBlock {
        unsafe { &*CCU81_CC80::ptr() }
    }
}
#[doc = "Capture Compare Unit 8 - Unit 1"]
pub struct CCU81_CC81 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU81_CC81 {}
impl CCU81_CC81 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ccu80_cc80::RegisterBlock {
        1073889792 as *const _
    }
}
impl Deref for CCU81_CC81 {
    type Target = ccu80_cc80::RegisterBlock;
    fn deref(&self) -> &ccu80_cc80::RegisterBlock {
        unsafe { &*CCU81_CC81::ptr() }
    }
}
#[doc = "Capture Compare Unit 8 - Unit 1"]
pub struct CCU81_CC82 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU81_CC82 {}
impl CCU81_CC82 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ccu80_cc80::RegisterBlock {
        1073890048 as *const _
    }
}
impl Deref for CCU81_CC82 {
    type Target = ccu80_cc80::RegisterBlock;
    fn deref(&self) -> &ccu80_cc80::RegisterBlock {
        unsafe { &*CCU81_CC82::ptr() }
    }
}
#[doc = "Capture Compare Unit 8 - Unit 1"]
pub struct CCU81_CC83 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU81_CC83 {}
impl CCU81_CC83 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ccu80_cc80::RegisterBlock {
        1073890304 as *const _
    }
}
impl Deref for CCU81_CC83 {
    type Target = ccu80_cc80::RegisterBlock;
    fn deref(&self) -> &ccu80_cc80::RegisterBlock {
        unsafe { &*CCU81_CC83::ptr() }
    }
}
#[doc = "High Resolution PWM Unit"]
pub struct HRPWM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HRPWM0 {}
impl HRPWM0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const hrpwm0::RegisterBlock {
        1073875200 as *const _
    }
}
impl Deref for HRPWM0 {
    type Target = hrpwm0::RegisterBlock;
    fn deref(&self) -> &hrpwm0::RegisterBlock {
        unsafe { &*HRPWM0::ptr() }
    }
}
#[doc = "High Resolution PWM Unit"]
pub mod hrpwm0;
#[doc = "High Resolution PWM Unit"]
pub struct HRPWM0_CSG0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HRPWM0_CSG0 {}
impl HRPWM0_CSG0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const hrpwm0_csg0::RegisterBlock {
        1073875456 as *const _
    }
}
impl Deref for HRPWM0_CSG0 {
    type Target = hrpwm0_csg0::RegisterBlock;
    fn deref(&self) -> &hrpwm0_csg0::RegisterBlock {
        unsafe { &*HRPWM0_CSG0::ptr() }
    }
}
#[doc = "High Resolution PWM Unit"]
pub mod hrpwm0_csg0;
#[doc = "High Resolution PWM Unit"]
pub struct HRPWM0_CSG1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HRPWM0_CSG1 {}
impl HRPWM0_CSG1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const hrpwm0_csg0::RegisterBlock {
        1073875712 as *const _
    }
}
impl Deref for HRPWM0_CSG1 {
    type Target = hrpwm0_csg0::RegisterBlock;
    fn deref(&self) -> &hrpwm0_csg0::RegisterBlock {
        unsafe { &*HRPWM0_CSG1::ptr() }
    }
}
#[doc = "High Resolution PWM Unit"]
pub struct HRPWM0_CSG2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HRPWM0_CSG2 {}
impl HRPWM0_CSG2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const hrpwm0_csg0::RegisterBlock {
        1073875968 as *const _
    }
}
impl Deref for HRPWM0_CSG2 {
    type Target = hrpwm0_csg0::RegisterBlock;
    fn deref(&self) -> &hrpwm0_csg0::RegisterBlock {
        unsafe { &*HRPWM0_CSG2::ptr() }
    }
}
#[doc = "High Resolution PWM Unit"]
pub struct HRPWM0_HRC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HRPWM0_HRC0 {}
impl HRPWM0_HRC0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const hrpwm0_hrc0::RegisterBlock {
        1073877760 as *const _
    }
}
impl Deref for HRPWM0_HRC0 {
    type Target = hrpwm0_hrc0::RegisterBlock;
    fn deref(&self) -> &hrpwm0_hrc0::RegisterBlock {
        unsafe { &*HRPWM0_HRC0::ptr() }
    }
}
#[doc = "High Resolution PWM Unit"]
pub mod hrpwm0_hrc0;
#[doc = "High Resolution PWM Unit"]
pub struct HRPWM0_HRC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HRPWM0_HRC1 {}
impl HRPWM0_HRC1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const hrpwm0_hrc0::RegisterBlock {
        1073878016 as *const _
    }
}
impl Deref for HRPWM0_HRC1 {
    type Target = hrpwm0_hrc0::RegisterBlock;
    fn deref(&self) -> &hrpwm0_hrc0::RegisterBlock {
        unsafe { &*HRPWM0_HRC1::ptr() }
    }
}
#[doc = "High Resolution PWM Unit"]
pub struct HRPWM0_HRC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HRPWM0_HRC2 {}
impl HRPWM0_HRC2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const hrpwm0_hrc0::RegisterBlock {
        1073878272 as *const _
    }
}
impl Deref for HRPWM0_HRC2 {
    type Target = hrpwm0_hrc0::RegisterBlock;
    fn deref(&self) -> &hrpwm0_hrc0::RegisterBlock {
        unsafe { &*HRPWM0_HRC2::ptr() }
    }
}
#[doc = "High Resolution PWM Unit"]
pub struct HRPWM0_HRC3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HRPWM0_HRC3 {}
impl HRPWM0_HRC3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const hrpwm0_hrc0::RegisterBlock {
        1073878528 as *const _
    }
}
impl Deref for HRPWM0_HRC3 {
    type Target = hrpwm0_hrc0::RegisterBlock;
    fn deref(&self) -> &hrpwm0_hrc0::RegisterBlock {
        unsafe { &*HRPWM0_HRC3::ptr() }
    }
}
#[doc = "Position Interface 0"]
pub struct POSIF0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for POSIF0 {}
impl POSIF0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const posif0::RegisterBlock {
        1073905664 as *const _
    }
}
impl Deref for POSIF0 {
    type Target = posif0::RegisterBlock;
    fn deref(&self) -> &posif0::RegisterBlock {
        unsafe { &*POSIF0::ptr() }
    }
}
#[doc = "Position Interface 0"]
pub mod posif0;
#[doc = "Position Interface 1"]
pub struct POSIF1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for POSIF1 {}
impl POSIF1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const posif0::RegisterBlock {
        1073922048 as *const _
    }
}
impl Deref for POSIF1 {
    type Target = posif0::RegisterBlock;
    fn deref(&self) -> &posif0::RegisterBlock {
        unsafe { &*POSIF1::ptr() }
    }
}
#[doc = "Port 0"]
pub struct PORT0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT0 {}
impl PORT0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const port0::RegisterBlock {
        1208123392 as *const _
    }
}
impl Deref for PORT0 {
    type Target = port0::RegisterBlock;
    fn deref(&self) -> &port0::RegisterBlock {
        unsafe { &*PORT0::ptr() }
    }
}
#[doc = "Port 0"]
pub mod port0;
#[doc = "Port 1"]
pub struct PORT1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT1 {}
impl PORT1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const port1::RegisterBlock {
        1208123648 as *const _
    }
}
impl Deref for PORT1 {
    type Target = port1::RegisterBlock;
    fn deref(&self) -> &port1::RegisterBlock {
        unsafe { &*PORT1::ptr() }
    }
}
#[doc = "Port 1"]
pub mod port1;
#[doc = "Port 2"]
pub struct PORT2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT2 {}
impl PORT2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const port2::RegisterBlock {
        1208123904 as *const _
    }
}
impl Deref for PORT2 {
    type Target = port2::RegisterBlock;
    fn deref(&self) -> &port2::RegisterBlock {
        unsafe { &*PORT2::ptr() }
    }
}
#[doc = "Port 2"]
pub mod port2;
#[doc = "Port 3"]
pub struct PORT3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT3 {}
impl PORT3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const port3::RegisterBlock {
        1208124160 as *const _
    }
}
impl Deref for PORT3 {
    type Target = port3::RegisterBlock;
    fn deref(&self) -> &port3::RegisterBlock {
        unsafe { &*PORT3::ptr() }
    }
}
#[doc = "Port 3"]
pub mod port3;
#[doc = "Port 4"]
pub struct PORT4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT4 {}
impl PORT4 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const port4::RegisterBlock {
        1208124416 as *const _
    }
}
impl Deref for PORT4 {
    type Target = port4::RegisterBlock;
    fn deref(&self) -> &port4::RegisterBlock {
        unsafe { &*PORT4::ptr() }
    }
}
#[doc = "Port 4"]
pub mod port4;
#[doc = "Port 5"]
pub struct PORT5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT5 {}
impl PORT5 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const port5::RegisterBlock {
        1208124672 as *const _
    }
}
impl Deref for PORT5 {
    type Target = port5::RegisterBlock;
    fn deref(&self) -> &port5::RegisterBlock {
        unsafe { &*PORT5::ptr() }
    }
}
#[doc = "Port 5"]
pub mod port5;
#[doc = "Port 14"]
pub struct PORT14 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT14 {}
impl PORT14 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const port14::RegisterBlock {
        1208126976 as *const _
    }
}
impl Deref for PORT14 {
    type Target = port14::RegisterBlock;
    fn deref(&self) -> &port14::RegisterBlock {
        unsafe { &*PORT14::ptr() }
    }
}
#[doc = "Port 14"]
pub mod port14;
#[doc = "Port 15"]
pub struct PORT15 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT15 {}
impl PORT15 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const port15::RegisterBlock {
        1208127232 as *const _
    }
}
impl Deref for PORT15 {
    type Target = port15::RegisterBlock;
    fn deref(&self) -> &port15::RegisterBlock {
        unsafe { &*PORT15::ptr() }
    }
}
#[doc = "Port 15"]
pub mod port15;
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "PPB"]
    pub PPB: PPB,
    #[doc = "DLR"]
    pub DLR: DLR,
    #[doc = "ERU0"]
    pub ERU0: ERU0,
    #[doc = "ERU1"]
    pub ERU1: ERU1,
    #[doc = "GPDMA0"]
    pub GPDMA0: GPDMA0,
    #[doc = "GPDMA0_CH0"]
    pub GPDMA0_CH0: GPDMA0_CH0,
    #[doc = "GPDMA0_CH1"]
    pub GPDMA0_CH1: GPDMA0_CH1,
    #[doc = "GPDMA0_CH2"]
    pub GPDMA0_CH2: GPDMA0_CH2,
    #[doc = "GPDMA0_CH3"]
    pub GPDMA0_CH3: GPDMA0_CH3,
    #[doc = "GPDMA0_CH4"]
    pub GPDMA0_CH4: GPDMA0_CH4,
    #[doc = "GPDMA0_CH5"]
    pub GPDMA0_CH5: GPDMA0_CH5,
    #[doc = "GPDMA0_CH6"]
    pub GPDMA0_CH6: GPDMA0_CH6,
    #[doc = "GPDMA0_CH7"]
    pub GPDMA0_CH7: GPDMA0_CH7,
    #[doc = "FCE"]
    pub FCE: FCE,
    #[doc = "FCE_KE0"]
    pub FCE_KE0: FCE_KE0,
    #[doc = "FCE_KE1"]
    pub FCE_KE1: FCE_KE1,
    #[doc = "FCE_KE2"]
    pub FCE_KE2: FCE_KE2,
    #[doc = "FCE_KE3"]
    pub FCE_KE3: FCE_KE3,
    #[doc = "PBA0"]
    pub PBA0: PBA0,
    #[doc = "PBA1"]
    pub PBA1: PBA1,
    #[doc = "FLASH0"]
    pub FLASH0: FLASH0,
    #[doc = "PREF"]
    pub PREF: PREF,
    #[doc = "PMU0"]
    pub PMU0: PMU0,
    #[doc = "WDT"]
    pub WDT: WDT,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "SCU_CLK"]
    pub SCU_CLK: SCU_CLK,
    #[doc = "SCU_OSC"]
    pub SCU_OSC: SCU_OSC,
    #[doc = "SCU_PLL"]
    pub SCU_PLL: SCU_PLL,
    #[doc = "SCU_GENERAL"]
    pub SCU_GENERAL: SCU_GENERAL,
    #[doc = "SCU_INTERRUPT"]
    pub SCU_INTERRUPT: SCU_INTERRUPT,
    #[doc = "SCU_PARITY"]
    pub SCU_PARITY: SCU_PARITY,
    #[doc = "SCU_TRAP"]
    pub SCU_TRAP: SCU_TRAP,
    #[doc = "SCU_HIBERNATE"]
    pub SCU_HIBERNATE: SCU_HIBERNATE,
    #[doc = "SCU_POWER"]
    pub SCU_POWER: SCU_POWER,
    #[doc = "SCU_RESET"]
    pub SCU_RESET: SCU_RESET,
    #[doc = "LEDTS0"]
    pub LEDTS0: LEDTS0,
    #[doc = "ETH0_CON"]
    pub ETH0_CON: ETH0_CON,
    #[doc = "ETH0"]
    pub ETH0: ETH0,
    #[doc = "USB0"]
    pub USB0: USB0,
    #[doc = "USB0_EP0"]
    pub USB0_EP0: USB0_EP0,
    #[doc = "USB0_EP1"]
    pub USB0_EP1: USB0_EP1,
    #[doc = "USB0_EP2"]
    pub USB0_EP2: USB0_EP2,
    #[doc = "USB0_EP3"]
    pub USB0_EP3: USB0_EP3,
    #[doc = "USB0_EP4"]
    pub USB0_EP4: USB0_EP4,
    #[doc = "USB0_EP5"]
    pub USB0_EP5: USB0_EP5,
    #[doc = "USB0_EP6"]
    pub USB0_EP6: USB0_EP6,
    #[doc = "USB0_CH0"]
    pub USB0_CH0: USB0_CH0,
    #[doc = "USB0_CH1"]
    pub USB0_CH1: USB0_CH1,
    #[doc = "USB0_CH2"]
    pub USB0_CH2: USB0_CH2,
    #[doc = "USB0_CH3"]
    pub USB0_CH3: USB0_CH3,
    #[doc = "USB0_CH4"]
    pub USB0_CH4: USB0_CH4,
    #[doc = "USB0_CH5"]
    pub USB0_CH5: USB0_CH5,
    #[doc = "USB0_CH6"]
    pub USB0_CH6: USB0_CH6,
    #[doc = "USB0_CH7"]
    pub USB0_CH7: USB0_CH7,
    #[doc = "USB0_CH8"]
    pub USB0_CH8: USB0_CH8,
    #[doc = "USB0_CH9"]
    pub USB0_CH9: USB0_CH9,
    #[doc = "USB0_CH10"]
    pub USB0_CH10: USB0_CH10,
    #[doc = "USB0_CH11"]
    pub USB0_CH11: USB0_CH11,
    #[doc = "USB0_CH12"]
    pub USB0_CH12: USB0_CH12,
    #[doc = "USB0_CH13"]
    pub USB0_CH13: USB0_CH13,
    #[doc = "USIC0"]
    pub USIC0: USIC0,
    #[doc = "USIC1"]
    pub USIC1: USIC1,
    #[doc = "USIC0_CH0"]
    pub USIC0_CH0: USIC0_CH0,
    #[doc = "USIC0_CH1"]
    pub USIC0_CH1: USIC0_CH1,
    #[doc = "USIC1_CH0"]
    pub USIC1_CH0: USIC1_CH0,
    #[doc = "USIC1_CH1"]
    pub USIC1_CH1: USIC1_CH1,
    #[doc = "CAN"]
    pub CAN: CAN,
    #[doc = "CAN_NODE0"]
    pub CAN_NODE0: CAN_NODE0,
    #[doc = "CAN_NODE1"]
    pub CAN_NODE1: CAN_NODE1,
    #[doc = "CAN_MO0"]
    pub CAN_MO0: CAN_MO0,
    #[doc = "CAN_MO1"]
    pub CAN_MO1: CAN_MO1,
    #[doc = "CAN_MO2"]
    pub CAN_MO2: CAN_MO2,
    #[doc = "CAN_MO3"]
    pub CAN_MO3: CAN_MO3,
    #[doc = "CAN_MO4"]
    pub CAN_MO4: CAN_MO4,
    #[doc = "CAN_MO5"]
    pub CAN_MO5: CAN_MO5,
    #[doc = "CAN_MO6"]
    pub CAN_MO6: CAN_MO6,
    #[doc = "CAN_MO7"]
    pub CAN_MO7: CAN_MO7,
    #[doc = "CAN_MO8"]
    pub CAN_MO8: CAN_MO8,
    #[doc = "CAN_MO9"]
    pub CAN_MO9: CAN_MO9,
    #[doc = "CAN_MO10"]
    pub CAN_MO10: CAN_MO10,
    #[doc = "CAN_MO11"]
    pub CAN_MO11: CAN_MO11,
    #[doc = "CAN_MO12"]
    pub CAN_MO12: CAN_MO12,
    #[doc = "CAN_MO13"]
    pub CAN_MO13: CAN_MO13,
    #[doc = "CAN_MO14"]
    pub CAN_MO14: CAN_MO14,
    #[doc = "CAN_MO15"]
    pub CAN_MO15: CAN_MO15,
    #[doc = "CAN_MO16"]
    pub CAN_MO16: CAN_MO16,
    #[doc = "CAN_MO17"]
    pub CAN_MO17: CAN_MO17,
    #[doc = "CAN_MO18"]
    pub CAN_MO18: CAN_MO18,
    #[doc = "CAN_MO19"]
    pub CAN_MO19: CAN_MO19,
    #[doc = "CAN_MO20"]
    pub CAN_MO20: CAN_MO20,
    #[doc = "CAN_MO21"]
    pub CAN_MO21: CAN_MO21,
    #[doc = "CAN_MO22"]
    pub CAN_MO22: CAN_MO22,
    #[doc = "CAN_MO23"]
    pub CAN_MO23: CAN_MO23,
    #[doc = "CAN_MO24"]
    pub CAN_MO24: CAN_MO24,
    #[doc = "CAN_MO25"]
    pub CAN_MO25: CAN_MO25,
    #[doc = "CAN_MO26"]
    pub CAN_MO26: CAN_MO26,
    #[doc = "CAN_MO27"]
    pub CAN_MO27: CAN_MO27,
    #[doc = "CAN_MO28"]
    pub CAN_MO28: CAN_MO28,
    #[doc = "CAN_MO29"]
    pub CAN_MO29: CAN_MO29,
    #[doc = "CAN_MO30"]
    pub CAN_MO30: CAN_MO30,
    #[doc = "CAN_MO31"]
    pub CAN_MO31: CAN_MO31,
    #[doc = "CAN_MO32"]
    pub CAN_MO32: CAN_MO32,
    #[doc = "CAN_MO33"]
    pub CAN_MO33: CAN_MO33,
    #[doc = "CAN_MO34"]
    pub CAN_MO34: CAN_MO34,
    #[doc = "CAN_MO35"]
    pub CAN_MO35: CAN_MO35,
    #[doc = "CAN_MO36"]
    pub CAN_MO36: CAN_MO36,
    #[doc = "CAN_MO37"]
    pub CAN_MO37: CAN_MO37,
    #[doc = "CAN_MO38"]
    pub CAN_MO38: CAN_MO38,
    #[doc = "CAN_MO39"]
    pub CAN_MO39: CAN_MO39,
    #[doc = "CAN_MO40"]
    pub CAN_MO40: CAN_MO40,
    #[doc = "CAN_MO41"]
    pub CAN_MO41: CAN_MO41,
    #[doc = "CAN_MO42"]
    pub CAN_MO42: CAN_MO42,
    #[doc = "CAN_MO43"]
    pub CAN_MO43: CAN_MO43,
    #[doc = "CAN_MO44"]
    pub CAN_MO44: CAN_MO44,
    #[doc = "CAN_MO45"]
    pub CAN_MO45: CAN_MO45,
    #[doc = "CAN_MO46"]
    pub CAN_MO46: CAN_MO46,
    #[doc = "CAN_MO47"]
    pub CAN_MO47: CAN_MO47,
    #[doc = "CAN_MO48"]
    pub CAN_MO48: CAN_MO48,
    #[doc = "CAN_MO49"]
    pub CAN_MO49: CAN_MO49,
    #[doc = "CAN_MO50"]
    pub CAN_MO50: CAN_MO50,
    #[doc = "CAN_MO51"]
    pub CAN_MO51: CAN_MO51,
    #[doc = "CAN_MO52"]
    pub CAN_MO52: CAN_MO52,
    #[doc = "CAN_MO53"]
    pub CAN_MO53: CAN_MO53,
    #[doc = "CAN_MO54"]
    pub CAN_MO54: CAN_MO54,
    #[doc = "CAN_MO55"]
    pub CAN_MO55: CAN_MO55,
    #[doc = "CAN_MO56"]
    pub CAN_MO56: CAN_MO56,
    #[doc = "CAN_MO57"]
    pub CAN_MO57: CAN_MO57,
    #[doc = "CAN_MO58"]
    pub CAN_MO58: CAN_MO58,
    #[doc = "CAN_MO59"]
    pub CAN_MO59: CAN_MO59,
    #[doc = "CAN_MO60"]
    pub CAN_MO60: CAN_MO60,
    #[doc = "CAN_MO61"]
    pub CAN_MO61: CAN_MO61,
    #[doc = "CAN_MO62"]
    pub CAN_MO62: CAN_MO62,
    #[doc = "CAN_MO63"]
    pub CAN_MO63: CAN_MO63,
    #[doc = "VADC"]
    pub VADC: VADC,
    #[doc = "VADC_G0"]
    pub VADC_G0: VADC_G0,
    #[doc = "VADC_G1"]
    pub VADC_G1: VADC_G1,
    #[doc = "VADC_G2"]
    pub VADC_G2: VADC_G2,
    #[doc = "VADC_G3"]
    pub VADC_G3: VADC_G3,
    #[doc = "DSD"]
    pub DSD: DSD,
    #[doc = "DSD_CH0"]
    pub DSD_CH0: DSD_CH0,
    #[doc = "DSD_CH1"]
    pub DSD_CH1: DSD_CH1,
    #[doc = "DSD_CH2"]
    pub DSD_CH2: DSD_CH2,
    #[doc = "DSD_CH3"]
    pub DSD_CH3: DSD_CH3,
    #[doc = "DAC"]
    pub DAC: DAC,
    #[doc = "CCU40"]
    pub CCU40: CCU40,
    #[doc = "CCU41"]
    pub CCU41: CCU41,
    #[doc = "CCU42"]
    pub CCU42: CCU42,
    #[doc = "CCU43"]
    pub CCU43: CCU43,
    #[doc = "CCU40_CC40"]
    pub CCU40_CC40: CCU40_CC40,
    #[doc = "CCU40_CC41"]
    pub CCU40_CC41: CCU40_CC41,
    #[doc = "CCU40_CC42"]
    pub CCU40_CC42: CCU40_CC42,
    #[doc = "CCU40_CC43"]
    pub CCU40_CC43: CCU40_CC43,
    #[doc = "CCU41_CC40"]
    pub CCU41_CC40: CCU41_CC40,
    #[doc = "CCU41_CC41"]
    pub CCU41_CC41: CCU41_CC41,
    #[doc = "CCU41_CC42"]
    pub CCU41_CC42: CCU41_CC42,
    #[doc = "CCU41_CC43"]
    pub CCU41_CC43: CCU41_CC43,
    #[doc = "CCU42_CC40"]
    pub CCU42_CC40: CCU42_CC40,
    #[doc = "CCU42_CC41"]
    pub CCU42_CC41: CCU42_CC41,
    #[doc = "CCU42_CC42"]
    pub CCU42_CC42: CCU42_CC42,
    #[doc = "CCU42_CC43"]
    pub CCU42_CC43: CCU42_CC43,
    #[doc = "CCU43_CC40"]
    pub CCU43_CC40: CCU43_CC40,
    #[doc = "CCU43_CC41"]
    pub CCU43_CC41: CCU43_CC41,
    #[doc = "CCU43_CC42"]
    pub CCU43_CC42: CCU43_CC42,
    #[doc = "CCU43_CC43"]
    pub CCU43_CC43: CCU43_CC43,
    #[doc = "CCU80"]
    pub CCU80: CCU80,
    #[doc = "CCU81"]
    pub CCU81: CCU81,
    #[doc = "CCU80_CC80"]
    pub CCU80_CC80: CCU80_CC80,
    #[doc = "CCU80_CC81"]
    pub CCU80_CC81: CCU80_CC81,
    #[doc = "CCU80_CC82"]
    pub CCU80_CC82: CCU80_CC82,
    #[doc = "CCU80_CC83"]
    pub CCU80_CC83: CCU80_CC83,
    #[doc = "CCU81_CC80"]
    pub CCU81_CC80: CCU81_CC80,
    #[doc = "CCU81_CC81"]
    pub CCU81_CC81: CCU81_CC81,
    #[doc = "CCU81_CC82"]
    pub CCU81_CC82: CCU81_CC82,
    #[doc = "CCU81_CC83"]
    pub CCU81_CC83: CCU81_CC83,
    #[doc = "HRPWM0"]
    pub HRPWM0: HRPWM0,
    #[doc = "HRPWM0_CSG0"]
    pub HRPWM0_CSG0: HRPWM0_CSG0,
    #[doc = "HRPWM0_CSG1"]
    pub HRPWM0_CSG1: HRPWM0_CSG1,
    #[doc = "HRPWM0_CSG2"]
    pub HRPWM0_CSG2: HRPWM0_CSG2,
    #[doc = "HRPWM0_HRC0"]
    pub HRPWM0_HRC0: HRPWM0_HRC0,
    #[doc = "HRPWM0_HRC1"]
    pub HRPWM0_HRC1: HRPWM0_HRC1,
    #[doc = "HRPWM0_HRC2"]
    pub HRPWM0_HRC2: HRPWM0_HRC2,
    #[doc = "HRPWM0_HRC3"]
    pub HRPWM0_HRC3: HRPWM0_HRC3,
    #[doc = "POSIF0"]
    pub POSIF0: POSIF0,
    #[doc = "POSIF1"]
    pub POSIF1: POSIF1,
    #[doc = "PORT0"]
    pub PORT0: PORT0,
    #[doc = "PORT1"]
    pub PORT1: PORT1,
    #[doc = "PORT2"]
    pub PORT2: PORT2,
    #[doc = "PORT3"]
    pub PORT3: PORT3,
    #[doc = "PORT4"]
    pub PORT4: PORT4,
    #[doc = "PORT5"]
    pub PORT5: PORT5,
    #[doc = "PORT14"]
    pub PORT14: PORT14,
    #[doc = "PORT15"]
    pub PORT15: PORT15,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| if unsafe { DEVICE_PERIPHERALS } { None } else { Some(unsafe { Peripherals::steal() }) })
    }
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            PPB: PPB { _marker: PhantomData },
            DLR: DLR { _marker: PhantomData },
            ERU0: ERU0 { _marker: PhantomData },
            ERU1: ERU1 { _marker: PhantomData },
            GPDMA0: GPDMA0 { _marker: PhantomData },
            GPDMA0_CH0: GPDMA0_CH0 { _marker: PhantomData },
            GPDMA0_CH1: GPDMA0_CH1 { _marker: PhantomData },
            GPDMA0_CH2: GPDMA0_CH2 { _marker: PhantomData },
            GPDMA0_CH3: GPDMA0_CH3 { _marker: PhantomData },
            GPDMA0_CH4: GPDMA0_CH4 { _marker: PhantomData },
            GPDMA0_CH5: GPDMA0_CH5 { _marker: PhantomData },
            GPDMA0_CH6: GPDMA0_CH6 { _marker: PhantomData },
            GPDMA0_CH7: GPDMA0_CH7 { _marker: PhantomData },
            FCE: FCE { _marker: PhantomData },
            FCE_KE0: FCE_KE0 { _marker: PhantomData },
            FCE_KE1: FCE_KE1 { _marker: PhantomData },
            FCE_KE2: FCE_KE2 { _marker: PhantomData },
            FCE_KE3: FCE_KE3 { _marker: PhantomData },
            PBA0: PBA0 { _marker: PhantomData },
            PBA1: PBA1 { _marker: PhantomData },
            FLASH0: FLASH0 { _marker: PhantomData },
            PREF: PREF { _marker: PhantomData },
            PMU0: PMU0 { _marker: PhantomData },
            WDT: WDT { _marker: PhantomData },
            RTC: RTC { _marker: PhantomData },
            SCU_CLK: SCU_CLK { _marker: PhantomData },
            SCU_OSC: SCU_OSC { _marker: PhantomData },
            SCU_PLL: SCU_PLL { _marker: PhantomData },
            SCU_GENERAL: SCU_GENERAL { _marker: PhantomData },
            SCU_INTERRUPT: SCU_INTERRUPT { _marker: PhantomData },
            SCU_PARITY: SCU_PARITY { _marker: PhantomData },
            SCU_TRAP: SCU_TRAP { _marker: PhantomData },
            SCU_HIBERNATE: SCU_HIBERNATE { _marker: PhantomData },
            SCU_POWER: SCU_POWER { _marker: PhantomData },
            SCU_RESET: SCU_RESET { _marker: PhantomData },
            LEDTS0: LEDTS0 { _marker: PhantomData },
            ETH0_CON: ETH0_CON { _marker: PhantomData },
            ETH0: ETH0 { _marker: PhantomData },
            USB0: USB0 { _marker: PhantomData },
            USB0_EP0: USB0_EP0 { _marker: PhantomData },
            USB0_EP1: USB0_EP1 { _marker: PhantomData },
            USB0_EP2: USB0_EP2 { _marker: PhantomData },
            USB0_EP3: USB0_EP3 { _marker: PhantomData },
            USB0_EP4: USB0_EP4 { _marker: PhantomData },
            USB0_EP5: USB0_EP5 { _marker: PhantomData },
            USB0_EP6: USB0_EP6 { _marker: PhantomData },
            USB0_CH0: USB0_CH0 { _marker: PhantomData },
            USB0_CH1: USB0_CH1 { _marker: PhantomData },
            USB0_CH2: USB0_CH2 { _marker: PhantomData },
            USB0_CH3: USB0_CH3 { _marker: PhantomData },
            USB0_CH4: USB0_CH4 { _marker: PhantomData },
            USB0_CH5: USB0_CH5 { _marker: PhantomData },
            USB0_CH6: USB0_CH6 { _marker: PhantomData },
            USB0_CH7: USB0_CH7 { _marker: PhantomData },
            USB0_CH8: USB0_CH8 { _marker: PhantomData },
            USB0_CH9: USB0_CH9 { _marker: PhantomData },
            USB0_CH10: USB0_CH10 { _marker: PhantomData },
            USB0_CH11: USB0_CH11 { _marker: PhantomData },
            USB0_CH12: USB0_CH12 { _marker: PhantomData },
            USB0_CH13: USB0_CH13 { _marker: PhantomData },
            USIC0: USIC0 { _marker: PhantomData },
            USIC1: USIC1 { _marker: PhantomData },
            USIC0_CH0: USIC0_CH0 { _marker: PhantomData },
            USIC0_CH1: USIC0_CH1 { _marker: PhantomData },
            USIC1_CH0: USIC1_CH0 { _marker: PhantomData },
            USIC1_CH1: USIC1_CH1 { _marker: PhantomData },
            CAN: CAN { _marker: PhantomData },
            CAN_NODE0: CAN_NODE0 { _marker: PhantomData },
            CAN_NODE1: CAN_NODE1 { _marker: PhantomData },
            CAN_MO0: CAN_MO0 { _marker: PhantomData },
            CAN_MO1: CAN_MO1 { _marker: PhantomData },
            CAN_MO2: CAN_MO2 { _marker: PhantomData },
            CAN_MO3: CAN_MO3 { _marker: PhantomData },
            CAN_MO4: CAN_MO4 { _marker: PhantomData },
            CAN_MO5: CAN_MO5 { _marker: PhantomData },
            CAN_MO6: CAN_MO6 { _marker: PhantomData },
            CAN_MO7: CAN_MO7 { _marker: PhantomData },
            CAN_MO8: CAN_MO8 { _marker: PhantomData },
            CAN_MO9: CAN_MO9 { _marker: PhantomData },
            CAN_MO10: CAN_MO10 { _marker: PhantomData },
            CAN_MO11: CAN_MO11 { _marker: PhantomData },
            CAN_MO12: CAN_MO12 { _marker: PhantomData },
            CAN_MO13: CAN_MO13 { _marker: PhantomData },
            CAN_MO14: CAN_MO14 { _marker: PhantomData },
            CAN_MO15: CAN_MO15 { _marker: PhantomData },
            CAN_MO16: CAN_MO16 { _marker: PhantomData },
            CAN_MO17: CAN_MO17 { _marker: PhantomData },
            CAN_MO18: CAN_MO18 { _marker: PhantomData },
            CAN_MO19: CAN_MO19 { _marker: PhantomData },
            CAN_MO20: CAN_MO20 { _marker: PhantomData },
            CAN_MO21: CAN_MO21 { _marker: PhantomData },
            CAN_MO22: CAN_MO22 { _marker: PhantomData },
            CAN_MO23: CAN_MO23 { _marker: PhantomData },
            CAN_MO24: CAN_MO24 { _marker: PhantomData },
            CAN_MO25: CAN_MO25 { _marker: PhantomData },
            CAN_MO26: CAN_MO26 { _marker: PhantomData },
            CAN_MO27: CAN_MO27 { _marker: PhantomData },
            CAN_MO28: CAN_MO28 { _marker: PhantomData },
            CAN_MO29: CAN_MO29 { _marker: PhantomData },
            CAN_MO30: CAN_MO30 { _marker: PhantomData },
            CAN_MO31: CAN_MO31 { _marker: PhantomData },
            CAN_MO32: CAN_MO32 { _marker: PhantomData },
            CAN_MO33: CAN_MO33 { _marker: PhantomData },
            CAN_MO34: CAN_MO34 { _marker: PhantomData },
            CAN_MO35: CAN_MO35 { _marker: PhantomData },
            CAN_MO36: CAN_MO36 { _marker: PhantomData },
            CAN_MO37: CAN_MO37 { _marker: PhantomData },
            CAN_MO38: CAN_MO38 { _marker: PhantomData },
            CAN_MO39: CAN_MO39 { _marker: PhantomData },
            CAN_MO40: CAN_MO40 { _marker: PhantomData },
            CAN_MO41: CAN_MO41 { _marker: PhantomData },
            CAN_MO42: CAN_MO42 { _marker: PhantomData },
            CAN_MO43: CAN_MO43 { _marker: PhantomData },
            CAN_MO44: CAN_MO44 { _marker: PhantomData },
            CAN_MO45: CAN_MO45 { _marker: PhantomData },
            CAN_MO46: CAN_MO46 { _marker: PhantomData },
            CAN_MO47: CAN_MO47 { _marker: PhantomData },
            CAN_MO48: CAN_MO48 { _marker: PhantomData },
            CAN_MO49: CAN_MO49 { _marker: PhantomData },
            CAN_MO50: CAN_MO50 { _marker: PhantomData },
            CAN_MO51: CAN_MO51 { _marker: PhantomData },
            CAN_MO52: CAN_MO52 { _marker: PhantomData },
            CAN_MO53: CAN_MO53 { _marker: PhantomData },
            CAN_MO54: CAN_MO54 { _marker: PhantomData },
            CAN_MO55: CAN_MO55 { _marker: PhantomData },
            CAN_MO56: CAN_MO56 { _marker: PhantomData },
            CAN_MO57: CAN_MO57 { _marker: PhantomData },
            CAN_MO58: CAN_MO58 { _marker: PhantomData },
            CAN_MO59: CAN_MO59 { _marker: PhantomData },
            CAN_MO60: CAN_MO60 { _marker: PhantomData },
            CAN_MO61: CAN_MO61 { _marker: PhantomData },
            CAN_MO62: CAN_MO62 { _marker: PhantomData },
            CAN_MO63: CAN_MO63 { _marker: PhantomData },
            VADC: VADC { _marker: PhantomData },
            VADC_G0: VADC_G0 { _marker: PhantomData },
            VADC_G1: VADC_G1 { _marker: PhantomData },
            VADC_G2: VADC_G2 { _marker: PhantomData },
            VADC_G3: VADC_G3 { _marker: PhantomData },
            DSD: DSD { _marker: PhantomData },
            DSD_CH0: DSD_CH0 { _marker: PhantomData },
            DSD_CH1: DSD_CH1 { _marker: PhantomData },
            DSD_CH2: DSD_CH2 { _marker: PhantomData },
            DSD_CH3: DSD_CH3 { _marker: PhantomData },
            DAC: DAC { _marker: PhantomData },
            CCU40: CCU40 { _marker: PhantomData },
            CCU41: CCU41 { _marker: PhantomData },
            CCU42: CCU42 { _marker: PhantomData },
            CCU43: CCU43 { _marker: PhantomData },
            CCU40_CC40: CCU40_CC40 { _marker: PhantomData },
            CCU40_CC41: CCU40_CC41 { _marker: PhantomData },
            CCU40_CC42: CCU40_CC42 { _marker: PhantomData },
            CCU40_CC43: CCU40_CC43 { _marker: PhantomData },
            CCU41_CC40: CCU41_CC40 { _marker: PhantomData },
            CCU41_CC41: CCU41_CC41 { _marker: PhantomData },
            CCU41_CC42: CCU41_CC42 { _marker: PhantomData },
            CCU41_CC43: CCU41_CC43 { _marker: PhantomData },
            CCU42_CC40: CCU42_CC40 { _marker: PhantomData },
            CCU42_CC41: CCU42_CC41 { _marker: PhantomData },
            CCU42_CC42: CCU42_CC42 { _marker: PhantomData },
            CCU42_CC43: CCU42_CC43 { _marker: PhantomData },
            CCU43_CC40: CCU43_CC40 { _marker: PhantomData },
            CCU43_CC41: CCU43_CC41 { _marker: PhantomData },
            CCU43_CC42: CCU43_CC42 { _marker: PhantomData },
            CCU43_CC43: CCU43_CC43 { _marker: PhantomData },
            CCU80: CCU80 { _marker: PhantomData },
            CCU81: CCU81 { _marker: PhantomData },
            CCU80_CC80: CCU80_CC80 { _marker: PhantomData },
            CCU80_CC81: CCU80_CC81 { _marker: PhantomData },
            CCU80_CC82: CCU80_CC82 { _marker: PhantomData },
            CCU80_CC83: CCU80_CC83 { _marker: PhantomData },
            CCU81_CC80: CCU81_CC80 { _marker: PhantomData },
            CCU81_CC81: CCU81_CC81 { _marker: PhantomData },
            CCU81_CC82: CCU81_CC82 { _marker: PhantomData },
            CCU81_CC83: CCU81_CC83 { _marker: PhantomData },
            HRPWM0: HRPWM0 { _marker: PhantomData },
            HRPWM0_CSG0: HRPWM0_CSG0 { _marker: PhantomData },
            HRPWM0_CSG1: HRPWM0_CSG1 { _marker: PhantomData },
            HRPWM0_CSG2: HRPWM0_CSG2 { _marker: PhantomData },
            HRPWM0_HRC0: HRPWM0_HRC0 { _marker: PhantomData },
            HRPWM0_HRC1: HRPWM0_HRC1 { _marker: PhantomData },
            HRPWM0_HRC2: HRPWM0_HRC2 { _marker: PhantomData },
            HRPWM0_HRC3: HRPWM0_HRC3 { _marker: PhantomData },
            POSIF0: POSIF0 { _marker: PhantomData },
            POSIF1: POSIF1 { _marker: PhantomData },
            PORT0: PORT0 { _marker: PhantomData },
            PORT1: PORT1 { _marker: PhantomData },
            PORT2: PORT2 { _marker: PhantomData },
            PORT3: PORT3 { _marker: PhantomData },
            PORT4: PORT4 { _marker: PhantomData },
            PORT5: PORT5 { _marker: PhantomData },
            PORT14: PORT14 { _marker: PhantomData },
            PORT15: PORT15 { _marker: PhantomData },
        }
    }
}
