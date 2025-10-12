// In src/math_utils.rs
use crate::body::{Acc, Agglomerate, Body, EMPTY_ACC};
use crate::Real;
use mpi::traits::*;
use std::collections::HashMap;

const VELOCITY_THRESHOLD: Real = 50.0; // m/s - threshold for sticking vs bouncing
const COEFFICIENT_OF_RESTITUTION: Real = 0.8; // Elasticity for bouncing collisions

pub fn detect_and_handle_collisions(
    bodies: &mut Vec<Body>,
    agglomerates: &mut HashMap<i32, Agglomerate>,
    next_agg_id: &mut i32,
    lower: usize,
    upper: usize,
) {
    // Loop through all pairs of bodies to check for collisions
    for i in lower..upper {
        for j in (i + 1)..bodies.len() {
            // Skip if both bodies are already part of the same agglomerate
            if bodies[i].agglomerate_id != -1
                && bodies[i].agglomerate_id == bodies[j].agglomerate_id
            {
                continue;
            }

            // Calculate distance between bodies
            let dx = bodies[j].x - bodies[i].x;
            let dy = bodies[j].y - bodies[i].y;
            let dz = bodies[j].z - bodies[i].z;
            let distance_squared = dx * dx + dy * dy + dz * dz;
            let combined_radius = bodies[i].radius + bodies[j].radius;

            // Check if collision occurred
            if distance_squared <= combined_radius * combined_radius * 1.1 {
                // Calculate relative velocity
                let dvx = bodies[j].vx - bodies[i].vx;
                let dvy = bodies[j].vy - bodies[i].vy;
                let dvz = bodies[j].vz - bodies[i].vz;

                // Project relative velocity onto collision normal
                let distance = distance_squared.sqrt();
                let nx = dx / distance;
                let ny = dy / distance;
                let nz = dz / distance;

                let rel_vel_mag = dvx * nx + dvy * ny + dvz * nz;

                // Skip if bodies are moving away from each other
                if rel_vel_mag > 0.0 {
                    continue;
                }

                let rel_vel_magnitude = (dvx * dvx + dvy * dvy + dvz * dvz).sqrt();

                if rel_vel_magnitude > VELOCITY_THRESHOLD {
                    // Elastic collision (bouncing)
                    handle_elastic_collision(bodies, i, j);
                    println!("Bouncing collision between particles {} and {}", i, j);
                } else {
                    // Inelastic collision (sticking)
                    println!("Sticker collision between particles {} and {}", i, j);
                    handle_inelastic_collision(bodies, agglomerates, next_agg_id, i, j);
                }
            }
        }
    }
}

fn handle_elastic_collision(bodies: &mut Vec<Body>, i: usize, j: usize) {
    let mi = bodies[i].m;
    let mj = bodies[j].m;
    let total_mass = mi + mj;

    // Calculate distance vector
    let dx = bodies[j].x - bodies[i].x;
    let dy = bodies[j].y - bodies[i].y;
    let dz = bodies[j].z - bodies[i].z;
    let distance = (dx * dx + dy * dy + dz * dz).sqrt();

    // Normal vector
    let nx = dx / distance;
    let ny = dy / distance;
    let nz = dz / distance;

    // Relative velocity
    let dvx = bodies[j].vx - bodies[i].vx;
    let dvy = bodies[j].vy - bodies[i].vy;
    let dvz = bodies[j].vz - bodies[i].vz;

    // Velocity along normal
    let v_normal = dvx * nx + dvy * ny + dvz * nz;

    // Impulse scalar
    let impulse = (-(1.0 + COEFFICIENT_OF_RESTITUTION) * v_normal) / (1.0 / mi + 1.0 / mj);

    // Apply impulse
    bodies[i].vx -= impulse * nx / mi;
    bodies[i].vy -= impulse * ny / mi;
    bodies[i].vz -= impulse * nz / mi;

    bodies[j].vx += impulse * nx / mj;
    bodies[j].vy += impulse * ny / mj;
    bodies[j].vz += impulse * nz / mj;

    // Separate overlapping bodies
    let overlap = bodies[i].radius + bodies[j].radius - distance;
    if overlap > 0.0 {
        let correction_i = overlap * mj / total_mass;
        let correction_j = overlap * mi / total_mass;

        bodies[i].x -= nx * correction_i;
        bodies[i].y -= ny * correction_i;
        bodies[i].z -= nz * correction_i;

        bodies[j].x += nx * correction_j;
        bodies[j].y += ny * correction_j;
        bodies[j].z += nz * correction_j;
    }
}

