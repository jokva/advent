#!/usr/bin/env guile
!#

(use-modules (ice-9 rdelim))
(use-modules (srfi srfi-1))

(define (read-program)
  (map string->number
    (string-split (read-line) #\,)))

(define (cartesian-product . lists)
  (fold-right (lambda (xs ys)
                (append-map (lambda (x)
                              (map (lambda (y)
                                     (cons x y))
                                   ys))
                            xs))
              '(())
              lists))

(define (run-op mem pc)
  (let* ((opcode (vector-ref mem (+ pc 0)))
         (op1    (vector-ref mem (+ pc 1)))
         (op2    (vector-ref mem (+ pc 2)))
         (dst    (vector-ref mem (+ pc 3)))
         (fun (if (eq? opcode 99) #nil
              (if (eq? opcode 1) +
              (if (eq? opcode 2) *
              (throw 'bad-opcode))))))
    (if fun (vector-set! mem dst
        (fun (vector-ref mem op1) (vector-ref mem op2))))
    (if fun (run-op mem (+ pc 4)))
    mem))

(define (transmogrify answer)
  (+ (* 100 (list-ref answer 0)) (list-ref answer 1)))

(define (run-program stream)
  (let ((mem (list->vector stream))
        (key 19690720)
        (candidates (cartesian-product (iota 100) (iota 100))))
    (vector-set! mem 1 12)
    (vector-set! mem 2 2)
    (display (vector-ref (run-op (vector-copy mem) 0) 0))
    (newline)
    (let ((consider (lambda (cand)
      (begin
        (vector-set! mem 1 (list-ref cand 0))
        (vector-set! mem 2 (list-ref cand 1))
        (eq? key (vector-ref (run-op (vector-copy mem) 0) 0))))))
    (display (transmogrify (find consider candidates)))
    (newline))))

(run-program (read-program))
