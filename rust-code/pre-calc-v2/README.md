# Pre-Calc V2
![example-usage](https://raw.githubusercontent.com/JDSeiler/personal-repo/master/rust-code/pre-calc-v2/example.png)
## Purpose
The original Pre-Calc was a fun project to work on, but I was very new to Rust when I wrote it and frankly the code is pretty bad.
V2 was written as a way to "redeem" the original Pre-Calc by fixing some of the problems with the original, and writing it in 
more idiomatic Rust.
## Problems with the Original
As a general note, a lot of the things I will point out here are very picky. The point here is not that these old
'mistakes' were very detrimental to the performance of the program. Rather these 'mistakes' were sloppy and poor
craftsmanship. Moving on:

### Heap allocating a new String for every expression
There is no reason to be allocating every time an expression is read in. In V2 a buffer is created and reused instead.

### Program would break on malformed input
Probably the biggest problem with the original is that there is basically no error handling. Anything wacky would cause
the program to totally crash. One reason V2 has over 3x the LoC of the original is that the logic in V2 is smarter.
The original Pre-Calc is pretty 'dumb' in that it can't explain why something is broken. V2 isn't perfect either,
but it does a much better job of not hard-crashing and explains problems when it can.

### Poor handle on the type system
When I wrote the original, I had a very poor grasp of ownership and types in Rust. I remember struggling late into the night
moving `&`s around trying to get the compiler to be happy. This 'programming by coincidence' (changing it till it works)
resulted in some silly things like cloning the input in `find_operand` because it's passed in as a `&String`. Or as another
example, `find_operand` returns a `Option<(String, usize)>` but I just `unwrap()` the Option! The only reason I did that was
to appease the type system, not because I was actually thinking about error cases. I've since learned much more about Rust 
and the new code is far more idiomatic.

### Obscure logic
The original Pre-Calc is written very much like a script. This isn't surprising since my main language at the time was
Python. There is very little abstraction, and the code is not 'self-documenting' (if you believe in that sort of thing).
The resulting code is dense, difficult to read let alone understand. V2 has more code overall, but it is better structured
and each function is more clear in what it does. There is still room to improve (some of my variable names are still poor),
but V2 is miles ahead of the original.

### find_operand
I'm dedicating an entire section to the operand parsing because it's really bad in the original Pre-Calc. In Python, strings
can be indexed by default, but in Rust this isn't the case (`&str` can be indexed, but not `String`). Instead of writing code
that was idiomatic to Rust, I tried to force it to be like Python by calling `string.chars().nth(n).unwrap()` OVER AND OVER again.
Every time I'm doing this, I'm creating a *brand new* iterator from the source string, and consuming that iterator until I reach
the `n`th element. Suffice it to say, this is hard to read and also amazingly wasteful. There are other silly things like 
declaring `let len = exp.len()` and then using `exp.len()` anyway later in the code. 

V2 parses operands in a much more thoughtful way. It uses a finite-state-machine to parse both operands in one go. This approach
has a *downside* in that per-char processing uses more memory. A `char` is always 4 bytes in Rust, and there is also memory
overhead in containing them inside Vectors (which are heap allocated). However, I think the memory trade-off is worth it
because the resulting code is more efficient in terms of operations, and is easier to read.
