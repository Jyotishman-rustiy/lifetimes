Perfect 👏 — since your crate implements a **generic, customizable string splitting iterator in Rust**, here’s a clean and professional `README.md` you can put in your repo root:

---

````markdown
# StrSplit — Custom String Split Iterator in Rust

`StrSplit` is a lightweight and extensible Rust implementation of a string-splitting iterator.  
It mimics `str::split`, but allows you to define your own **delimiter behavior** using the `Delimeter` trait.

---

##  Features

- Split strings by either a `char` **or** `&str`
- Custom delimiter support via the `Delimeter` trait
- Skips empty tokens automatically (handles consecutive delimiters)
- Works seamlessly with the standard Rust iterator API
- Fully tested with unit tests

---

## Example Usage

```rust
use strsplit::{StrSplit, Delimeter};

fn main() {
    let haystack = "a b c  d e";
    let parts: Vec<_> = StrSplit::new(haystack, " ").collect();

    println!("{:?}", parts);
    // Output: ["a", "b", "c", "d", "e"]
}
````

---

##  How It Works

### 1. The `StrSplit` Iterator

`StrSplit` walks through the input string (`haystack`), calling `find_next()` on its delimiter each time.

When a delimiter is found, it yields the slice before the delimiter and advances.
When no delimiter is found, it yields the remaining slice and finishes.

### 2. The `Delimeter` Trait

You can implement your own custom splitting logic:

```rust
pub trait Delimeter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}
```

Two built-in implementations are included:

* `impl Delimeter for &str`
* `impl Delimeter for char`

So you can do:

```rust
StrSplit::new("a,b,c", ',');    // split by char
StrSplit::new("key=value", "="); // split by &str
```

---

##  Tests

```bash
cargo test
```

Sample test output:

```
running 3 tests
test until_char_test ... ok
test itworks ... ok
test tail ... ok

test result: ok. 3 passed; 0 failed
```

---

##  Example: Custom Delimiter

You can define a new delimiter by implementing the `Delimeter` trait:

```rust
struct Pipe;

impl Delimeter for Pipe {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find('|').map(|start| (start, start + 1))
    }
}

fn main() {
    let text = "foo|bar|baz";
    let parts: Vec<_> = StrSplit::new(text, Pipe).collect();
    println!("{:?}", parts); // ["foo", "bar", "baz"]
}
```

---

##  Project Structure

```
src/
 ├── lib.rs        # Implementation of StrSplit and Delimeter
 └── tests.rs      # Unit tests (inline in lib.rs)
```

---

##  Running Locally

```bash
git clone https://github.com/<your-username>/<repo-name>.git
cd <repo-name>
cargo test
```

---

