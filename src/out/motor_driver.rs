use rppal::pwm::{Channel, Polarity, Pwm};

const MIN_PULSE_WIDTH: f64 = 1f64;
const MAX_PULSE_WIDTH: f64 = 2f64;

pub struct MotorDriver {
    pwm: Pwm
}

impl MotorDriver {
    pub fn new(channel: Channel) -> Self {
        let pwm = Pwm::with_frequency(channel, 50f64, 0f64, Polarity::Normal, true)
            .expect("PWM Init");

        Self { pwm }
    }

    pub fn set_angle(&mut self, angle: f64) {
        let angle = angle.clamp(0f64, 180f64);
        
        let pulse_width = MIN_PULSE_WIDTH + (MAX_PULSE_WIDTH - MIN_PULSE_WIDTH) * (angle / 180f64);
        let duty_cycle = pulse_width / (1000f64 / 50f64);

        self.pwm.set_duty_cycle(duty_cycle).expect("Duty cycle was bad");
    }

    pub fn set_duty(&mut self, duty: f64) {
        self.pwm.set_duty_cycle(duty).expect("Duty cycle was bad");
    }
}
