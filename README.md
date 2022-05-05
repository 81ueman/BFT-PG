# bft-pg
次の論文の実装

Béchet, Denis, Annie Foret, and Isabelle Tellier. 2007. “Learnability of Pregroup Grammars.” Studia Logica. An International Journal for Symbolic Logic 87 (2): 225–52.

feature-tagged exampleを入力としてCPSソルバーsugar用のインプットファイル(.cps)を作成し解かせてから入力の文章に対するパースを表示する。


## 入力ファイル
```
(word type+ ;)+
```
## example(input.txt)
```
he pi_3;loves s_1 pi_3 n;small n n;cats n 
```
## 
``` 
cargo run --bin bft-pg input.txt > hoge.cps
```

## 出力例

```
(int x_he_0_0 1 1)
(int u_he_0_0 -3 3)
(int x_loves_1_0 1 3)
(int u_loves_1_0 -3 3)
(int x_loves_1_1 1 3)
(int u_loves_1_1 -3 3)
(int x_loves_1_2 1 3)
(int u_loves_1_2 -3 3)
(int x_small_2_0 1 2)
(int u_small_2_0 -3 3)
(int x_small_2_1 1 2)
(int u_small_2_1 -3 3)
(int x_cats_3_0 1 1)
(int u_cats_3_0 -3 3)
(< x_small_2_0 x_small_2_1)
(int cost 0 1000)
(= cost (+ (abs u_he_0_0) (abs u_loves_1_0) (abs u_loves_1_1) (abs u_loves_1_2) (abs u_small_2_0) (abs u_small_2_1) (abs u_cats_3_0) ))
(objective minimize cost)
(< x_loves_1_1 x_loves_1_0)
(< x_loves_1_1 x_loves_1_2)
(= (+ u_he_0_0 1) u_loves_1_1)
(< x_loves_1_0 x_loves_1_2)
(< x_small_2_0 x_small_2_1)
(= (+ u_loves_1_2 1) u_small_2_0)
(= (+ u_small_2_1 1) u_cats_3_0)
```

sugar に投げると
```
s SATISFIABLE
o 7
a x_he_0_0      1
a u_he_0_0      -1
a x_loves_1_0   2
a u_loves_1_0   0
a x_loves_1_1   1
a u_loves_1_1   0
a x_loves_1_2   3
a u_loves_1_2   2
a x_small_2_0   1
a u_small_2_0   3
a x_small_2_1   2
a u_small_2_1   -1
a x_cats_3_0    1
a u_cats_3_0    0
a cost  7
a
o 3
a x_he_0_0      1
a u_he_0_0      0
a x_loves_1_0   2
a u_loves_1_0   0
a x_loves_1_1   1
a u_loves_1_1   1
a x_loves_1_2   3
a u_loves_1_2   0
a x_small_2_0   1
a u_small_2_0   1
a x_small_2_1   2
a u_small_2_1   0
a x_cats_3_0    1
a u_cats_3_0    1
a cost  3
a
s OPTIMUM FOUND
```
というように帰ってくるのでこれを元に文法を復元する(したい)。