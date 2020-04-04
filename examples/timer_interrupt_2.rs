#![no_main]
#![no_std]

extern crate panic_halt;

use core::cell::RefCell;
use core::ops::DerefMut;
use cortex_m;
use cortex_m::interrupt::{free, Mutex};
use cortex_m_rt::entry;
use stm32f3xx_hal::{
    gpio::gpioe::PE9,
    gpio::{Output, PushPull},
    prelude::*,
    stm32,
    stm32::{interrupt, Interrupt},
    timer::{Event, Timer},
};

static LED: Mutex<RefCell<Option<PE9<Output<PushPull>>>>> = Mutex::new(RefCell::new(None));
static TIMER_TIM2: Mutex<RefCell<Option<Timer<stm32::TIM2>>>> = Mutex::new(RefCell::new(None));

#[interrupt]
fn TIM2() {
    free(|cs| {
        if let Some(ref mut tim2) = TIMER_TIM2.borrow(cs).borrow_mut().deref_mut() {
            tim2.clear_update_interrupt_flag();
        }
        if let Some(ref mut led) = LED.borrow(cs).borrow_mut().deref_mut() {
            led.toggle().unwrap();
        }
    });
}

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    // Set up the system clock
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Set up the LED
    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);
    let led = gpioe
        .pe9
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    // Set up the interrupt timer
    let mut timer = Timer::tim2(dp.TIM2, 10.hz(), clocks, &mut rcc.apb1);
    timer.listen(Event::Update);

    // Move shared resources to Mutex
    free(|cs| {
        TIMER_TIM2.borrow(cs).replace(Some(timer));
        LED.borrow(cs).replace(Some(led));
    });

    // Enable interrupt
    stm32::NVIC::unpend(Interrupt::TIM2);
    unsafe {
        stm32::NVIC::unmask(Interrupt::TIM2);
    }

    loop {}
}
