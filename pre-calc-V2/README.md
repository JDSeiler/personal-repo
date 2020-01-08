# Purpose
Pre-Calc was a fun project to work on, and Rust was a fun language to learn.
However, over time I've learned much more about programming. As an exercise,
I've decided to re-write pre-calc, applying all of the things I've learned
to hopefully create a better program.

## Analysis of Pre-Calc V1
As much of a good exercise Pre-Calc was when I first wrote it, it has always had
many problems. Foremost is its input format! The parenthesis heavy prefix
notation is verbose and unintuitive, which makes the end result totally
impractical. Not that you would use this program when opening a Python interpreter
gives you the same thing (and so much more), but that's beside the point.
Prefix notation is not pleasant to type.

Internally there are more problems, including but not limited to:
1. Code duplication
2. Inconsistent style
3. Poor style (deceptive function names, poor variable names)
4. Bad, incorrect, or no documentation
5. Minimal error handling (crashes on a malformed input)

## Considerations for Pre-Calc V2
1. Ditch prefix and parse standard notation (e.g. 2+2, not +2 2)
2. Improve program architecture
3. Improve code style
4. Improve documentation

### Langauge considerations
I was interested in Rust at the time I made this program, so I wrote it in Rust.
However, I have decided to re-write Pre-Calc in Python, two main reasons.

1. Rust's main selling points of memory safety and speed are lost on such a simple program.
2. Python's comparative portability and interpretive nature keeps the program simple. There 
   would be no need to install Rust or compile binaries.

In conclusion, while Rust is a very good language well suited for many tasks, there is no 
discernible benefit to using it in this specific case.

# Structure of Pre-Calc V2
The program will be broken into 3 distinct modules:

## main.py
Responsible for input handling, display of errors, and display of results.

## parser.py
Responsible for turning traditional arithmetic expressions like 2 * (5 - 3) into
postfix expressions like 53-2*.

## evaluator.py
Responsible for evaluating postfix expressions and returning the results.
