import numpy as np
import matplotlib.pyplot as plt
import mpl_toolkits.mplot3d.axes3d as p3
import matplotlib.animation as animation
import sys
import os
import pandas as pd


class Planet:

    def __init__(self, df):
        self.x = [float(df["x"])]
        self.y = [float(df["y"])]
        self.z = [float(df["z"])]
        # self.m = float(df["m"])
        # self.dt = [float(df["dt"])]
        # self.t = [float(df["t"])]
        self.color = self.get_color()
        self.color = "yellow"

    def add(self, df):
        self.x.append(float(df["x"]))
        self.y.append(float(df["y"]))
        self.z.append(float(df["z"]))
        # self.dt.append(float(df["dt"]))
        # self.t.append(float(df["t"]))

    def get_color(self):
        if self.y[0] > 0:
            return "red"
        else:
            return "yellow"


def read_binary(path):
    # path = "out_10.bin"
    x = np.fromfile(path, dtype=np.float64)

    x = x.reshape((9, 9))

    d = {"x": x[:, 0],
         "y": x[:, 1],
         "z": x[:, 2],
         "vx": x[:, 3],
         "vy": x[:, 4],
         "vz": x[:, 5],
         }
    df = pd.DataFrame(data=d)
    print(df.head())
    return df


x = []
y = []
z = []

planets = []

j = 0

ThreeD = False

while True:
    if not os.path.exists(f'out{j:05d}.dat'):
        break
    print(f'reading out{j:05d}.dat')
    master_file = f'out{j:05d}.dat'

    # if j >= 100:
    #     break
    if j % 10 == 0:
        df = read_binary(master_file)
        if j == 0:
            for i in range(len(df)):
                planets.append(Planet(df.loc[i]))
        else:
            for i in range(len(df)):
                planets[i].add(df.loc[i])
    j += 1
if j == 0:
    print("no files found, exiting...")
    exit()
else:
    print("found {} files".format(j))

j = len(planets[0].x)
planets = planets[::1]

print("found {} objects".format(len(planets)))

# plt.plot(range(len(planets[0].dt)), planets[0].dt)
# plt.savefig("dt.png")

fig = plt.figure()

fig.set_size_inches(3.6 * 0.5, 3.2 * 0.5, True)

if ThreeD:
    ax = p3.Axes3D(fig)
else:
    ax = fig.add_subplot(111)

# ax.set_facecolor("black")

colors = ["yellow" for i in range(len(planets))]

if ThreeD:
    lines = [ax.plot([], [], [], '-', linewidth=0.05, c=planet.color)[0]
             for planet in planets]
    pts = [ax.plot([], [], [], color=planet.color, marker='.', lw=0, markersize=0.5, linestyle="")[0]
           for planet in planets]
else:
    lines = [ax.plot([], [], '-', linewidth=0.05, c=planet.color)[0]
             for planet in planets]
    pts = [ax.plot([], [], color=planet.color, marker='.', lw=0, markersize=0.5, linestyle="")[0]
           for planet in planets]

data = planets


def init():
    for line, pt in zip(lines, pts):
        line.set_data([], [])
        if ThreeD:
            line.set_3d_properties([])
        pt.set_data([], [])
        if ThreeD:
            pt.set_3d_properties([])
    return lines + pts


def animate(i):
    print("processing step: ", i)
    x = []
    y = []
    z = []
    counter = 0
    # print("T + {}".format(planets[0].t[i]))
    # ax.set_title("T + {:.4f}".format(planets[0].t[i]), color="white", fontsize=4)
    for line, pt, planet in zip(lines, pts, data):

        x = planet.x[i]  # planet[0][i]
        y = planet.y[i]  # planet[1][i]
        z = planet.z[i]  # planet[2][i]

        xline = planet.x[:i]  # planet[0][:i]
        yline = planet.y[:i]  # planet[1][:i]
        zline = planet.z[:i]  # planet[2][:i]

        pt.set_data(x, y)
        if ThreeD:
            pt.set_3d_properties(z)

        line.set_data(xline, yline)
        if ThreeD:
            line.set_3d_properties(zline)
        counter += 1


AU = 1.5e11

size = 1.6 * AU

com = np.zeros(3)
m = 0

# for planet in planets:
#     com += np.mean(np.array([planet.x,planet.y,planet.z]),axis=1)*planet.m
#     m += planet.m

# com /= m

print(com)

# for planet in planets:
#     planet.x += com[0]
#     planet.y += com[1]
#     planet.z += com[2]

R = [np.linalg.norm([planet.x, planet.y, planet.z]) for planet in planets]
size = np.max(R) / 4

# size = 1

plt.axis('off')
# Setting the axes properties
ax.set_xlim(-2, 2)
ax.set_ylim(-2, 2)

# ax.set_xlim(-1.5, 1)
# ax.set_ylim(8, 10)

x = np.max([abs(p.x[0]) for p in planets])
y = np.max([abs(p.y[0]) for p in planets])
ax.set_xlim(-1.5 * x, 1.5 * x)
ax.set_ylim(-1.5 * y, 1.5 * y)

if ThreeD:
    ax.set_zlim(-size, size)
    ax.set_zlim(-10, 10)

gif = False
mp4 = True  # True
show = False

ax.set_aspect('equal')

# ax.set_facecolor("black")

ani = animation.FuncAnimation(fig, animate, init_func=init,
                              frames=j, interval=1, blit=False)

# save animation
if gif:
    print("saving gif...")
    ani.save('galaxy.gif', savefig_kwargs={'facecolor': 'black'}, writer='imagemagick', dpi=100)
if mp4:
    # needs ffmpeg to be installed
    mywriter = animation.FFMpegWriter(fps=24)
    print("saving mp4...")
    ani.save('galaxy.mp4', savefig_kwargs={'facecolor': 'black'}, writer=mywriter, dpi=600)
if show:
    print("showing")
    plt.show()
