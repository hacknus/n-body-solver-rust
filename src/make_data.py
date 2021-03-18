from astroquery.jplhorizons import Horizons
import numpy as np
import pandas as pd


np.random.seed(0)

dic = {
    "name": [],
    "m": [],
    "x": [],
    "y": [],
    "z": [],
    "vx": [],
    "vy": [],
    "vz": []
}

ids_planets = ["Sun", 199, 299, 399, 499, 599, 699, 799, 899]
masses_planets = [1, 1.6601367952719304e-07, 2.447838938855945e-06, 3.0034895963231186e-06,
                  3.2271514450538743e-07, 0.0009547919384243222, 0.0002858859806661029,
                  4.3662440433515637e-05, 5.151389020535497e-05]

for i, m in zip(ids_planets, masses_planets):
    print(str(i))
    try:
        obj = Horizons(id=i, location='@sun', epochs=2458133.33546, id_type='majorbody')
        row = obj.vectors()[0]
    except ValueError as e:
        print(e)
        continue
    dic["name"].append(row["targetname"])
    dic["m"].append(m)
    dic["x"].append(row["x"])
    dic["y"].append(row["y"])
    dic["z"].append(row["z"])
    dic["vx"].append(row["vx"])
    dic["vy"].append(row["vy"])
    dic["vz"].append(row["vz"])

df = pd.read_csv("../jfc.csv")
jfc_ids = np.random.choice(np.array(df.id, dtype=np.int), 100)
ids_comets = [13699] + list(jfc_ids)



print(ids_comets)
for i in ids_comets:
    print(str(i))
    try:
        obj = Horizons(id=str(i), location='@sun', epochs=2458133.33546, id_type='designation')
        row = obj.vectors()[0]
    except ValueError as e:
        print(e)
        continue
    dic["name"].append(row["targetname"])
    dic["m"].append(1e-20)
    dic["x"].append(row["x"])
    dic["y"].append(row["y"])
    dic["z"].append(row["z"])
    dic["vx"].append(row["vx"])
    dic["vy"].append(row["vy"])
    dic["vz"].append(row["vz"])

df = pd.DataFrame(data=dic)
print(df)
df.to_csv("../solar_jfc.dat", index=False)
