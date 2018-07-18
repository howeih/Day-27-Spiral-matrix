#Day 27: Spiral matrix
<br>
Spiral matrix is an old toy algorithm. A funny way to implement the spiral would be to fill the top row in and rotate the matrix to unfold the sequence.
<br>
Fortunately, rotations and symmetries are in close relationship and any rotation can be expressed using point and line symmetry or using two line symmetries, which is our case. All we need is to flip the matrix horizontally and transpose.
<br>

matrix:

```
    let matrix = array![
        [1, 16, 15, 14, 13],
        [2, 17, 24, 23, 12],
        [3, 18, 25, 22, 11],
        [4, 19, 20, 21, 10],
        [5, 6, 7, 8, 9],
    ];

```

<br>
run:

```
1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25

```