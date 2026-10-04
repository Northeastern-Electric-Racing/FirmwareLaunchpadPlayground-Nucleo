#![no_std]
#![no_main]

use cortex_m::peripheral::SCB;
use cortex_m_rt::{ExceptionFrame, exception};
use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::{wdg::IndependentWatchdog};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    // PA5 is the user LED
    // PC13 is the user button

    let mut iwdg = IndependentWatchdog::new(p.IWDG, 1_000_000);
    iwdg.unleash();

    let mut alt = false;

    loop {

        if !alt {
            info!(".");
        } else {
            info!("..");
        }

        iwdg.pet();
        
        alt = !alt;

        Timer::after_millis(500).await;
    }
}

#[exception]
unsafe fn HardFault(_info: &ExceptionFrame) -> ! {
    SCB::sys_reset(); // reset STM
}
