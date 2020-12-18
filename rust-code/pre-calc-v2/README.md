# Pre-Calc V2
![example-usage](https://raw.githubusercontent.com/JDSeiler/personal-repo/master/rust-code/pre-calc-v2/example.png)
## Purpose
The original Pre-Calc was a fun project to work on, but I was very new to Rust when I wrote it and the code has some problems.
V2 was written as a way to re-create the original now that I have become a more experienced programmer, both in Rust specifically
and in general.
## Problems with the Original
After reviewing the old code, these are the things I was least happy with:
### Heap allocating a new String for every expression
There is no reason to be allocating every time an expression is read in. In V2 a buffer is created and reused.

### Program would break on malformed input
The original Pre-Calc had no error handling to speak of. So on bad input the program would totally crash.
If my calculator shut off every time I made a typo I would be pretty mad, so I resolved to fix the crashing in V2.
This new code is far from perfect, but it doesn't hard crash on bad input and gives at least some sort of error message.

### Poor handle on the type system
When I wrote the original, I had a very poor grasp of ownership and types in Rust. I remember struggling late into the night
moving `&`s around trying to get the compiler to be happy. This 'programming by coincidence' (changing it till it works)
resulted in some silly things like cloning the input in `find_operand` because it's passed in as a `&String`. Or as another
example, `find_operand` returns an `Option<(String, usize)>` but I just `unwrap()` the Option! The only reason I did that was
to appease the type system, not because I was actually thinking about error cases. I've since learned much more about Rust 
and the new code is more sensible.

### Obscure logic
The original Pre-Calc is written very much like a script. This isn't surprising since my main language at the time was
Python. There is very little abstraction, and the code is not 'self-documenting' (if you believe in that sort of thing).
The resulting code is dense and difficult to read let alone understand. V2 has more code overall, but it is better structured
and each function is more clear in what it does. There is still room to improve (some of my variable names are still poor),
but V2 is miles ahead of the original.

### find_operand
The `find_operand` function was not good in the original Pre-Calc. The worst offender is my repeated use of `String.chars().nth()` to
accesses specific characters of the String. This creates and consumes and entirely new iterator every time, so it's hugely wasteful.

V2 parses operands in a much more thoughtful way. It uses state based parsing to grab both operands in one go. This approach
has a *downside* in that per-char processing uses more memory. A `char` is always 4 bytes in Rust, and there is also memory
overhead in containing them inside Vectors (which are heap allocated). However, I think the memory trade-off is worth it
because the resulting code is more efficient in terms of operations, and is easier to read.
