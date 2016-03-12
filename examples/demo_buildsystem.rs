extern crate chrono_engine;

use chrono_engine::physics::{System, Body};

// This example is a direct port of the C++ example.

fn main() {
    {
        //
        // EXAMPLE 1:
        //

        println!("Example: Create a physical system.");

        // The physical system: it contains all physical objects.
        let my_system = System::new(1000, 1000.0, true);

        // Create a bunch of rigid bodies
        let my_body_a = Body::new_shared();
        let my_body_b = Body::new_shared();
        let my_body_c = Body::new_shared();

        // Create some markers..
        // Markers are 'auxiliary coordinate systems' to be added
        // to rigid bodies.

        // TODO
    }
    {
        //
        // EXAMPLE 2
        //

        println!("Example: Create a slider-crank system.");

        // The physical system: it contains all physical objects.
        let my_system = System::new(1000, 1000.0, true);

        // Create three rigid bodies and add them to the system:
        let my_body_a = Body::new_shared();
        let my_body_b = Body::new_shared();
        let my_body_c = Body::new_shared();

        // TODO
    }
}
