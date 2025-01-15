use embedded_graphics_core::pixelcolor::Rgb565;
use embedded_hal_async::delay::DelayNs;

use crate::{
    dcs::{
        BitsPerPixel, ExitSleepMode, InterfaceExt, PixelFormat, SetAddressMode, SetDisplayOn,
        SetInvertMode, SetPixelFormat,
    },
    interface::Interface,
    models::Model,
    options::ModelOptions,
};

/// ST7735s display in Rgb565 color mode.
pub struct ST7735s;

impl Model for ST7735s {
    type ColorFormat = Rgb565;
    const FRAMEBUFFER_SIZE: (u16, u16) = (132, 162);

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

        delay.delay_us(200_000).await;

        di.write_command(ExitSleepMode).await?; // turn off sleep
        delay.delay_us(120_000).await;

        di.write_command(SetInvertMode::new(options.invert_colors)).await?; // set color inversion
        di.write_raw(0xB1, &[0x05, 0x3A, 0x3A]).await?; // set frame rate
        di.write_raw(0xB2, &[0x05, 0x3A, 0x3A]).await?; // set frame rate
        di.write_raw(0xB3, &[0x05, 0x3A, 0x3A, 0x05, 0x3A, 0x3A]).await?; // set frame rate
        di.write_raw(0xB4, &[0b0000_0011]).await?; // set inversion control
        di.write_raw(0xC0, &[0x62, 0x02, 0x04]).await?; // set power control 1
        di.write_raw(0xC1, &[0xC0]).await?; // set power control 2
        di.write_raw(0xC2, &[0x0D, 0x00]).await?; // set power control 3
        di.write_raw(0xC3, &[0x8D, 0x6A]).await?; // set power control 4
        di.write_raw(0xC4, &[0x8D, 0xEE]).await?; // set power control 5
        di.write_raw(0xC5, &[0x0E]).await?; // set VCOM control 1
        di.write_raw(
            0xE0,
            &[
                0x10, 0x0E, 0x02, 0x03, 0x0E, 0x07, 0x02, 0x07, 0x0A, 0x12, 0x27, 0x37, 0x00, 0x0D,
                0x0E, 0x10,
            ],
        ).await?; // set GAMMA +Polarity characteristics
        di.write_raw(
            0xE1,
            &[
                0x10, 0x0E, 0x03, 0x03, 0x0F, 0x06, 0x02, 0x08, 0x0A, 0x13, 0x26, 0x36, 0x00, 0x0D,
                0x0E, 0x10,
            ],
        ).await?; // set GAMMA -Polarity characteristics

        let pf = PixelFormat::with_all(BitsPerPixel::from_rgb_color::<Self::ColorFormat>());
        di.write_command(SetPixelFormat::new(pf)).await?; // set interface pixel format, 16bit pixel into frame memory

        di.write_command(madctl).await?; // set memory data access control, Top -> Bottom, RGB, Left -> Right
        di.write_command(SetDisplayOn).await?; // turn on display

        Ok(madctl)
    }
}
