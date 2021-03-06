#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, prelude::*, Delay, Leds};

#[entry]
fn main() -> ! {
let (mut delay, mut leds): (Delay, Leds) = aux5::init();

let half_period = 500_u16;

loop {
leds[0].on();
delay.delay_ms(half_period);
leds[1].on();
delay.delay_ms(half_period);
leds[2].on();
delay.delay_ms(half_period);
leds[3].on();
delay.delay_ms(half_period);
leds[4].on();
delay.delay_ms(half_period);
leds[5].on();
delay.delay_ms(half_period);
leds[6].on();
delay.delay_ms(half_period);
leds[7].on();
delay.delay_ms(half_period);
leds[0].off();
delay.delay_ms(half_period);
leds[1].off();
delay.delay_ms(half_period);
leds[2].off();
delay.delay_ms(half_period);
leds[3].off();
delay.delay_ms(half_period);
leds[4].off();
delay.delay_ms(half_period);
leds[5].off();
delay.delay_ms(half_period);
leds[6].off();
delay.delay_ms(half_period);
leds[7].off();
delay.delay_ms(half_period);
}
}
