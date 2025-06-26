// Minimal example for sd card on stm32 L433RC
//
// Circuit layout (with 47k pullup resistors shown)
// ┌──────────────┐        ┌──────────────┐
// │   SD Card    │        │  STM32L433RC │
// │              │        │              │
// │       CMD ●──┼───┬────┼─► PD2        │
// │              │   │    │              │
// │              │ [47K]  │ STM32L433RC  │
// │              │   │    │              │
// │              │  3V3   │              │
// │              │        │              │
// │        CK ●──┼────────┼──► PC12      │
// │              │        │              │
// │        D0 ●──┼───┬────┼──► PC8       │
// │              │   │    │              │
// │              │ [47K]  │              │
// │              │   │    │              │
// │              │  3V3   │              │
// │              │        │              │
// │       VDD ●──┼────────┼──► 3V3       │
// │              │        │              │
// │       VCC ●──┼────────┼──► GND       │
// │              │        └──────────────┘  
// │      D1   ●──┼─[47K]──► 3V3 
// │              │          
// │      D2   ●──┼─[47K]──► 3V3 
// │              │ 
// │      D3   ●──┼─[47K]──► 3V3 
// │              │ 
// └──────────────┘

// The current issue is that it just times out, output is:
//
// INFO  Initializing SD Card
// └─ stm32l4_sd_example::____embassy_main_task::{async_fn#0} @ src\main.rs:65
// INFO  Timeout
// └─ stm32l4_sd_example::____embassy_main_task::{async_fn#0} @ src\main.rs:70

#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::sdmmc::{self,Sdmmc};
use embassy_stm32::time::Hertz;
use embassy_stm32::{bind_interrupts, peripherals};
use embassy_stm32::pac;
use embassy_stm32::Config;
use {defmt_rtt as _, panic_probe as _};
bind_interrupts!(struct Irqs {
    SDMMC1 => sdmmc::InterruptHandler<peripherals::SDMMC1>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // pac::RCC.apb2enr().modify(|w| w.set_sdmmcen(true)); // sdmmc clock enable
    // pac::RCC.ccipr().modify(|w| w.set_clk48sel(pac::rcc::vals::Clk48sel::HSI48)); // set sdmmc clock to HSI48
    let p = embassy_stm32::init(Default::default());
    let mut sd_config = embassy_stm32::sdmmc::Config::default();
    // sd_config.data_transfer_timeout= 0xFFFFFFFF;
    let mut sd_card = Sdmmc::new_1bit(p.SDMMC1, Irqs, p.DMA2_CH4, p.PC12, p.PD2, p.PC8, sd_config);

    info!("Initializing SD Card");
    loop{// added a loop to auto-retry the initialization.
        if let Err(e) = sd_card.init_sd_card(embassy_stm32::time::mhz(24)).await{
            match e {
                sdmmc::Error::Timeout=>{
                    info!("Timeout");
                }
                sdmmc::Error::SoftwareTimeout=>{
                    info!("Software Timeout");
                }
                sdmmc::Error::UnsupportedCardVersion=>{
                    info!("Unsupported Card Version");
                }
                sdmmc::Error::UnsupportedCardType=>{
                    info!("Unsupported Card Type");
                }
                sdmmc::Error::UnsupportedVoltage=>{
                    info!("Unsupported Voltage");
                }
                sdmmc::Error::Crc=>{
                    info!("CRC Error");
                }
                sdmmc::Error::NoCard=>{
                    info!("No Card Inserted");
                }
                sdmmc::Error::BusWidth=>{
                    info!("8-lane buses are not supported for SD cards");
                }
                sdmmc::Error::BadClock=>{
                    info!("Bad Clock Supplied to the SDMMC Peripheral");
                }
                sdmmc::Error::SignalingSwitchFailed=>{
                    info!("Signaling Switch Failed");
                }
                sdmmc::Error::Underrun=>{
                    info!("Underrun Error");
                }
                sdmmc::Error::StBitErr=>{
                    info!("ST Bit Error");
                }
                _ => {
                    info!("Unknown error");
                }

            }
        }else{
            info!("initialized");
            // break;
        }
    }
    loop {
        cortex_m::asm::nop();
    }
}

