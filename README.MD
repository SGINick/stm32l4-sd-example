Minimal example for sd card on the stm32l433rc, starting from basics

The sd card does not initialize.  There was originally an issue with the clock source that was fixed in

https://github.com/embassy-rs/stm32-data/pull/625

This isn't in the main branch of embassy though, so to run this correctly, embassy needs to be cloned to the main directory

```
 git clone https://github.com/embassy-rs/embassy.git
```

Then the commented out line in cargo.toml needs to be switched out like this

``` toml
# embassy-stm32 = { git = "https://github.com/embassy-rs/embassy" , rev = "b5ab3276dce7322e33946e974770fa91b98124a4", features = ["stm32l433rc", "time-driver-any","memory-x", "exti", "unstable-pac", "defmt"]  }

embassy-stm32 = { version = "0.2.0", path = "./embassy/embassy-stm32", features = ["stm32l433rc", "time-driver-any","memory-x", "exti", "unstable-pac", "defmt"]  }

```

 and the embassy-stm32 cargo.toml needs to have it's stm32-metapac git source changed to

``` rust
stm32-metapac = { git = "https://github.com/embassy-rs/stm32-data-generated", tag = "stm32-data-6b44770affff65c336d11c337d7db4f95774fab2" } // on line 84 of embassy/embassy-stm32/cargo.toml
```

and 
``` rust
stm32-metapac = { git = "https://github.com/embassy-rs/stm32-data-generated", tag = "stm32-data-6b44770affff65c336d11c337d7db4f95774fab2", default-features = false, features = ["metadata"] } // on line 113 of embassy/embassy-stm32/cargo.toml
```


Circuit layout (with 47k pullup resistors shown)
``` 
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