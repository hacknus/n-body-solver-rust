import os

import matplotlib.animation as animation
import matplotlib.pyplot as plt
import numpy as np
import pandas as pd

# Configuration
file_name = "pebble_collapse"
ThreeD = True  # Set to True for 3D visualization
max_particles_to_display = 1000  # Limit for performance
show_trails = False  # Set to False for better performance with many particles
save_gif = False
save_mp4 = True
show_plot = True
output_dir = "."  # Fixed output directory path


def read_binary(path):
    """Read binary output files from the Rust simulation"""
    try:
        x = np.fromfile(path, dtype=np.float64)
        # Each particle now has 11 values (including escaped flag)
        x = x.reshape((x.shape[0] // 12, 12))

        d = {"x": x[:, 0],
             "y": x[:, 1],
             "z": x[:, 2],
             "vx": x[:, 3],
             "vy": x[:, 4],
             "vz": x[:, 5],
             "ax": x[:, 6],
             "ay": x[:, 7],
             "az": x[:, 8],
             "radius": x[:, 9],
             "id": x[:, 10],
             "escaped": x[:, 11]}
        return pd.DataFrame(data=d)
    except Exception as e:
        print(f"Error reading file {path}: {e}")
        return None


# Find all output files
frames = []
j = 0
while True:
    filepath = os.path.join(output_dir, f'out{j:05d}.dat')
    if not os.path.exists(filepath):
        print(f"Stopping search at frame {j}, file not found: {filepath}")
        break
    frames.append(j)
    j += 500  # Using step 10 since files are saved every 10 frames

if len(frames) == 0:
    print(f"No output files found in {output_dir}, exiting...")
    exit()
else:
    print(f"Found {len(frames)} output files")

# First, read all frames to track which particles ever escape
all_data = []
for frame_num in frames:
    df = read_binary(os.path.join(output_dir, f'out{frame_num:05d}.dat'))
    if df is not None:
        all_data.append(df)
    else:
        print(f"Could not read frame {frame_num}")

if not all_data:
    print("No valid data frames found")
    exit()

# Find particles that ever escape
num_particles = len(all_data[0])
ever_escaped = np.zeros(num_particles, dtype=bool)

for df in all_data:
    ever_escaped = ever_escaped | (df["escaped"] > 0.5)
last = all_data[-1]
r = last["x"] ** 2 + last["y"] ** 2 + last["z"] ** 2
ever_escaped |= np.sqrt(r) > 1000

print(f"Total particles: {num_particles}")
print(f"Particles that never escape: {np.sum(~ever_escaped)}")


# Create particle data structure for non-escaped particles
class Particle:
    def __init__(self, index, x, y, z, radius, particle_id):
        self.index = index
        self.x = [float(x)]
        self.y = [float(y)]
        self.z = [float(z)]
        self.radius = float(radius)
        self.ids = [int(particle_id)]  # Store ID history

    def add(self, x, y, z, particle_id):
        self.x.append(float(x))
        self.y.append(float(y))
        self.z.append(float(z))
        self.ids.append(int(particle_id))


# Get indices of non-escaped particles
non_escaped_indices = np.where(~ever_escaped)[0]

# If too many particles, sample randomly for visualization
if len(non_escaped_indices) > max_particles_to_display:
    print(f"Limiting visualization to {max_particles_to_display} particles")
    particle_indices = np.random.choice(non_escaped_indices, max_particles_to_display, replace=False)
else:
    particle_indices = non_escaped_indices

# Initialize particles
particles = []
first_frame = all_data[0]
for idx in particle_indices:
    p = Particle(
        idx,
        first_frame["x"][idx],
        first_frame["y"][idx],
        first_frame["z"][idx],
        first_frame["radius"][idx],
        first_frame["id"][idx]  # Pass the ID
    )
    particles.append(p)


# Add positions from remaining frames
for df in all_data[1:]:
    for particle in particles:
        idx = particle.index
        try:
            particle.add(df["x"][idx], df["y"][idx], df["z"][idx], df["id"][idx])
        except Exception as e:
            break
            print(f"Error adding data for particle {idx}: {e}")

# Normalize data for better visualization
# Find overall min/max for plotting
all_x_values = []
all_y_values = []
all_z_values = []

for p in particles:
    all_x_values.extend(p.x)
    all_y_values.extend(p.y)
    all_z_values.extend(p.z)

x_min, x_max = min(all_x_values), max(all_x_values)
y_min, y_max = min(all_y_values), max(all_y_values)
z_min, z_max = min(all_z_values), max(all_z_values)

# Setup plot
fig = plt.figure(figsize=(10, 8))

if ThreeD:
    ax = fig.add_subplot(111, projection='3d')
else:
    ax = fig.add_subplot(111)

# Collect all unique IDs across all frames
all_ids = set()
for df in all_data:
    for idx in particle_indices:
        try:
            all_ids.add(int(df["id"][idx]))
        except Exception as e:
            print(f"Error collecting ID for particle {idx}: {e}")
            break

unique_ids = sorted(list(all_ids))

print("Unique particle IDs across all frames:", unique_ids)
print("Number of unique particle IDs:", len(unique_ids))
num_unique_ids = len(unique_ids)

# Create a color map for IDs
cmap = plt.cm.tab20 if num_unique_ids <= 20 else plt.cm.hsv
id_to_color = {id_val: cmap(i / num_unique_ids) for i, id_val in enumerate(unique_ids)}

# Assign initial colors based on first frame particle ID
colors = [id_to_color[first_frame["id"][idx]] for idx in particle_indices]

# Set up lines and points for animation
if ThreeD:
    if show_trails:
        lines = [ax.plot([], [], [], '-', lw=0.3, color=colors[i], alpha=0.3)[0] for i in range(len(particles))]
    else:
        lines = []
    pts = [ax.plot([], [], [], 'o', ms=3, color=colors[i])[0] for i in range(len(particles))]
else:
    if show_trails:
        lines = [ax.plot([], [], '-', lw=0.3, color=colors[i], alpha=0.3)[0] for i in range(len(particles))]
    else:
        lines = []
    pts = [ax.plot([], [], 'o', ms=3, color=colors[i])[0] for i in range(len(particles))]

# Create time label
time_label = ax.text2D(0.05, 0.95, "", transform=ax.transAxes) if ThreeD else ax.text(0.05, 0.95, "",
                                                                                      transform=ax.transAxes)


def init():
    time_label.set_text('')
    for pt in pts:
        pt.set_data([], [])
        if ThreeD:
            pt.set_3d_properties([])

    if show_trails:
        for line in lines:
            line.set_data([], [])
            if ThreeD:
                line.set_3d_properties([])

    return pts + (lines if show_trails else []) + [time_label]


def animate(i):
    print(f"Processing frame: {i + 1}/{min(len(particles[0].x), num_frames)}")
    time_label.set_text(f"Time step: {i}")

    try:
        if show_trails:
            for j, (line, pt, particle) in enumerate(zip(lines, pts, particles)):
                if i < len(particle.x):
                    x = particle.x[i]
                    y = particle.y[i]
                    z = particle.z[i] if ThreeD else 0

                    # Update color based on current ID
                    current_id = particle.ids[i]
                    pt.set_color(id_to_color.get(current_id, colors[j]))

                    xline = particle.x[:i + 1]
                    yline = particle.y[:i + 1]
                    zline = particle.z[:i + 1] if ThreeD else []

                    pt.set_data([x], [y])
                    if ThreeD:
                        pt.set_3d_properties([z])

                    line.set_data(xline, yline)
                    if ThreeD:
                        line.set_3d_properties(zline)
        else:
            for j, (pt, particle) in enumerate(zip(pts, particles)):
                if i < len(particle.x):
                    x = particle.x[i]
                    y = particle.y[i]
                    z = particle.z[i] if ThreeD else 0

                    # Update color based on current ID
                    current_id = particle.ids[i]
                    pt.set_color(id_to_color.get(current_id, colors[j]))

                    pt.set_data([x], [y])
                    if ThreeD:
                        pt.set_3d_properties([z])
    except Exception as e:
        print(f"Error in animation frame {i}: {e}")

    return pts + (lines if show_trails else []) + [time_label]


# Calculate appropriate axis limits with some margin
all_x = [p.x for p in particles]
all_y = [p.y for p in particles]
all_z = [p.z for p in particles]

flat_x = [item for sublist in all_x for item in sublist]
flat_y = [item for sublist in all_y for item in sublist]
flat_z = [item for sublist in all_z for item in sublist]

x_range = max(flat_x) - min(flat_x)
y_range = max(flat_y) - min(flat_y)
z_range = max(flat_z) - min(flat_z) if ThreeD else 0

max_range = max(x_range, y_range, z_range) * 0.6
margin = max_range * 0.2  # Add 20% margin

mid_x = (max(flat_x) + min(flat_x)) / 2
mid_y = (max(flat_y) + min(flat_y)) / 2
mid_z = (max(flat_z) + min(flat_z)) / 2 if ThreeD else 0

# Set plot properties
ax.set_xlim(mid_x - max_range - margin, mid_x + max_range + margin)
ax.set_ylim(mid_y - max_range - margin, mid_y + max_range + margin)
#ax.set_xlim(-1000, 1000)
#ax.set_ylim(-1000, 1000)
if ThreeD:
    ax.set_zlim(mid_z - max_range - margin, mid_z + max_range + margin)
    #ax.set_zlim(-1000, 1000)

ax.set_xlabel('X (m)')
ax.set_ylabel('Y (m)')
if ThreeD:
    ax.set_zlabel('Z (m)')
ax.set_title('Pebble Cloud Collapse')

# Create animation
num_frames = min(len(particles[0].x), 1000000)  # Limit frames for performance
print(f"Creating animation with {num_frames} frames")

ani = animation.FuncAnimation(
    fig, animate, init_func=init,
    frames=num_frames, interval=50, blit=True
)

# Save and show animation
if save_gif:
    print("Saving GIF animation...")
    ani.save(f'{file_name}.gif', writer='pillow', fps=20, dpi=100)

if save_mp4:
    print("Saving MP4 animation...")
    try:
        # Higher quality for final render
        writer = animation.FFMpegWriter(fps=30, bitrate=3000)
        ani.save(f'{file_name}.mp4', writer=writer, dpi=150)
    except Exception as e:
        print(f"Error saving MP4: {e}")
        print("Make sure FFmpeg is installed and in your PATH")

if show_plot:
    plt.tight_layout()
    plt.show()
