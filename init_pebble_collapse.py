import numpy as np
import pandas as pd


def generate_pebble_cloud(
        filename,
        num_pebbles,
        cloud_radius_m,
        total_mass_kg,
        pebble_density_kg_m3,
        density_profile="uniform",
        rotation_factor=0.1,
        random_velocity_factor=0.05,
        visualize=True
):
    # Constants
    G = 6.67430e-11

    # Calculate individual pebble mass and radius
    pebble_mass_kg = total_mass_kg / num_pebbles
    pebble_radius_m = (3 * pebble_mass_kg / (4 * np.pi * pebble_density_kg_m3)) ** (1 / 3)
    print(f"Pebble radius: {pebble_radius_m} m")

    # Create arrays for positions
    x = np.zeros(num_pebbles)
    y = np.zeros(num_pebbles)
    z = np.zeros(num_pebbles)

    # Place particles with no overlap
    placed_count = 0
    max_attempts = 1000  # Prevent infinite loops

    while placed_count < num_pebbles:
        # Generate a candidate position
        if density_profile == "uniform":
            phi = np.random.uniform(0, 2 * np.pi)
            costheta = np.random.uniform(-1, 1)
            theta = np.arccos(costheta)
            u = np.random.uniform(0, 1)
            r = cloud_radius_m * np.cbrt(u)
        else:  # centrally_condensed
            phi = np.random.uniform(0, 2 * np.pi)
            costheta = np.random.uniform(-1, 1)
            theta = np.arccos(costheta)
            u = np.random.uniform(0, 1)
            r = cloud_radius_m * np.sqrt(u)

        # Convert to cartesian coordinates
        new_x = r * np.sin(theta) * np.cos(phi)
        new_y = r * np.sin(theta) * np.sin(phi)
        new_z = r * np.cos(theta)

        # Check for overlap with already placed particles
        if placed_count > 0:
            distances = np.sqrt((x[:placed_count] - new_x)**2 +
                                (y[:placed_count] - new_y)**2 +
                                (z[:placed_count] - new_z)**2)
            min_distance = np.min(distances)

            # Allow placement if no overlap (add a small margin)
            if min_distance > 2.1 * pebble_radius_m:  # 2.1 times radius for safety
                x[placed_count] = new_x
                y[placed_count] = new_y
                z[placed_count] = new_z
                placed_count += 1
        else:
            # First particle - just place it
            x[0] = new_x
            y[0] = new_y
            z[0] = new_z
            placed_count = 1

        # Safety check - if we can't place all particles, adjust radius or number
        if placed_count > 0 and placed_count % 100 == 0:
            print(f"Placed {placed_count} particles")

    # Calculate velocities
    escape_vel_m_s = np.sqrt(2 * G * total_mass_kg / cloud_radius_m)
    vx = np.zeros(num_pebbles)
    vy = np.zeros(num_pebbles)
    vz = np.zeros(num_pebbles)

    # Add rotation around z-axis (if desired)
    for i in range(num_pebbles):
        distance_from_center = np.sqrt(x[i]**2 + y[i]**2 + z[i]**2)
        if distance_from_center > 0:
            # Rotational velocity proportional to distance from rotation axis
            keplerian_v = np.sqrt(G * total_mass_kg / distance_from_center)
            xy_distance = np.sqrt(x[i]**2 + y[i]**2)

            if xy_distance > 0:
                vx[i] = -y[i] / xy_distance * keplerian_v * rotation_factor
                vy[i] = x[i] / xy_distance * keplerian_v * rotation_factor

    vx = np.zeros(num_pebbles)
    vy = np.zeros(num_pebbles)
    vz = np.zeros(num_pebbles)

    # Create dataframe for output
    data = {
        'index': range(num_pebbles),
        'mass': [pebble_mass_kg] * num_pebbles,
        'x': x,
        'y': y,
        'z': z,
        'vx': vx,
        'vy': vy,
        'vz': vz,
        'radius': [pebble_radius_m] * num_pebbles
    }

    df = pd.DataFrame(data)
    df.to_csv(filename, index=False)

    print(f"Generated pebble cloud with {num_pebbles} pebbles")
    print(f"Total mass: {total_mass_kg:.2e} kg")
    print(f"Cloud radius: {cloud_radius_m / 1000:.0f} km")
    print(f"Individual pebble mass: {pebble_mass_kg:.2e} kg")
    print(f"Individual pebble radius: {pebble_radius_m:.2e} m")

if __name__ == "__main__":
    # Example usage
    pebble_density = 500  # kg/m^3, typical for fluffy pebbles material
    num_pebbles = 1000
    cloud_radius_km = 0.01  # 100 m
    pebble_radius = 0.01  # meter

    cloud_radius_m = cloud_radius_km * 1000  # Convert km to m
    pebble_mass_kg = pebble_density * (4 / 3) * np.pi * pebble_radius ** 3  # Volume of sphere * density
    total_mass_kg = num_pebbles * pebble_mass_kg

    generate_pebble_cloud('input/pebble_cloud.csv', pebble_density_kg_m3=pebble_density, cloud_radius_m=cloud_radius_m,
                          num_pebbles=num_pebbles, total_mass_kg=total_mass_kg, density_profile="centrally_condensed")
