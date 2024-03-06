use vex_rt::prelude::*;

pub struct Intake {
    pub intake: Motor,
}

impl Intake {
    pub fn run(&mut self, intake: bool) {
        if intake {
            self.intake.move_i8(127).unwrap();
        } else {
            self.intake.move_i8(-127).unwrap();
        }
    }
    pub fn stop(&mut self) {
        self.intake.move_velocity(0).unwrap();
    }
}
