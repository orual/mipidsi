use embedded_graphics_core::pixelcolor::Rgb565;
use embedded_hal_async::delay::DelayNs;

use crate::{
    dcs::{
        BitsPerPixel, EnterNormalMode, ExitSleepMode, InterfaceExt, PixelFormat, SetAddressMode,
        SetDisplayOn, SetInvertMode, SetPixelFormat,
    },
    interface::Interface,
    models::Model,
    options::ModelOptions,
};

/// ST7789 display in Rgb565 color mode.
pub struct ST7789;

impl Model for ST7789 {
    type ColorFormat = Rgb565;
    const FRAMEBUFFER_SIZE: (u16, u16) = (240, 320);

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
        let madctl = SetAddressMode::from(options);

        delay.delay_us(150_000).await;

        di.write_command(ExitSleepMode).await?;
        delay.delay_us(10_000).await;

        // set hw scroll area based on framebuffer size
        di.write_command(madctl).await?;

        di.write_command(SetInvertMode::new(options.invert_colors)).await?;

        let pf = PixelFormat::with_all(BitsPerPixel::from_rgb_color::<Self::ColorFormat>());
        di.write_command(SetPixelFormat::new(pf)).await?;
        delay.delay_us(10_000).await;
        di.write_command(EnterNormalMode).await?;
        delay.delay_us(10_000).await;
        di.write_command(SetDisplayOn).await?;

        // DISPON requires some time otherwise we risk SPI data issues
        delay.delay_us(120_000).await;

        Ok(madctl)
    }
}
