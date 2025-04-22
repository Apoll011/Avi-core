While Loop
==========

`while` loops follow C syntax.

`continue` can be used to skip to the next iteration, by-passing all following statements;
`break` can be used to break out of the loop unconditionally.

```rust
let x = 10;

while x > 0 {
    x -= 1;
    if x < 6 { continue; }  // skip to the next iteration
    print(x);
    if x == 5 { break; }    // break out of while loop
}
```


While Expression
----------------

`while` statements can also be used as _expressions_.

The `break` statement takes an optional expression that provides the return value.

The default return value of a `while` expression is `()`.

```js
let x = 0;

// 'while' can be used just like an expression
let result = while x < 100 {
    if is_magic_number(x) {
        // if the 'while' loop breaks here, return a specific value
        break get_magic_result(x);
    }

    x += 1;

    // ... if the 'while' loop exits here, the return value is ()
};

if result == () {
    print("Magic number not found!");
} else {
    print(`Magic result = ${result}!`);
}
```
