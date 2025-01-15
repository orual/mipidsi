use embedded_graphics_core::pixelcolor::Rgb565;

use embedded_hal_async::delay::DelayNs;


use crate::dcs::{SetColumnAddress, SetPageAddress};
use crate::{
    dcs::{
        BitsPerPixel, EnterNormalMode, ExitSleepMode, InterfaceExt, PixelFormat, SetAddressMode,
        SetDisplayOn, SetInvertMode, SetPixelFormat,
    },
    interface::Interface,
    models::Model,
    options::ModelOptions,
};

/// ST7789V display in Rgb565 color mode.
/// Uses init sequence 2 from https://github.com/Xinyuan-LilyGO/T-Deck/blob/master/lib/TFT_eSPI/TFT_Drivers/ST7789_Init.h
/// May require inverting colors
pub struct ST7789V;

impl Model for ST7789V {
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
        //di.write_command(SoftReset).await?;
        delay.delay_us(150_000).await;

        di.write_command(ExitSleepMode).await?;
        delay.delay_us(120_000).await;

        // set hw scroll area based on framebuffer size
        di.write_command(madctl).await?;

        
        
        di.write_raw(0x3A, &[0x55]).await?; // ST7789_COLMOD
        delay.delay_us(10_000).await;
        // set frame rate
        di.write_raw(0xB2, &[0x0C, 0x0C, 0x00, 0x33, 0x33]).await?; // ST7789_PORCTRL
        di.write_raw(0xB7, &[0x75]).await?; // ST7789_GCTRL

        // power settings
        di.write_raw(0xBB, &[0x1A]).await?; // ST7789_VCOMS
        di.write_raw(0xC0, &[0x2C]).await?; // ST7789_LCMCTRL
        di.write_raw(0xC2, &[0x01]).await?; // sST7789_VDVVRHEN
        di.write_raw(0xC3, &[0x13]).await?; // ST7789_VRHS
        di.write_raw(0xC4, &[0x20]).await?; // ST7789_VDVSET
        di.write_raw(0xC6, &[0x0F]).await?; // ST7789_FRCTR2
        di.write_raw(0xD0, &[0xA4, 0xA1]).await?; // ST7789_PWCTRL1
        di.write_raw(
            0xE0,
            &[
                0xD0, 0x0D, 0x14, 0x0D, 0x0D, 0x09, 0x38, 0x44, 0x4E, 0x3A, 0x17, 0x18, 0x2F, 0x30,
            ],
        ).await?; // set GAMMA +Polarity characteristics
        di.write_raw(
            0xE1,
            &[
                0xD0, 0x09, 0x0F, 0x08, 0x07, 0x14, 0x37, 0x44, 0x4D, 0x38, 0x15, 0x16, 0x2C, 0x3E,
            ],
        ).await?; // set GAMMA -Polarity characteristics

        di.write_command(SetInvertMode::new(options.invert_colors)).await?;

        di.write_command(SetColumnAddress::new(0, 239)).await?;
        di.write_command(SetPageAddress::new(0, 319)).await?;

        let pf = PixelFormat::with_all(BitsPerPixel::from_rgb_color::<Self::ColorFormat>());
        di.write_command(SetPixelFormat::new(pf)).await?;
        
        delay.delay_us(10_000).await;
        di.write_command(EnterNormalMode).await?;
        delay.delay_us(120_000).await;
        di.write_command(SetDisplayOn).await?;

        // DISPON requires some time otherwise we risk SPI data issues
        delay.delay_us(120_000).await;

        Ok(madctl)
    }
}
