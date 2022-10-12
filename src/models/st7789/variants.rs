use display_interface::WriteOnlyDataCommand;

use crate::{DisplayBuilder, ModelOptions, Orientation};

use super::ST7789;

impl<DI> DisplayBuilder<DI, ST7789>
where
    DI: WriteOnlyDataCommand,
{
    ///
    /// Creates a new [Display] instance with [ST7789] as the [Model] with
    /// general variant using display size of 240x320
    ///
    /// # Arguments
    ///
    /// * `di` - a [DisplayInterface](WriteOnlyDataCommand) for talking with the display
    ///
    pub fn st7789(di: DI) -> Self {
        Self::new(di, ST7789, ModelOptions::with_display_size(240, 320))
    }

    ///
    /// Creates a new [Display] instance with [ST7789] as the [Model] with
    /// general variant using display size of 240x240
    ///
    /// # Arguments
    ///
    /// * `di` - a [DisplayInterface](WriteOnlyDataCommand) for talking with the display
    ///
    pub fn st7789_240x240(di: DI) -> Self {
        Self::new(di, ST7789, ModelOptions::with_display_size(240, 240))
    }
    ///
    /// Creates a new [Display] instance with [ST7789] as the [Model] with
    /// general variant using display size of 240x240 but a frame buffer of 240x320 and adjusting the offset
    ///
    /// # Arguments
    ///
    /// * `di` - a [DisplayInterface](WriteOnlyDataCommand) for talking with the display
    ///
    pub fn st7789_240x240_b240x320(di: DI) -> Self {
        Self::new(
            di,
            ST7789,
            ModelOptions::with_all((240, 240), (240, 320), y80_offset),
        )
    }

    ///
    /// Creates a new [Display] instance with [ST7789] as the [Model] with
    /// pico1 variant using display size of 135x240 and a clipping offset
    ///
    /// # Arguments
    ///
    /// * `di` - a [DisplayInterface](WriteOnlyDataCommand) for talking with the display
    ///
    pub fn st7789_pico1(di: DI) -> Self {
        // pico v1 is cropped to 135x240 size with an offset of (40, 53)
        Self::new(
            di,
            ST7789,
            ModelOptions::with_all((135, 240), (135, 240), pico1_offset),
        )
    }
}

// ST7789 pico1 variant with variable offset
pub(crate) fn pico1_offset(orientation: Orientation) -> (u16, u16) {
    match orientation {
        Orientation::Portrait(false) => (52, 40),
        Orientation::Portrait(true) => (53, 40),
        Orientation::Landscape(false) => (40, 52),
        Orientation::Landscape(true) => (40, 53),
        Orientation::PortraitInverted(false) => (53, 40),
        Orientation::PortraitInverted(true) => (52, 40),
        Orientation::LandscapeInverted(false) => (40, 53),
        Orientation::LandscapeInverted(true) => (40, 52),
    }
}

// An offset of 80 y pixels for the st7789 when the display is 240x240 but the frame buffer
// is 240x320
pub(crate) fn y80_offset(orientation: Orientation) -> (u16, u16) {
    match orientation {
        Orientation::Portrait(false) => (0, 0),
        Orientation::Portrait(true) => (0, 0),
        Orientation::Landscape(false) => (0, 0),
        Orientation::Landscape(true) => (0, 0),
        Orientation::PortraitInverted(false) => (0, 80),
        Orientation::PortraitInverted(true) => (0, 80),
        Orientation::LandscapeInverted(false) => (80, 0),
        Orientation::LandscapeInverted(true) => (80, 0),
    }
}
