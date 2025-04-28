Standard String Functions
=========================
The following standard methods operate on [strings](../../types/strings-chars.md) (and possibly characters).

| Function                       | Parameter(s)                                                                                                                                    | Description                                                                                                                                                |
| ------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `len` method and property      | _none_                                                                                                                                          | returns the number of characters (**not** number of bytes) in the string                                                                                   |
| `bytes` method and property    | _none_                                                                                                                                          | returns the number of bytes making up the UTF-8 string; for strings containing only ASCII characters, this is much faster than `len`                       |
| `is_empty` method and property | _none_                                                                                                                                          | returns `true` if the string is empty                                                                                                                      |
| `to_blob`                      | _none_                                                                                                                                          | converts the string into an UTF-8 encoded byte-stream and returns it as a [BLOB](../../types/blobs.md).                                                                |
| `to_chars`                     | _none_                                                                                                                                          | splits the string by individual characters, returning them as an [array](../../types/arrays.md)                                                                        |
| `get`                          | position, counting from end if < 0                                                                                                              | gets the character at a certain position (`()` if the position is not valid)                                                                               |
| `set`                          | <ol><li>position, counting from end if < 0</li><li>new character</li></ol>                                                                      | sets a certain position to a new character (no effect if the position is not valid)                                                                        |
| `pad`                          | <ol><li>target length</li><li>character/string to pad</li></ol>                                                                                 | pads the string with a character or a string to at least a specified length                                                                                |
| `append`, `+=` operator        | item to append                                                                                                                                  | adds the display text of an item to the end of the string                                                                                                  |
| `remove`                       | character/string to remove                                                                                                                      | removes a character or a string from the string                                                                                                            |
| `pop`                          | _(optional)_ number of characters to remove, none if ≤ 0, entire string if ≥ length                                                             | removes the last character (if no parameter) and returns it (`()` if empty); otherwise, removes the last number of characters and returns them as a string |
| `clear`                        | _none_                                                                                                                                          | empties the string                                                                                                                                         |
| `truncate`                     | target length                                                                                                                                   | cuts off the string at exactly a specified number of characters                                                                                            |
| `to_upper`                     | _none_                                                                                                                                          | converts the string/character into upper-case as a new string/character and returns it                                                                     |
| `to_lower`                     | _none_                                                                                                                                          | converts the string/character into lower-case as a new string/character and returns it                                                                     |
| `make_upper`                   | _none_                                                                                                                                          | converts the string/character into upper-case                                                                                                              |
| `make_lower`                   | _none_                                                                                                                                          | converts the string/character into lower-case                                                                                                              |
| `trim`                         | _none_                                                                                                                                          | trims the string of whitespace at the beginning and end                                                                                                    |
| `contains`                     | character/sub-string to search for                                                                                                              | checks if a certain character or sub-string occurs in the string                                                                                           |
| `starts_with`                  | string                                                                                                                                          | returns `true` if the string starts with a certain string                                                                                                  |
| `ends_with`                    | string                                                                                                                                          | returns `true` if the string ends with a certain string                                                                                                    |
| `min`                          | <ol><li>first character/string</li><li>second character/string</li><ol>                                                                         | returns the smaller of two characters/strings                                                                                                              |
| `max`                          | <ol><li>first character/string</li><li>second character/string</li><ol>                                                                         | returns the larger of two characters/strings                                                                                                               |
| `index_of`                     | <ol><li>character/sub-string to search for</li><li>_(optional)_ start position, counting from end if < 0, end if ≥ length</li></ol>             | returns the position that a certain character or sub-string occurs in the string, or −1 if not found                                                       |
| `sub_string`                   | <ol><li>start position, counting from end if < 0</li><li>_(optional)_ number of characters to extract, none if ≤ 0, to end if omitted</li></ol> | extracts a sub-string                                                                                                                                      |
| `sub_string`                   | [range](../../variables/ranges.md) of characters to extract, from beginning if ≤ 0, to end if ≥ length                                                          | extracts a sub-string                                                                                                                                      |
| `split`                        | _none_                                                                                                                                          | splits the string by whitespaces, returning an [array](../../types/arrays.md) of string segments                                                                       |
| `split`                        | position to split at (in number of characters), counting from end if < 0, end if ≥ length                                                       | splits the string into two segments at the specified character position, returning an [array](../../arrays) of two string segments                               |
| `split`                        | <ol><li>delimiter character/string</li><li>_(optional)_ maximum number of segments, 1 if < 1</li></ol>                                          | splits the string by the specified delimiter, returning an [array](../../arrays) of string segments                                                              |
| `split_rev`                    | <ol><li>delimiter character/string</li><li>_(optional)_ maximum number of segments, 1 if < 1</li></ol>                                          | splits the string by the specified delimiter in reverse order, returning an [array](../../arrays) of string segments                                             |
| `crop`                         | <ol><li>start position, counting from end if < 0</li><li>_(optional)_ number of characters to retain, none if ≤ 0, to end if omitted</li></ol>  | retains only a portion of the string                                                                                                                       |
| `crop`                         | [range](../../variables/ranges.md) of characters to retain, from beginning if ≤ 0, to end if ≥ length                                                           | retains only a portion of the string                                                                                                                       |
| `replace`                      | <ol><li>target character/sub-string</li><li>replacement character/string</li></ol>                                                              | replaces a sub-string with another                                                                                                                         |
| `chars` method and property    | <ol><li>_(optional)_ start position, counting from end if < 0</li><li>_(optional)_ number of characters to iterate, none if ≤ 0</li></ol>       | allows iteration of the characters inside the string                                                                                                       |