fn handle_inelastic_collision(
    bodies: &mut Vec<Body>,
    agglomerates: &mut HashMap<i32, Agglomerate>,
    next_agg_id: &mut i32,
    i: usize,
    j: usize,
) {
    let i_agg_id = bodies[i].agglomerate_id;
    let j_agg_id = bodies[j].agglomerate_id;

    if i_agg_id == -1 && j_agg_id == -1 {
        // Both are free particles - create new agglomerate
        let new_id = *next_agg_id;
        *next_agg_id += 1;

        let mut new_agg = Agglomerate::new(new_id, i, bodies);
        new_agg.member_indices.push(j);

        bodies[i].agglomerate_id = new_id;
        bodies[j].agglomerate_id = new_id;

        // Update velocities and properties for the new agglomerate
        new_agg.update_properties(bodies);

        // Apply forces to create rotation
        apply_collision_rotation(bodies, &new_agg, i, j);

        agglomerates.insert(new_id, new_agg);
    } else if i_agg_id != -1 && j_agg_id == -1 {
        // i is in agglomerate, j is free
        if let Some(agg) = agglomerates.get_mut(&i_agg_id) {
            agg.member_indices.push(j);
            bodies[j].agglomerate_id = i_agg_id;

            // Update agglomerate properties
            agg.update_properties(bodies);

            // Apply collision effects
            apply_collision_to_agglomerate(bodies, agg, j);
        } else {
            // Create a new agglomerate if the expected one doesn't exist
            let new_id = *next_agg_id;
            *next_agg_id += 1;

            let mut new_agg = Agglomerate::new(new_id, i, bodies);
            new_agg.member_indices.push(j);

            bodies[i].agglomerate_id = new_id;
            bodies[j].agglomerate_id = new_id;

            new_agg.update_properties(bodies);
            agglomerates.insert(new_id, new_agg);
        }
    } else if i_agg_id == -1 && j_agg_id != -1 {
        // Similar safe handling for this case
        if let Some(agg) = agglomerates.get_mut(&j_agg_id) {
            // Existing code...
        } else {
            // Create new agglomerate...
        }
    } else if i_agg_id != j_agg_id {
        // Safe handling for merging agglomerates...
    }
}

pub fn synchronize_agglomerates<C: mpi::traits::Communicator>(
    bodies: &Vec<Body>,
    agglomerates: &mut HashMap<i32, Agglomerate>,
    next_agg_id: &mut i32,
    world: &C,
) {
    // Collect all agglomerate IDs in the system
    let mut all_agg_ids: Vec<i32> = bodies
        .iter()
        .filter(|body| body.agglomerate_id != -1)
        .map(|body| body.agglomerate_id)
        .collect();

    // Remove duplicates
    all_agg_ids.sort();
    all_agg_ids.dedup();

    // Ensure each process has all agglomerates
    for &agg_id in &all_agg_ids {
        if !agglomerates.contains_key(&agg_id) {
            // Find particles belonging to this agglomerate
            let members: Vec<usize> = bodies
                .iter()
                .enumerate()
                .filter(|(_, body)| body.agglomerate_id == agg_id)
                .map(|(idx, _)| idx)
                .collect();

            if !members.is_empty() {
                let mut new_agg = Agglomerate::new(agg_id, members[0], bodies);
                // Add remaining members
                for &idx in &members[1..] {
                    new_agg.member_indices.push(idx);
                }
                new_agg.update_properties(bodies);
                agglomerates.insert(agg_id, new_agg);
            }
        }
    }

    // Update the next_agg_id to be larger than any existing ID
    if let Some(&max_id) = all_agg_ids.iter().max() {
        *next_agg_id = max_id + 1;
    }
}

