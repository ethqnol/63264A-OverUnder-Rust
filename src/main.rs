#![no_std]
#![no_main]

use core::time::Duration;

use vex_rt::{prelude::*, select};

mod drive;
mod intake;
mod lift;

struct Robot63264A {
    drive: Mutex<drive::Drive>,
    intake: Mutex<intake::Intake>,
    lift: Mutex<lift::Lift>,
    controller: Controller,
}

const RED_RPM : i16 = 100;
const GREEN_RPM : i16 = 200;
const BLUE_RPM : i16 = 600;


impl Robot for Robot63264A {
    fn new(peripherals: Peripherals) -> Self {
        Self {
            drive: Mutex::new(drive::Drive {
                left_front_drive: peripherals
                    .port10
                    .into_motor(Gearset::ThirtySixToOne, EncoderUnits::Degrees, false)
                    .unwrap(),
                left_back_drive: peripherals
                    .port09
                    .into_motor(Gearset::ThirtySixToOne, EncoderUnits::Degrees, false)
                    .unwrap(),
                right_front_drive: peripherals
                    .port20
                    .into_motor(Gearset::ThirtySixToOne, EncoderUnits::Degrees, true)
                    .unwrap(),
                right_back_drive: peripherals
                    .port19
                    .into_motor(Gearset::ThirtySixToOne, EncoderUnits::Degrees, true)
                    .unwrap(),

            }),

            intake: Mutex::new(intake::Intake {
                intake: peripherals
                    .port08
                    .into_motor(Gearset::EighteenToOne, EncoderUnits::Degrees, false)
                    .unwrap(),
            }),

            lift: Mutex::new(lift::Lift {
                lift: peripherals
                    .port03
                    .into_motor(Gearset::SixToOne, EncoderUnits::Degrees, false);
            }),

            controller: peripherals.master_controller,
        }
    }

    fn initialize(&mut self, _ctx: Context) {
        // Do any extra initialization here.
    }

    fn autonomous(&mut self, _ctx: Context) {
        println!("autonomous");
        // Write your autonomous routine here.
    }

    fn opcontrol(&mut self, ctx: Context) {
        println!("opcontrol");

        // loop with delay of 10ms
        let mut l = Loop::new(Duration::from_millis(10));
        loop {
            // Update the motors.
            self.drive.lock().run(
                self.controller.left_stick.get_y().unwrap(),
                self.controller.right_stick.get_y().unwrap(),
            );

            if self.controller.up.is_pressed().unwrap() {
                self.lift.lock().run(true);
            } else if self.controller.down.is_pressed().unwrap() {
                self.lift.lock().run(false);
            } else {
                self.lift.lock().lift.
            }

            select! {
                // If the driver control period is done, break out of the loop.
                _ = ctx.done() => break,

                // Otherwise, when it's time for the next loop cycle, continue.
                _ = l.select() => continue,
            }
        }
    }

    fn disabled(&mut self, _ctx: Context) {
        println!("disabled");
        // This runs when the robot is in disabled mode.
    }
}

entry!(Robot63264A);
