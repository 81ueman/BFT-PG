# bft-pg
次の論文の実装

Béchet, Denis, Annie Foret, and Isabelle Tellier. 2007. “Learnability of Pregroup Grammars.” Studia Logica. An International Journal for Symbolic Logic 87 (2): 225–52.

feature-tagged exampleを入力としてCPSソルバーsugar用のインプットファイル(.cps)を作成し解かせてから入力の文章に対するパースを表示する。


## 入力
```
(word type+ ;)+
```
## example
```
he pi_3;loves s_1 pi_3 n;small n n;cats n 
```

## 出力例

```
(int x_0_0 1 1)
(int u_0_0 -3 3)
(int x_1_0 1 3)
(int u_1_0 -3 3)
(int x_1_1 1 3)
(int u_1_1 -3 3)
(int x_1_2 1 3)
(int u_1_2 -3 3)
(int x_2_0 1 2)
(int u_2_0 -3 3)
(int x_2_1 1 2)
(int u_2_1 -3 3)
(int x_3_0 1 1)
(int u_3_0 -3 3)
(int cost 0 1000)
(= cost (+ (abs u_0_0) (abs u_1_0) (abs u_1_1) (abs u_1_2) (abs u_2_0) (abs u_2_1) (abs u_3_0) ))
(objective minimize cost)
(< x_1_1 x_1_0)
(< x_1_1 x_1_2)
(= (+ u_0_0 1) u_1_1)
(< x_1_0 x_1_2)
(< x_2_0 x_2_1)
(= (+ u_1_2 1) u_2_0)
(= (+ u_2_1 1) u_3_0)
```

sugar に投げると
```
s SATISFIABLE
o 4
a x_1_0 2
a x_2_0 2
a x_2_1 0
a x_2_2 2
a x_3_0 2
a u_1_0 1
a u_2_0 2
a u_2_1 0
a u_2_2 0
a u_3_0 1
a cost  4
a
o 2
a x_1_0 1
a x_2_0 1
a x_2_1 0
a x_2_2 1
a x_3_0 1
a u_1_0 0
a u_2_0 1
a u_2_1 0
a u_2_2 -1
a u_3_0 0
a cost  2
a
s OPTIMUM FOUND
```
というように帰ってくるのでこれを元に文法を復元する。