fn apply_collision_rotation(bodies: &mut Vec<Body>, agg: &Agglomerate, i: usize, j: usize) {
    let com = agg.com;

    // Position vectors relative to COM
    let r_i_x = bodies[i].x - com.0;
    let r_i_y = bodies[i].y - com.1;
    let r_i_z = bodies[i].z - com.2;

    let r_j_x = bodies[j].x - com.0;
    let r_j_y = bodies[j].y - com.1;
    let r_j_z = bodies[j].z - com.2;

    // Relative velocity
    let dv_x = bodies[j].vx - bodies[i].vx;
    let dv_y = bodies[j].vy - bodies[i].vy;
    let dv_z = bodies[j].vz - bodies[i].vz;

    // Calculate impulse at collision point (simplified)
    let impact_factor = 0.5; // Adjust as needed

    // Calculate angular momentum contribution (cross product of r_j and impulse)
    let ang_mom_x = impact_factor * (r_j_y * dv_z - r_j_z * dv_y) * bodies[j].m;
    let ang_mom_y = impact_factor * (r_j_z * dv_x - r_j_x * dv_z) * bodies[j].m;
    let ang_mom_z = impact_factor * (r_j_x * dv_y - r_j_y * dv_x) * bodies[j].m;

    // Calculate moment of inertia (simplified as spherical distribution)
    let inertia = bodies[i].m * (r_i_x * r_i_x + r_i_y * r_i_y + r_i_z * r_i_z)
        + bodies[j].m * (r_j_x * r_j_x + r_j_y * r_j_y + r_j_z * r_j_z);

    // Set angular velocity (if inertia is non-zero)
    let mut ang_vel = (0.0, 0.0, 0.0);
    if inertia > 1e-10 {
        ang_vel = (
            ang_mom_x / inertia,
            ang_mom_y / inertia,
            ang_mom_z / inertia,
        );
    }

    // Update velocities for both particles to match agglomerate velocity
    bodies[i].vx = agg.vel.0;
    bodies[i].vy = agg.vel.1;
    bodies[i].vz = agg.vel.2;

    bodies[j].vx = agg.vel.0;
    bodies[j].vy = agg.vel.1;
    bodies[j].vz = agg.vel.2;

    // Add rotational component to velocities
    bodies[i].vx += ang_vel.1 * r_i_z - ang_vel.2 * r_i_y;
    bodies[i].vy += ang_vel.2 * r_i_x - ang_vel.0 * r_i_z;
    bodies[i].vz += ang_vel.0 * r_i_y - ang_vel.1 * r_i_x;

    bodies[j].vx += ang_vel.1 * r_j_z - ang_vel.2 * r_j_y;
    bodies[j].vy += ang_vel.2 * r_j_x - ang_vel.0 * r_j_z;
    bodies[j].vz += ang_vel.0 * r_j_y - ang_vel.1 * r_j_x;
}

fn apply_collision_to_agglomerate(bodies: &mut Vec<Body>, agg: &Agglomerate, free_idx: usize) {
    // Update free particle to match agglomerate velocity
    bodies[free_idx].vx = agg.vel.0;
    bodies[free_idx].vy = agg.vel.1;
    bodies[free_idx].vz = agg.vel.2;

    // In a complete implementation, you would:
    // 1. Calculate impact point
    // 2. Apply impulse to agglomerate
    // 3. Calculate resulting angular momentum change
    // 4. Update angular velocity of agglomerate
}

