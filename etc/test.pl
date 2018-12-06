push([], X, [X]).
push([A|B], X, [A|C]) :- push(B, X, C).

pop([X], X, []).
pop([X|A], Y, [X|B]) :- pop(A, Y, B).

unshift([], X, [X]).
unshift(A, X, [X|A]).

shift([X|A], X, A).

contains([X|_], X).
contains([_|A], X) :- contains(A, X).

indexOf([X|_], X, 0).
indexOf([_|A], X, I) :- indexOf(A, X, J), I is J + 1.

eq(X, X).