Beware that functions that involve indexing into a [string](../../types/strings-chars.md) to get at individual
[characters](../../types/strings-chars.md), e.g. `sub_string`, require walking through the entire UTF-8 encoded
bytes stream to extract individual Unicode characters and counting them, which can be slow for long
[strings](../../types/strings-chars.md).


Building Strings
----------------

[strings](../../types/strings-chars.md) can be built from segments via the `+` operator.

| Operator                             | Description                                                                                                   |
| ------------------------------------ | ------------------------------------------------------------------------------------------------------------- |
| [string](../../types/strings-chars.md) `+=` item | convert the item into a [string](../../types/strings-chars.md), then append it to the first [string](../../types/strings-chars.md)    |
| [string](../../types/strings-chars.md) `+` item  | convert the item into a [string](../../types/strings-chars.md), then concatenate them as a new [string](../../types/strings-chars.md) |
| item `+` [string](../../types/strings-chars.md)  | convert the item into a [string](../../types/strings-chars.md), then concatenate them as a new [string](../../types/strings-chars.md) |

```rust
let x = 42;

// Build string with '+'
let s = "The answer is: " + x + "!!!";

// Prints: "The answer is: 42!!!"
print(s);
```

### Standard Operators Between Strings and/or Characters

The following standard operators inter-operate between [strings](../../types/strings-chars.md) and/or
[characters](../../types/strings-chars.md).

When one (or both) of the operands is a [character](../../types/strings-chars.md), it is first converted into a
one-character [string](../../types/strings-chars.md) before running the operator.

| Operator  | Description                                                                                        |
| --------- | -------------------------------------------------------------------------------------------------- |
| `+`, `+=` | [character](../../types/strings-chars.md)/[string](../../types/strings-chars.md) concatenation                             |
| `-`, `-=` | remove [character](../../strings-chars.md/[sub-string](../../types/strings-chars.md) from [string](../../types/strings-chars.md) |
| `==`      | equals to                                                                                          |
| `!=`      | not equals to                                                                                      |
| `>`       | greater than                                                                                       |
| `>=`      | greater than or equals to                                                                          |
| `<`       | less than                                                                                          |
| `<=`      | less than or equals to                                                                             |

### Interop with BLOB's

For convenience, when a [BLOB](../../types/blobs.md) is appended to a [string](../../types/strings-chars.md), or vice
versa, it is treated as a UTF-8 encoded byte stream and automatically first converted into the appropriate
[string](../../types/strings-chars.md) value.

That is because it is rarely useful to append a [BLOB](../../types/blobs.md) into a string, but extremely useful
to be able to directly manipulate UTF-8 encoded text.

| Operator  | Description                                                                                             |
| --------- | ------------------------------------------------------------------------------------------------------- |
| `+`, `+=` | append a [BLOB](../../types/blobs.md) (as a UTF-8 encoded byte stream) to the end of the [string](../../types/strings-chars.md) |
| `+`       | concatenate a [BLOB](../../types/blobs.md) (as a UTF-8 encoded byte stream) with a [string](../../types/strings-chars.md)       |


Examples
--------

```rust
let full_name == " Bob C. Davis ";
full_name.len == 14;

full_name.trim();
full_name.len == 12;
full_name == "Bob C. Davis";

full_name.pad(15, '$');
full_name.len == 15;
full_name == "Bob C. Davis$$$";

let n = full_name.index_of('$');
n == 12;

full_name.index_of("$$", n + 1) == 13;

full_name.sub_string(n, 3) == "$$$";
full_name.sub_string(n..n+3) == "$$$";

full_name.truncate(6);
full_name.len == 6;
full_name == "Bob C.";

full_name.replace("Bob", "John");
full_name.len == 7;
full_name == "John C.";

full_name.contains('C') == true;
full_name.contains("John") == true;

full_name.crop(5);
full_name == "C.";

full_name.crop(0, 1);
full_name == "C";

full_name.clear();
full_name.len == 0;
```
