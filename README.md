# Examples for stm32f3xx_hal

A collection of small examples built with `stm32f3xx_hal`. I also have [another collection](https://github.com/lonesometraveler/stm32f4xx-examples) for `stm32f4xx_hal`.


## Overview
You will find the follwoings in `examples`:

- `timer_interrupt_1.rs`: Timer interrupt flips a `bool`. A LED gets flipped based on the bool in the main.
- `timer_interrupt_2.rs`: Timer interrupt toggles a LED.

I am planning to add more.

## Usage

**Note** I wrote these for [STM32F3DISCOVERY board](https://www.st.com/en/evaluation-tools/stm32f3discovery.html) which has a STM32F303VCT6 microcontroller. If you use a different microcontroller, you need to adjust the settings accordingly.

1. Clone this repo.
``` console
$ git clone https://github.com/lonesometraveler/stm32f3xx-examples.git
```

2. If necessary, set a default target in `.cargo/config` and edit the memory region info in `memory.x`.

3. Build the examples.

``` console
$ cargo build --examples
```

### Cortex Debug

The config file for [Cortex-Debug extension for VS Code](https://marketplace.visualstudio.com/items?itemName=marus25.cortex-debug) is in `.vscode` folder. If your board is STM32FDISCOVERY and you plan to use OpenOCD, it's pretty much ready to go. Just specify an executable in `.vscode/launch.json`.
