use log::info;
use std::thread;
use std::time::{Duration, Instant};

pub struct Animator {
	frame_delay: Duration,
	steps:       u16
}

impl Animator {
	pub fn new(duration_seconds: f32, fps: u16) -> Self {
		let steps = (fps as f32 * duration_seconds).round() as u16;
		let frame_delay = Duration::from_millis((duration_seconds * 1000.0 / steps as f32) as u64);
		info!(
		      "Animation: {}s@{}fps – {} steps with {}μs delay ",
		      duration_seconds,
		      fps,
		      steps,
		      frame_delay.as_micros()
		);
		Self { frame_delay, steps: steps as u16 }
	}

	pub fn animate<V>(&self, visualizer: V)
		where V: Fn(f32) {
		let mut max_duration_μs = self.show_frame_and_sleep(&visualizer, 0.0); // guarantee exact 0.0 as first frame
		for i in 1..self.steps {
			max_duration_μs =
				max_duration_μs.max(self.show_frame_and_sleep(&visualizer, i as f32 / self.steps as f32));
		}
		visualizer(1.0); // guarantee exact 1.0 as last frame
		info!("max frame rendering duration: {}μs", max_duration_μs);
	}

	fn show_frame_and_sleep<V>(&self, visualizer: &V, t: f32) -> u128
		where V: Fn(f32) {
		let start_time = Instant::now();
		visualizer(t);
		let render_duration = start_time.elapsed();
		thread::sleep(self.frame_delay - render_duration);
		render_duration.as_micros()
	}
}