fn apply_agglomerate_collision(
    bodies: &mut Vec<Body>,
    merged_agg: &mut Agglomerate,
    agg_i: &Agglomerate,
    agg_j: &Agglomerate,
) {
    // Find approximate collision point by determining closest particles between agglomerates
    let mut min_dist = f32::MAX;
    let mut collision_i_idx = 0;
    let mut collision_j_idx = 0;

    // Find closest pair of particles between the two agglomerates
    for &i_idx in &agg_i.member_indices {
        for &j_idx in &agg_j.member_indices {
            let dx = bodies[j_idx].x - bodies[i_idx].x;
            let dy = bodies[j_idx].y - bodies[i_idx].y;
            let dz = bodies[j_idx].z - bodies[i_idx].z;
            let dist_squared = dx * dx + dy * dy + dz * dz;

            if dist_squared < min_dist {
                min_dist = dist_squared;
                collision_i_idx = i_idx;
                collision_j_idx = j_idx;
            }
        }
    }

    // Calculate relative velocity at collision point
    let rel_vx = bodies[collision_j_idx].vx - bodies[collision_i_idx].vx;
    let rel_vy = bodies[collision_j_idx].vy - bodies[collision_i_idx].vy;
    let rel_vz = bodies[collision_j_idx].vz - bodies[collision_i_idx].vz;

    // Calculate collision normal
    let dx = bodies[collision_j_idx].x - bodies[collision_i_idx].x;
    let dy = bodies[collision_j_idx].y - bodies[collision_i_idx].y;
    let dz = bodies[collision_j_idx].z - bodies[collision_i_idx].z;
    let distance = (dx * dx + dy * dy + dz * dz).sqrt();

    if distance < 1e-10 {
        // If particles are too close, just use merged velocity without rotation
        return;
    }

    let nx = dx / distance;
    let ny = dy / distance;
    let nz = dz / distance;

    // Calculate impulse magnitude (simplified model)
    let impulse_mag = agg_i.total_mass * agg_j.total_mass / (agg_i.total_mass + agg_j.total_mass)
        * (rel_vx * nx + rel_vy * ny + rel_vz * nz).abs();

    // Calculate angular momentum contribution from collision
    let com = merged_agg.com;

    // Vector from COM to collision point (using i's particle as reference)
    let r_x = bodies[collision_i_idx].x - com.0;
    let r_y = bodies[collision_i_idx].y - com.1;
    let r_z = bodies[collision_i_idx].z - com.2;

    // Angular momentum = r × impulse
    let ang_mom_x = impulse_mag * (r_y * nz - r_z * ny);
    let ang_mom_y = impulse_mag * (r_z * nx - r_x * nz);
    let ang_mom_z = impulse_mag * (r_x * ny - r_y * nx);

    // Calculate moment of inertia tensor (simplified as spherical)
    let mut inertia = 0.0;

    for &idx in &merged_agg.member_indices {
        let rx = bodies[idx].x - com.0;
        let ry = bodies[idx].y - com.1;
        let rz = bodies[idx].z - com.2;
        let r_squared = rx * rx + ry * ry + rz * rz;
        inertia += bodies[idx].m * r_squared;
    }

    // Avoid division by zero
    if inertia < 1e-10 {
        // Just update linear velocities
        for &idx in &merged_agg.member_indices {
            bodies[idx].vx = merged_agg.vel.0;
            bodies[idx].vy = merged_agg.vel.1;
            bodies[idx].vz = merged_agg.vel.2;
        }
        return;
    }

    // Calculate angular velocity from angular momentum and inertia
    let ang_vel_x = ang_mom_x / inertia;
    let ang_vel_y = ang_mom_y / inertia;
    let ang_vel_z = ang_mom_z / inertia;

    // Store angular velocity in merged agglomerate
    merged_agg.ang_vel = (
        ang_vel_x
            + agg_i.ang_vel.0 * (agg_i.total_mass / merged_agg.total_mass)
            + agg_j.ang_vel.0 * (agg_j.total_mass / merged_agg.total_mass),
        ang_vel_y
            + agg_i.ang_vel.1 * (agg_i.total_mass / merged_agg.total_mass)
            + agg_j.ang_vel.1 * (agg_j.total_mass / merged_agg.total_mass),
        ang_vel_z
            + agg_i.ang_vel.2 * (agg_i.total_mass / merged_agg.total_mass)
            + agg_j.ang_vel.2 * (agg_j.total_mass / merged_agg.total_mass),
    );

    // Update velocities of all particles in the merged agglomerate
    for &idx in &merged_agg.member_indices {
        // Position relative to COM
        let rel_x = bodies[idx].x - com.0;
        let rel_y = bodies[idx].y - com.1;
        let rel_z = bodies[idx].z - com.2;

        // Linear velocity + rotational component (v = v_com + ω × r)
        bodies[idx].vx =
            merged_agg.vel.0 + (merged_agg.ang_vel.1 * rel_z - merged_agg.ang_vel.2 * rel_y);
        bodies[idx].vy =
            merged_agg.vel.1 + (merged_agg.ang_vel.2 * rel_x - merged_agg.ang_vel.0 * rel_z);
        bodies[idx].vz =
            merged_agg.vel.2 + (merged_agg.ang_vel.0 * rel_y - merged_agg.ang_vel.1 * rel_x);
    }
}

