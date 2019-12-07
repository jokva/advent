#!/usr/bin/env swipl

increasing([]) :-
    true.

increasing([_]) :-
    true.

increasing([A, B|T]) :-
    A @=< B, increasing([B|T]).


consecutive_digits([]) :-
    false.

consecutive_digits([H, H|_]) :-
    true.

consecutive_digits([A, B|T]) :-
    consecutive_digits([B|T]).

% Pack consecutive duplicates of list elements into sublists
pack([],[]).
pack([X|Xs],[Z|Zs]) :- transfer(X,Xs,Ys,Z), pack(Ys,Zs).

transfer(X,[],[],[X]).
transfer(X,[Y|Ys],[Y|Ys],[X]) :- X \= Y.
transfer(X,[X|Xs],Ys,[X|Zs]) :- transfer(X,Xs,Ys,Zs).

long_consecutive_digits(L) :-
    pack(L, X),
    maplist(length, X, Chunks),
    member(2, Chunks).

password(P) :-
    consecutive_digits(P), increasing(P).

short_password(P) :-
    long_consecutive_digits(P), increasing(P).

valid_passwords(Low, High, R1, R2) :-
    numlist(Low, High, In),
    maplist(number_chars, In, Cands),
    partition(password, Cands, R1, Excluded1),
    partition(short_password, R1, R2, Excluded2).

:- initialization(main, main).
main :-
     valid_passwords(128392, 643281, R1, R2),
     length(R1, Len1),
     length(R2, Len2),
     write(Len1), nl,
     write(Len2), nl.
