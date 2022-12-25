#lang racket
;; whatever imports i need for this
(require advent-of-code
         racket/set
         racket/pretty
         (for-syntax advent-of-code
                     racket/base))
;; aoc utilities - some challenges require my personal input
(define current-date (current-aoc-time))
(define current-day (date-day current-date))
(define current-year 2022)
{define-syntax (def-day s)
  ;; config vars, probably won't work
  (define all-days #f)
  (define day-override #f)
  (define current-day (syntax-e (cadr (syntax-e s))))
  (if (or all-days (= current-day (if day-override day-override (date-day (current-aoc-time)))))
      #`{begin
          (printf "== day ~a ==\n" #,current-day)
          (set! input (fetch-aoc-input (find-session) current-year #,current-day #:cache #t))
          #,@(cddr (syntax-e s))
          (void)}
      #'{begin})}
(define input #f)
{define (submit1 v)
  (aoc-submit (find-session) current-year current-day 1 v)}
{define (submit2 v)
  (aoc-submit (find-session) current-year current-day 2 v)}
{define-syntax-rule (dbg v)
  {let ([res v])
    (printf "╭─ ~a\n╰─>> ~a\n" 'v res)
    res}}

;; actual day things (a decent bit of answer determining happens in the interactions panel & is pasted into dbg's)
{def-day 1
  (define data (map {lambda (v) (apply + (map string->number (string-split v "\n")))}
                    (string-split input "\n\n")))
  (dbg (apply max data))
  (dbg (take (sort data >) 3))}
{def-day 2
  (define data (map {lambda (v)
                      (map {lambda (v)
                             (match v
                               ["A" 1]
                               ["B" 2]
                               ["C" 3]
                               ["X" 1]
                               ["Y" 2]
                               ["Z" 3])}
                           (string-split v " "))}
                    (string-split input "\n")))
  {define (score1 v)
    (+ (cadr v) (* 3 (match v
      ['(1 1) 1]
      ['(1 2) 2]
      ['(1 3) 0]
      ['(2 1) 0]
      ['(2 2) 1]
      ['(2 3) 2]
      ['(3 1) 2]
      ['(3 2) 0]
      ['(3 3) 1])))}
  (dbg (apply + (map score1 data)))
  {define (score2 v)
    (match v
      ['(1 1) (+ 3 0)]
      ['(1 2) (+ 1 3)]
      ['(1 3) (+ 2 6)]
      ['(2 1) (+ 1 0)]
      ['(2 2) (+ 2 3)]
      ['(2 3) (+ 3 6)]
      ['(3 1) (+ 2 0)]
      ['(3 2) (+ 3 3)]
      ['(3 3) (+ 1 6)])}
  (dbg (apply + (map score2 data)))}
{def-day 3
  (define data (map {lambda (v)
                      (define all (map {lambda (v)
                             (if (char-lower-case? v)
                                 (+ (char->integer v) #x-60)
                                 (+ (char->integer v) #x-40 26))}
                           (string->list v)))
                      (define-values (a b) (split-at all (/ (string-length v) 2)))
                      #;(set-intersect (list->set a) (list->set b))
                      (list (list->set a) (list->set b) (list->set all))}
                    (string-split input "\n")))
  #;(dbg (apply + (map (compose1 car set->list) data)))
  {define (p2 data)
    {let loop ([out 0]
               [data data])
      (if (null? data)
          out
          (loop (+ out (car (set->list (set-intersect (caddr (car data))
                                                      (caddr (cadr data))
                                                      (caddr (caddr data))))))
                (cdddr data)))}}}
{def-day 4
;  (define input "2-4,6-8\n2-3,4-5\n2-8,3-7\n6-6,4-6")
  (define data (map {lambda (v)
                      (map {lambda (v)
                             (define st (map string->number (string-split v "-")))
                             {for/set ([i (in-inclusive-range (car st) (cadr st))])
                               i}}
                           (string-split v ","))}
                    (string-split input "\n")))
  (dbg (length (filter values (map {lambda (v)
                                     (define i (set-intersect (car v) (cadr v)))
                                     (or (equal? i (car v))
                                         (equal? i (cadr v)))} data))))
  (dbg (length (filter values (map {lambda (v)
                                     (define i (set-intersect (car v) (cadr v)))
                                     (> (set-count i) 0)} data))))}
{def-day 5
  ; easier than trying to parse it :)
  (define towers '((m f c w t d l b)
                   (l b n)
                   (v l t h c j)
                   (w j p s)
                   (r l t f c s z)
                   (z n h b g d w)
                   (n c g v p s m f)
                   (z c v f j r q w)
                   (h l m p r)))
  (define inputs (map {lambda (v)
                        (define s (map string->number (string-split v " ")))
                        (list
                         (list-ref s 1)
                         (sub1 (list-ref s 3))
                         (sub1 (list-ref s 5)))}
                      (cddr (cddddr (cddddr (string-split input "\n"))))))
  {define (do towers reverse)
    {for ([i inputs])
      (define in-list (vector-ref towers (cadr i)))
      (define out-list (vector-ref towers (caddr i)))
      (vector-set! towers (caddr i) (append (reverse (take in-list (car i))) out-list))
      (vector-set! towers (cadr i) (drop in-list (car i)))}
    (map car (vector->list towers))}
  (dbg (do (list->vector towers) reverse))
  (dbg (do (list->vector towers) values))}
{def-day 6
  {define (unique str)
    (equal? (set-count (list->set (string->list str))) (string-length str))}
  {define (test i n input)
    (if (unique (substring input (- i n) i))
        i
        (test (add1 i) n input))}
  (dbg (test 4 4 input))
  (dbg (test 14 14 input))}
{def-day 7
  (define pwd (list))
  (define tree (make-hash))
  {define (pwd-dir)
    {let loop ([dir tree]
               [rem (reverse pwd)])
      (if (null? rem)
          dir
          (loop (hash-ref dir (car rem)) (cdr rem)))}}
  {define (pwd-set! id value)
    (hash-set! (pwd-dir) id value)}
  {for ([i (string-split input "\n")])
    (match (substring i 0 3)
      ["$ c"
       (define dir (substring i 5))
       (set! pwd (match dir
                   ["/" pwd]
                   [".." (cdr pwd)]
                   [_ (cons dir pwd)]))]
      ["$ l" (void)]
      ["dir" (pwd-set! (substring i 4) (make-hash))]
      [_
       (define sp (string-split i " "))
       (pwd-set! (cdr sp) (string->number (car sp)))])}
  {define (part1 t)
    (define size 0)
    (define out (list))
    {for ([(k v) t])
      (if (number? v)
          (set! size (+ size v))
          {let ([res (part1 v)])
            (set! size (+ size (car res)))
            (set! out (append out (cdr res)))})}
    (cons size (if (<= size 100000)
                   (cons size out)
                   out))}
  {define (part2 t)
    (define total-size (car (part1 tree)))
    (define need-remove (- total-size 40000000))
    {define (better? a b)
      (and (< a b) (>= a need-remove))}
    {define (inner t)
      (define size 0)
      (define best 30000000)
      {for ([(k v) t])
        (if (number? v)
            (set! size (+ size v))
            {let ([res (inner v)])
              (set! size (+ size (car res)))
              {when (better? (cdr res) best)
                (set! best (cdr res))}})}
      (cons size (if (better? size best)
                     size
                     best))}
    (inner t)}
  (dbg (apply + (cdr (part1 tree))))
  (dbg (cdr (part2 tree)))}
{def-day 8
  (define grid (map (compose1 (curry map (compose1 string->number string)) string->list) (string-split input "\n")))
  (define w (length grid))
  (define h (length (car grid)))
  {define (at x y)
    (list-ref (list-ref grid y) x)}
  {define (grid-map f)
    (map {lambda (v y)
           (map f v (build-list w values) (build-list w {lambda _ y}))} grid (build-list h values))}
  {define (scan-visible-x mx x y v)
    {let/cc brk
      {for ([i (in-range mx x)])
        {when (>= (at i y) v)
          (brk #f)}}
      #t}}
  {define (scan-visible-y x my y v)
    {let/cc brk
      {for ([i (in-range my y)])
        {when (>= (at x i) v)
          (brk #f)}}
      #t}}
  {define (score-visible--x x y v)
    {let/cc brk
      {for ([i (in-range 0 x)])
        {when (>= (at (- x i 1) y) v)
          (brk (add1 i))}}
      x}}
  {define (score-visible-+x x y v)
    {let/cc brk
      {for ([i (in-range (add1 x) w)])
        {when (>= (at i y) v)
          (brk (- i x))}}
      (- w x 1)}}
  {define (score-visible--y x y v)
    {let/cc brk
      {for ([i (in-range 0 y)])
        {when (>= (at x (- y i 1)) v)
          (brk (add1 i))}}
      y}}
  {define (score-visible-+y x y v)
    {let/cc brk
      {for ([i (in-range (add1 y) h)])
        {when (>= (at x i) v)
          (brk (- i y))}}
      (- h y 1)}}
  (define vis (grid-map {lambda (v x y)
                          (or (scan-visible-x 0 x y v)
                              (scan-visible-x (add1 x) w y v)
                              (scan-visible-y x 0 y v)
                              (scan-visible-y x (add1 y) h v))}))
  (define vis2 (grid-map {lambda (v x y)
                           (* (score-visible--x x y v)
                                 (score-visible-+x x y v)
                                 (score-visible--y x y v)
                                 (score-visible-+y x y v))}))
  (dbg (length (filter values (flatten vis))))
  (dbg (apply max (flatten vis2)))}
{def-day 9
  (define pos (vector (cons 0 0) (cons 0 0) (cons 0 0) (cons 0 0) (cons 0 0) (cons 0 0) (cons 0 0) (cons 0 0) (cons 0 0) (cons 0 0)))
  (define tail-visit (make-hash))
  {define (visit i)
    (hash-set! tail-visit (vector-ref pos i) #t)}
  {define (offset i x y)
    (define tail-pos (vector-ref pos i))
    (vector-set! pos i (cons (+ (car tail-pos) x)
                             (+ (cdr tail-pos) y)))}
  {define (tick-tail i)
    (define head-pos (vector-ref pos (sub1 i)))
    (define tail-pos (vector-ref pos i))
    (define offx (- (car head-pos) (car tail-pos)))
    (define offy (- (cdr head-pos) (cdr tail-pos)))
    {when (or (> (abs offx) 1) (> (abs offy) 1))
      (define xo (cond
                   [(> offx 0) 1]
                   [(< offx 0) -1]
                   [else 0]))
      (define yo (cond
                   [(> offy 0) 1]
                   [(< offy 0) -1]
                   [else 0]))
      (offset i xo yo)}}
  {define (tick-tails)
    (define len (vector-length pos))
    {for ([i (in-range 1 len)])
      (tick-tail i)}
    (visit (sub1 len))}
  {for ([c (string-split input "\n")])
    (define n (string->number (substring c 2)))
    {define (do-loop f)
      {for ([_ (in-range n)])
        (f)}}
    (match (string-ref c 0)
      [#\U (do-loop {lambda ()
                      (offset 0 0 1)
                      (tick-tails)})]
      [#\D (do-loop {lambda ()
                      (offset 0 0 -1)
                      (tick-tails)})]
      [#\R (do-loop {lambda ()
                      (offset 0 1 0)
                      (tick-tails)})]
      [#\L (do-loop {lambda ()
                      (offset 0 -1 0)
                      (tick-tails)})])}
  (dbg (hash-count tail-visit))}
{def-day 10
  (define current-cycle 0)
  (define x 1)
  (define strength 0)
  {define (tick-cycle)
    (set! current-cycle (add1 current-cycle))
    (define sx (modulo current-cycle 40))
    {when (= sx 20)
      (set! strength (+ strength (* current-cycle x)))}
    (if (<= x sx (+ x 2))
      (printf "#")
      (printf " "))
    {when (= sx 0)
      (printf "\n")}}
  {for ([i (string-split input "\n")])
    (match (substring i 0 4)
      ["noop" (tick-cycle)]
      ["addx" (tick-cycle)
              (tick-cycle)
              (set! x (+ x (string->number (substring i 5))))])}
  (dbg strength)}
{def-day 11
  (struct monkey (items op test tr fl ins) #:transparent #:mutable)
  (define max-div 1)
  (define start-monkeys
    {for/list ([i (string-split input "\n\n")])
      (define lines (string-split i "\n"))
      (define arg-val (string->number (substring (list-ref lines 2) 25)))
      (define arg (if arg-val
                      {lambda _ arg-val}
                      values))
      (define test (string->number (substring (list-ref lines 3) 21)))
      (set! max-div (lcm max-div test))
      (monkey (map string->number (string-split (substring (list-ref lines 1) 18) ", "))
              (match (string-ref (list-ref lines 2) 23)
                [#\* {lambda (v) (* v (arg v))}]
                [#\+ {lambda (v) (+ v (arg v))}])
              test
              (string->number (substring (list-ref lines 4) 29))
              (string->number (substring (list-ref lines 5) 30))
              0)})
  (define (dup-monkeys) (map {lambda (v)
                               (monkey (monkey-items v)
                                       (monkey-op v)
                                       (monkey-test v)
                                       (monkey-tr v)
                                       (monkey-fl v)
                                       (monkey-ins v))} start-monkeys))
  (define monkeys (dup-monkeys))
  {define (round div)
    {for ([id (in-range (length monkeys))])
      (define i (list-ref monkeys id))
      (define items (monkey-items i))
      (set-monkey-items! i (list))
      {for ([j items])
        (set-monkey-ins! i (add1 (monkey-ins i)))
        (define worry (floor (/ ((monkey-op i) j) div)))
        (define target (list-ref monkeys (if (= (modulo worry (monkey-test i)) 0)
                                             (monkey-tr i)
                                             (monkey-fl i))))
        (set-monkey-items! target (cons (modulo worry max-div) (monkey-items target)))}
      i}}
  {define (monkey-shit)
    (define v (sort (map monkey-ins monkeys) >=))
    (* (list-ref v 0)
       (list-ref v 1))}
  {for ([_ (in-range 20)])
    (round 3)}
  (dbg (monkey-shit))
  (set! monkeys (dup-monkeys))
  {for ([i (in-range 10000)])
    ;(printf "... ~a\n" i)
    (round 1)}
  (dbg (monkey-shit))
  ;(dbg monkeys)
  }
{def-day 12
  (define raw-grid (map string->list (string-split input "\n")))
  (define height (length raw-grid))
  (define width (length (car raw-grid)))
  (define start-pos #f)
  (define end-pos #f)
  (define grid (map {lambda (v y)
                      (map {lambda (v x)
                             (match v
                               [#\S (set! start-pos (cons x y))
                                    0]
                               [#\E (set! end-pos (cons x y))
                                    25]
                               [c (- (char->integer c) #x61)])}
                           v
                           (build-list width values))} raw-grid (build-list height values)))
  {define (offset pos x y)
    (cons (+ (car pos) x)
          (+ (cdr pos) y))}
  {define (at-pos pos)
    (if (and (< -1 (car pos) width) (< -1 (cdr pos) height))
        (list-ref (list-ref grid (cdr pos)) (car pos))
        99)}
  (define visited (make-hash))
  (define queue (list (cons 0 start-pos)))
  {define (search pos)
    (define this-len (hash-ref visited pos))
    {for ([i (list (offset pos 0 -1)
                   (offset pos 0 1)
                   (offset pos -1 0)
                   (offset pos 1 0))])
      {when (and (< (add1 this-len) (hash-ref visited i 999999))
                 (<= (at-pos i) (add1 (at-pos pos))))
        (hash-set! visited i (add1 this-len))
        (search i)}}}
  ;(hash-set! visited start-pos 0)
  {for ([x (in-range width)])
    {for ([y (in-range height)])
      (define tile (at-pos (cons x y)))
      {when (= tile 0)
        (hash-set! visited (cons x y) 0)}}}
  {for ([x (in-range width)])
    {for ([y (in-range height)])
      (define tile (at-pos (cons x y)))
      {when (= tile 0)
        (printf "search ~a ~a\n" x y)
        (search (cons x y))}}}
  (dbg (hash-ref visited end-pos))}
{def-day 13
  {define (parse-part msg)
    (read (open-input-string
           (apply string (map {lambda (v)
                                (match v
                                  [#\, #\ ]
                                  [v v])}
                              (string->list msg)))))}
  {define (parse-message msg)
    (map parse-part (string-split msg "\n"))}
  {define (compare-trees l r)
    (match (list l r)
      [(list (list a ...) (list b ...))
       {let/cc brk
         {for ([l a]
               [r b])
           (define res (compare-trees l r))
           {unless (equal? res '?)
             (brk res)}}
         (cond
           [(< (length a) (length b)) 'v]
           [(> (length a) (length b)) 'x]
           [else '?])}]
      [(list (list a ...) b) (compare-trees a (list b))]
      [(list a (list b ...)) (compare-trees (list a) b)]
      [(list a b) (cond
                    [(< a b) 'v]
                    [(> a b) 'x]
                    [else '?])])}
  (define messages (map parse-message (string-split input "\n\n")))
  (define sum 0)
  {for ([i (in-range (length messages))]
        [msg messages])
    {when (equal? (compare-trees (car msg) (cadr msg)) 'v)
      (set! sum (+ sum i 1))}}
  (dbg sum)
  {define (tree<? a b)
    (match (compare-trees a b)
      ['v #t]
      ['x #f])}
  (define messages2 (filter {lambda (v)
                              (not (eof-object? v))} (map parse-part (string-split input "\n"))))
  (set! messages2 (append messages2 (list (list (list 2))
                                          (list (list 6)))))
  (define sorted (map list
                      (sort messages2 tree<?)
                      (build-list (length messages2) values)))
  (define k1 (findf {lambda (v)
                     (equal? (car v) (list (list 2)))}
                   sorted))
  (define k2 (findf {lambda (v)
                     (equal? (car v) (list (list 6)))}
                   sorted))
  (dbg k1)
  (dbg k2)
  (dbg (* (add1 (cadr k1)) (add1 (cadr k2))))}
{def-day 14
  {define (parse-segment s)
    (define res (map string->number (string-split s ",")))
    (cons (car res) (cadr res))}
  {define (parse-line l)
    (map parse-segment (string-split l " -> "))}
  (define lines (map parse-line (string-split input "\n")))
  (define grid (make-hash))
  (define maxy 0)
  {for ([line lines])
    (define start (car line))
    {for ([seg (cdr line)])
      (define mixv (min (car start) (car seg)))
      (define maxv (max (car start) (car seg)))
      (define miyv (min (cdr start) (cdr seg)))
      (define mayv (max (cdr start) (cdr seg)))
      (set! maxy (max maxy (+ mayv 2)))
      (if (= (car seg) (car start))
          {for ([y (in-inclusive-range miyv mayv)])
            (hash-set! grid (cons (car seg) y) #t)}
          {for ([x (in-inclusive-range mixv maxv)])
            (hash-set! grid (cons x (cdr seg)) #t)})
      (set! start seg)}}
  {define (offset p x y)
    (cons (+ (car p) x)
          (+ (cdr p) y))}
  {define (at-grid p)
    (if (>= (cdr p) maxy)
        #t
        (hash-ref grid p #f))}
  {define (simulate)
    {let loop ([pos (cons 500 0)])
      (define next (list (offset pos 0 1)
                         (offset pos -1 1)
                         (offset pos 1 1)))
      {let/cc superbrk
        {when (> (cdr pos) 200)
          (superbrk #f)}
        (loop {let/cc brk
                {for ([i next])
                  {unless (at-grid i)
                    (brk i)}}
                (hash-set! grid pos 'sand)
                (superbrk #t)})}}}
  (define res {let loop ([i 0])
         (printf "i: ~a\n" i)
         (if (and (not (at-grid (cons 500 0))) (simulate))
             (loop (add1 i))
             i)})
  (dbg res)
  {for ([y (in-range 200)])
    {for ([x (in-range 400 550)])
      (display (match (list x y (at-grid (cons x y)))
                 [(list 500 0 _) #\+]
                 [(list _ _ #f) #\.]
                 [(list _ _ #t) #\#]
                 [(list _ _ 'sand) #\o]))}
    (newline)}}
{def-day 15
  {define (distance a b)
    (+ (abs (- (car a) (car b)))
       (abs (- (cdr a) (cdr b))))}
  {define beacons (make-hash)}
  {define (parse-sensor l)
    (define v (map string->number (string-split l #rx"[=,:]")))
    (define pos (cons (list-ref v 1)
                      (list-ref v 3)))
    (define beacon (cons (list-ref v 5)
                         (list-ref v 7)))
    (printf "~a,~a,~a,~a\\n" (car pos) (cdr pos) (car beacon) (cdr beacon))
    (hash-set! beacons beacon #t)
    (list pos (distance pos beacon))}
  (define sensors (map parse-sensor (string-split input "\n")))
  (newline)
  (define minx 9999999999)
  (define maxx 0)
  {for ([i sensors])
    (define dst (- (cadr i) (distance (car i) (cons (caar i) 2000000))))
    (set! minx (min minx (- (caar i) dst)))
    (set! maxx (max maxx (+ (caar i) dst)))}
  (define res 0)
  {define (no-sensor pos)
    {let/cc brk
      {when (hash-ref beacons pos #f)
        (brk #f)}
      {for ([i sensors])
        {when (<= (distance (car i) pos) (cadr i))
          (brk #t)}}
      #f}}
  {for ([i (in-inclusive-range minx maxx)])
    {when (no-sensor (cons i 2000000))
      (set! res (add1 res))}}
  (dbg res)}
{def-day 16
  (struct valve (p c))
  {define (parse-line s)
    (define l (string-split s #rx"[ =;]|, "))
    (values (list-ref l 1)
            (valve (string->number (list-ref l 5))
                   (cddddr (cddddr (cdddr l)))))}
  (define valves {for/hash ([i (string-split input "\n")])
                   (parse-line i)})
  (define iter (vector 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0))
  {define (check id v time vis opened hist)
    {define (max-by a b)
      (if (> (car b) (car a))
          b
          a)}
    ;(define indent (build-string (* (hash-count vis) 2) {lambda _ #\ }))
    (define new-hist (cons id hist))
    ;(printf "~acheck ~a ~a ~a\n" indent id time hist)
    (define visited (hash-set vis id #t))
    {define (check-sub time visited opened hist)
      (define m (cons 0 hist))
      {for ([i (valve-c v)])
        {unless (hash-ref visited i #f)
          (define res (check i (hash-ref valves i) (sub1 time) visited opened hist))
          (set! m (max-by m res))}}
      m}
    (define new-opened (hash-set opened id #t))
    (define res (cond
                  [(<= time 0) (cons 0 hist)]
                  [(and (> time 1) (> (valve-p v) 0) (not (hash-ref opened id #f)))
                   (max-by {let ([res (check-sub (sub1 time) (hash) new-opened (cons #f new-hist))])
                             ;(printf "~a.. ~a\n" indent (car res))
                             (cons (+ (car res) (* (sub1 time) (valve-p v)))
                                   (cdr res))}
                           (check-sub time visited opened new-hist))]
                  [else (check-sub time visited opened new-hist)]))
    ;(printf "~a-> ~a\n" indent res)
    {when (> (car res) 0)
      {when (> (car res) (vector-ref iter time))
        (vector-set! iter time (car res))
        (printf "check ~a ~a\n" time res)}}
    res}
  ;(dbg (check "AA" (hash-ref valves "AA") 26 (hash) (hash) (list)))
  ;'(1301 #f "XS" "JC" "XK" "UE" "DC" "NM" "DX" #f "YP" "DX" #f "NM" "VE" #f "YH" "CD" #f "XK" "TP" "RA" "TK" "AA")
  (dbg (check "AA" (hash-ref valves "AA") 26 (hash) (hash "XS" #t
                                                          "YP" #t
                                                          "NM" #t
                                                          "YH" #t
                                                          "XK" #t) (list)))}
{def-day 17
  (define dirs (apply vector (filter (compose1 not null?)
                                     (map {lambda (v) (match v [#\> #f] [#\< #t] [_ '()])}
                                          (string->list input)))))
  (define cur-dir 0)
  (define pieces (vector '((0 . 0) (1 . 0) (2 . 0) (3 . 0))
                         '((1 . 0) (0 . 1) (1 . 1) (2 . 1) (1 . 2))
                         '((0 . 0) (1 . 0) (2 . 0) (2 . 1) (2 . 2))
                         '((0 . 0) (0 . 1) (0 . 2) (0 . 3))
                         '((0 . 0) (1 . 0) (0 . 1) (1 . 1))))
  (define cur-piece 0)
  (define flr -1)
  (define gc-offset 0)
  {define-syntax-rule (next l c)
    {begin0
      (vector-ref l c)
      (set! c (modulo (add1 c) (vector-length l)))}}
  (define grid (make-hash))
  {define (offset a b)
    (cons (+ (car a) (car b))
          (+ (cdr a) (cdr b)))}
  {define (place! p)
    (hash-set! grid p #t)}
  {define (place-all! p pos)
    {for ([i p])
      (define r (offset i pos))
      (set! flr (max flr (cdr r)))
      (place! r)}}
  {define (at pos)
    (if (or (< (car pos) 0) (< (cdr pos) 0) (>= (car pos) 7))
        #t
        (hash-ref grid pos #f))}
  {define (fits? p pos)
    {let/cc brk
      {for ([i p])
        {when (at (offset pos i))
          (brk #f)}}
      #t}}
  (define hist (make-hash))
  {define (draw-grid [y flr] [extra (hash)])
    (if (< y 0)
        (printf "'-------'\n")
        {begin
          (printf "|")
          {for ([x (in-range 7)])
            (display (cond
                       [(hash-ref extra (cons x y) #f) #\@]
                       [(at (cons x y)) #\#]
                       [else #\.]))}
          (printf "|\n")
          (draw-grid (sub1 y) extra)})}
  {define (garbage-collect)
    {when (>= flr 1000)
      (define off (- flr 1000))
      (define old-hash grid)
      (set! grid (make-hash))
      {for ([(p _) old-hash])
        (define out (cons (car p) (- (cdr p) off)))
        {when (>= (cdr out) 0)
          (place! out)}}
      (set! flr (- flr off))
      (set! gc-offset (+ gc-offset off))}}
  {define (simulate-rock)
    (define p (next pieces cur-piece))
    (define final
      {let loop ([pos (cons 2 (+ flr 4))])
        #;(draw-grid (+ (cdr pos) 3) {for/hash ([i (map (curry offset pos) p)])
                                     (values i #t)})
        (define new-pos (if (next dirs cur-dir)
                            (offset pos '(-1 . 0))
                            (offset pos '(1 . 0))))
        (define newer-pos (if (fits? p new-pos) new-pos pos))
        {let ([newerer-pos (offset newer-pos '(0 . -1))])
          (if (fits? p newerer-pos) (loop newerer-pos) newer-pos)}})
    (place-all! p final)}
  ;{for ([i (in-range 2022)])
  ;  (simulate-rock)}
  ;(dbg (add1 flr))
  ;(set! flr -1)
  ;(set! grid (make-hash))
  ;(set! cur-piece 0)
  ;(set! cur-dir 0)
  ;(define prev 0)
  ;(define rep-n
  ;  {let loop ([n 0])
  ;    {when (= (modulo n 10000) 0)
  ;      (garbage-collect)
  ;      (printf "n ~a: ~a + ~a -> ~a ; ~a/~a +~a\n" n flr gc-offset (+ flr gc-offset) cur-dir (vector-length dirs) (modulo (- cur-dir prev) (vector-length dirs)))
  ;      (set! prev cur-dir)}
  ;    ;(define pat (map {lambda (v)
  ;    ;                   {let loop ([y 0])
  ;    ;                     (if (at (cons v (- flr y)))
  ;    ;                         y
  ;    ;                         (loop (add1 y)))}}
  ;    ;                 (build-list 7 values)))
  ;    ;{when (and (hash-ref hist pat #f) (= (- n (hash-ref hist pat)) 1745))
  ;    ;  (printf "cycle! ~a = ~a (~a)\n" (hash-ref hist pat) n (- n (hash-ref hist pat)))}
  ;    (hash-set! hist pat n)
  ;    (simulate-rock)
  ;    (loop (add1 n))})
  ;(dbg rep-n)
  (define off-n 219)
  (define inc-n 1745)
  (define target-n 1000000000000)
  {for ([i (in-range off-n)])
    (simulate-rock)}
  (define low-floor (add1 flr))
  (dbg low-floor)
  {for ([i (in-range inc-n)])
    (simulate-rock)}
  (define step-floor (- (add1 flr) low-floor))
  (define step-r (add1 flr))
  (dbg step-floor)
  (define-values (q r) (quotient/remainder (- target-n off-n) inc-n))
  (define auto-n (+ low-floor (* q step-floor)))
  (dbg auto-n)
  (dbg r)
  {for ([i (in-range r)])
    (simulate-rock)}
  (define r-floor (- (add1 flr) step-r))
  (dbg r-floor)
  (dbg (+ auto-n r-floor))}
{def-day 18
  (define max-p (list 0 0 0))
  (define cubes (make-hash))
  {define (parse-cube i)
    (define l (map string->number (string-split i ",")))
    (set! max-p (map max max-p l))
    (hash-set! cubes l 'rock)
    l}
  (define lst (map parse-cube (string-split input "\n")))
  {define (at p)
    (if (or (< (car p) 0)
            (< (cadr p) 0)
            (< (caddr p) 0)
            (> (car p) (car max-p))
            (> (cadr p) (cadr max-p))
            (> (caddr p) (caddr max-p)))
        'water
        (hash-ref cubes p 'air))}
  (define needs-water (make-hash))
  {for ([x (in-inclusive-range 0 (car max-p))])
    {for ([y (in-inclusive-range 0 (cadr max-p))])
      {for ([z (in-inclusive-range 0 (caddr max-p))])
        {when (equal? (at (list x y z)) 'air)
          (hash-set! needs-water (list x y z) #t)}}}}
  {define (water-iter i)
    (printf "water iter ~a\n" i)
    (define n 0)
    {for ([(p _) needs-water])
      {when (or (equal? (at (map + p (list 0 0 1))) 'water)
                (equal? (at (map + p (list 0 1 0))) 'water)
                (equal? (at (map + p (list 1 0 0))) 'water)
                (equal? (at (map + p (list 0 0 -1))) 'water)
                (equal? (at (map + p (list 0 -1 0))) 'water)
                (equal? (at (map + p (list -1 0 0))) 'water))
        (hash-set! cubes p 'water)
        (hash-remove! needs-water p)
        (set! n (add1 n))}}
    {unless (= n 0)
      (water-iter (add1 i))}}
  (water-iter 0)
  (define surface-area 0)
  {for ([cube lst])
    (define surfaces (length (filter {lambda (v)
                                       (equal? v 'water)}
                                     (map {lambda (v)
                                            (at (map + v cube))}
                                          '((0 0 1)
                                            (0 1 0)
                                            (1 0 0)
                                            (0 0 -1)
                                            (0 -1 0)
                                            (-1 0 0))))))
    (set! surface-area (+ surface-area surfaces))}
  (dbg surface-area)}
{def-day 19
  (printf "day 19 is in ./19")}
{def-day 20
  (define counter -1)
  {define (parse-line l)
    (set! counter (add1 counter))
    (cons (string->number l) counter)}
  (define start-buffer (map parse-line (string-split input "\n")))
  {define (index-of buf proc i [t 0])
    {when (null? buf)
      (error "item not in buffer:" i)}
    (if (equal? (proc (car buf)) i)
        t
        (index-of (cdr buf) proc i (add1 t)))}
  {define (run buffer start-buffer)
    {for ([item start-buffer])
      (define index (index-of buffer values item))
      (define-values (l t) (split-at buffer index))
      (define v (caar t))
      (define f (append l (cdr t)))
      (define of (+ index v))
      (define off of #;(if (or (<= of 0)
                               (>= of (sub1 (length buffer))))
                           (+ of (sgn v))
                           of))
      (define-values (a b) (split-at f (modulo off (sub1 (length buffer)))))
      (set! buffer (append a (cons item b)))
      #;(displayln buffer)}
    (define zero-idx (index-of buffer car 0))
    (define v0 (car (list-ref buffer (modulo (+ zero-idx 1000) (length buffer)))))
    (define v1 (car (list-ref buffer (modulo (+ zero-idx 2000) (length buffer)))))
    (define v2 (car (list-ref buffer (modulo (+ zero-idx 3000) (length buffer)))))
    (cons (+ v0 v1 v2) buffer)}
  (dbg (car (run start-buffer start-buffer)))
  (define starter-buffer (map {lambda (v) (cons (* (car v) 811589153) (cdr v))} start-buffer))
  (define res-buf (cons #f starter-buffer))
  {for ([i (in-range 10)])
    (set! res-buf (run (cdr res-buf) starter-buffer))}
  (dbg (car res-buf))}
{def-day 21
  {define (parse-line-a s)
    (define l (string-split s " "))
    (cons (substring (car l) 0 4)
          (if (= (length l) 2)
              (list #f (string->number (cadr l)))
              (list #f (cadr l) (cadddr l)
                    (match (caddr l)
                      ["+" +]
                      ["-" -]
                      ["*" *]
                      ["/" /]))))}
  {define (parse-line-b s)
    (define l (string-split s " "))
    (define id (substring (car l) 0 4))
    (if (= (length l) 2)
        (format "m_{~a}=~a" id (cadr l))
        (match (caddr l)
          ["+" (format "m_{~a}=m_{~a}+m_{~a}" id (cadr l) (cadddr l))]
          ["-" (format "m_{~a}=m_{~a}-m_{~a}" id (cadr l) (cadddr l))]
          ["*" (format "m_{~a}=m_{~a}\\cdot m_{~a}" id (cadr l) (cadddr l))]
          ["/" (format "m_{~a}=\\frac{m_{~a}}{m_{~a}}" id (cadr l) (cadddr l))]))}
  (define monkeys (make-hash))
  (define res "")
  {for ([i (map parse-line-a (string-split input "\n"))])
    (hash-set! monkeys (car i) (cdr i))}
  {for ([i (map parse-line-b (string-split input "\n"))])
    (set! res (string-append res i "\n"))}
  #;(hash-set! monkeys "humn" (list #f (+ 7560831729513.75)))
  {define (evaluate-monkey id)
    (define val (hash-ref monkeys id))
    (printf "eval ~a: ~a\n" id val)
    {define (eval-slow)
      (if (= (length val) 2)
          (cadr val)
          ((cadddr val) (evaluate-monkey (cadr val))
                        (evaluate-monkey (caddr val))))}
    (if (car val)
        (car val)
        {let ([res (eval-slow)])
          (hash-set! monkeys id (cons res (cdr val)))
          res})}
  (dbg (evaluate-monkey "root"))
  (printf "part2 in desmos ('res' variable)")}
{def-day 22
  (define line-mode #f)
  (define instrs #f)
  (define y 1)
  {define grid-hash (make-hash)}
  (define max-x 0)
  (define bounds-x-hash (make-hash))
  {define (parse-grid s)
    (hash-set! bounds-x-hash y (cons (- (string-length s) (string-length (string-trim s)) -1) (string-length s)))
    (set! max-x (max max-x (add1 (string-length s))))
    {for ([i s] [x (in-range (string-length s))])
      (hash-set! grid-hash (cons (add1 x) y)
                 {match i
                   [#\  'wrap]
                   [#\. 'pass]
                   [#\# 'wall]})}}
  {define (parse-instrs s)
    (define tmp (list))
    (define n 0)
    {for ([i s])
      (match i
        [#\R
         (set! tmp (cons (cons n 1) tmp))
         (set! n 0)]
        [#\L
         (set! tmp (cons (cons n -1) tmp))
         (set! n 0)]
        [(or #\0 #\1 #\2 #\3 #\4 #\5 #\6 #\7 #\8 #\9)
         (set! n (+ (* n 10) (string->number (string i))))])}
         (set! tmp (cons (cons n 0) tmp))
    (set! instrs (list->vector (reverse tmp)))}
  {define (parse-line s)
    (match (list line-mode s)
      [(list #f "") (set! line-mode #t)]
      [(list #f line)
       (parse-grid s)
       (set! y (add1 y))]
      [(list #t "") (void)]
      [(list #t line) (parse-instrs s)])}
  {for ([i (string-split input "\n")])
    (parse-line i)}
  (define max-y y)
  (define grid {for/vector ([y (in-range 1 max-y)])
                 {for/vector ([x (in-range 1 max-x)])
                   (hash-ref grid-hash (cons x y) 'wrap)}})
  {define (vref v i)
    (vector-ref v (sub1 i))}
  {define (at-grid pos)
    (if (or (< (car pos) 1)
            (< (cdr pos) 1)
            (>= (car pos) max-x)
            (>= (cdr pos) max-y))
        'wrap
        (vref (vref grid (cdr pos)) (car pos)))}
  (define bounds-x {for/vector ([y (in-range 1 max-y)])
                     (hash-ref bounds-x-hash y)})
  (define bounds-y {for/vector ([x (in-range 1 max-x)])
                     (cons {let/cc brk
                             {for ([y (in-range 1 max-y)])
                               {unless (equal? (at-grid (cons x y)) 'wrap)
                                 (brk y)}}}
                           {let/cc brk
                             {for ([y (reverse (stream->list (in-range 1 max-y)))])
                               {unless (equal? (at-grid (cons x y)) 'wrap)
                                 (brk y)}}})})
  {define (offset a b)
    (cons (+ (car a) (car b))
          (+ (cdr a) (cdr b)))}
  {define (offsetf p f)
    (match f
      [0 (offset p (cons 1 0))]
      [1 (offset p (cons 0 1))]
      [2 (offset p (cons -1 0))]
      [3 (offset p (cons 0 -1))])}
  (define pos (cons (car (vref bounds-x 1)) 1))
  (define success-facing 0)
  (define facing 0)
  (define part1 #t)
  {define (attempt-goto new-pos inner-facing)
    #;{unless inner-facing
      (printf "-> ~a ~a\n" new-pos facing)}
    {match (at-grid new-pos)
      [(or 'pass 0 1 2 3)
       (set! pos new-pos)
       (set! facing (modulo (+ facing success-facing) 4))
       #t]
      ['wall #f]
      ['wrap #;(printf "wrap!\n")
             (if part1
                 (attempt-goto (match inner-facing
                   [0 (cons (car (vref bounds-x (cdr new-pos))) (cdr new-pos))]
                   [1 (cons (car new-pos) (car (vref bounds-y (car new-pos))))]
                   [2 (cons (cdr (vref bounds-x (cdr new-pos))) (cdr new-pos))]
                   [3 (cons (car new-pos) (cdr (vref bounds-y (car new-pos))))]
                   [#f (error "double recurse!")]) #f)
                 {let ([cell-x (floor (/ (sub1 (car new-pos)) 50))]
                       [cell-y (floor (/ (sub1 (cdr new-pos)) 50))]
                       [px (car new-pos)]
                       [py (cdr new-pos)])
                   {define-syntax-rule (rot n)
                     (set! success-facing n)}
                   #;(printf "wrap ~a ~a ~a\n" new-pos inner-facing (cons cell-x cell-y))
                   (attempt-goto (match (list cell-x cell-y inner-facing)
                                   [(list _ _ #f) (error "double recurse!")]
                                   [(list 1 -1 _) (rot 1) (cons 1 (+ 100 px))]
                                   [(list 2 -1 _) (offset new-pos (cons -100 200))]
                                   [(list 0 0 _) (rot 2) (cons 1 (- 151 py))]
                                   [(list 3 0 _) (rot 2) (cons 100 (- 151 py))]
                                   [(list 0 1 2) (rot -1) (cons (- py 50) 101)]
                                   [(list 0 1 3) (rot 1) (cons 51 (+ px 50))]
                                   [(list 2 1 1) (rot 1) (cons 100 (- px 50))]
                                   [(list 2 1 0) (rot -1) (cons (+ py 50) 50)]
                                   [(list -1 2 _) (rot 2) (cons 51 (- 151 py))]
                                   [(list 2 2 _) (rot 2) (cons 150 (- 151 py))]
                                   [(list -1 3 _) (rot -1) (cons (- py 100) 1)]
                                   [(list 1 3 1) (rot 1) (cons 50 (+ px 100))]
                                   [(list 1 3 0) (rot -1) (cons (- py 100) 150)]
                                   [(list 0 4 _) (offset new-pos (cons 100 -200))]
                                   ) #f)})]}}
  #;{define (show-grid)
    (define res "")
    {for ([x (in-range 1 max-x)])
      {for ([y (in-range 1 max-y)])
        (set! res (string-append res (match (at-grid (cons x y))
                   [0 "^ "]
                   [1 "< "]
                   [2 "v "]
                   [3 "> "]
                   ['pass "  "]
                   ['wall "##"]
                   ['wrap "  "])))}
      (set! res (string-append res "\n"))}
    (display res)}
  {define (hit-da-bricks)
    {for ([i instrs])
      #;(printf "state: ~a ~a\n" pos facing)
      {let/cc brk
        {for ([_ (in-range (car i))])
          (vector-set! (vref grid (cdr pos)) (sub1 (car pos)) facing)
          (set! success-facing 0)
          {unless (attempt-goto (offsetf pos facing) facing)
            (brk)}}}
      #;(show-grid)
      (set! facing (modulo (+ facing (cdr i)) 4))}
    #;(printf "state: ~a ~a\n" pos facing)
    (dbg (list pos facing))
    (dbg (+ (* 1000 (cdr pos)) (* 4 (car pos)) facing))}
  (hit-da-bricks)
  (set! pos (cons (car (vref bounds-x 1)) 1))
  (set! facing 0)
  (set! part1 #f)
  (hit-da-bricks)}
{def-day 23
  (define grid (make-hash))
  {define (parse-line i y)
    {for ([i (string->list i)]
          [x (in-range 999999)])
      {when (equal? i #\#)
        (hash-set! grid (cons x y) 'elf)}}}
  {for ([i (string-split input "\n")]
        [y (in-range 999999)])
    (parse-line i (- y))}
  {define (at pos)
    (hash-ref grid pos 'no)}
  {define (offset a b)
    (cons (+ (car a) (car b))
          (+ (cdr a) (cdr b)))}
  (define I-NW 0)
  (define I-N 1)
  (define I-NE 2)
  (define I-E 3)
  (define I-SE 4)
  (define I-S 5)
  (define I-SW 6)
  (define I-W 7)
  {define (neighbours pos)
    (vector (at (offset pos (cons -1 1)))
            (at (offset pos (cons 0 1)))
            (at (offset pos (cons 1 1)))
            (at (offset pos (cons 1 0)))
            (at (offset pos (cons 1 -1)))
            (at (offset pos (cons 0 -1)))
            (at (offset pos (cons -1 -1)))
            (at (offset pos (cons -1 0))))}
  {define (no? v) (equal? v 'no)}
  {define (elf? v) (equal? v 'elf)}
  {define (prop? v) (equal? v 'prop)}
  {define (cancel? v) (equal? v 'cancel)}
  {define ((gen-check a b c) sides)
    (not (or (elf? (vector-ref sides a))
             (elf? (vector-ref sides b))
             (elf? (vector-ref sides c))))}
  (define current-order (vector (cons (gen-check I-N I-NE I-NW) (cons 0 1))
                                (cons (gen-check I-S I-SE I-SW) (cons 0 -1))
                                (cons (gen-check I-W I-NW I-SW) (cons -1 0))
                                (cons (gen-check I-E I-NE I-SE) (cons 1 0))))
  {define (draw-grid)
    (define min-x 9999)
    (define min-y 9999)
    (define max-x -9999)
    (define max-y -9999)
    {for ([(pos _) grid])
      (set! min-x (min min-x (car pos)))
      (set! min-y (min min-y (cdr pos)))
      (set! max-x (max max-x (car pos)))
      (set! max-y (max max-y (cdr pos)))}
    (define tiles 0)
    {for ([ry (in-inclusive-range min-y max-y)])
      (define y (+ (- max-y ry) min-y))
      {for ([x (in-inclusive-range min-x max-x)])
        (display (match (at (cons x y))
                   ['no (set! tiles (add1 tiles))
                        #\.]
                   ['elf #\#]
                   ['prop #\?]
                   ['cancel #\x]))}
      (newline)}
    tiles}
  {define (simulate)
    ;(printf "simulate\n")
    (define elves-prop (make-hash))
    (define all-prop (make-hash))
    (define done #t)
    {for ([(elf ty) grid] #:when (elf? ty))
      (define sides (neighbours elf))
      #;(printf "for ~a: ~a\n" elf sides)
      (if (= (length (filter {lambda (v) (equal? v 'elf)} (vector->list sides))) 0)
          (void)
          {let/cc brk
            {for ([i current-order])
              {when ((car i) sides)
                #;(printf "go ~a\n" (cdr i))
                (define off (offset elf (cdr i)))
                (match (at off)
                  ['no (hash-set! grid off 'prop)
                       (hash-set! elves-prop elf off)
                       (hash-set! all-prop off #t)]
                  ['elf (void)]
                  ['prop (hash-set! grid off 'cancel)]
                  ['cancel (void)])
                (brk)}}})}
    {for ([(elf off) elves-prop])
      {when (prop? (at off))
        (hash-remove! grid elf)
        (hash-set! grid off 'elf)
        (set! done #f)}}
    {for ([(pos _) all-prop])
      {unless (elf? (at pos))
        (hash-remove! grid pos)}}
    (set! current-order (vector (vector-ref current-order 1)
                                (vector-ref current-order 2)
                                (vector-ref current-order 3)
                                (vector-ref current-order 0)))
    done}
  {for ([i (in-range 10)])
    (simulate)}
  (dbg (draw-grid))
  {let loop ([n 11])
    (if (simulate)
        (dbg n)
        (loop (add1 n)))}}
{def-day 24
  (printf "day 24 is in ./24")}
{def-day 25
  {define (snafu->number s)
    (define res 0)
    {for ([i s])
      (set! res (+ (* res 5) (match i
                               [#\2 2]
                               [#\1 1]
                               [#\0 0]
                               [#\- -1]
                               [#\= -2])))}
    res}
  {define (number->snafu n)
    ; digit but 5
    (define cinqits
      (list->vector
       (cons 0
             {let loop ([out (list)]
                        [in n])
               (if (= in 0)
                   out
                   (loop (cons (remainder in 5) out)
                         (quotient in 5)))})))
    {define (iter)
      (printf "iter\n")
      (define change #f)
      {for ([i (in-range (vector-length cinqits))]
            [c cinqits])
        (match c
          [3 (vector-set! cinqits (sub1 i) (add1 (vector-ref cinqits (sub1 i))))
             (vector-set! cinqits i -2)
             (set! change #t)]
          [4 (vector-set! cinqits (sub1 i) (add1 (vector-ref cinqits (sub1 i))))
             (vector-set! cinqits i -1)
             (set! change #t)]
          [_ (void)])}
      {when change (iter)}}
    (iter)
    (define cl (vector->list cinqits))
    {when (zero? (car cl))
      (set! cl (cdr cl))}
    (apply string (map {lambda (v)
                         (match v
                           [0 #\0]
                           [1 #\1]
                           [2 #\2]
                           [-1 #\-]
                           [-2 #\=])} cl))}
  (define lst (map snafu->number (string-split input "\n")))
  (dbg lst)
  (define sum (apply + lst))
  (dbg sum)
  (dbg (number->snafu sum))}
