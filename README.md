# bft-pg
次の論文を少し拡張してみる

Béchet, Denis, Annie Foret, and Isabelle Tellier. 2007. “Learnability of Pregroup Grammars.” Studia Logica. An International Journal for Symbolic Logic 87 (2): 225–52.

feature-tagged exampleではなく各単語の持つ型の数だけを頼りに型を推論する。

```
Alice 1; likes 3; Bob 1
```
で```Alice likes Bob```という文でAliceが型を1つ、likesが3つ...というように指定する。

結果は各単語が持つ型と随伴。できるだけ型の種類と随伴の位数が少なく、低くなるようにしてある。

型の個数も指定しないで良いようにしたい

## 入力ファイル
```
((word type+ ;)+ \n)+
```
### 入力例(input.txt)
```
Alice 3;likes 3;Bob 1
Bob 1; likes 3; Alice 1
```
## 動作
``` 
cargo run --bin bft-pg input.txt > hoge.cps
sugar hoge.cps
```

## 出力例(hoge.cps)

```
(int x_Alice_0 0 10)
(int u_Alice_0 -3 3)
(int x_likes_0 0 10)
(int u_likes_0 -3 3)
(int x_likes_1 0 10)
(int u_likes_1 -3 3)
(int x_likes_2 0 10)
(int u_likes_2 -3 3)
(int x_Bob_0 0 10)
(int u_Bob_0 -3 3)
(int xcost 0 10)
(nvalue xcost (x_Alice_0 x_likes_0 x_likes_1 x_likes_2 x_Bob_0 ))
(int ucost 0 30)
(= ucost (+ (abs u_Alice_0) (abs u_likes_0) (abs u_likes_1) (abs u_likes_2) (abs u_Bob_0) ))
(int cost 0 20)
(= cost (+ xcost ucost))
(objective minimize cost)

; 0th sentences
(count 0 (x_Bob_0 x_likes_0 x_likes_1 x_likes_2 x_Alice_0 ) eq 1)
(predicate (cont_0_0_1) (and (= x_Bob_0 x_likes_0) (= (+ u_Bob_0 1) u_likes_0)))
(predicate (cont_0_0_3) (and (= x_Bob_0 x_likes_2) (= (+ u_Bob_0 1) u_likes_2)))
(predicate (cont_0_1_2) (and (= x_likes_0 x_likes_1) (= (+ u_likes_0 1) u_likes_1)))
(predicate (cont_0_1_4) (and (= x_likes_0 x_Alice_0) (= (+ u_likes_0 1) u_Alice_0)))
(predicate (cont_0_2_3) (and (= x_likes_1 x_likes_2) (= (+ u_likes_1 1) u_likes_2)))
(predicate (cont_0_3_4) (and (= x_likes_2 x_Alice_0) (= (+ u_likes_2 1) u_Alice_0)))
(! (cont_0_1_2)) 
(! (cont_0_2_3)) 
(predicate (comp_0_0_1) (or (and (cont_0_0_1) )))
(predicate (comp_0_1_2) (or (and (cont_0_1_2) )))
(predicate (comp_0_2_3) (or (and (cont_0_2_3) )))
(predicate (comp_0_3_4) (or (and (cont_0_3_4) )))
(predicate (comp_0_0_3) (or (and (cont_0_0_1) (comp_0_2_3) )(and (cont_0_0_3) (comp_0_1_2) )))
(predicate (comp_0_1_4) (or (and (cont_0_1_2) (comp_0_3_4) )(and (cont_0_1_4) (comp_0_2_3) )))
(or (and (= x_Bob_0 0) (comp_0_1_4))(and (= x_likes_1 0) (comp_0_0_1) (comp_0_3_4))(and (= x_Alice_0 0) (comp_0_0_3) ))

; 1th sentences
(count 0 (x_Alice_0 x_likes_0 x_likes_1 x_likes_2 x_Bob_0 ) eq 1)
(predicate (cont_1_0_1) (and (= x_Alice_0 x_likes_0) (= (+ u_Alice_0 1) u_likes_0)))
(predicate (cont_1_0_3) (and (= x_Alice_0 x_likes_2) (= (+ u_Alice_0 1) u_likes_2)))
(predicate (cont_1_1_2) (and (= x_likes_0 x_likes_1) (= (+ u_likes_0 1) u_likes_1)))
(predicate (cont_1_1_4) (and (= x_likes_0 x_Bob_0) (= (+ u_likes_0 1) u_Bob_0)))
(predicate (cont_1_2_3) (and (= x_likes_1 x_likes_2) (= (+ u_likes_1 1) u_likes_2)))
(predicate (cont_1_3_4) (and (= x_likes_2 x_Bob_0) (= (+ u_likes_2 1) u_Bob_0)))
(! (cont_1_1_2)) 
(! (cont_1_2_3)) 
(predicate (comp_1_0_1) (or (and (cont_1_0_1) )))
(predicate (comp_1_1_2) (or (and (cont_1_1_2) )))
(predicate (comp_1_2_3) (or (and (cont_1_2_3) )))
(predicate (comp_1_3_4) (or (and (cont_1_3_4) )))
(predicate (comp_1_0_3) (or (and (cont_1_0_1) (comp_1_2_3) )(and (cont_1_0_3) (comp_1_1_2) )))
(predicate (comp_1_1_4) (or (and (cont_1_1_2) (comp_1_3_4) )(and (cont_1_1_4) (comp_1_2_3) )))
(or (and (= x_Alice_0 0) (comp_1_1_4))(and (= x_likes_1 0) (comp_1_0_1) (comp_1_3_4))(and (= x_Bob_0 0) (comp_1_0_3) ))
```
sugar に投げると
```
s SATISFIABLE
o 6
a x_Alice_0     5
a u_Alice_0     0
a x_likes_0     5
a u_likes_0     1
a x_likes_1     0
a u_likes_1     -2
a x_likes_2     5
a u_likes_2     -1
a x_Bob_0       5
a u_Bob_0       0
a xcost 2
a ucost 4
a cost  6
a
o 5
a x_Alice_0     8
a u_Alice_0     0
a x_likes_0     8
a u_likes_0     1
a x_likes_1     0
a u_likes_1     -1
a x_likes_2     8
a u_likes_2     -1
a x_Bob_0       8
a u_Bob_0       0
a xcost 2
a ucost 3
a cost  5
a
o 4
a x_Alice_0     9
a u_Alice_0     0
a x_likes_0     9
a u_likes_0     1
a x_likes_1     0
a u_likes_1     0
a x_likes_2     9
a u_likes_2     -1
a x_Bob_0       9
a u_Bob_0       0
a xcost 2
a ucost 2
a cost  4
a
s OPTIMUM FOUND
```
というように帰ってくるのでこれを元に文法を復元する(したい)。
