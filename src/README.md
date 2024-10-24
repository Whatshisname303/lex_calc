# Overview

Having to look up an online calculator or wait for Matlab to load when I just want to multiply matrices has been really annoying. Also for some reason any calculator that can do those things sucks for just general arithmetic. There definitely is something good out there, but I thought it would be fun to make my own calculator that would be fast and have all the little syntax features that make me like using it.

In the future maybe I rewrite this and make a full TUI app along with better config and stuff, but this works pretty well for right now.

### Installation

Clone the repo:

```
git clone https://github.com/Whatshisname303/lex_calc
```

Run the project:

```
cargo run
```

Everything is done with 0 dependencies which I thought was pretty neat, although might end up using `serde` or something while adding config in the future.

# Syntax

The general syntax is about what you would expect from a calculator. You can type in expressions like `5+5` and it will print out `10`. There is also support for vectors and matrices with a handful of built in functions for each of them. Then you can also define functions in the interpreter for calculating simple repeated values.

```
: 5 + 5    # simple expression
10

: b = 12    # variable assignment
12

: [1; 2; 3] + [4; 5; 6]    # vectors
[5, 7, 9]

: [1 2 3; 4 5 6] * [1 2; 3 4; 5 6]    # matrices
[
	22, 28,
	49, 64,
]

: def add(a, b) a + b
new function

: add(b + 2, 4)
18
```

**Implied ans**

Similar to any normal calculator, if you start a statement with an operator, `ans` will automatically be inserted at the start of the statement. This means that if you just calculated `2*2` and you now want to add `5`, you can just type `+5` which will automatically be converted to `ans + 5`.

```
: 5 + 5
10
: / 2
5
: => var
5
: 15 / var
3
```

**Minus sign**

The `-` operator can act as either a unary operator (making a number negative) or a binary operator (a minus sign). The interpreter prioritizes inserting `ans` whenever it can, so for example, if you type something like `5+5`, this will set `ans` to `10`. Then if your next statement is `-2 + 3`, this will be interpreted as `ans - 2 + 3` setting `ans` to `11` instead of `1`. If `ans` is already `0` though, then you shouldn't notice any difference, and also you could wrap the expression in parentheses `(-2 + 3)` if you really want to do it that way.

```
: 5 + 5
10
: -2 + 3
11
: (-2 + 3)
1
```

### Variables

**Naming**

Variables can be made up of any combination of numbers and letters. Unlike most programming languages, you are allowed to start variables with numbers. So if you want to use `1dog, 2dog...` rather than `dog1, dog2...` for some reason, you can. You can't use any special characters in variable names, and you can't assign variables to words already used in commands so setting something like `clearvars = 5` will not work.

**Reassignment**

There are no static types, and you can reassign variables just by assigning them again. You can reassign the built in constants like `pi` or even the built in functions if you want. This might be useful if scripts or imports are included in the future. There is no way to delete a single variable, although you can delete all variables with the `clearvars` command.

By default, the program is loaded with a few constants like `pi` and `e`  which you can use, however you like. After running `clearvars`, these variables will be reassigned to their default values if you've messed with them at all. In the future there would probably be a config for default variables if you're doing physics or something and are always using the same values, for now though there isn't any way to preserve variables between sessions.

### Data Types

The three included data types are numbers, vectors, and matrices. The syntax for defining vectors and matrices is based on how its done in Matlab since that seemed like a pretty solid syntax, and I haven't used any tools long enough to know if another solution would be better for most use cases.

```
# Defining a number
: a = 5
# Defining a vector
: a = [1; 2; 3]
# Defining a matrix
: a = [1 2 3; 4 5 6; 7 8 9]
```

Semicolons essentially go to the next row when defining a matrix while spaces go to the next column within a row. A matrix with only a single column is a vector.

### User Functions

**Definition**

You can define your own functions to do basic repeated operations. Functions are defined on their own line starting with the keyword `def` followed by the arguments `(a, b, ...)` and then a function body.

```
: def add(a, b) a + b
```

Parentheses are optional, and even commas are optional in the function definition. If you're composing multiple functions though, you need to keep parentheses so that the order of execution is explicit.

```
: def double num num * 2
new function
: double 4
8
: double(double 4)
16
```

**Function Scope**

Functions each have their own scope, so if you have a function with parameters `a` and `b`, while also having globally defined variables for `a` and `b`, then the function will execute using the passed arguments without affecting the global variables. Functions are allowed to access global variables though if they haven't been shadowed by any parameters.

```
: def add(a b) a + b
new function
: a = 5
5
: b = 4
4
: add(2, a)
7
: a
5
: b
4
```

I can't think of a reason this knowledge would be useful, but functions are stored distinctly from variables. So if you want, you can use the same name for a variable and a function.

```
: def y(x) x^2 + 2*x + 1
new function
: y = 3
3
: y(2)
9
: y
3
```

### Built-In Functions

These are hard-coded in and are meant to do repeated stuff that are either unnecessary or impossible to implement with user functions. You can still overwrite them with your own user functions if you want.

TODO: Document current list of functions

### Operators

Operators are about what you would expect, each requiring a value on the left and right (except for the `-` operator which can have just a value on the right). The table shows the order of operations, and the data types you can use each operator with.

| Operator | Priority | Purpose          | Type Implementations          |
| -------- | -------- | ---------------- | ----------------------------- |
| ^        | 1        | Exponents        | `number-number`               |
| *        | 2        | Multiplication   | `number-any`, `matrix-matrix` |
| /        | 2        | Division         | `any-number`                  |
| //       | 2        | Integer Division | `any-number`                  |
| +        | 3        | Addition         | `any-any(same type)`          |
| -        | 3        | Subtraction      | `any-any(same-type)`          |
| =        | 4        | Assignment       | `text-any`                    |
| =>       | 4        | Alt Assignment   | `any-text`                    |

### Commands

Commands do specific things that operate on your current environment. No command has any return value, and so your `ans` variable won't be affected by calling one.

All of them should be pretty intuitive except `mode` which currently only can be used with `mode rad` or `mode deg`, although more modes for setting stuff like display modes might be added.

| Command                    | Purpose                  | Example                               |
| -------------------------- | ------------------------ | ------------------------------------- |
| `clear`                    | Clears the screen        | `clear`                               |
| `mode <setting> <param>`   | Display or update mode   | `mode rad`: sets trig mode to radians |
| `clearvars`                | Reset all user variables | `clearvars`                           |
| `quit`, `exit`, `q`        | Exit the program         | `quit`                                |
| `def <name><params><body>` | Define a user function   | `def add(a, b) a + b`                 |
