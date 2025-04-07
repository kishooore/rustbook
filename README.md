# Study Log

## 25-03-2025

### Error Handling

- Method Signature:

```rust
pub fn method_name() -> Result<String, io::Error> {}

// Java equivalent:
pubic void methodName() throws IOException {}
```

- To re throw the Error to the parent method use "?" symbol
- Error Handling on the source method:

```rust
match some_operation() {
    Ok() => { // equal to try block in java}
    Err(e) => { // equal to catch block in java}
}

// Java equivalent
try {
    someOperation();
} catch (Exception e) {

}

```

- "anyhow" crate enhances the error handling by making the return type less
verbose when multiple error types needs to be returned and also it provides
a "context" method to add additional context to the error before
re-throwing it.

- There is one more crate called "thiserror" which is more performance
efficient than "anyhow"

## 29-03-2025

### Ownership

In Rust, every value has a single owner a any given time. When the owner
goes out of scope the value is dropped. (deallocated automatically)

Rules of Ownership:

1. Each value has exactly one owner
2. When the owner goes out of scope, the value is dropped.
3. Ownership can be transferred(moved) to another variable.

Example-1: Ownership Transfer (moved)

```rust
fu main() {
  let s1 = String::new("hello");
  let s2 = s1; // Ownership moves from s1 to s2
  println!("{}", s2); // ERROR: s1 is nolonger valid
}
```

Explanation:

- String is stored in heap, and s1 owns the heap memory.
- When assigning s1 to s2, ownership moves to s2, and s1 becomes invalid.

Example-2: Cloning (Deep copy)

```rust
fn main() {
  let s1 = String::new("hello");
  let s2 = s1.clone(); // Creates a deep copy
  println!("s1: {}, s2: {}", s1, s2); // Both are valid
}
```

### Borrowing & References

Instead of transferring the ownership, Rust allows borrowing
by using references (&).

Rules of Borrowing:

1. You can have multiple immutable (&T) references.
2. You can have only one mutable (&mut T) reference any any time.
3. References must always be valid.

Example-3: immutable borrowing:

```rust
fn main() {
  let s = String::new("hello");
  
  let n = caluculate_length(&s); // Borrow s without moving it
  println!("The length of {} is {}", s, n);
}

fn caluculate_length(s: &String) -> usize {
  s.len()
}
```

- &s is immutable reference: s is not moved and remains usable
- Immutable references do not allow modification of the borrowed value.

Example-4: Mutable borrowing:

```rust
fn main() {
  let s = String::from("hello");
  
  change(&mut s); // Borrow s mutably
  println!("{}", s);
}

fn change(s: &mut String) {
  s.push_str(", world");
}
```

- &mut s is a mutable reference, it allows modifying.
- You can have only one mutable reference at a time to prevent data races.

Example-5: Multiple immutable references allowed:

```rust
fn main() {
  let s = String::from("hello");

  let r1 = &s;
  let r2 = &s;

  println!("{} and {}", r1, r2);
}
```

Example-6: Mutable and immutable references together (Not allowed):

```rust
fn main() {
  let s = String::from("hello");

  let r1 = &s;
  let r2 = &s;
  let r3 = &mut s; // ERROR: Cannot have mutable reference while immutable reference exist.

  println!("{}, {}, {}", r1, r2, r3);
}
```

Explanation:

- if r3 modifies s, r1 and r2 could be invalid, leanding to undefined behaviour.

Fixing above example:

```rust
fn main() {
  let s = String::from("hello");

  let r1 = &s;
  let r2 = &s;
  
  println!("{} and {}", r1, r2); // Use immutable references first.

  let r3 = &mut s; // Now mutable reference is fine.

  println!("{}", r3);
}

```

- Immutable references must go out of scope before a mutable reference is created.

### Slices: Borrowing parts of the data

- Slices allow borrowing of parts of the data

Example-7: String slices

```rust
fn main() {
  let s = String::from("hello, world");

  let hello = &s[0..5];
  let world = &s[6..];

  println!("{}, {}", hello, world);
}
```

- `&s[string..end]` creates a slice reference, which does not own the data.

Example-8: Preventing dangling references:

- Rust prevents dangling references automatically

```rust
fn dangle() -> &String { // ERROR: returns reference to dropped value
  let s = String::from("hello");
  &s //s is dropped after the function ends
}
```

Fix: Return the owned value instead:

```rust
fn no_dangle() -> String {
  let s = String::from("hello");
  s // Ownership moves to caller
}
```

### Summary

- Ownership: Each value has a owner, and when it goes out of scope, it's dropped.
- Move: Assigning a value to another variable transfers the Ownership.
- Clone: Creates a clone copy instead of transferring the ownership.
- Borrowing: Allows temporary references without transferring ownership.
- Immutable borrowing (&T): Mutliple read-only references allowed.
- Mutable Borrowing: (&mut T): Only one mutable reference allowed at a time.
- Slices (&[T] or &str): Allows borrowing parts of the data without ownership transfer.

## 30-03-2025

### Lifetimes

