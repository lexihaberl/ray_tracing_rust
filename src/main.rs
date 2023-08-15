use ray_tracing_rust::math::Tuple4D;

#[derive(Debug)]
struct Projectile {
    position: Tuple4D,
    velocity: Tuple4D,
}

struct Environment {
    gravity: Tuple4D,
    wind: Tuple4D,
}

fn tick(env: &Environment, projectile: &Projectile) -> Projectile {
    let new_position = projectile.position + projectile.velocity;
    let new_velocity = projectile.velocity + env.gravity + env.wind;
    Projectile {
        position: new_position,
        velocity: new_velocity,
    }
}

fn main() {
    let mut p = Projectile {
        position: Tuple4D::new_point(0.0, 1.0, 0.0),
        velocity: Tuple4D::new_vector(1.0, 1.0, 0.0).normalize(),
    };

    let e = Environment {
        gravity: Tuple4D::new_vector(0.0, -0.1, 0.0),
        wind: Tuple4D::new_vector(-0.01, 0.0, 0.0),
    };

    println!("{p:?}");
    while p.position.y > 0.0 {
        p = tick(&e, &p);
        println!("{p:?}");
    }
}
