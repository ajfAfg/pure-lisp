# Pure LISP

An implementation of 'McCarthy, John. "Recursive functions of symbolic expressions and their computation by machine, part I." _Communications of the ACM_ 3.4 (1960): 184-195.'.

Note that the semantics is in accordance with that paper, but the syntax is unique, similar to Common Lisp (To cut corners).

## Getting started

Run `cargo run` only. Note that it is useful to use [rlwrap](https://github.com/hanslub42/rlwrap) command together.

## Examples

### Values

```
> 'X
X
> '(X Y)
(X Y)
```

### Predicates

```
> (atom 'X)
T
> (atom '(X Y))
F
```

```
> (eq 'X 'X)
T
> (eq 'X 'Y)
F
> (eq '(X Y) '(X Y))
Semantics error
```

### Branch

```
> (cond ('T 'X))
X
> (cond ((atom '(X X)) 'X) ('T 'Y) ('T 'Z))
Y
> (cond ('F (eq '(X X) '(X X))) ('T 'X))
X
> (cond ('F 'X))
Semantics error
```

### List

```
> (car '(X (Y (Z NIL))))
X
> (cdr '(X (Y (Z NIL))))
(Y (Z NIL))
> (cons 'X '(Y (Z NIL)))
(X (Y (Z NIL)))
```

### Lambda Abstraction and Application

```
> (lambda (x) x)
<closure>
> (apply (lambda (x) x) (,'X))
X
> (apply (lambda (f x) (apply f (,x))) (,(lambda (x) x) ,'X))
X
```

### Recursive Lambda Abstraction

```
> (label last (lambda (xs) (cond ((atom (cdr xs)) (car xs)) ('T (apply last (,(cdr xs)))))))
<closure>
> (apply (label last (lambda (xs) (cond ((atom (cdr xs)) (car xs)) ('T (apply last (,(cdr xs))))))) (,'(X (Y (Z NIL)))))
Z
```

## Syntax

Syntax definition like BNF. Terminal symbols are set of strings enclosed in double quotes `"..."`, and non-terminal symbols are set of strings enclosed in angle brackets `<...>`. Curly brackets `{...}` denote zero or more repetitions, and parentheses `(...)` denote grouping.

```bnf
<letter> ::= "A".."Z" | "a".."z"

<ident> ::= (<letter> | "_") {<letter> | "0".."9" | "_" | "'"}

<s-expression> ::= <ident>                                 // Symbol
                | "(" <s-expression> <s-expression> ")"    // Pair

<term> ::= "(" "cond" {"(" <term> <term> ")"} ")"          // Branch
         | "(" "atom" <term> ")"                           // Checking if atom
         | "(" "eq" <term> <term> ")"                      // Checking atom equivalence
         | "(" "car" <term> ")"                            // Taking a head fromm list
         | "(" "cdr" <term> ")"                            // Taking a tail from list
         | "(" "cons" <term> <term> ")"                    // Cons
         | <ident>                                         // Variable
         | "(" "lambda" "(" {<ident>} ")" <term> ")"       // Abstraction
         | "(" "apply" <term> "(" {"," <term>} ")" ")"     // Application
         | "(" "label" <ident> <term> ")"                  // Recursive abstraction
         | "'" <s-expression>                              // Quote
```
