use std::thread;
use std::time::Duration;

pub struct Animator {
	duration: Duration,
	steps:    u16
}

impl Animator {
	pub fn new(duration_seconds: f32, fps: u16) -> Self {
		let steps = fps as f32 * duration_seconds;
		let duration = Duration::from_millis((duration_seconds * 1000.0 / steps) as u64);
		Self { duration, steps: steps as u16 }
	}

	pub fn animate<V>(&self, visualizer: V)
		where V: Fn(f32) {
		for i in 0..self.steps {
			visualizer(i as f32 / self.steps as f32);
			self.sleep();
		}
		visualizer(1.0);
	}

	fn sleep(&self) { thread::sleep(self.duration); }
}
