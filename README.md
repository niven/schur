# Schur Numbers

![Schur Coloring for 4,44](img/4_44.svg)

[View a relaxing animation](https://niven.github.io/schur/)

Inspired by [Numberphile: Schur Numbers (the world's biggest proof)](https://www.youtube.com/watch?v=57V8Ud7PL8k)

# Definition

A coloring of the numbers `{1, 2, ..., n}` is valid if:
- No two numbers of the same color can sum to another number of the same color

## Known numbers

- S(1) = 2
- S(2) = 5
- S(3) = 14
- S(4) = 45
- S(5) = 161

Note: This is the highest number for which there is NO coloring using that number of colors.

## Notes

- Many soloutions seem to be palindromes. It seems to me that they kind of have to be because of the constraints and the way the sums are constructed. If that's something one could prove then you would only need to find half solutions and mirror them.

- 

# Solutions

For any solution of K colors there are K! "recolorings": the same solution just with colors swapped.

Solutions for `S(2)`:
- abba
- baab
Which are the same.

Solutions for `S(3)`:
- abbaccaccabba
- bccbaacaabccb
- caacbbabbcaac
- cbbcaaaaacbbc


# Examples

The example directory contains files in the following format:
- 1 coloring per line
- Colors are denoted by unique characters
- Every character denotes the color of the digit at that position

Example:
```
abba
```
Defines a coloring for `{1,2,3,4}`, where color `a` is for `{1,4}` and color `b` is for `{2,3}`

## Filenames

Filenames are of the format `schur_k_n.txt`, where `k` is the number of colors used and `n` is the number of digits colored.

# Running

## Generate solutions

Run these with:
```
cargo run --bin schur -- --colors=3 --target=13 --algorithm=random_ban --attempts=100000
```
Parameters:
- `--colors=`: number of colors to use
- `--target=`: highest number to assign a color to
- `--algorithm`: The algorithm to use
- `--attempts`: (optional) limit for some algorithms.

Available algorithms: see ***Methods***

## Check solutions

```
cargo run --bin check -- --input=examples/schur_4_45.txt
```

# Visualization

Prerequisite: [graphviz](https://graphviz.org/download/) installed.

Use `create_dot` to create a dot file from a coloring, then use `dot` to create an image.
```bash
cargo run --bin create_dot -- --source=examples/schur_3_13.txt --destination=test.dot; and dot -Tsvg -otest.svg test.dot ; and open test.svg
```

# Methods

## Random color assignment

`algorithm=random`

Using default rand (which will divide numbers in more or less equal groups) running about a 100_000 attempts tends to find an answer for `(3,13)`

## Random color assigment with bans

`algorithm=random_ban`

Use default rand, but after assigning a color banning that color for higher numbers.
For example:
- if `1=a` then ban `a` for 2 (because 1+1=2)
- if `1=a`, `2=b`, `3=a` then ban `a` for `{4,6}`

This tends to find `(4,40)` in about 30_000 attempts. It cannot find `(4,44)` but instantly finds `(5,45)`

## Random Depth First Search

`algorithm=random_dfs`

Assign colors at random, keep any prefixes (prefix: sequence of color assignments starting at 1 up to N) that are valid and try to extend the longest prefix first.

Finds `(4,40)` in about two seconds.
Finds `(5,90)` in a minute, but at this point probably run with the `--release` profile.
Finds `(5,100)` in a many minutes, with a limit of 1_000_000_000

## Search: Depth First

`algorithm=search_dsf`

Classic DFS. This should be worse than the random version I think. Since it never gets lucky and there is still a bound.

This finds 
- `(3,13)` with a limit of 300.
- `(4,44)` with a limit of 7_000_000 (<10 seconds)

## Search:: Breadth First

For 44 the "hump" is at 27.

```
------ Stack size: 1 -- length: 2 ------
------ Stack size: 4 -- length: 3 ------
------ Stack size: 11 -- length: 4 ------
------ Stack size: 38 -- length: 5 ------
------ Stack size: 108 -- length: 6 ------
------ Stack size: 362 -- length: 7 ------
------ Stack size: 852 -- length: 8 ------
------ Stack size: 2790 -- length: 9 ------
------ Stack size: 5928 -- length: 10 ------
------ Stack size: 16652 -- length: 11 ------
------ Stack size: 37078 -- length: 12 ------
------ Stack size: 97974 -- length: 13 ------
------ Stack size: 190622 -- length: 14 ------
------ Stack size: 494294 -- length: 15 ------
------ Stack size: 817284 -- length: 16 ------
------ Stack size: 1840298 -- length: 17 ------
------ Stack size: 3195152 -- length: 18 ------
------ Stack size: 6688694 -- length: 19 ------
------ Stack size: 7665218 -- length: 20 ------
------ Stack size: 15849060 -- length: 21 ------
------ Stack size: 17624478 -- length: 22 ------
------ Stack size: 28332964 -- length: 23 ------
------ Stack size: 36048952 -- length: 24 ------
------ Stack size: 42490328 -- length: 25 ------
------ Stack size: 38278786 -- length: 26 ------
------ Stack size: 46903046 -- length: 27 ------
------ Stack size: 40796376 -- length: 28 ------
------ Stack size: 36638476 -- length: 29 ------
------ Stack size: 35022236 -- length: 30 ------
------ Stack size: 26190108 -- length: 31 ------
------ Stack size: 17892308 -- length: 32 ------
------ Stack size: 14722608 -- length: 33 ------
------ Stack size: 9641832 -- length: 34 ------
------ Stack size: 6164266 -- length: 35 ------
------ Stack size: 3968518 -- length: 36 ------
------ Stack size: 1647230 -- length: 37 ------
------ Stack size: 967224 -- length: 38 ------
------ Stack size: 760702 -- length: 39 ------
------ Stack size: 321386 -- length: 40 ------
------ Stack size: 148968 -- length: 41 ------
------ Stack size: 131802 -- length: 42 ------
------ Stack size: 123076 -- length: 43 ------
```

546 results, with 4 colors that means 4*3 extra "real" results because the search only checks starting with 2 different colors. (Wait, I think that's wrong?)

This won't of course find anything much above 50 given the amount of space that would use.

## Simulated Annealing

Treat the numbers as a column of particles where 1 is the lowest and coolest. Assign random colors. Then make small random changes to the entire column from 1..n with the chances of a change being higher the higher the "temperature" is, and the more correctly colored numbers below. The idea is to settle in the correct stuff while varying the incorrect and higher numbers.
