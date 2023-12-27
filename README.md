# Brainfuck 2.0 / bf++
This is an extension of the brainfuck syntax that allows the insertion of code from other files in a recursive way,
simplifying complex scripts.

## How to use
The current syntax (V0.1) is very simple and only adds the ability to insert code from other files in the directory of
the rust code itself, where your main bf code is contained. The insertion happens
between `{}`.

## Examples
One of the example snippets in the main directory is `add.bf` which simply looks like this:
```bf
[->>+>+<<<]>>>[-<<<+>>>]<<[->+>+<<]>>[-<<+>>]<
```
All it does is add two cells, where the first one in on the current pointer, without destroying the original cells.

To insert this file into your code, simply modify the `main.rs` file like this:

```rust
fn main() {
    let code = "some_code{add}some_other_code";
    let _tape = evaluate(compile(code));
}
```
Note: This syntax where you have to manually manipulate the rust is weird and temporary, I will soon replace it with
an option to select a main brainfuck filename when running the interpreter.

Another example is `duplicate`. A bf snippet that copies a cell to the next one, without destroying the original. This
one was very useful when working complex code in the previous versions.

This is also a good use case for the folder system. This is an extremely experimental feature, and **I would urge you
not to expect any sort of backwards compatibility** on it. I've added a duplicate folder, with `1.bf` and `2.bf` as
example snippets that copy the cell below the pointer 1 or 2 cells to the right. We can obviously add as many of
these as we want, and thus create a sort of function with parameter, with the insertion looking like `{duplicate/N}`
where N is the parameter. Again, this is a stupid and temporary way to do this, so don't count on it in the long term.