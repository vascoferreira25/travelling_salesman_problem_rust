# Travelling Salesman Problem

> "Given a list of cities and the distances between each pair of cities, what
> is the shortest possible route that visits each city and returns to the
> origin city?"
- [Wikipedia](https://en.wikipedia.org/wiki/Travelling_salesman_problem)

This implementations uses a genetic algorithm to find the shortest route
between cities.

## Installation

1. Clone this repo:
```shell
git clone https://github.com/vascoferreira25/travelling_salesman_problem_rust
```

## Purpose

Find the best possible route for a list of cities.
The cities are represented in the following chart:

![cities_map](./docs/cities_map.png "Map with the cities")

## Results

Run the simulation with `cargo run --release` and see how fast it gets the best
result.

The best result (so far) is: `565.685424949238`.

### Output

```
Epoch: 0/200 - Best Distance: 3040.1698124971404
Epoch: 1/200 - Best Distance: 3027.7058822370977
Epoch: 2/200 - Best Distance: 3027.7058822370977
Epoch: 3/200 - Best Distance: 3027.7058822370977
Epoch: 4/200 - Best Distance: 2983.655889444054
Epoch: 5/200 - Best Distance: 2983.655889444054
Epoch: 6/200 - Best Distance: 2983.655889444054
Epoch: 7/200 - Best Distance: 2983.228616391946
Epoch: 8/200 - Best Distance: 2784.780539200872
Epoch: 9/200 - Best Distance: 2702.381752643289
Epoch: 10/200 - Best Distance: 2702.381752643289
Epoch: 11/200 - Best Distance: 2629.939001379096
Epoch: 12/200 - Best Distance: 2583.846339452557
Epoch: 13/200 - Best Distance: 2583.846339452557
Epoch: 14/200 - Best Distance: 2498.1279566733297
Epoch: 15/200 - Best Distance: 2495.926859097641
Epoch: 16/200 - Best Distance: 2495.926859097641
Epoch: 17/200 - Best Distance: 2433.2254765747375
Epoch: 18/200 - Best Distance: 2433.2254765747375
Epoch: 19/200 - Best Distance: 2433.2254765747375
Epoch: 20/200 - Best Distance: 2433.2254765747375
Epoch: 21/200 - Best Distance: 2270.047209119925
Epoch: 22/200 - Best Distance: 2239.3496006477753
Epoch: 23/200 - Best Distance: 2202.5699270293258
Epoch: 24/200 - Best Distance: 1955.4697043035108
Epoch: 25/200 - Best Distance: 1955.4697043035108
Epoch: 26/200 - Best Distance: 1955.4697043035108
Epoch: 27/200 - Best Distance: 1955.4697043035108
Epoch: 28/200 - Best Distance: 1916.0804391406866
Epoch: 29/200 - Best Distance: 1916.0804391406866
Epoch: 30/200 - Best Distance: 1907.165405169398
Epoch: 31/200 - Best Distance: 1907.165405169398
Epoch: 32/200 - Best Distance: 1897.6853889785157
Epoch: 33/200 - Best Distance: 1897.6853889785157
Epoch: 34/200 - Best Distance: 1897.6853889785157
Epoch: 35/200 - Best Distance: 1894.1771746721047
Epoch: 36/200 - Best Distance: 1894.1771746721047
Epoch: 37/200 - Best Distance: 1879.1752411311804
Epoch: 38/200 - Best Distance: 1843.6162683689822
Epoch: 39/200 - Best Distance: 1842.5727156935632
Epoch: 40/200 - Best Distance: 1795.4792289659213
Epoch: 41/200 - Best Distance: 1781.1114280887023
Epoch: 42/200 - Best Distance: 1745.9847831126363
Epoch: 43/200 - Best Distance: 1681.7032791884849
Epoch: 44/200 - Best Distance: 1681.7032791884849
Epoch: 45/200 - Best Distance: 1681.7032791884849
Epoch: 46/200 - Best Distance: 1672.5069094579935
Epoch: 47/200 - Best Distance: 1670.4733200369024
Epoch: 48/200 - Best Distance: 1670.4733200369024
Epoch: 49/200 - Best Distance: 1670.4733200369024
Epoch: 50/200 - Best Distance: 1670.4733200369024
Epoch: 51/200 - Best Distance: 1670.4733200369024
Epoch: 52/200 - Best Distance: 1670.4733200369024
Epoch: 53/200 - Best Distance: 1670.4733200369024
Epoch: 54/200 - Best Distance: 1670.4733200369024
Epoch: 55/200 - Best Distance: 1654.8667852742128
Epoch: 56/200 - Best Distance: 1654.8667852742128
Epoch: 57/200 - Best Distance: 1608.9491761038457
Epoch: 58/200 - Best Distance: 1608.9491761038457
Epoch: 59/200 - Best Distance: 1608.9491761038457
Epoch: 60/200 - Best Distance: 1566.0327613659792
Epoch: 61/200 - Best Distance: 1566.0327613659792
Epoch: 62/200 - Best Distance: 1566.0327613659792
Epoch: 63/200 - Best Distance: 1566.0327613659792
Epoch: 64/200 - Best Distance: 1566.0327613659792
Epoch: 65/200 - Best Distance: 1566.0327613659792
Epoch: 66/200 - Best Distance: 1566.0327613659792
Epoch: 67/200 - Best Distance: 1566.0327613659792
Epoch: 68/200 - Best Distance: 1566.0327613659792
Epoch: 69/200 - Best Distance: 1566.0327613659792
Epoch: 70/200 - Best Distance: 1566.0327613659792
Epoch: 71/200 - Best Distance: 1566.0327613659792
Epoch: 72/200 - Best Distance: 1566.0327613659792
Epoch: 73/200 - Best Distance: 1566.0327613659792
Epoch: 74/200 - Best Distance: 1566.0327613659792
Epoch: 75/200 - Best Distance: 1566.0327613659792
Epoch: 76/200 - Best Distance: 1566.0327613659792
Epoch: 77/200 - Best Distance: 1566.0327613659792
Epoch: 78/200 - Best Distance: 1547.7780123868256
Epoch: 79/200 - Best Distance: 1486.7305768078354
Epoch: 80/200 - Best Distance: 1486.7305768078354
Epoch: 81/200 - Best Distance: 1486.7305768078354
Epoch: 82/200 - Best Distance: 1437.8389712617602
Epoch: 83/200 - Best Distance: 1435.319612239713
Epoch: 84/200 - Best Distance: 1406.1232425092219
Epoch: 85/200 - Best Distance: 1382.2302514067242
Epoch: 86/200 - Best Distance: 1362.4521812095736
Epoch: 87/200 - Best Distance: 1362.4521812095736
Epoch: 88/200 - Best Distance: 1345.5105089818126
Epoch: 89/200 - Best Distance: 1345.5105089818126
Epoch: 90/200 - Best Distance: 1333.1243572866929
Epoch: 91/200 - Best Distance: 1329.537207267151
Epoch: 92/200 - Best Distance: 1329.537207267151
Epoch: 93/200 - Best Distance: 1328.867868775054
Epoch: 94/200 - Best Distance: 1328.867868775054
Epoch: 95/200 - Best Distance: 1326.5694482970996
Epoch: 96/200 - Best Distance: 1290.1199377006064
Epoch: 97/200 - Best Distance: 1245.192649696382
Epoch: 98/200 - Best Distance: 1245.192649696382
Epoch: 99/200 - Best Distance: 1238.4572921459555
Epoch: 100/200 - Best Distance: 1225.192649696382
Epoch: 101/200 - Best Distance: 1210.7356321189473
Epoch: 102/200 - Best Distance: 1210.1730208984934
Epoch: 103/200 - Best Distance: 1210.1730208984934
Epoch: 104/200 - Best Distance: 1210.1730208984934
Epoch: 105/200 - Best Distance: 1184.833771534001
Epoch: 106/200 - Best Distance: 1184.833771534001
Epoch: 107/200 - Best Distance: 1184.833771534001
Epoch: 108/200 - Best Distance: 1184.833771534001
Epoch: 109/200 - Best Distance: 1184.833771534001
Epoch: 110/200 - Best Distance: 1184.833771534001
Epoch: 111/200 - Best Distance: 1183.1418490984215
Epoch: 112/200 - Best Distance: 1139.15442723347
Epoch: 113/200 - Best Distance: 1139.15442723347
Epoch: 114/200 - Best Distance: 1119.8904165601245
Epoch: 115/200 - Best Distance: 1119.8904165601245
Epoch: 116/200 - Best Distance: 1109.7226107775275
Epoch: 117/200 - Best Distance: 1109.7226107775275
Epoch: 118/200 - Best Distance: 1109.7226107775275
Epoch: 119/200 - Best Distance: 1109.7226107775275
Epoch: 120/200 - Best Distance: 1109.7226107775275
Epoch: 121/200 - Best Distance: 1109.7226107775275
Epoch: 122/200 - Best Distance: 1109.7226107775275
Epoch: 123/200 - Best Distance: 1073.8693598198747
Epoch: 124/200 - Best Distance: 1073.8693598198747
Epoch: 125/200 - Best Distance: 1073.8693598198747
Epoch: 126/200 - Best Distance: 1073.8693598198747
Epoch: 127/200 - Best Distance: 1073.8693598198747
Epoch: 128/200 - Best Distance: 1073.8693598198747
Epoch: 129/200 - Best Distance: 1036.052597777973
Epoch: 130/200 - Best Distance: 1009.8733099161838
Epoch: 131/200 - Best Distance: 1009.8733099161838
Epoch: 132/200 - Best Distance: 1009.8733099161838
Epoch: 133/200 - Best Distance: 999.0696796466748
Epoch: 134/200 - Best Distance: 999.0696796466748
Epoch: 135/200 - Best Distance: 999.0696796466748
Epoch: 136/200 - Best Distance: 981.5890386687219
Epoch: 137/200 - Best Distance: 955.8241264433071
Epoch: 138/200 - Best Distance: 953.30476742126
Epoch: 139/200 - Best Distance: 947.7065145512877
Epoch: 140/200 - Best Distance: 927.5398551958452
Epoch: 141/200 - Best Distance: 925.020496173798
Epoch: 142/200 - Best Distance: 925.020496173798
Epoch: 143/200 - Best Distance: 925.020496173798
Epoch: 144/200 - Best Distance: 925.020496173798
Epoch: 145/200 - Best Distance: 906.2953907436355
Epoch: 146/200 - Best Distance: 853.8437361714515
Epoch: 147/200 - Best Distance: 850.2808244739854
Epoch: 148/200 - Best Distance: 837.4066478689176
Epoch: 149/200 - Best Distance: 833.2897599461778
Epoch: 150/200 - Best Distance: 833.2897599461778
Epoch: 151/200 - Best Distance: 755.3910524340095
Epoch: 152/200 - Best Distance: 755.3910524340095
Epoch: 153/200 - Best Distance: 755.3910524340095
Epoch: 154/200 - Best Distance: 743.5438694890814
Epoch: 155/200 - Best Distance: 698.8225099390856
Epoch: 156/200 - Best Distance: 698.8225099390856
Epoch: 157/200 - Best Distance: 690.5382386916237
Epoch: 158/200 - Best Distance: 686.9753269941576
Epoch: 159/200 - Best Distance: 662.2539674441618
Epoch: 160/200 - Best Distance: 662.2539674441618
Epoch: 161/200 - Best Distance: 662.2539674441618
Epoch: 162/200 - Best Distance: 662.2539674441618
Epoch: 163/200 - Best Distance: 642.2539674441618
Epoch: 164/200 - Best Distance: 613.9696961966999
Epoch: 165/200 - Best Distance: 613.9696961966999
Epoch: 166/200 - Best Distance: 613.9696961966999
Epoch: 167/200 - Best Distance: 613.9696961966999
Epoch: 168/200 - Best Distance: 613.9696961966999
Epoch: 169/200 - Best Distance: 613.9696961966999
Epoch: 170/200 - Best Distance: 593.9696961966999
Epoch: 171/200 - Best Distance: 593.9696961966999
Epoch: 172/200 - Best Distance: 593.9696961966999
Epoch: 173/200 - Best Distance: 593.9696961966999
Epoch: 174/200 - Best Distance: 593.9696961966999
Epoch: 175/200 - Best Distance: 585.685424949238
Epoch: 176/200 - Best Distance: 585.685424949238
Epoch: 177/200 - Best Distance: 585.685424949238
Epoch: 178/200 - Best Distance: 585.685424949238
Epoch: 179/200 - Best Distance: 585.685424949238
Epoch: 180/200 - Best Distance: 585.685424949238
Epoch: 181/200 - Best Distance: 585.685424949238
Epoch: 182/200 - Best Distance: 585.685424949238
Epoch: 183/200 - Best Distance: 585.685424949238
Epoch: 184/200 - Best Distance: 585.685424949238
Epoch: 185/200 - Best Distance: 585.685424949238
Epoch: 186/200 - Best Distance: 565.685424949238
Epoch: 187/200 - Best Distance: 565.685424949238
Epoch: 188/200 - Best Distance: 565.685424949238
Epoch: 189/200 - Best Distance: 565.685424949238
Epoch: 190/200 - Best Distance: 565.685424949238
Epoch: 191/200 - Best Distance: 565.685424949238
Epoch: 192/200 - Best Distance: 565.685424949238
Epoch: 193/200 - Best Distance: 565.685424949238
Epoch: 194/200 - Best Distance: 565.685424949238
Epoch: 195/200 - Best Distance: 565.685424949238
Epoch: 196/200 - Best Distance: 565.685424949238
Epoch: 197/200 - Best Distance: 565.685424949238
Epoch: 198/200 - Best Distance: 565.685424949238
Epoch: 199/200 - Best Distance: 565.685424949238
```

### Best Individual

```
----------------------------------------------
               Individual Info
----------------------------------------------
- Route:
    - City 0: 16
    - City 1: 17
    - City 2: 18
    - City 3: 19
    - City 4: 20
    - City 5: 21
    - City 6: 22
    - City 7: 02
    - City 8: 23
    - City 9: 24
    - City 10: 25
    - City 11: 26
    - City 12: 27
    - City 13: 28
    - City 14: 29
    - City 15: 30
    - City 16: 31
    - City 17: 04
    - City 18: 32
    - City 19: 33
    - City 20: 34
    - City 21: 35
    - City 22: 36
    - City 23: 37
    - City 24: 38
    - City 25: 39
    - City 26: 40
    - City 27: 01
    - City 28: 05
    - City 29: 06
    - City 30: 07
    - City 31: 08
    - City 32: 09
    - City 33: 10
    - City 34: 11
    - City 35: 12
    - City 36: 13
    - City 37: 03
    - City 38: 14
    - City 39: 15
- Total distance: 565.685424949238
- Fitness: 0.00000000000000000000009536743164062499
- Normalized Fitness: 0.004181660926738634
----------------------------------------------
```
