# **rpn** --- **r**usty **p**something **n**something
> a command-line RPN calculator implemented in Rust

## About
Post-fix notation, also called Reverse Polish Notation (RPN) after its creator Jan Łukasiewicz, is a mathematical notation in which operators (such as `+` or `%`) follow their operands. For example, the infix expression `3 + 4` can be written in RPN as `3 4 +`. For operators with fixed _arity_ (fixed number of operands) this notation uniquely describes the expression without the need for parentheses to distinguish the order of operations.

    [postfix] A B C D * + / == (A (B (C D *) +) /) == [infix] A / (B + (C * D))
    [postfix] A B * C + D / == (((A B *) C +) D /) == [infix] ((A * B) + C) / D
    
**rpn** uses binary operators, i.e. all operations act on two operands. See e.g. [wikipedia](https://en.wikipedia.org/wiki/Reverse_Polish_notation) for more info. 

## Installation
Requires an installation of [Rust](https://www.rust-lang.org/tools/install). Recommended build profile is release:
    
    $ curl https://sh.rustup.rs -sSf | sh         # install Rust
    $ git clone https://github.com/drvog/yat-rs   # clone repository
    $ cd yat-rs                                   # change into source directory
    $ cargo run --release                         # compile and run

## Usage

**rpn** can be started in the root of the cargo directory structure:

    $ cd rpn
    $ cargo run --release

    ____________ _   _ 
    | ___ \ ___ \ \ | |
    | |_/ / |_/ /  \| |
    |    /|  __/| . ` |
    | |\ \| |   | |\  |
    \_| \_\_|   \_| \_/
         Version 0.1.0
    ___________________	 

    rpn> 3 4 +
    ===> 3 4 +
       = 7

From here you can input calculations (in postfix notation, of course) which will be evaluated. If you input a valid expression it will output after the `=`. To exit the interpreter you can enter `quit` or `exit`. The expression `help` will print out some useful explanation. Thanks to the handy [rustyline](https://github.com/kkawakam/rustyline) crate, previous input history can be accessed with the `UP`/`DOWN` keys.

## To Do

- Variable declaration?

## License

[MIT LICENSE](./LICENSE)