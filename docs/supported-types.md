# Supported Types

- `int`: Is treated as an `i32`
- `bigint`: Is treated as an `i64`
- `float`: Is treated as an `f32`
- `bigfloat`: Is treated as an `f64`
- `bool`: Is treated as a `bool` (or `u8`)
- `string`: Is treated as a `String`
  - Note that fixed length strings are not supported (yet)

## Operators

For all supported math operators, the general rule for the resulting type is that if they both are the same type, then the result is the same type, otherwise the result is the type of the larger operand (i.e. `int` + `bigint` = `bigint`, `bigint` + `bigfloat` = `bigfloat`, `float` + `bigfloat` = `bigfloat`). Additionally, the result between integer and floating point types is always floating point with the same larger operand rule still applying, (i.e. `bigint` + `float` = `bigfloat`).
