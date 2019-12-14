#!/usr/bin/env -S sbcl --script

(defun read-input ()
  (loop for line = (read-line *standard-input* nil)
    while line
    collect line))

(defun positions (str &optional (start 0))
  (loop for x = (position #\# str :test #'equal :start start)
    while x do (setq start (+ 1 x))
    collect x))

(defun asteroid-coordinates (rows)
  (reduce #'append
    (loop for row in rows and y from 0
        collect (mapcar (lambda (x) (list x y)) (positions row)))))

(defun rad-to-degree (a)
  (/ (* a 180.0) pi))

(defun approx (lhs rhs)
  (< (- lhs rhs) 1e-5))

(defun angle (fst snd)
  ; angle, but with some lies
  ; later, asteroids are ordered by angle, and we want up to be lowest (ordered
  ; first). this is made worse by the grid being "upside-down", i.e. y = 0 is
  ; top row, so add 90
  ; actually, add 91, because I couldn't figure out round (359.999 somehow
  ; ended up being 147), but unable to replicate in the repl. Still, the shift
  ; is constant so it doesn't affect correctness, but it makes so that the
  ; rounded-down 360 degree angle is the first element as it should be
  (let* ((x1 (car fst)) (y1 (second fst))
         (x2 (car snd)) (y2 (second snd))
         (real-angle (atan (- y1 y2) (- x1 x2)))
         (degrees (rad-to-degree real-angle)))
    (mod (+ 91 degrees) 360)))

(defun asteroid-angles (src asteroids)
  (loop for rock in asteroids
    collect (angle rock src)))

(defun count-visible (src asteroids)
  (length
    (remove-duplicates
      (asteroid-angles src asteroids))))

(defun try-all (fn asteroids)
  (loop for asteroid in asteroids
    collect
      (list
        asteroid 
        (funcall fn asteroid (remove asteroid asteroids :test #'equal)))))

(defvar asteroid-space (asteroid-coordinates (read-input)))

(defun max-pair (fst snd)
  (let ((lhs (second fst))
        (rhs (second snd)))
    (if (> lhs rhs) fst snd)))

(defvar monitoring-station
  (reduce 'max-pair
    (try-all 'count-visible asteroid-space)))

(print monitoring-station)

(defun pair-to-complex (l)
  (complex (first l) (second l)))

; map asteroids (x,y pairs) to (angle x,y) pairs
(defun angles-between-asteroids (src asteroids)
  ; cheeky, but also make asteroids into complex
  (mapcar #'list
    (asteroid-angles src asteroids)
    (mapcar 'pair-to-complex asteroids)))

; sort (angle x,y) pairs by angle
(defun sort-by-angle (xs)
  (sort xs (lambda (lhs rhs) (< (car lhs) (car rhs)))))

; group angles into (angle (xy1 xy2 xy3 ...))
(defun group-by-angle (xs)
  (reverse
    (reduce
      (lambda (result item)
        (let ((xy (second item))
              (angle (first item))
              (prev-angle (first (first result)))
              (prev-xys (second (first result))))
           (cond
             ((endp result) (list (list angle (list xy))))
             ((approx angle prev-angle)
               (cons (list prev-angle (cons xy prev-xys)) (rest result)))
             (t (cons (list angle (list xy)) result)))))
        xs
        :initial-value '())))

(defun distance (src dst)
  (let* ((diff (- src dst))
         (x (realpart diff))
         (y (imagpart diff)))
    (sqrt (+ (* x x) (* y y)))))

; sort the xy pairs by distance from monitoring station
; larger distance means its line-of-sight is blocked
(defun order-line-of-sight (src asteroid-clusters)
  (mapcar (lambda (line)
    (list
      (first line) ; keep the angle
      (sort (second line) ; sort the coordinates
        (lambda (lhs rhs)
          (< (distance src lhs)
             (distance src rhs))))))
    asteroid-clusters))

; only the *order* matters now, as they're grouped clockwise,
; and by distance
(defun strip-angle (asteroid-clusters)
  (mapcar #'second asteroid-clusters))

(defun flatten (ls)
  (labels ((mklist (x) (if (listp x) x (list x))))
    (mapcan #'(lambda (x) (if (atom x) (mklist x) (flatten x))) ls)))

; flatten by taking the first of each sublist until all sublists are empty
(defun zap-order (asteroid-clusters)
  (flatten
    (loop until (endp asteroid-clusters)
      with heads = (mapcar #'first asteroid-clusters) do
      (setf asteroid-clusters (remove nil (mapcar #'rest asteroid-clusters)))
      collect heads)))

(defvar vaporize-order
  (zap-order
    (strip-angle
      (order-line-of-sight (pair-to-complex (first monitoring-station))
        (group-by-angle
          (sort-by-angle
            (angles-between-asteroids
              (first monitoring-station)
              (remove (first monitoring-station) asteroid-space))))))))

(let ((bet (nth 199 vaporize-order)))
  (print (+ (* 100 (realpart bet)) (imagpart bet))))
