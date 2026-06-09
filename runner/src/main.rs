#![no_std]
#![no_main]

use embassy_executor::Spawner;
use defmt_rtt as _;
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let _p = embassy_stm32::init(Default::default());

    loop {}
}
