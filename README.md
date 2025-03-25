## 25-03-2025:
- Learnt about Error Handling
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