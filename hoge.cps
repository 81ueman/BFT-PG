(int x_come_0 0 10)
(int u_come_0 -3 3)
(int x_she_0 0 10)
(int u_she_0 -3 3)
(int x_will_0 0 10)
(int u_will_0 -3 3)
(int x_will_1 0 10)
(int u_will_1 -3 3)
(int x_will_2 0 10)
(int u_will_2 -3 3)
(int x_see_0 0 10)
(int u_see_0 -3 3)
(int x_see_1 0 10)
(int u_see_1 -3 3)
(int x_him_0 0 10)
(int u_him_0 -3 3)
(int xcost 0 10)
(nvalue xcost (x_come_0 x_she_0 x_will_0 x_will_1 x_will_2 x_see_0 x_see_1 x_him_0 ))
(int ucost 0 30)
(= ucost (+ (abs u_come_0) (abs u_she_0) (abs u_will_0) (abs u_will_1) (abs u_will_2) (abs u_see_0) (abs u_see_1) (abs u_him_0) ))
(int cost 0 20)
(= cost (+ xcost ucost))
(objective minimize cost)

; 0th sentences
(count 0 (x_she_0 x_will_0 x_will_1 x_will_2 x_come_0 ) eq 1)
(predicate (cont_0_0_1) (and (= x_she_0 x_will_0) (= (+ u_she_0 1) u_will_0)))
(predicate (cont_0_0_3) (and (= x_she_0 x_will_2) (= (+ u_she_0 1) u_will_2)))
(predicate (cont_0_1_2) (and (= x_will_0 x_will_1) (= (+ u_will_0 1) u_will_1)))
(predicate (cont_0_1_4) (and (= x_will_0 x_come_0) (= (+ u_will_0 1) u_come_0)))
(predicate (cont_0_2_3) (and (= x_will_1 x_will_2) (= (+ u_will_1 1) u_will_2)))
(predicate (cont_0_3_4) (and (= x_will_2 x_come_0) (= (+ u_will_2 1) u_come_0)))
(! (cont_0_1_2)) 
(! (cont_0_2_3)) 
(predicate (comp_0_0_1) (or (and (cont_0_0_1) )))
(predicate (comp_0_1_2) (or (and (cont_0_1_2) )))
(predicate (comp_0_2_3) (or (and (cont_0_2_3) )))
(predicate (comp_0_3_4) (or (and (cont_0_3_4) )))
(predicate (comp_0_0_3) (or (and (cont_0_0_1) (comp_0_2_3) )(and (cont_0_0_3) (comp_0_1_2) )))
(predicate (comp_0_1_4) (or (and (cont_0_1_2) (comp_0_3_4) )(and (cont_0_1_4) (comp_0_2_3) )))
(or (and (= x_she_0 0) (comp_0_1_4))(and (= x_will_1 0) (comp_0_0_1) (comp_0_3_4))(and (= x_come_0 0) (comp_0_0_3) ))

; 1th sentences
(count 0 (x_she_0 x_will_0 x_will_1 x_will_2 x_see_0 x_see_1 x_him_0 ) eq 1)
(predicate (cont_1_0_1) (and (= x_she_0 x_will_0) (= (+ u_she_0 1) u_will_0)))
(predicate (cont_1_0_3) (and (= x_she_0 x_will_2) (= (+ u_she_0 1) u_will_2)))
(predicate (cont_1_0_5) (and (= x_she_0 x_see_1) (= (+ u_she_0 1) u_see_1)))
(predicate (cont_1_1_2) (and (= x_will_0 x_will_1) (= (+ u_will_0 1) u_will_1)))
(predicate (cont_1_1_4) (and (= x_will_0 x_see_0) (= (+ u_will_0 1) u_see_0)))
(predicate (cont_1_1_6) (and (= x_will_0 x_him_0) (= (+ u_will_0 1) u_him_0)))
(predicate (cont_1_2_3) (and (= x_will_1 x_will_2) (= (+ u_will_1 1) u_will_2)))
(predicate (cont_1_2_5) (and (= x_will_1 x_see_1) (= (+ u_will_1 1) u_see_1)))
(predicate (cont_1_3_4) (and (= x_will_2 x_see_0) (= (+ u_will_2 1) u_see_0)))
(predicate (cont_1_3_6) (and (= x_will_2 x_him_0) (= (+ u_will_2 1) u_him_0)))
(predicate (cont_1_4_5) (and (= x_see_0 x_see_1) (= (+ u_see_0 1) u_see_1)))
(predicate (cont_1_5_6) (and (= x_see_1 x_him_0) (= (+ u_see_1 1) u_him_0)))
(! (cont_1_4_5)) 
(! (cont_1_1_2)) 
(! (cont_1_2_3)) 
(predicate (comp_1_0_1) (or (and (cont_1_0_1) )))
(predicate (comp_1_1_2) (or (and (cont_1_1_2) )))
(predicate (comp_1_2_3) (or (and (cont_1_2_3) )))
(predicate (comp_1_3_4) (or (and (cont_1_3_4) )))
(predicate (comp_1_4_5) (or (and (cont_1_4_5) )))
(predicate (comp_1_5_6) (or (and (cont_1_5_6) )))
(predicate (comp_1_0_3) (or (and (cont_1_0_1) (comp_1_2_3) )(and (cont_1_0_3) (comp_1_1_2) )))
(predicate (comp_1_1_4) (or (and (cont_1_1_2) (comp_1_3_4) )(and (cont_1_1_4) (comp_1_2_3) )))
(predicate (comp_1_2_5) (or (and (cont_1_2_3) (comp_1_4_5) )(and (cont_1_2_5) (comp_1_3_4) )))
(predicate (comp_1_3_6) (or (and (cont_1_3_4) (comp_1_5_6) )(and (cont_1_3_6) (comp_1_4_5) )))
(predicate (comp_1_0_5) (or (and (cont_1_0_1) (comp_1_2_5) )(and (cont_1_0_3) (comp_1_1_2) (comp_1_4_5) )(and (cont_1_0_5) (comp_1_1_4) )))
(predicate (comp_1_1_6) (or (and (cont_1_1_2) (comp_1_3_6) )(and (cont_1_1_4) (comp_1_2_3) (comp_1_5_6) )(and (cont_1_1_6) (comp_1_2_5) )))
(or (and (= x_she_0 0) (comp_1_1_6))(and (= x_will_1 0) (comp_1_0_1) (comp_1_3_6))(and (= x_see_0 0) (comp_1_0_3) (comp_1_5_6))(and (= x_him_0 0) (comp_1_0_5) ))

;(= x_will_0 1)
;(= u_will_0 1)
;(= x_will_1 0)
;(= u_will_1 0)
;(= x_will_2 2)
;(= x_will_2 -1)