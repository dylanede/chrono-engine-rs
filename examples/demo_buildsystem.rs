
extern crate chrono_engine;
extern crate nalgebra;

use nalgebra::Vector3;

use chrono_engine::{
    self as ch,
    make_shared,
    Body, Marker, Force,
    CoordSys,
    InheritsExt,
    Object
};
use std::f64::consts::PI;
// This example is a direct port of the C++ example.

fn main() {
    let logger = ch::stdout_logger();
    {
        println!("Example: Create a physical system.");

        let system = ch::SystemNSC::new(1000, 1000.0, true);

        let body_a = make_shared::<Body>(());
        let body_b = make_shared::<Body>(());
        let body_c = make_shared::<Body>(());

        let marker_a1 = make_shared::<Marker>(());
        let marker_a2 = make_shared::<Marker>(());
        let marker_b1 = make_shared::<Marker>(());
        let marker_b2 = make_shared::<Marker>(());

        let force_a1 = make_shared::<Force>(());
        let force_a2 = make_shared::<Force>(());

        body_a.add_marker(marker_a1.clone());
        body_a.add_marker(marker_a2.clone());
        body_a.add_force(force_a1.clone());
        body_a.add_force(force_a2.clone());
        body_b.add_marker(marker_b1);
        body_b.add_marker(marker_b2);

        system.add_body(body_a.clone());
        system.add_body(body_b.clone());
        system.add_body(body_c);

        println!("Here's the system hierarchy:");

        system.show_hierarchy(&logger);

        body_a.remove_all_forces();
        body_a.remove_all_markers();

        system.remove_body(body_a);

        body_b.add_marker(marker_a1.clone());
        body_b.add_marker(marker_a2);
        body_b.add_force(force_a1);
        body_b.add_force(force_a2);

        marker_a1.as_::<Object>().set_name("Foo");

        println!("Here's the system hierarchy after modifications:");

        system.show_hierarchy(&logger);
    }
    {
        //
        // EXAMPLE 2
        //

        println!("Example: Create a slider-crank system.");

        // The physical system: it contains all physical objects.
        let system = ch::SystemNSC::new(1000, 1000.0, true);

        // Create three rigid bodies and add them to the system:
        let body_a = make_shared::<Body>(());
        let body_b = make_shared::<Body>(());
        let body_c = make_shared::<Body>(());

        body_a.set_name("truss");
        body_b.set_name("crank");
        body_c.set_name("rod");

        system.add_body(body_a.clone());
        system.add_body(body_b.clone());
        system.add_body(body_c.clone());

        // Set initial position of the bodies (centre of mass)
        body_a.set_body_fixed(true); // truss does not move
        body_b.set_pos(&Vector3::new(1.0, 0.0, 0.0));
        body_c.set_pos(&Vector3::new(4.0, 0.0, 0.0));

        let marker_b = make_shared::<Marker>(());
        let marker_c = make_shared::<Marker>(());

        marker_b.as_::<Object>().set_name("crank_rev");
        marker_c.as_::<Object>().set_name("rod_rev");

        body_b.add_marker(marker_b.clone());
        body_c.add_marker(marker_c.clone());

        // Set absolute position of the two markers.
        // for the initial position of the "rod-crank" link:
        marker_b.impose_abs_coord(&CoordSys::from_pos(Vector3::new(2.0, 0.0, 0.0)));
        marker_c.impose_abs_coord(&CoordSys::from_pos(Vector3::new(2.0, 0.0, 0.0)));

        // Now create a mechanical link (a revolute joint)
        // between these two markers, and insert in system:
        let link_bc = make_shared::<ch::LinkLockRevolute>(());
        link_bc.initialize_from_markers(marker_b, marker_c);
        link_bc.as_::<Object>().set_name("REVOLUTE crank-rod");
        system.add_link(link_bc);

        // Note that there's an easier way to create a link,
        // without needing the two markers (they will be
        // automatically created and added to the two bodies)
        // i.e. is using two bodies and a position as arguments..
        // For example, to create the rod-truss constraint:
        let link_ca = make_shared::<ch::LinkLockPointLine>(());
        link_ca.initialize_from_bodies(body_c, body_a.clone(), &CoordSys::from_pos(Vector3::new(6.0, 0.0, 0.0)));
        system.add_link(link_ca.clone());

        link_ca.marker_1().unwrap().as_::<Object>().set_name("rod_pointline");
        link_ca.marker_2().unwrap().as_::<Object>().set_name("truss_pointline");
        link_ca.as_::<Object>().set_name("POINTLINE rod-truss");

        // Now create a "motor" link between crank and truss,
        // in "imposed speed" mode:
        let link_ab = make_shared::<ch::LinkEngine>(());
        link_ab.initialize_from_bodies(body_a, body_b, &CoordSys::from_pos(Vector3::new(0.0, 0.0, 0.0)));
        link_ab.set_eng_mode(ch::EngineMode::Speed);
        link_ab.set_speed_fn(make_shared::<ch::function::Const>(PI));
        system.add_link(link_ab.clone());
        link_ab.marker_1().unwrap().as_::<Object>().set_name("truss_engine");
        link_ab.marker_2().unwrap().as_::<Object>().set_name("crank_engine");
        link_ab.as_::<Object>().set_name("ENGINE truss-crank");

        println!("Here's the system hierarchy for the slider-crank:");

        system.show_hierarchy(&logger);

//        for link in system.links_iter() {
//            println!("    Link class: {}, leaves n.DOFs: {}", CppTypeId::of_value(link).name(), link.get_left_dof());
//        }

        // A very simple simulation loop:
        let mut chrono_time = 0.0;
        while chrono_time < 2.5 {
            chrono_time += 0.01;

            // Perform simulation up to chrono_time
            system.do_frame_dynamics(chrono_time);

            // Print something on the console
            println!("Time: {}  Slider X position: {}  Engine torque: {}", chrono_time, link_ca.marker_1().unwrap().abs_coord().position[0], link_ab.mot_retorque());
        }
    }
}