pub fn update_agglomerates(bodies: &mut Vec<Body>, agglomerates: &mut HashMap<i32, Agglomerate>) {
    // Update positions of particles in agglomerates based on rotation and translation
    for (_, agg) in agglomerates.iter_mut() {
        agg.update_properties(bodies);

        let (com_x, com_y, com_z) = agg.com;
        let (vel_x, vel_y, vel_z) = agg.vel;
        let (ang_x, ang_y, ang_z) = agg.ang_vel;

        for &idx in &agg.member_indices {
            // Calculate particle position relative to COM
            let rel_x = bodies[idx].x - com_x;
            let rel_y = bodies[idx].y - com_y;
            let rel_z = bodies[idx].z - com_z;

            // Reduce the rotation effect to prevent jumping
            let rot_factor = 0.001; // Smaller value for more stable rotation
            let rot_x = rel_x + (ang_y * rel_z - ang_z * rel_y) * rot_factor;
            let rot_y = rel_y + (ang_z * rel_x - ang_x * rel_z) * rot_factor;
            let rot_z = rel_z + (ang_x * rel_y - ang_y * rel_x) * rot_factor;

            // Update position
            bodies[idx].x = com_x + rot_x;
            bodies[idx].y = com_y + rot_y;
            bodies[idx].z = com_z + rot_z;

            // Update velocity (includes both translation and rotation component)
            bodies[idx].vx = vel_x + (ang_y * rel_z - ang_z * rel_y);
            bodies[idx].vy = vel_y + (ang_z * rel_x - ang_x * rel_z);
            bodies[idx].vz = vel_z + (ang_x * rel_y - ang_y * rel_x);
        }
    }
}

pub fn check_escaped_particles(bodies: &mut Vec<Body>, lower: usize, upper: usize) {
    // Calculate total mass of the system
    let total_mass: Real = bodies.iter().map(|b| b.m).sum();

    // Find approximate radius of the system
    let mut max_r_squared: Real = 0.0;
    for body in bodies.iter() {
        let r_squared = body.x * body.x + body.y * body.y + body.z * body.z;
        if r_squared > max_r_squared {
            max_r_squared = r_squared;
        }
    }

    if max_r_squared < 1e-10 {
        return; // Avoid division by near-zero
    }

    // Calculate escape velocity (0.5 * √(2GM/r))
    let g: Real = 6.67408e-11;
    let escape_vel_threshold = 0.5 * (2.0 * g * total_mass / max_r_squared.sqrt());

    // Check each particle
    for i in lower..upper {
        // Calculate velocity magnitude
        let vel_squared =
            bodies[i].vx * bodies[i].vx + bodies[i].vy * bodies[i].vy + bodies[i].vz * bodies[i].vz;

        // Also check if particle is far from system center
        let dist_squared =
            bodies[i].x * bodies[i].x + bodies[i].y * bodies[i].y + bodies[i].z * bodies[i].z;

        // Only mark as escaped if velocity exceeds threshold AND particle is far from center
        if vel_squared >= escape_vel_threshold * escape_vel_threshold
            && dist_squared > 0.75 * max_r_squared
        {
            bodies[i].escaped = true;
        }
    }
}

