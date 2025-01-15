use embedded_graphics_core::pixelcolor::{Rgb565, Rgb666};
use embedded_hal_async::delay::DelayNs;

use crate::{
    dcs::{BitsPerPixel, PixelFormat, SetAddressMode},
    interface::Interface,
    models::{ili934x, Model},
    options::ModelOptions,
};

/// ILI9342C display in Rgb565 color mode.
pub struct ILI9342CRgb565;

/// ILI9342C display in Rgb666 color mode.
pub struct ILI9342CRgb666;

impl Model for ILI9342CRgb565 {
    type ColorFormat = Rgb565;
    const FRAMEBUFFER_SIZE: (u16, u16) = (320, 240);

    async fn init<DELAY, DI>(
        &mut self,
        di: &mut DI,
        delay: &mut DELAY,
        options: &ModelOptions,
    ) -> Result<SetAddressMode, DI::Error>
    where
        DELAY: DelayNs,
        DI: Interface,
    {
        let pf = PixelFormat::with_all(BitsPerPixel::from_rgb_color::<Self::ColorFormat>());
        ili934x::init_common(di, delay, options, pf).await.map_err(Into::into)
    }
}

impl Model for ILI9342CRgb666 {
    type ColorFormat = Rgb666;
    const FRAMEBUFFER_SIZE: (u16, u16) = (320, 240);

    async fn init<DELAY, DI>(
        &mut self,
        di: &mut DI,
        delay: &mut DELAY,
        options: &ModelOptions,
    ) -> Result<SetAddressMode, DI::Error>
    where
        DELAY: DelayNs,
        DI: Interface,
    {
        let pf = PixelFormat::with_all(BitsPerPixel::from_rgb_color::<Self::ColorFormat>());
        ili934x::init_common(di, delay, options, pf).await.map_err(Into::into)
    }
}
