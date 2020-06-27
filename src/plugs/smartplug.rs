pub trait Smartplug {
    /// Set the power state of a smart plug. `true` means it's powered on, and `false` means off.
    fn set_power(&self, power: bool) -> Result<(), std::io::Error>;
}
