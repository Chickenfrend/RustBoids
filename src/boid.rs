use cgmath::Vector2;

struct Boid {
	position: Vector2<f64>,
	velocity: Vector2<f64>, 
	acceleration: vector2<f64>,
	max_speed: f64,
	max_force: f64
}

struct Boids {
	boids: Vec
}

impl Boid {
	fn update(self) {
		self.acceleration.mul_s(0.4);
		if velocity.length() < max_speed {
			self.velocity.add_v(acceleration);
		}

		location.add_v(velocity);
		acceleration.mul_s(0);
	}
	fn seperationRule(self, boids: &Boids){
		goalSeperation = 20;
	}
}

