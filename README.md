A calculator made in rust.

To start in GUI mode, run the application with no arguments (or, with '/d:gui').

To start in Console mode, run the application with '/d:console'.

### GUI mode

WIP

### Console mode

Can be executed through the terminal (by running with '/d:Console'), starting a loop allowing the user to enter an expression, and printing the evaluated result.

Additionally:
 - Entering ':exit', the loop will break and the program will exit.
 - Entering ':clear' will clear the screen. 
 - Entering ':clear-hist' will clear the calculator's history.
 - Entering ':clear-mem' will clear the calculator's memory.

#### Accessing History:
----
In an expression, previous results can be accessed using the syntax '\${n}', where '{n}' is represents the result's position in history, with 0 being the most recent.

i.e.: '\$0' would fetch the most recent result, whereas '\$5' would fetch the 5th most recent.

#### Accessing and Assigning to Memory:
----
Additionally, the calculator's memory can be accessed using '\$m{n}', where '{n}' is the index of the stored result. A result can be stored in memory using the syntax '\$mn:{expr}', where '{expr}' is the expression whose result should be stored in memory.

i.e.: '\$m5: 3 * 2' would store '6' in memory at index 5, and '\$m5' would fetch '6' from index of memory.

Memory assignment returns the stored value, so the expression '(\$m0: sqrt(4)) + 6' would result in '8', and '2' would be stored in \$m0.

#### Functions:
----
This calculator has several built-in functions.
  - For a function *f* that takes no arguments, it is executed as *f()*,
    - Some no-argument functions are constants, and can also be executed using their name only, as *f*.
      - For example, the function *PI* can be executed either as *PI()* or *PI*.
  - For a function *f* that takes 1 argument, it is executed as *f(a)*
  - For a function *f* that takes more than one argument it is executed as *f(a, b,...)*
    - A function *f* that takes 2 arguments can also be written in infix notation, as *a f b*

#### List of functions:
----
 - ADD(...a)
   - *a + b + c +...*
 - SUB(...a)
   - *a - b - c -...*
 - MULT(...a)
   - *a \* b \* c \*...*
 - DIV(...a)
   - *a / b / c /...*
 - REM(...a)
   - *a % b % c %...*
 - NEG(a)
   - *-a*
 - FAC(a)
   - *a!*
 - MAX(...a)
   - The greatest value in *a, b, ...*.
 - MIN(...a)
   - The lowest value in *a, b, ...*.
 - MOD(a, b)
   - The euclidean modulo function *a mod b*, returning the remainder of euclidean division of *a* by *b*.
 - CEIL(a)
   - Round *a* up to the nearest integer.
 - FLOOR(a)
   - Round *a* down to the nearest integer.
 - ROUND(a)
   - Round *a* to the nearest integer.
 - FRACT(a)
   - Extract the fractional component of floating point number *a*.
 - SQRT(a)
   - The square root of *a*.
 - EXP(a)
   - Euler's number *e* raised to the *a*th power
 - EXP2(a)
   - 2 raised to the *a*the power.
 - POW(a, b)
   - *a^b*.
 - SIN(a)
   - The sine of *a*.
 - COS(a)
   - The cosine of *a*.
 - TAN(a)
   - The tangent of *a*.
 - ASIN(a)
   - The inverse sine of *a*.
 - ACOS(a)
   - The inverse cosine of *a*.
 - ATAN(a)
   - The inverse tangent of *a*.
 - CSC(a)
   - The cosecant of *a*.
 - SEC(a)
   - The secant of *a*.
 - COT(a)
   - The cotangent of *a*.
 - ACSC(a)
   - The inverse cosecant of *a*.
 - ASEC(a)
   - The inverse secant of *a*.
 - ACOT(a)
   - The inverse cotangent of *a*.
 - SINH(a)
   - The hyperbolic sin of *a*.
 - COSH(a)
   - The hyperbolic cosine of *a*.
 - TANH(a)
   - The hyperbolic tangent of *a*.
 - ASINH(a)
   - The inverse hyperbolic sin of *a*.
 - ACOSH(a)
   - The inverse hyperbolic cosine of *a*.
 - ATANH(a)
   - The inverse hyperbolic tangent of *a*.
 - LOG(a)
   - The log base 10 of *a*.
 - LOGB(a, b)
   - The log base *b* of *a*.
 - LOG2(a)
   - The log base 2 of *a*.
 - LN(a)
   - The log base *e* of *a*, where *e* is Euler's number.
 - FRAND()
   - Returns a random floating point number in the range [0, 1).
 - RFRAND(a, b)
   - Returns a random floating point number in the range [*a*, *b*).
 - RFRANDI(a, b)
   - Returns a random floating point number in the range [*a*, *b*].
 - RAND()
   - Returns a random 32-bit signed integer number.
 - RRAND(a, b)
   - Returns a random integer number in the range [*a*, *b*).
 - RRANDI(a, b)
   - Returns a random integer number in the range [*a*, *b*].
 - SIGN(a)
   - Returns *1* if a is non-negative, and returns *-1* otherwise.
 - COND(a, b, c, d)
   - If *a == b*, returns *c*, and returns *d* otherwise.
 - E
   - Can also be written in function notation as E().
   - Returns Euler's number, *e*.
 - PI
   - Can also be written in function notation as PI().
   - Returns *pi*.

### TODO:
----
  - ~~Report error reasons during parsing.~~ (Done)
  - Implement GUI?
    - Using slint (https://crates.io/crates/slint)