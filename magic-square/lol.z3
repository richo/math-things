; This example illustrates basic arithmetic and
; uninterpreted functions

(declare-const magic Int)
(declare-const a1 Int)
(declare-const a2 Int)
(declare-const a3 Int)

(declare-const b1 Int)
(declare-const b2 Int)
(declare-const b3 Int)

(declare-const c1 Int)
(declare-const c2 Int)
(declare-const c3 Int)

; (assert (= magic 6561))

(assert (>= a1 1))
(assert (>= a2 1))
(assert (>= a3 1))
(assert (>= b1 1))
(assert (>= b2 1))
(assert (>= b3 1))
(assert (>= c1 1))
(assert (>= c2 1))
(assert (>= c3 1))

; rows
(assert (= (+ (* a1 a1) (* a2 a2) (* a3 a3)) magic))
(assert (= (+ (* b1 b1) (* b2 b2) (* b3 b3)) magic))
(assert (= (+ (* c1 c1) (* c2 c2) (* c3 c3)) magic))

; columns
(assert (= (+ (* a1 a1) (* b1 b1) (* c1 c1)) magic))
(assert (= (+ (* a2 a2) (* b2 b2) (* c2 c2)) magic))
(assert (= (+ (* a3 a3) (* b3 b3) (* c3 c3)) magic))

; diagonals
; (assert (= (+ (* a1 a1) (* b2 b2) (* c3 c3)) magic))
; (assert (= (+ (* a3 a3) (* b2 b2) (* c1 c1)) magic))

(assert (distinct a1 a2 a3 b1 b2 b3 c1 c2 c3))

(check-sat)
(get-model)

