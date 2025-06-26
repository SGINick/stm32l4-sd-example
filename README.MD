Minimal example for sd card on the stm32l433rc, starting from basics

The sd card does not initialize.  There was originally an issue with the clock source that was fixed in

https://github.com/embassy-rs/stm32-data/pull/625

to flash on the NUCLEO-L433RC-P, hook up as shown below and then `cargo run`

```
Circuit layout (with 47k pullup resistors shown)
┌──────────────┐        ┌──────────────┐
│   SD Card    │        │  STM32L433RC │
│              │        │              │
│       CMD ●──┼───┬────┼─► PD2        │
│              │   │    │              │
│              │ [47K]  │ STM32L433RC  │
│              │   │    │              │
│              │  3V3   │              │
│              │        │              │
│        CK ●──┼────────┼──► PC12      │
│              │        │              │
│        D0 ●──┼───┬────┼──► PC8       │
│              │   │    │              │
│              │ [47K]  │              │
│              │   │    │              │
│              │  3V3   │              │
│              │        │              │
│       VDD ●──┼────────┼──► 3V3       │
│              │        │              │
│       VCC ●──┼────────┼──► GND       │
│              │        └──────────────┘  
│      D1   ●──┼─[47K]──► 3V3 
│              │          
│      D2   ●──┼─[47K]──► 3V3 
│              │ 
│      D3   ●──┼─[47K]──► 3V3 
│              │ 
└──────────────┘
```

after that, it still fails, although the correct initialization clock speed is now being used.

