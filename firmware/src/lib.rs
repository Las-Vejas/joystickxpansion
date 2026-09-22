#![no_std]

use xpanse_api::{
    bus::allocator::BusAllocator,
    driver::{Driver, DriverError, DriverMeta},
    gpio_bank::{BankPins, GpioBank},
    interfaces::buttons::{A, Down, Up, Left, Right, pin_button},
    metadata::{ModuleDetectResistor, ModuleID, ModuleSlot},
    registry::Registry,
};

pub struct JoystickDriver;

impl DriverMeta for JoystickDriver {
    const ID: ModuleID = ModuleID {
        md0: ModuleDetectResistor::R1K,
        md1: ModuleDetectResistor::R100K,
    };
}

impl<G: BankPins> Driver<G> for JoystickDriver {
    async fn create(
        gpio_bank: GpioBank<G>,
        slot: ModuleSlot,
        registry: &mut Registry,
        bus_allocator: &mut BusAllocator,
    ) -> Result<(), DriverError> {
        registry.register(slot, Self::ID, pin_button::<Up>(gpio_bank.gpio0.into()));
        registry.register(slot, Self::ID, pin_button::<Right>(gpio_bank.gpio1.into()));
        registry.register(slot, Self::ID, pin_button::<Down>(gpio_bank.gpio2.into()));
        registry.register(slot, Self::ID, pin_button::<Left>(gpio_bank.gpio3.into()));
        // Pressing the stick in acts as the primary action button
        registry.register(slot, Self::ID, pin_button::<A>(gpio_bank.gpio4.into()));

        // Plain GPIO inputs, no buses needed
        let _ = bus_allocator;

        Ok(())
}
}