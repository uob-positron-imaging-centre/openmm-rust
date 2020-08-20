/**
 * File   : hello_argon.rs
 * License: MIT
 * Author : Andrei Leonard Nicusan <a.l.nicusan@bham.ac.uk>
 * Date   : 10.08.2020
 */

// Example using OpenMM's C Wrapper from Rust to simulate three argon atoms in vacuum. This is
// translated almost 1:1 from https://github.com/openmm/openmm/blob/master/examples/HelloArgonInC.c

/* -----------------------------------------------------------------------------
 *             OpenMM(tm) HelloArgon example in C (June 2009)
 * -----------------------------------------------------------------------------
 * This program demonstrates a simple molecular simulation using the OpenMM
 * API for GPU-accelerated molecular dynamics simulation. The primary goal is
 * to make sure you can compile, link, and run with OpenMM and view the output.
 * The example is available in C++, C, and Fortran 95.
 *
 * The system modeled here is a small number of argon atoms in a vacuum.
 * A multi-frame PDB file is written to stdout which  can be read by VMD or 
 * other visualization tool to produce an animation of the resulting trajectory.
 * -------------------------------------------------------------------------- */

use openmm_sys::*;

unsafe fn simulate_argon() {
    // Create a system with nonbonded forces. System takes ownership of Force; don't destroy it
    // yourself.
    let system = OpenMM_System_create();
    let nonbond = OpenMM_NonbondedForce_create();

    OpenMM_System_addForce(system, nonbond as *mut OpenMM_Force);

    // Create three atoms.
    let init_pos = OpenMM_Vec3Array_create(3);

    for a in 0..3 {
        // Location, in nm
        let pos = OpenMM_Vec3 {
            x: 0.95 * a as f64,
            y: 0.95 * a as f64,
            z: 0.95 * a as f64,
        };

        OpenMM_Vec3Array_set(init_pos, a, pos);

        OpenMM_System_addParticle(system, 39.95);                           /*mass of Ar, grams/mole*/

        // charge, L-J sigma (nm), well depth (kJ)
        OpenMM_NonbondedForce_addParticle(nonbond, 0.0, 0.3350, 0.996);     /*vdWRad(Ar)=.188 nm*/
    }

    // Create particular integrator, and recast to generic one.
    let integrator = OpenMM_VerletIntegrator_create(0.004) as *mut OpenMM_Integrator;

    // Let OpenMM Context choose best platform.
    let context = OpenMM_Context_create(system, integrator);
    let platform = OpenMM_Context_getPlatform(context);

    // Print the chosen platform. Cast NULL-terminated C-string to a Rust CStr.
    let platform_name = std::ffi::CStr::from_ptr(OpenMM_Platform_getName(platform));
    println!("REMARK  Using OpenMM platform {:?}\n", platform_name);

    // Set starting positions of the atoms. Leave time and velocity zero.
    OpenMM_Context_setPositions(context, init_pos);

    // Simulate
    for frame_num in 1.. {
        // Output current state information.
        let state = OpenMM_Context_getState(
            context,
            OpenMM_State_DataType_OpenMM_State_Positions as i32,
            0
        );
        let time = OpenMM_State_getTime(state);
        write_pdb_frame(frame_num, state);

        OpenMM_State_destroy(state);
        if time >= 1. {
            break
        }

        // Advance state many steps at a time, for efficient use of OpenMM.
        OpenMM_Integrator_step(integrator, 10); // Use a lot more than this normally
    }

    // Free heap space for all the objects created above.
    OpenMM_Vec3Array_destroy(init_pos);
    OpenMM_Context_destroy(context);
    OpenMM_Integrator_destroy(integrator);
    OpenMM_System_destroy(system);

}

fn main() {
    unsafe {
        simulate_argon();
    }
}

// Handy homebrew PDB writer for quick-and-dirty trajectory output.
unsafe fn write_pdb_frame(frame_num: i32, state: *mut OpenMM_State) {
    // Reference atomic positions in the OpenMM State.
    let pos_state = OpenMM_State_getPositions(state);

    // Use PDB MODEL cards to number trajectory frames.
    println!("MODEL     {}", frame_num);

    for a in 0..OpenMM_Vec3Array_getSize(pos_state) {
        let pos = OpenMM_Vec3_scale(*OpenMM_Vec3Array_get(pos_state, a), 1.);

        println!("ATOM  {:>5}  AR   AR     1    ", a + 1);              /*atom number*/
        println!("{}  {}  {}  1.00  0.00\n", pos.x, pos.y, pos.z);      /*coordinates*/
    }

    println!("ENDMDL"); /*end of frame*/
}