pub fn calc_direct_force(bodies: &mut Vec<Body>, lower: usize, upper: usize) {
    let g: Real = 6.67408e-11;
    let mut x: Real;
    let mut y: Real;
    let mut z: Real;
    let mut r: Real;

    let mut a: Vec<Acc> = vec![EMPTY_ACC; bodies.len()];
    for ((i, bi), acci) in bodies
        .iter()
        .enumerate()
        .zip(a.iter_mut())
        .skip(lower)
        .take(upper - lower)
    {
        if bi.escaped {
            continue; // Skip escaped particles
        }

        for (j, bj) in bodies.iter().enumerate() {
            if i != j && !bj.escaped {
                x = bj.x - bi.x;
                y = bj.y - bi.y;
                z = bj.z - bi.z;
                r = (x * x + y * y + z * z + bi.softening * bj.softening)
                    .sqrt()
                    .powi(3);
                acci.x += g * bj.m * x / r;
                acci.y += g * bj.m * y / r;
                acci.z += g * bj.m * z / r;
            }
        }
    }
    for (bi, acci) in bodies
        .iter_mut()
        .zip(a.iter())
        .skip(lower)
        .take(upper - lower)
    {
        bi.ax = acci.x;
        bi.ay = acci.y;
        bi.az = acci.z;
    }
}

pub fn leapfrog<C: mpi::traits::Communicator>(
    bodies: &mut Vec<Body>,
    dt: Real,
    lower: usize,
    upper: usize,
    world: &C,
) {
    for bi in bodies.iter_mut().skip(lower).take(upper - lower) {
        if !bi.escaped {
            bi.x = bi.x + bi.vx * 0.5 * dt;
            bi.y = bi.y + bi.vy * 0.5 * dt;
            bi.z = bi.z + bi.vz * 0.5 * dt;
        }
    }

    for proc in 0..world.size() {
        let ai: usize = (bodies.len() as f32 / world.size() as f32 * proc as f32) as usize;
        let bi: usize = (bodies.len() as f32 / world.size() as f32 * (proc + 1) as f32) as usize;
        world
            .process_at_rank(proc)
            .broadcast_into(&mut bodies[ai..bi]);
    }
    calc_direct_force(bodies, lower, upper);

    for bi in bodies.iter_mut().skip(lower).take(upper - lower) {
        bi.vx = bi.vx + bi.ax * dt;
        bi.vy = bi.vy + bi.ay * dt;
        bi.vz = bi.vz + bi.az * dt;
        bi.x = bi.x + bi.vx * 0.5 * dt;
        bi.y = bi.y + bi.vy * 0.5 * dt;
        bi.z = bi.z + bi.vz * 0.5 * dt;
    }
    for proc in 0..world.size() {
        let ai: usize = (bodies.len() as f32 / world.size() as f32 * proc as f32) as usize;
        let bi: usize = (bodies.len() as f32 / world.size() as f32 * (proc + 1) as f32) as usize;
        world
            .process_at_rank(proc)
            .broadcast_into(&mut bodies[ai..bi]);
    }
}
pub fn get_dt<C: mpi::traits::Communicator>(
    bodies: &Vec<Body>,
    lower: usize,
    upper: usize,
    world: &C,
) -> Real {
    let mut dt: Vec<Real> = vec![0.0; upper - lower];
    let softening: Real = 0.01;
    let min_dt: Real;
    let mut a_mag: Real;
    for (i, bi) in bodies.iter().skip(lower).take(upper - lower).enumerate() {
        a_mag = (bi.ax * bi.ax + bi.ay * bi.ay + bi.az * bi.az).sqrt();
        dt[i] = (softening / a_mag).sqrt();
    }
    min_dt = dt.iter().fold(Real::INFINITY, |ai, &bi| ai.min(bi));
    println!("min_dt is {:.32}", min_dt);
    return min_dt;
}