- Lifetimes prevent dangling references and use-after-free errors enforcing strict
  borrowing at compile time.
- Lifetimes are annotations that describe how long references are valid.

Example:

```rust
fn dangling() -> &String {
  let s = String::from("hello");
  &s  // ERROR: s goes out of scope here, but its reference is returned
}
```

- Lifetime are represented with generic annotations lie `'a` They appear on function
  signature where references are used.

```rust
fn longest<'a'>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() { x } else { y }
}
```

Here:

- `'a` is a generic lifetime
- `x` and `y` must have same lifetime `'a`
- The returned reference must not outlive either `x` or `y`

```rust
fn main() {
  let s1 = String::from("long");
  let s2 = String::from("short");

  let result = longest(s1, s2);
  println!("result: {}", result);
}
```

Elision Rules (Omitting explicit lifetime annotations)

Rust applies Elision rules to simplify function signature

1. Each reference parameter gets its own lifetime.
2. If there is exactly one reference parameter, its lifetime is assigned to the
   return type.
3. If `self` or `&self` is present, the return type uses the lifetime of `self`.

Example: (without explicit lifetimes):

```rust
fn first_word(s: &str) -> &str {
  &s[..s.find(' ').unwrap_or(s.len())]
}
```

- This is valid because Rust automatically assigns lifetimes.

Structs with Lifetimes:

- when a Struct holds reference, it needs explicit lifetimes:

```rust
struct Book<'a> {
  title: &'a str,
}


fn main() {
  let name = String::from("Rust Book");
  let book = Book { title: &name}; // book cannot outlive name
}
```

Without `'a` `Book` might outlive `name`, causing an invalid reference.

Static Lifetimes:

- The `'static` lifetime means the reference lives for the entire program.

```rust
let s: &'statis str = "Hello, World";
```

- String literals always have `'static` because they're stored in binary.

Lifetime Subtyping:

```rust
fn foo<'short, 'long: 'short>(x: &'long str) -> &'short str {
  x
}
```

- Here `'long: 'short` means `'long` outlives `'short`.

### Traits

1. What is Trait?
   A Trait defines a set of methods that a type must implement
   it models capabilities or behaviour, not category or identity.

   ```rust
   trait Speak {
     fn speak(&self) -> String;
   }
   ```

2. Why `&self` is used in trait methods?
   It allows immutable access to the instance data,
   without taking ownership.

   - `&self` -> read-only access
   - `&mut self` -> mutable access
   - `self` -> takes ownership

3. Why are Traits and method names often the same?
   It's a convention for clarity, not a rule.
   - `trait Greet { fn greet(&self) -> String; }` is idiomatic
   - Trait = capability-noun, method = verb
   - They can have different names

4. Can Traits have multiple methods?
   Yes -- Traits can group related methods under a single capability.

   ```rust
   trait Drawable {
     fn draw(&self);
     fn resize(&self, w: u32, h: u32);
   }
   ```

   Use When:
   - Methods are tightly related.
   - Represents a single logical behaviour.

5. Isn't `Animal` a category, not a trait?
   Correct -- `Animal` is a category (noun), not a capability.

   ```rust
   trait Eat { fn eat(&self); }
   trait Sleep { fn sleep(&self); }
   trait Move { fn move(&self); }

   trait Animal: Eat + Sleep + Move {}  // Composed trait
   ```

6. Is `trait Animal: Eat + Sleep + Move {}` a design/code smell?
   Not if used as a capability group, not to mimic class inheritance.
   Use cases:
   - Semantic clarity(`T: Animal`) means it can eat/sleep/move
   - Groups reusable behaviour
   - Cleaner trait bounds in generics

   Avoid:
   - Forcing it as a base class
   - Stuffing unrelated methods

### Generics + Trait Bounds

1. What are generics?
   Generics allow you to write type agnostic code.

   ```rust
   fn identity<T>(value: T) -> T {
     value
   }
   ```

2. Adding Trait bounds to Generics
   you often wants to restrict `T` only types that implement certain traits.

   ```rust
   fn print_something(T: std::fmt::Display)(item: T) {
     println!("{}", item);
   }
   ```

3. Multiple Trait Bounds

   ```rust
   fn act<T: Speak + Eat>(thing: T) {
     thing.speak();
     thing.eat();
   }
   ```

4. `where` clause syntax
   Same as above, but cleaner for complex bounds:

   ```rust
   fn act<T>(thing: T)
   where
     T: Speak + Eat,
   {
     thing.speak();
     thing.eat();
   }
   ```

5. Trait Bounds on Structs

   ```rust
   struct Wrapper<T: Display> {
     value: T
   }
   ```

   Or with where

   ```rust
   struct Wrapper<T>
   where 
     T: Display
   {
     value: T
   }
   ```

6. Return type with Trait (`impl Trait`)

   ```rust
   fn get_speaker() -> impl Speak {
     Dog {}
   }
   ```

7. Generic Trait implementations

   ```rust
   impl<T: Display> Wrapper<T> {
     fn print(&self) {
       println!("{}", self.value);
     }
   }
   ```
