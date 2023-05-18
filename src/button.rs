use arduino_hal::{
    hal::port::PC1,
    port::{
        mode::{Floating, Input},
        Pin,
    },
};

#[derive(PartialEq, Clone, Copy)]
enum ButtonState {
    Pressed,
    Released,
}

pub struct Button {
    pin: Pin<Input<Floating>, PC1>,
    state: ButtonState,
}

impl Button {
    pub fn new(pin: Pin<Input<Floating>, PC1>) -> Self {
        Self {
            pin,
            state: ButtonState::Released,
        }
    }

    pub fn is_pressed(&self) -> bool {
        return self.pin.is_high();
    }

    pub fn update(&mut self) {
        self.state = if self.is_pressed() {
            ButtonState::Pressed
        } else {
            ButtonState::Released
        };
    }

    pub fn toggle_detected(&mut self) -> bool {
        let previous_state = self.state;
        self.update();
        let current_state = self.state;
        return previous_state == ButtonState::Released && current_state == ButtonState::Pressed;
    }
}
