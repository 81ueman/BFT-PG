# bft-pg
次の論文をもう少し拡張してみる
Béchet, Denis, Annie Foret, and Isabelle Tellier. 2007. “Learnability of Pregroup Grammars.” Studia Logica. An International Journal for Symbolic Logic 87 (2): 225–52.

コーパスだけから推論できるようにする。

inferwithnumブランチで単語＋型の数から推測できるようにはしておいたので各単語について脳筋全探索で合計の型の数が少ないものからどんどん試していく。

例えば文中で
```
Alice  likes  Bob 
```
の３単語があるなら

```
Alice 1 likes 1 Bob 1
Alice 2 likes 1 Bob 1
Alice 1 likes 2 Bob 2
...
```
といった感じで試して最初に成功したものを出力する。


## 入力ファイル
```
((word' ')+\n)+ 
```
### 入力例(input.txt)
nltkのdemo_grammarでgenerateしたもの。
```
the man slept
the man saw the man
```
## 動作
``` 
cargo run input.txt
```

## tmp.cps(制約を吐き出す一時ファイル)

```
;{"saw": 1, "slept": 1, "man": 1, "the": 1}
;0th pattern

;variable definitions
(int x_saw_0 0 10)
(int u_saw_0 -3 3)
(int x_slept_0 0 10)
(int u_slept_0 -3 3)
(int x_man_0 0 10)
(int u_man_0 -3 3)
(int x_the_0 0 10)
(int u_the_0 -3 3)
(int xcost 0 10)
(nvalue xcost (x_saw_0 x_slept_0 x_man_0 x_the_0 ))
(int ucost 0 30)
(= ucost (+ (abs u_saw_0) (abs u_slept_0) (abs u_man_0) (abs u_the_0) ))
(int cost 0 20)
(= cost (+ xcost ucost))
(objective minimize cost)

; 0th sentences
;sum of types:3
(count 0 (x_the_0 x_man_0 x_slept_0 ) eq 1)
;["the_0", "man_0", "slept_0"]
(predicate (cont_0_0_1) (and (= x_the_0 x_man_0) (= (+ u_the_0 1) u_man_0)))
(predicate (cont_0_1_2) (and (= x_man_0 x_slept_0) (= (+ u_man_0 1) u_slept_0)))
(predicate (comp_0_0_1) (or (and (cont_0_0_1) )))
(predicate (comp_0_1_2) (or (and (cont_0_1_2) )))
(or (and (= x_the_0 0) (comp_0_1_2))(and (= x_slept_0 0) (comp_0_0_1) ))

; 1th sentences
;sum of types:5
(count 0 (x_the_0 x_man_0 x_saw_0 x_the_0 x_man_0 ) eq 1)
;["the_0", "man_0", "saw_0", "the_0", "man_0"]
(predicate (cont_1_0_1) (and (= x_the_0 x_man_0) (= (+ u_the_0 1) u_man_0)))
(predicate (cont_1_0_3) (and (= x_the_0 x_the_0) (= (+ u_the_0 1) u_the_0)))
(predicate (cont_1_1_2) (and (= x_man_0 x_saw_0) (= (+ u_man_0 1) u_saw_0)))
(predicate (cont_1_1_4) (and (= x_man_0 x_man_0) (= (+ u_man_0 1) u_man_0)))
(predicate (cont_1_2_3) (and (= x_saw_0 x_the_0) (= (+ u_saw_0 1) u_the_0)))
(predicate (cont_1_3_4) (and (= x_the_0 x_man_0) (= (+ u_the_0 1) u_man_0)))
(predicate (comp_1_0_1) (or (and (cont_1_0_1) )))
(predicate (comp_1_1_2) (or (and (cont_1_1_2) )))
(predicate (comp_1_2_3) (or (and (cont_1_2_3) )))
(predicate (comp_1_3_4) (or (and (cont_1_3_4) )))
(predicate (comp_1_0_3) (or (and (cont_1_0_1) (comp_1_2_3) )(and (cont_1_0_3) (comp_1_1_2) )))
(predicate (comp_1_1_4) (or (and (cont_1_1_2) (comp_1_3_4) )(and (cont_1_1_4) (comp_1_2_3) )))
(or (and (= x_the_0 0) (comp_1_1_4))(and (= x_saw_0 0) (comp_1_0_1) (comp_1_3_4))(and (= x_man_0 0) (comp_1_0_3) ))

```
これをsugarに投げて成功したら
```
a u_the_0       0
a x_the_0       6
a u_man_0       1
a x_man_0       6
a u_slept_0     0
a x_slept_0     0
a u_saw_0       0
a x_saw_0       0
```
というように帰ってくるのでこれを元に文法を復元する。

復元語は標準出力に入力文とその対応する文法を出力する。markdownで読めそうな感じにして合る。
```the man slept ```

$gg^{r}s$

```the man saw the man```

$gg^{r}sgg^{r}$

といった要領。

探索空間が単語種類^型数オーダーなのでまずい。


# 注 
何もしないと基本型をsとそれ以外の二つに絞る傾向にあるのでこの単語はこの型を持つ！みたいなの指定できたほうが良さそう（未実装）