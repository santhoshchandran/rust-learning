# Scalar Types
1. Integer Types
	Length	Signed	Unsigned
	8-bit	i8	u8
	16-bit	i16	u16
	32-bit	i32	u32
	64-bit	i64	u64
	128-bit	i128	u128
	arch	isize	usize
In release builds, Rust does not check for overflow, and instead will do something called “two’s complement wrapping.” In short, 256 becomes 0, 257 becomes 1, etc. Relying on overflow is considered an error, even if this behavior happens. 
2. Floating-Point Types - f32 and f64
3. Boolean - bool
4. Character

# Compound Types
	Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.
1. Tuple - A tuple is a general way of grouping together some number of other values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.
```
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```
2. Array - Another way to have a collection of multiple values is with an array. Unlike a tuple, every element of an array must have the same type. Arrays in Rust are different from arrays in some other languages because arrays in Rust have a fixed length, like tuples.
```
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```
