### Expert System, solves some logic things

The algorithm is still incomplete and there are many unused funcs I though would be required.

#### Syntax:
* `A + B` means AND
* `A | B` means OR
* `A ^ B` means XOR
* `!A` means NOT
* `A | B => C + D` means if `A` or `B` are true, then `C` and `D` are also true
* `X ^ (A | B) => !K` means if left side is true, then `K` is certainly false
* `=AJK` describes initial facts, in this case `A`, `J`, `K` are true
* `?XY` says that we want to know whether `X` and `Y` are true

Example of program input and output:
```
A => J ^ K
B => X ^ Y
J + !K => !A | C
C => D
=A
?D
```
```
( true: A  unknown: D )
Using C => D
  ( true: A  unknown: C, D )
  Using C => D
  No match
  Using J + !K => !A | C
    ( true: A  unknown: C, D, J, K )
    Using C => D
    No match
    Using J + !K => !A | C
    No match
    Using A => J ^ K
    2 possible outcomes
    Trying with ( true: J  false: K )
      ( true: A, J  false: K  unknown: C, D )
      Using C => D
      No match
      Using J + !K => !A | C
      3 possible outcomes
      Trying with ( false: A )
      Conflict
      Trying with ( true: C )
        ( true: A, C, J  false: K  unknown: D )
        Using C => D
        1 possible outcome
        Trying with ( true: D )
          ( true: A, C, D, J  false: K )
          Unknown list is empty, returning
Result: ( true: A, C, D, J  false: K )
```
