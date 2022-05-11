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
