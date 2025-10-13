// In src/math_utils.rs
use crate::body::{Acc, Agglomerate, Body, EMPTY_ACC};
use crate::Real;
use mpi::traits::*;
use std::collections::HashMap;

const VELOCITY_THRESHOLD: Real = 50.0; // m/s - threshold for sticking vs bouncing
const COEFFICIENT_OF_RESTITUTION: Real = 0.8; // Elasticity for bouncing collisions


/// Calculate adhesion force between particles in the same agglomerate
/// This prevents agglomerated particles from being pulled apart easily
pub fn calc_adhesion_force(bodies: &mut Vec<Body>, lower: usize, upper: usize, adhesion_strength: Real) {
    for i in lower..upper {
        if bodies[i].escaped || bodies[i].agglomerate_id < 0 {
            continue; // Skip free particles and escaped ones
        }

        let agg_id = bodies[i].agglomerate_id;

        for j in 0..bodies.len() {
            if i == j || bodies[j].agglomerate_id != agg_id || bodies[j].escaped {
                continue; // Only interact with same agglomerate
            }

            let dx = bodies[j].x - bodies[i].x;
            let dy = bodies[j].y - bodies[i].y;
            let dz = bodies[j].z - bodies[i].z;

            let distance_squared = dx * dx + dy * dy + dz * dz;
            let distance = distance_squared.sqrt();

            // Avoid division by zero
            if distance < 1e-10 {
                continue;
            }

            // Contact distance (sum of radii)
            let contact_distance = bodies[i].radius + bodies[j].radius;

            // Apply adhesion within a certain range (e.g., 3x contact distance)
            let adhesion_range = contact_distance * 3.0;

            if distance > adhesion_range {
                continue;
            }

            // Force magnitude increases as particles get farther apart
            // F = adhesion_strength * (distance / range) to pull them back together
            let normalized_distance = distance / adhesion_range;
            let force_magnitude = adhesion_strength * normalized_distance / distance;

            // Direction: pull particles together
            bodies[i].ax += force_magnitude * dx;
            bodies[i].ay += force_magnitude * dy;
            bodies[i].az += force_magnitude * dz;
        }
    }
}


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
            let distance = distance_squared.sqrt();
            let combined_radius = bodies[i].radius + bodies[j].radius;

            // Check if collision occurred
            if distance_squared <= combined_radius * combined_radius {
                // **NEW: Resolve overlap first by pushing particles apart**
                if distance < combined_radius {
                    let overlap = combined_radius - distance;

                    // Avoid division by zero
                    let safe_distance = if distance < 1e-10 { 1e-10 } else { distance };

                    // Normalized direction vector from i to j
                    let nx = dx / safe_distance;
                    let ny = dy / safe_distance;
                    let nz = dz / safe_distance;

                    // Calculate mass ratio for proportional separation
                    let total_mass = bodies[i].m + bodies[j].m;
                    let correction_i = overlap * bodies[j].m / total_mass;
                    let correction_j = overlap * bodies[i].m / total_mass;

                    // Move particles apart along the collision axis
                    bodies[i].x -= nx * correction_i;
                    bodies[i].y -= ny * correction_i;
                    bodies[i].z -= nz * correction_i;

                    bodies[j].x += nx * correction_j;
                    bodies[j].y += ny * correction_j;
                    bodies[j].z += nz * correction_j;
                }

                // Calculate relative velocity
                let dvx = bodies[j].vx - bodies[i].vx;
                let dvy = bodies[j].vy - bodies[i].vy;
                let dvz = bodies[j].vz - bodies[i].vz;

                // Recalculate normal after position correction
                let dx = bodies[j].x - bodies[i].x;
                let dy = bodies[j].y - bodies[i].y;
                let dz = bodies[j].z - bodies[i].z;
                let distance = (dx * dx + dy * dy + dz * dz).sqrt();

                // Avoid division by zero
                if distance < 1e-10 {
                    continue;
                }

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
                    handle_elastic_collision(bodies, i, j);
                } else {
                    handle_inelastic_collision(bodies, agglomerates, next_agg_id, i, j);
                    println!("Sticking collision between particles {} and {}", i, j);
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

pub fn handle_inelastic_collision(
    bodies: &mut Vec<Body>,
    agglomerates: &mut HashMap<i32, Agglomerate>,
    next_agg_id: &mut i32,
    i: usize,
    j: usize,
) {
    let i_agg_id = bodies[i].agglomerate_id;
    let j_agg_id = bodies[j].agglomerate_id;

    if i_agg_id == -1 && j_agg_id == -1 {
        // Both particles are free - create new agglomerate
        let new_id = *next_agg_id;
        *next_agg_id += 1;

        bodies[i].agglomerate_id = new_id;
        bodies[j].agglomerate_id = new_id;

        let mut new_agg = Agglomerate::new(new_id, i, bodies);
        new_agg.member_indices.push(j);
        new_agg.update_properties(bodies);

        // Calculate center of mass velocity for conservation of momentum
        let mi = bodies[i].m;
        let mj = bodies[j].m;
        let total_mass = mi + mj;

        let com_vx = (bodies[i].vx * mi + bodies[j].vx * mj) / total_mass;
        let com_vy = (bodies[i].vy * mi + bodies[j].vy * mj) / total_mass;
        let com_vz = (bodies[i].vz * mi + bodies[j].vz * mj) / total_mass;

        // Set both particles to have same velocity (the COM velocity)
        bodies[i].vx = com_vx;
        bodies[i].vy = com_vy;
        bodies[i].vz = com_vz;

        bodies[j].vx = com_vx;
        bodies[j].vy = com_vy;
        bodies[j].vz = com_vz;

        // **Position particles at exactly 2×radius apart**
        position_at_rest_length(bodies, i, j);

        agglomerates.insert(new_id, new_agg);

    } else if i_agg_id != -1 && j_agg_id == -1 {
        // i is in agglomerate, j is free
        if let Some(agg) = agglomerates.get_mut(&i_agg_id) {
            // Add j to existing agglomerate
            agg.member_indices.push(j);
            bodies[j].agglomerate_id = i_agg_id;

            // Update agglomerate properties including the new particle
            let old_mass = agg.total_mass;
            let new_mass = old_mass + bodies[j].m;

            // Update COM position
            let new_com_x = (agg.com.0 * old_mass + bodies[j].x * bodies[j].m) / new_mass;
            let new_com_y = (agg.com.1 * old_mass + bodies[j].y * bodies[j].m) / new_mass;
            let new_com_z = (agg.com.2 * old_mass + bodies[j].z * bodies[j].m) / new_mass;

            // Update COM velocity
            let new_vel_x = (agg.vel.0 * old_mass + bodies[j].vx * bodies[j].m) / new_mass;
            let new_vel_y = (agg.vel.1 * old_mass + bodies[j].vy * bodies[j].m) / new_mass;
            let new_vel_z = (agg.vel.2 * old_mass + bodies[j].vz * bodies[j].m) / new_mass;

            agg.com = (new_com_x, new_com_y, new_com_z);
            agg.vel = (new_vel_x, new_vel_y, new_vel_z);
            agg.total_mass = new_mass;

            // Set new particle to have same velocity as agglomerate
            bodies[j].vx = new_vel_x;
            bodies[j].vy = new_vel_y;
            bodies[j].vz = new_vel_z;

            // Also update all other particles in agglomerate
            for &idx in &agg.member_indices {
                bodies[idx].vx = new_vel_x;
                bodies[idx].vy = new_vel_y;
                bodies[idx].vz = new_vel_z;
            }

            // **Position particles at exactly 2×radius apart**
            position_at_rest_length(bodies, i, j);
        }
    } else if i_agg_id == -1 && j_agg_id != -1 {
        // i is free, j is in agglomerate
        if let Some(agg) = agglomerates.get_mut(&j_agg_id) {
            agg.member_indices.push(i);
            bodies[i].agglomerate_id = j_agg_id;

            let old_mass = agg.total_mass;
            let new_mass = old_mass + bodies[i].m;

            let new_com_x = (agg.com.0 * old_mass + bodies[i].x * bodies[i].m) / new_mass;
            let new_com_y = (agg.com.1 * old_mass + bodies[i].y * bodies[i].m) / new_mass;
            let new_com_z = (agg.com.2 * old_mass + bodies[i].z * bodies[i].m) / new_mass;

            let new_vel_x = (agg.vel.0 * old_mass + bodies[i].vx * bodies[i].m) / new_mass;
            let new_vel_y = (agg.vel.1 * old_mass + bodies[i].vy * bodies[i].m) / new_mass;
            let new_vel_z = (agg.vel.2 * old_mass + bodies[i].vz * bodies[i].m) / new_mass;

            agg.com = (new_com_x, new_com_y, new_com_z);
            agg.vel = (new_vel_x, new_vel_y, new_vel_z);
            agg.total_mass = new_mass;

            bodies[i].vx = new_vel_x;
            bodies[i].vy = new_vel_y;
            bodies[i].vz = new_vel_z;

            for &idx in &agg.member_indices {
                bodies[idx].vx = new_vel_x;
                bodies[idx].vy = new_vel_y;
                bodies[idx].vz = new_vel_z;
            }

            // **Position particles at exactly 2×radius apart**
            position_at_rest_length(bodies, i, j);
        }
    } else if i_agg_id != j_agg_id {
        // Merge two agglomerates - no direct positioning needed
        // The spring forces will handle relative positioning within the merged agglomerate
        if let (Some(agg_i), Some(agg_j)) = (agglomerates.get(&i_agg_id), agglomerates.get(&j_agg_id)) {
            let mass_i = agg_i.total_mass;
            let mass_j = agg_j.total_mass;
            let total_mass = mass_i + mass_j;

            let new_vel_x = (agg_i.vel.0 * mass_i + agg_j.vel.0 * mass_j) / total_mass;
            let new_vel_y = (agg_i.vel.1 * mass_i + agg_j.vel.1 * mass_j) / total_mass;
            let new_vel_z = (agg_i.vel.2 * mass_i + agg_j.vel.2 * mass_j) / total_mass;

            let new_com_x = (agg_i.com.0 * mass_i + agg_j.com.0 * mass_j) / total_mass;
            let new_com_y = (agg_i.com.1 * mass_i + agg_j.com.1 * mass_j) / total_mass;
            let new_com_z = (agg_i.com.2 * mass_i + agg_j.com.2 * mass_j) / total_mass;

            let j_members = agg_j.member_indices.clone();

            if let Some(agg_i) = agglomerates.get_mut(&i_agg_id) {
                agg_i.member_indices.extend(j_members.clone());
                agg_i.com = (new_com_x, new_com_y, new_com_z);
                agg_i.vel = (new_vel_x, new_vel_y, new_vel_z);
                agg_i.total_mass = total_mass;

                for &idx in &agg_i.member_indices {
                    bodies[idx].agglomerate_id = i_agg_id;
                    bodies[idx].vx = new_vel_x;
                    bodies[idx].vy = new_vel_y;
                    bodies[idx].vz = new_vel_z;
                }
            }

            agglomerates.remove(&j_agg_id);
        }
    }
}

/// Helper function to position two particles at exactly rest length (2×radius) apart
fn position_at_rest_length(bodies: &mut Vec<Body>, i: usize, j: usize) {
    let dx = bodies[j].x - bodies[i].x;
    let dy = bodies[j].y - bodies[i].y;
    let dz = bodies[j].z - bodies[i].z;
    let current_distance = (dx * dx + dy * dy + dz * dz).sqrt();

    if current_distance < 1e-10 {
        return; // Avoid division by zero
    }

    let target_distance = bodies[i].radius + bodies[j].radius;

    // Only apply correction if there's a significant difference
    if (current_distance - target_distance).abs() < 1e-10 {
        return;
    }

    // Normalized direction from i to j
    let nx = dx / current_distance;
    let ny = dy / current_distance;
    let nz = dz / current_distance;

    let mi = bodies[i].m;
    let mj = bodies[j].m;
    let total_mass = mi + mj;

    // Displacement needed (positive = need to push apart, negative = need to pull together)
    let displacement = target_distance - current_distance;

    // How much each particle moves (proportional to OTHER particle's mass)
    // Heavier particles move less
    let move_i = displacement * mj / total_mass;
    let move_j = displacement * mi / total_mass;

    // Move along the normal direction
    // If displacement > 0 (need to separate): i moves backward (-), j moves forward (+)
    // If displacement < 0 (need to compress): i moves forward (+), j moves backward (-)
    bodies[i].x += nx * move_i;
    bodies[i].y += ny * move_i;
    bodies[i].z += nz * move_i;

    bodies[j].x += nx * move_j;
    bodies[j].y += ny * move_j;
    bodies[j].z += nz * move_j;
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
    let mi = bodies[i].m;
    let mj = bodies[j].m;
    let total_mass = mi + mj;

    // Properly calculate center of mass velocity using conservation of momentum
    let com_vx = (bodies[i].vx * mi + bodies[j].vx * mj) / total_mass;
    let com_vy = (bodies[i].vy * mi + bodies[j].vy * mj) / total_mass;
    let com_vz = (bodies[i].vz * mi + bodies[j].vz * mj) / total_mass;

    // Position vectors relative to COM
    let r_i_x = bodies[i].x - com.0;
    let r_i_y = bodies[i].y - com.1;
    let r_i_z = bodies[i].z - com.2;

    let r_j_x = bodies[j].x - com.0;
    let r_j_y = bodies[j].y - com.1;
    let r_j_z = bodies[j].z - com.2;

    // Calculate relative velocity (for angular momentum)
    let dv_x = bodies[j].vx - bodies[i].vx;
    let dv_y = bodies[j].vy - bodies[i].vy;
    let dv_z = bodies[j].vz - bodies[i].vz;

    // Calculate angular momentum contribution
    let impact_factor = 0.3; // Reduced to prevent excessive rotation

    // Angular momentum from relative motion
    let ang_mom_x = impact_factor * (r_j_y * dv_z - r_j_z * dv_y) * mj;
    let ang_mom_y = impact_factor * (r_j_z * dv_x - r_j_x * dv_z) * mj;
    let ang_mom_z = impact_factor * (r_j_x * dv_y - r_j_y * dv_x) * mj;

    // Calculate moment of inertia
    let inertia = mi * (r_i_x * r_i_x + r_i_y * r_i_y + r_i_z * r_i_z)
        + mj * (r_j_x * r_j_x + r_j_y * r_j_y + r_j_z * r_j_z);

    // Set angular velocity if inertia is non-zero
    let mut ang_vel = (0.0, 0.0, 0.0);
    if inertia > 1e-10 {
        ang_vel = (
            ang_mom_x / inertia,
            ang_mom_y / inertia,
            ang_mom_z / inertia,
        );
    }

    // Update velocities to match COM velocity first
    bodies[i].vx = com_vx;
    bodies[i].vy = com_vy;
    bodies[i].vz = com_vz;

    bodies[j].vx = com_vx;
    bodies[j].vy = com_vy;
    bodies[j].vz = com_vz;

    // Add rotational component with a smaller factor to prevent excessive speeds
    let rot_scale = 0.7; // Dampening factor for rotation
    bodies[i].vx += rot_scale * (ang_vel.1 * r_i_z - ang_vel.2 * r_i_y);
    bodies[i].vy += rot_scale * (ang_vel.2 * r_i_x - ang_vel.0 * r_i_z);
    bodies[i].vz += rot_scale * (ang_vel.0 * r_i_y - ang_vel.1 * r_i_x);

    bodies[j].vx += rot_scale * (ang_vel.1 * r_j_z - ang_vel.2 * r_j_y);
    bodies[j].vy += rot_scale * (ang_vel.2 * r_j_x - ang_vel.0 * r_j_z);
    bodies[j].vz += rot_scale * (ang_vel.0 * r_j_y - ang_vel.1 * r_j_x);
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
    // Get a safe list of agglomerate IDs to avoid borrow checker issues
    let agg_ids: Vec<i32> = agglomerates.keys().cloned().collect();

    for agg_id in agg_ids {
        if let Some(agg) = agglomerates.get_mut(&agg_id) {
            // Recalculate COM and velocity
            let mut total_mass = 0.0;
            let mut com_x = 0.0;
            let mut com_y = 0.0;
            let mut com_z = 0.0;
            let mut vel_x = 0.0;
            let mut vel_y = 0.0;
            let mut vel_z = 0.0;

            // Filter out invalid indices to prevent segfaults
            let valid_indices: Vec<usize> = agg.member_indices.iter()
                .filter(|&&idx| idx < bodies.len())
                .cloned()
                .collect();

            // Update member indices with only valid ones
            agg.member_indices = valid_indices;

            // Skip empty agglomerates
            if agg.member_indices.is_empty() {
                continue;
            }

            // Calculate properties
            for &idx in &agg.member_indices {
                total_mass += bodies[idx].m;
                com_x += bodies[idx].x * bodies[idx].m;
                com_y += bodies[idx].y * bodies[idx].m;
                com_z += bodies[idx].z * bodies[idx].m;
                vel_x += bodies[idx].vx * bodies[idx].m;
                vel_y += bodies[idx].vy * bodies[idx].m;
                vel_z += bodies[idx].vz * bodies[idx].m;
            }

            if total_mass > 0.0 {
                com_x /= total_mass;
                com_y /= total_mass;
                com_z /= total_mass;
                vel_x /= total_mass;
                vel_y /= total_mass;
                vel_z /= total_mass;
            }

            // Update agglomerate properties
            agg.com = (com_x, com_y, com_z);
            agg.vel = (vel_x, vel_y, vel_z);
            agg.total_mass = total_mass;

            // Update particle velocities
            for &idx in &agg.member_indices {
                bodies[idx].vx = vel_x;
                bodies[idx].vy = vel_y;
                bodies[idx].vz = vel_z;
            }
        }
    }

    // Clean up empty agglomerates
    let empty_aggs: Vec<i32> = agglomerates.iter()
        .filter(|(_, agg)| agg.member_indices.is_empty())
        .map(|(id, _)| *id)
        .collect();

    for id in empty_aggs {
        agglomerates.remove(&id);
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

pub fn apply_gas_drag(bodies: &mut Vec<Body>, lower: usize, upper: usize) {
    // Calculate center of mass of the system
    let mut total_mass: Real = 0.0;
    let mut com_x: Real = 0.0;
    let mut com_y: Real = 0.0;
    let mut com_z: Real = 0.0;

    for body in bodies.iter() {
        if !body.escaped {
            total_mass += body.m;
            com_x += body.x * body.m;
            com_y += body.y * body.m;
            com_z += body.z * body.m;
        }
    }

    if total_mass > 0.0 {
        com_x /= total_mass;
        com_y /= total_mass;
        com_z /= total_mass;
    }

    // Parameters for the gas density profile
    let base_density: Real = 1.0e-6; // Base gas density
    let density_scale: Real = 500.0; // How quickly density increases toward center
    let max_density: Real = 1.0e-4; // Maximum gas density at the center
    let drag_coefficient: Real = 10000.0; // Strength of drag force

    // Apply drag to each particle
    for i in lower..upper {
        if bodies[i].escaped || bodies[i].agglomerate_id >= 0 {
            continue; // Skip escaped particles and agglomerates
        }

        // Calculate distance from center of mass
        let dx = bodies[i].x - com_x;
        let dy = bodies[i].y - com_y;
        let dz = bodies[i].z - com_z;
        let distance = (dx * dx + dy * dy + dz * dz).sqrt();

        // Calculate gas density at this distance (exponential profile)
        let density =
            base_density + (max_density - base_density) * (-distance * density_scale).exp();

        // Calculate gas drag force
        let speed_squared =
            bodies[i].vx * bodies[i].vx + bodies[i].vy * bodies[i].vy + bodies[i].vz * bodies[i].vz;

        if speed_squared > 0.0 {
            let speed = speed_squared.sqrt();

            // Drag force magnitude (proportional to velocity squared and density)
            let drag_force =
                drag_coefficient * density * speed_squared * bodies[i].radius * bodies[i].radius;

            // Calculate drag acceleration
            let drag_factor = drag_force / (bodies[i].m * speed);

            // Apply drag acceleration (opposite to velocity direction)
            bodies[i].ax -= bodies[i].vx * drag_factor;
            bodies[i].ay -= bodies[i].vy * drag_factor;
            bodies[i].az -= bodies[i].vz * drag_factor;
        }
    }
}

/// Apply spring forces between touching particles in the same agglomerate
/// This keeps particles at their equilibrium distance (2 × radius)
pub fn apply_spring_forces(bodies: &mut Vec<Body>, lower: usize, upper: usize, spring_constant: Real) {
    const G: Real = 6.67408e-11;

    for i in lower..upper {
        if bodies[i].escaped || bodies[i].agglomerate_id < 0 {
            continue;
        }

        let agg_id = bodies[i].agglomerate_id;

        for j in (i + 1)..bodies.len() {
            if bodies[j].agglomerate_id != agg_id || bodies[j].escaped {
                continue;
            }

            let dx = bodies[j].x - bodies[i].x;
            let dy = bodies[j].y - bodies[i].y;
            let dz = bodies[j].z - bodies[i].z;

            let distance = (dx * dx + dy * dy + dz * dz).sqrt();

            if distance < 1e-10 {
                continue;
            }

            let rest_length = bodies[i].radius + bodies[j].radius;

            // Apply spring forces in a wider range to prevent compression
            let min_threshold = rest_length * 0.5;
            let max_threshold = rest_length * 2.0;

            if distance < min_threshold || distance > max_threshold {
                continue;
            }

            // Calculate gravitational force between these two particles
            let grav_force = G * bodies[i].m * bodies[j].m / (distance * distance + bodies[i].softening * bodies[j].softening);

            // Calculate spring constant needed to balance gravity at rest_length
            // We want spring force to be stronger than gravity to prevent compression
            let adaptive_spring_constant = grav_force / (rest_length * 0.01).max(1e-10); // Balance gravity with 1% displacement

            // Use the larger of user-specified or adaptive spring constant
            let effective_spring_constant = spring_constant.max(adaptive_spring_constant * 10.0); // 10× stronger to actively resist

            // Hooke's law: F = -k × displacement
            let displacement = distance - rest_length;

            // Unit direction vector
            let nx = dx / distance;
            let ny = dy / distance;
            let nz = dz / distance;

            // Force magnitude
            let force_magnitude = -effective_spring_constant * displacement;

            // Apply forces
            bodies[i].ax += force_magnitude * nx / bodies[i].m;
            bodies[i].ay += force_magnitude * ny / bodies[i].m;
            bodies[i].az += force_magnitude * nz / bodies[i].m;

            bodies[j].ax -= force_magnitude * nx / bodies[j].m;
            bodies[j].ay -= force_magnitude * ny / bodies[j].m;
            bodies[j].az -= force_magnitude * nz / bodies[j].m;
        }
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

    apply_gas_drag(bodies, lower, upper);

    let spring_constant: Real = 1e-4; // Adjust this value based on your simulation scale
    apply_spring_forces(bodies, lower, upper, spring_constant);

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
