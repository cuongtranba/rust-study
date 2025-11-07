# Learning Notes: Match & Chain Methods

## 📚 MATCH - Pattern Matching

### 1. Basic Match

```rust
match todo.status() {
    TodoListStatus::Pending => {
        println!("Not done");
    }
    TodoListStatus::Completed => {
        println!("Done");
    }
    TodoListStatus::Archived => {
        println!("Archived");
    }
    TodoListStatus::Error { err_code, err_msg } => {
        println!("Error {}: {}", err_code, err_msg);
    }
}
```

**Explanation:**
- `match` checks ALL possible cases
- Must handle ALL enum variants
- Compiler will error if any case is missing

---

### 2. Match with Guard (additional condition)

```rust
match (todo.status(), todo.id()) {
    (TodoListStatus::Completed, id) if id > 2 => {
        println!("Completed and ID > 2");
    }
    _ => {} // Skip other cases
}
```

**Explanation:**
- `if id > 2` is guard - additional condition
- Only matches when BOTH pattern AND guard are true
- `_` = catch-all pattern (catches all remaining)

---

### 3. Match with Tuple

```rust
match (todo.status(), todo.id()) {
    (TodoListStatus::Pending, 1) => println!("First pending"),
    (TodoListStatus::Completed, id) if id % 2 == 0 => println!("Even ID completed"),
    (TodoListStatus::Archived, _) => println!("Archived"),
    _ => {}
}
```

**Explanation:**
- Match multiple values at once with tuple `(a, b)`
- `_` in tuple = ignore that value

---

### 4. Match with OR Pattern

```rust
match todo.status() {
    TodoListStatus::Pending | TodoListStatus::Completed => {
        println!("Active todo");
    }
    _ => {}
}
```

**Explanation:**
- `|` = OR operator in match
- Matches if Pending OR Completed

---

### 5. matches! Macro - Shorthand

```rust
// Instead of:
match todo.status() {
    TodoListStatus::Pending => true,
    _ => false,
}

// Use:
matches!(todo.status(), TodoListStatus::Pending)
```

**Explanation:**
- `matches!()` returns `bool`
- More concise when only need to check true/false
- Commonly used in `.filter()`

---

## 🔗 CHAIN METHODS on Iterator

### Iterator Pipeline:

```
Vec → .iter() → .filter() → .map() → .collect() → Vec
```

---

### 1. iter() - Create Iterator

```rust
let todos = vec![...];
todos.iter()  // Create iterator from &Vec
```

**Explanation:**
- `.iter()` = iterator with references `&T`
- `.iter_mut()` = iterator with mutable references `&mut T`
- `.into_iter()` = iterator with ownership `T`

---

### 2. filter() - Filter Elements

```rust
todos
    .iter()
    .filter(|todo| matches!(todo.status(), TodoListStatus::Pending))
```

**Explanation:**
- Keep elements that meet condition
- Closure returns `bool`
- `|todo|` is closure syntax (like `f=>` in JS)

---

### 3. map() - Transform Elements

```rust
todos
    .iter()
    .map(|todo| format!("✓ {}", todo.title()))
    .collect::<Vec<String>>()
```

**Explanation:**
- Transform each element into another value
- `&TodoListItem` → `String`
- Must `.collect()` to get final result

---

### 4. collect() - Collect results

```rust
let result: Vec<String> = todos
    .iter()
    .filter(...)
    .map(...)
    .collect();  // Iterator → Vec
```

**Explanation:**
- Convert iterator to collection (Vec, HashSet, etc.)
- Needs type annotation or turbofish `::<Vec<_>>`

---

### 5. for_each() - Execute for each element

```rust
todos
    .iter()
    .filter(...)
    .for_each(|todo| println!("{}", todo));
```

**Explanation:**
- Like `for` loop but functional style
- Does NOT return value
- Commonly used for print or side effects

---

### 6. count() - Count elements

```rust
let count = todos
    .iter()
    .filter(|todo| matches!(todo.status(), TodoListStatus::Pending))
    .count();
```

**Explanation:**
- Count number of elements in iterator
- Returns `usize`

---

### 7. any() - Check if ANY element meets condition

```rust
let has_pending = todos
    .iter()
    .any(|todo| matches!(todo.status(), TodoListStatus::Pending));
// true if there's AT LEAST 1 pending
```

