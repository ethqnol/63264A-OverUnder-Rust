use vex_rt::prelude::*;

pub struct Drive {
    pub left_front_drive: Motor,
    pub left_back_drive: Motor,
    pub right_front_drive: Motor,
    pub right_back_drive: Motor,
}

impl Drive {
    pub fn run(&mut self, left: i8, right: i8) {
        self.left_front_drive.move_i8(left).unwrap();
        self.left_back_drive.move_i8(left).unwrap();
        self.right_front_drive.move_i8(right).unwrap();
        self.right_back_drive.move_i8(right).unwrap();
    }
}
