/*!
# Launchpad Mini MK3 low-level API

![Picture](https://imgur.com/sdLy3XK.png)
*/

mod input;
pub use input::*;

mod output;
pub use output::*;

pub use crate::protocols::Button80 as Button;

#[doc(hidden)]
pub struct Spec;

impl crate::DeviceSpec for Spec {
    const BOUNDING_BOX_WIDTH: u32 = 9;
    const BOUNDING_BOX_HEIGHT: u32 = 9;
    const COLOR_PRECISION: u16 = 64;

    type Input = Input;
    type Output = Output;

    fn is_valid(x: u32, y: u32) -> bool {
        if x > 8 || y > 8 {
            return false;
        }
        if x == 8 && y == 0 {
            return false;
        }
        true
    }

    fn flush(
        canvas: &mut crate::DeviceCanvas<Self>,
        changes: &[(u32, u32, (u8, u8, u8))],
    ) -> Result<(), crate::MidiError> {
        let changes = changes.iter().map(|&(x, y, (r, g, b))| {
            let color = RgbColor::new(r, g, b);

            let button = Button::from_abs(x as u8, y as u8);

            (button, color)
        });
        canvas.output.light_multiple_rgb(changes)
    }

    fn convert_message(msg: Message) -> Option<crate::CanvasMessage> {
        match msg {
            Message::Press { button } => Some(crate::CanvasMessage::Press {
                x: button.abs_x() as u32,
                y: button.abs_y() as u32,
            }),
            Message::Release { button } => Some(crate::CanvasMessage::Release {
                x: button.abs_x() as u32,
                y: button.abs_y() as u32,
            }),
            Message::TextEndedOrLooped
            | Message::DeviceInquiry(_)
            | Message::VersionInquiry(_)
            | Message::ProgrammerMode
            | Message::Pass => None,
        }
    }

    fn setup(output: &mut Self::Output) -> Result<(), crate::MidiError> {
        output.set_programmer_mode()
    }
}

pub type Canvas<'a> = crate::DeviceCanvas<Spec>;
