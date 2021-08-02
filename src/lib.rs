    use core::convert::TryInto;
    use embedded_graphics::{
        pixelcolor::{Rgb888, RgbColor},
        prelude::*,
    };

    use framebuffer::Framebuffer;

    pub struct FrameBufferDisplay {
        framebuffer: Vec<u8>,
        iface: Framebuffer,
    }

    impl FrameBufferDisplay{
        // Send buffer to the display
        pub fn flush(&mut self) -> Result<(), ()> {
            self.iface.write_frame(&self.framebuffer);
            Ok(())
        }
    }

    impl DrawTarget for FrameBufferDisplay {
        type Color = Rgb888;
        type Error = core::convert::Infallible;

        // Map draw onto the frame buffer
        fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
        where
            I: IntoIterator<Item = Pixel<Self::Color>>,
        {
            let xres = self.iface.var_screen_info.xres;
            let yres = self.iface.var_screen_info.yres;
            let bytespp = self.iface.var_screen_info.bits_per_pixel / 8;

            for Pixel(coord, color) in pixels.into_iter() {
                let x: i32 = coord.x.try_into().unwrap();
                let y: i32 = coord.y.try_into().unwrap();
                if 0 <= x && x < xres as i32 && 0 <= y && y < yres as i32  {
                    let index: u32 = (x as u32 + y as u32 * xres)*bytespp;
                    self.framebuffer[index as usize] = color.b();
                    self.framebuffer[index as usize + 1] = color.g();
                    self.framebuffer[index as usize + 2] = color.r();
                }

            }

            Ok(())

        }
    }

    impl OriginDimensions for FrameBufferDisplay {
        fn size(&self) -> Size {
            Size::new(self.iface.var_screen_info.xres, self.iface.var_screen_info.yres)
        }
    }

    impl FrameBufferDisplay {
        pub fn new() -> FrameBufferDisplay {
            let framebuffer = Framebuffer::new("/dev/fb0").unwrap();
            let h = framebuffer.var_screen_info.yres;
            let line_length = framebuffer.fix_screen_info.line_length;

            FrameBufferDisplay {
                framebuffer: vec![0u8; (line_length * h) as usize],
                iface: Framebuffer::new("/dev/fb0").unwrap(),
            }
        }
    }

