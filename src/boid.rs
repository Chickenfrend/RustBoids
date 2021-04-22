use cgmath::Vector2;

struct Boid {
	position: Vector2<f64>,
	velocity: Vector2<f64>, 
	acceleration: vector2<f64>,
	max_speed: f64,
	max_force: f64,
	flock: &Boids
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

	fn seperation_rule(self, boids: &Boids){
		goal_seperation = 20.0;
		Vector2<f64> steer = (0,0);
		boids_near = 0;

		for boid in flock{
			dist = cgmath.distance(self.position, boid.position);
			if 0 < dist < goal_seperation{
				Vector2<f64> diff = (0,0);		

				diff = diff.normalize();
				diff = diff.div_assign(dist);

				steer = steer.add_v(diff);
				boids_near += 1;
			}

			if boids_near > 0{
				steer.div_s(boids_near as f64)
			}
			if steer.magnitude() > 0{
				steer = steer.normalize();
				steer = steer.mul_s(self.max_speed);
				steer = steer.sub_v(self.velocity);
			}
		}
	}

	fn allignment(self){
		fov = 50.0;
		Vector2<f64> vec_sum = (0, 0);
		count = 0;

		for boid in self.flock {
			distance = self.position.distance(boid.position);
			if distance > 0 && distance < fov{
				vec_sum.add_v(boid.velocity);
				count += 1
			}

			if count > 0{
				vec_sum = vec_sum.div_s(count as f64);
				vec_sum = vec_sum.normalize();
				vec_sum = vec_sum.mul_s(self.max_speed);

				Vector2<f64> steer;

				steer = vec_sum.sub_v(self.velocity);

				if(steer.magnitude() > max_force){
					steer = steer.div_s(steer.magnitude())
				}
				steer
			} else{
				Vector2<f64> empty = (0, 0);	
				empty
			}
		}
	}

	fn seek(velocity :Vector2<f64>){
		Vector2<f64> desired = (0, 0);
		desired = desired.sub_v(velocity);
		desired.normalize();
		desired = desired.mul_s(max_speed);

		self.acceleration = desired.sub_v(velocity);
		if(self.acceleration > max_force){
			acceleration = acceleration.div_s(self.max_force)
		}
		acceleration
	}

	fn cohesion(self){
		fov = 50.0;	
		Vector2<f64> vec_sum = (0, 0);

		count = 0;

		for boid in self.flock{
			distance = self.position.distance(boid.position);

			if distance > 0 && distance < fov{
				vec_sum = vec_sum.add_v(boid.position);
				count += 1
			}
		}

		if count > 0{
			sum = sum.div_s(count as float);
			acceleration()
			
		} else{
			(0, 0)
		}
	}

	fn check_borders(){
		if position.x < 0{
			position.x == 0;
		}
		if position.y < 0{
			position.y == 0;
		}
		if position.x > 1000{
			position.x == 1000;
		}
		if position.y > 1000{
			position.x == 1000;
		}
	}
}

