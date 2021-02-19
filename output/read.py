import numpy as np
import matplotlib.pyplot as plt
import mpl_toolkits.mplot3d.axes3d as p3
import matplotlib.animation as animation
import sys
import os
import pandas as pd


def read_binary(path):
    # path = "out_10.bin"
    x = np.fromfile(path, dtype=np.float64)

    x = x.reshape((8, 9))

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


if __name__ == "__main__":
    read_binary("out00001.dat")
