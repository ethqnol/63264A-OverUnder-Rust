use vex_rt::prelude::*;

pub struct Lift {
    pub lift: Motor,
}

impl Lift {
    pub fn run(&mut self, up: bool) {
        if up {
            self.lift.move_i8(127).unwrap();
        } else {
            self.lift.move_i8(-127).unwrap();
        }
    }

    pub fn stop(&mut self) {
        self.lift.move_velocity(0).unwrap();
    }
}
