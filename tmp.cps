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