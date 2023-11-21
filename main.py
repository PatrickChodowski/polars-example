# import pandas as pd 
from datetime import datetime
import polars as pl 
startTime = datetime.now()
# df = pd.read_csv("./data/big.csv")

# print(datetime.now() - startTime)

df = pl.read_csv("./data/big.csv", ignore_errors=True)

print(datetime.now() - startTime)
# print(df)

m = df.mean()
print(m)

s = df.sum()
print(s)

print(datetime.now() - startTime)

