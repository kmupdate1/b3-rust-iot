# Architecture

This document describes the logical target architecture of this project.

The current repository may contain all components in a single monorepo.
Directories and Rust crates do not necessarily correspond 1:1.

Components may be split into independent repositories in the future if reuse,
release management, or ownership boundaries make that useful.

The important rule is to keep responsibilities and dependency directions clear.


## Target Structure

```text
iot/
├── runtime/
│   ├── runtime-core/
│   ├── provisioning/
│   ├── registration/
│   └── ota/
│
├── hal/
│   ├── capability/
│   ├── rp235x/
│   ├── esp32/
│   └── embassy-net/
│
├── driver/
│   ├── driver-core/
│   ├── sensor/
│   │   ├── bme280/
│   │   ├── sht40/
│   │   └── ina219/
│   └── actuator/
│       ├── pump/
│       └── relay/
│
├── domain/
│   ├── field-edge/
│   ├── irrigation/
│   └── energy/
│
├── board/
│   ├── field-edge-rp235x/
│   └── other-products/
│
└── examples/
```


## runtime/

The runtime layer controls the lifecycle of the device after power-on.

Responsibilities may include:

- startup orchestration
- first-time provisioning
- Wi-Fi / network connection lifecycle
- device registration
- device identity handling
- OTA firmware update
- recovery and rollback
- launching the domain application

The runtime should avoid depending directly on a specific MCU implementation.

Example:

```text
runtime-core
    ↓
capability traits
```

The runtime should request capabilities such as networking, storage, reboot,
Bluetooth, OTA storage, or device identity without knowing whether they are
implemented by RP235x, ESP32, or another platform.


## hal/

The HAL layer defines hardware capabilities and their platform-specific
implementations.

### hal/capability/

Defines hardware-independent capabilities.

Examples:

- DigitalInput
- DigitalOutput
- AnalogInput
- I2c
- Spi
- Uart
- Wifi
- Bluetooth
- Tcp
- Udp
- Storage
- Reboot
- Timer

Capability definitions must not depend on a specific MCU vendor SDK.

Example:

```text
driver
    ↓
capability::I2c
```

A sensor driver should not know whether I2C is provided by RP235x, ESP32,
STM32, or another platform.


### hal/rp235x/

Implements HAL capabilities for RP235x hardware.

This layer may depend on:

- embassy-rp
- RP235x peripherals
- RP235x interrupts
- RP235x pin mappings

Examples:

Rp235xI2c
Rp235xWifi
DigitalOutputPin
Rp235xAdc


### hal/esp32/

Future implementation of the same capabilities for ESP32.

Ideally, domain code and device drivers should not require changes when
switching from RP235x to ESP32.


### hal/embassy-net/

Networking implementations shared between hardware platforms where useful.

This layer may contain reusable Embassy networking adapters that are not
specific to one MCU.


## driver/

Drivers represent reusable hardware components.

Drivers depend on capabilities, not on concrete MCU implementations.

Example:

Bme280<I: I2c>

instead of:

Bme280<Rp235xI2c>

Typical driver categories:

driver/sensor/
driver/actuator/
driver/network/
driver/display/


A driver knows:

- device registers
- device protocol
- device address
- conversion formulas
- hardware component behavior

A driver should not know:

- which MCU is being used
- which physical pins are connected
- how the device is registered in the backend
- product-specific business logic


## domain/

The domain layer describes what the product does.

Examples:

- field-edge
- irrigation control
- energy monitoring
- environmental monitoring

The domain layer combines drivers and capabilities into product behavior.

Examples:

FieldEdge
PumpController
EnergyMeter
ObservationSender

The domain should contain product rules and use cases, but should avoid direct
dependencies on MCU-specific HAL implementations.


## board/

The board layer describes how a concrete physical product is assembled.

Responsibilities include:

- MCU selection
- peripheral selection
- pin assignment
- sensor selection
- actuator selection
- driver construction
- physical hardware topology

Example:

field-edge-rp235x

may define:

RP235x
I2C0
SDA -> PIN_4
SCL -> PIN_5
Pump -> PIN_0
Moisture sensor -> ADC0
CYW43 -> Wi-Fi

The board layer is allowed to know concrete HAL implementations.

Example:

```text
board
    ↓
hal/rp235x
    ↓
embassy-rp
```


The board layer creates concrete hardware objects and passes them into runtime
and domain layers.


## examples/

Examples are used for isolated capability and driver verification.

Examples may directly use platform-specific implementations.

Examples:

examples/gpio
examples/i2c
examples/wifi
examples/bme280
examples/iot

They are primarily development and verification tools and are not required to
follow the same abstraction boundaries as production domain code.


## Dependency Direction

Preferred dependency direction:

```text
runtime
    ↓
domain
    ↓
driver
    ↓
hal/capability
```

Platform implementation:

board
    ↓
hal/rp235x
    ↓
external MCU framework

Concrete objects are assembled at the application / board boundary.

Generic layers should depend inward toward capabilities and abstractions.


## Product Composition

A final firmware image is produced by selecting components.

Example product:

Runtime:
    runtime-core

HAL:
    rp235x

Drivers:
    bme280
    ina219
    pump

Domain:
    field-edge

Board:
    field-edge-rp235x

Then:

```text
board + runtime + domain + drivers + HAL
                    ↓
                 build
                    ↓
             firmware image
```


## Future Repository Split

The current monorepo may eventually be split into separate repositories.

Possible future structure:

b3-runtime
b3-hal
b3-drivers
b3-field-edge
b3-irrigation

This is optional.

Repository boundaries should only be introduced when independent versioning,
reuse, ownership, or release management makes the split useful.

Until then, the monorepo remains the preferred development environment.


## Design Principles

1. MCU-specific code stays in HAL implementation and board layers.

2. Drivers depend on capabilities rather than concrete MCU implementations.

3. Domain code describes product behavior, not hardware details.

4. Runtime manages the device lifecycle, provisioning, networking, registration,
   and OTA.

5. Board code defines the physical hardware composition.

6. main.rs should remain a thin composition entry point where possible.

7. Logical architecture boundaries are more important than directory names.

8. A directory may contain one crate or multiple crates.

9. Crates may later become independent repositories without changing the core
   dependency model.

10. Prefer reuse through clear interfaces rather than sharing concrete platform
    implementations.
