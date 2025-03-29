## 25-03-2025:
### Error Handling:

- Method Signature:
```
pub fn method_name() -> Result<String, io::Error> {}

// Java equivalent:
pubic void methodName() throws IOException {}
```
- To rethrow the Error to the parent method use "?" symbol
- Error Handling on the source method:
```
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

- "anyhow" crate enhances the error handling by making the return type less verbose when multiple error types needs to be returned and also it provides a "context" method to add additional context to the error before rethrowing it.

- There is one more crate called "thiserror" which is more performance efficient than "anyhow"


## 29-03-2025:

### Ownership:

In Rust, every value has a single owner a any given time. When the owner goes out of scope the value is dropped. (deallocated automatically)

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
```
fn main() {
  let s1 = String::new("hello");
  let s2 = s1.clone(); // Creates a deep copy
  println!("s1: {}, s2: {}", s1, s2); // Both are valid
}
```


### Borrowing & References:

Instead of transferring the ownership, Rust allows borrowing by using references (&).

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


### Slices: Borrowing parts of the data:

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
- &s[string..end] creates a slice reference, which does not own the data.


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


Summary:
- Ownership: Each value has a owner, and when it goes out of scope, it's dropped.
- Move: Assigning a value to another variable transfers the Ownership.
- Clone: Creates a clone copy instead of transferring the ownership.
- Borrowing: Allows temporary references without transferring ownership.
- Immutable borrowing (&T): Mutliple read-only references allowed.
- Mutable Borrowing: (&mut T): Only one mutable reference allowed at a time.
- Slices (&[T] or &str): Allows borrowing parts of the data without ownership transfer.












































