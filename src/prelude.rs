pub use crate::analog::adc::AdcExt as _;
pub use crate::crc::CrcExt as _;
pub use crate::exti::ExtiExt as _;
pub use crate::gpio::GpioExt as _;
pub use crate::i2c::I2cExt as _;
pub use crate::power::PowerExt as _;
pub use crate::rcc::LSCOExt as _;
pub use crate::rcc::MCOExt as _;
pub use crate::rcc::RccExt as _;
pub use crate::rtc::RtcExt as _;
pub use crate::serial::SerialExt as _;
pub use crate::spi::SpiExt as _;
pub use crate::time::U32Ext as _;
pub use crate::timer::delay::DelayExt as _;
pub use crate::timer::opm::OpmExt as _;
pub use crate::timer::pwm::PwmExt as _;
pub use crate::timer::qei::QeiExt as _;
pub use crate::timer::stopwatch::StopwatchExt as _;
pub use crate::timer::TimerExt as _;
pub use crate::watchdog::IWDGExt as _;
pub use crate::watchdog::WWDGExt as _;
pub use fugit::{ExtU32 as _, RateExtU32 as _};
pub use hal::digital::PinState;

#[cfg(feature = "i2c-nonblocking")]
pub use crate::i2c::nonblocking::I2cSlave;

#[cfg(feature = "i2c-blocking")]
pub use crate::i2c::blocking::I2cSlave;
