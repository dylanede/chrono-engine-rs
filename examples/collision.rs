extern crate chrono_engine;
extern crate nalgebra;

use nalgebra::Vector3;

use chrono_engine::{
    self as ch,
    make_shared,
    Body
};

use std::iter;

fn main() {
    // Two spheres - one fixed in place.
    // Constant force on the free sphere towards the other sphere.
    // With restitution, bouncing should be observed.
    let system = ch::SystemNSC::new(1000, 1000.0, true);
    system.set_g_acc(&Vector3::new(-2.0, 0.0, 0.0)); // constant force

    let material = make_shared::<ch::MaterialSurfaceNSC>(());
    material.set_restitution(0.5);

    let dynamic_body = make_shared::<Body>(());
    dynamic_body.set_collide(true);
    dynamic_body.set_material_surface(material.clone());
    {
        let model = dynamic_body.get_collision_model().unwrap();
        model.clear_model();
        model.add_sphere(1.0, &Vector3::new(0.0, 0.0, 0.0));
        model.build_model();
    }
    dynamic_body.set_pos(&Vector3::new(4.0, 0.0, 0.0));
    system.add_body(dynamic_body.clone());

    let fixed_body = make_shared::<Body>(());
    fixed_body.set_collide(true);
    fixed_body.set_material_surface(material.clone());
    fixed_body.set_body_fixed(true);
    {
        let model = fixed_body.get_collision_model().unwrap();
        model.clear_model();
        model.add_sphere(1.0, &Vector3::new(0.0, 0.0, 0.0));
        model.build_model();
    }
    fixed_body.set_pos(&Vector3::new(-1.0, 0.0, 0.0));
    system.add_body(fixed_body.clone());

    let mut chrono_time = 0.0;
    while chrono_time < 5.0 {
        chrono_time += 0.05;
        system.do_frame_dynamics(chrono_time);
        let pos = dynamic_body.pos()[0]; // between 1.0 and 4.0
        let pos = (pos - 1.0) * 79.0 / 3.0; // between 0.0 and 79.0
        let string = iter::repeat(' ')
            .take(pos.round() as usize)
            .chain(iter::once('*'))
            .collect::<String>();
        println!("{}", string);
    }
}