**Explanation:**
- Returns `bool`
- Stops immediately when finds element meeting condition
- Equivalent to `||` (OR)

---

### 8. all() - Check if ALL elements meet condition

```rust
let all_completed = todos
    .iter()
    .all(|todo| matches!(todo.status(), TodoListStatus::Completed));
// true if ALL are completed
```

**Explanation:**
- Returns `bool`
- Stops immediately when finds element NOT meeting condition
- Equivalent to `&&` (AND)

---

### 9. find() - Find FIRST element meeting condition

```rust
let first_completed = todos
    .iter()
    .find(|todo| matches!(todo.status(), TodoListStatus::Completed));
// Option<&TodoListItem>
```

**Explanation:**
- Returns `Option<&T>`
- `Some(&todo)` if found
- `None` if not found

---

### 10. take() - Take first N elements

```rust
todos
    .iter()
    .filter(...)
    .take(2)  // Only take first 2
    .for_each(|todo| println!("{}", todo));
```

**Explanation:**
- Limit number of elements
- Commonly used with pagination

---

### 11. enumerate() - Add index

```rust
todos
    .iter()
    .enumerate()  // (index, &todo)
    .filter(|(_, todo)| ...)
    .for_each(|(index, todo)| {
        println!("Index {}: {}", index, todo.title());
    });
```

**Explanation:**
- Converts `&T` → `(usize, &T)`
- `index` starts from 0

---

## 🎯 COMMON PATTERNS

### Pattern 1: Filter + Count
```rust
let pending_count = todos
    .iter()
    .filter(|todo| matches!(todo.status(), TodoListStatus::Pending))
    .count();
```

### Pattern 2: Filter + Map + Collect
```rust
let titles: Vec<String> = todos
    .iter()
    .filter(|todo| todo.id() > 2)
    .map(|todo| todo.title().to_string())
    .collect();
```

### Pattern 3: Filter + Take + ForEach
```rust
todos
    .iter()
    .filter(|todo| matches!(todo.status(), TodoListStatus::Pending))
    .take(5)
    .for_each(|todo| println!("{}", todo));
```

### Pattern 4: Match in Filter
```rust
let active = todos
    .iter()
    .filter(|todo| match todo.status() {
        TodoListStatus::Pending | TodoListStatus::Completed => true,
        _ => false,
    })
    .collect::<Vec<_>>();
```

---

## ⚡ PERFORMANCE TIPS

1. **Iterators are lazy** - Don't compute until calling `.collect()` or `.for_each()`

2. **Zero-cost abstractions** - Iterator chains compile to equivalent code as `for` loop

3. **Use `matches!()` in filter** - More concise and clearer

4. **`any()` and `find()` stop early** - Don't iterate through all if already found

---

## 📖 COMPARISON with JavaScript

| Rust | JavaScript |
|------|------------|
| `.iter()` | (not needed, array has built-in methods) |
| `.filter(\|x\| ...)` | `.filter(x => ...)` |
| `.map(\|x\| ...)` | `.map(x => ...)` |
| `.collect()` | (not needed, automatically returns array) |
| `.for_each(\|x\| ...)` | `.forEach(x => ...)` |
| `.any(\|x\| ...)` | `.some(x => ...)` |
| `.all(\|x\| ...)` | `.every(x => ...)` |
| `.find(\|x\| ...)` | `.find(x => ...)` |
| `matches!(x, Pattern)` | (no equivalent) |

---

## 🔥 KEY DIFFERENCES

### Rust:
```rust
todos
    .iter()           // ✅ Must call .iter() first
    .filter(|x| ...)  // ✅ |x| syntax
    .collect()        // ✅ Must collect() to get Vec
```

### JavaScript:
```javascript
todos
    .filter(x => ...) // ✅ Don't need .iter()
    .map(x => ...)    // ✅ x => syntax
                      // ✅ Automatically returns array
```

---

## 🎓 EXERCISES

Try implementing the following functions:

1. Find all pending todos with title containing "Rust"
2. Count completed todos with even ID
3. Get titles of first 3 pending
4. Check if any todo is archived
5. Transform all todos into format: "ID: TITLE (STATUS)"
