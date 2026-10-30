# Enum

```rust
enum Status {
    Success,
    Failure(String),
    Pending(u32),
}
```

```rust
let a = Status::Success;
let b = Status::Failure(
    String::from("Connection refused")
);
let c = Status::Pending(10);
```

Rust 的枚举变体（variant）可以携带数据，而且不同变体可以携带不同类型的数据。

- Success：不携带额外数据。
- Failure(String)：携带一个 String。
- Pending(u32)：携带一个 u32。

三个值的类型都是 Status，但内部存储的具体数据不同。

----

Rust 通常使用 match 对枚举进行模式匹配。

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

fn handle(msg: Message) {
    match msg {
        Message::Quit => {
            println!("Quit");
        }

        Message::Move { x, y } => {
            println!("Move to ({x}, {y})");
        }

        Message::Write(text) => {
            println!("Message: {text}");
        } } }
```

1. match 必须穷尽所有可能的变体。如果忘记处理 Message::Write，编译器就会报错。
也可以使用 _ 匹配所有剩余情况：

    ```rust
    match msg {
        Message::Quit => println!("Quit"),
        _ => println!("Other message"),
    }
    ```

2. 模式匹配可以直接解构数据。

    ```rust
    Message::Move { x, y }
    ```

3. 匹配可能转移所有权

    ```rust
    match msg {
        Message::Write(text) => println!("{text}"),
        _ => {}
    }
    ```

如果 `msg` 是按值匹配的，`text` 会把内部 `String` 移走

## Result

Ruesult is an enum.

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

两个泛型参数：

- T：成功时返回的数据类型。
- E：失败时返回的错误类型。

Result<T, E> 不是两个值，也不是同时包含成功和错误数据的结构体。

一个 Result 实例只能处于 Ok 或 Err 其中一个状态。

```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}
```

----

Use match to deal with Result

```rust
fn main() {
    let result = divide(10, 2);

    match result {
        Ok(value) => {
            println!("Result: {value}");
        }

        Err(error) => {
            println!("Error: {error}");
        }
    }
}
```

----

`?` operator: transmit Error

```rust
use std::fs;
use std::io;

fn load_config() -> Result<String, io::Error> {
    let contents = fs::read_to_string("config.toml");

    match contents {
        Ok(text) => Ok(text),
        Err(error) => Err(error),
    }
}
```

读取成功返回内容，失败则把错误直接交给调用者。

`?` version

```rust
fn load_config() -> Result<String, io::Error> {
    let text = fs::read_to_string("config.toml")?;
    Ok(text)
}
```

`?` 成功时解包数据，失败时提前返回错误.

If success,

`load_config()` returns `OK(text)`, same as no `?` version

If fail,

`load_config()` returns `Err(error)` in advance, same as no `?` version

----

Methods of Result

`unwrap()`: 提取成功值；失败时 panic

`expect("msg)`: 与 unwrap 类似，但提供自定义 panic 信息

----

## Option

```rust
enum Option<T> {
    Some(T),
    None,
}
```

与 Result 的区别是，Option 只表示值是否存在，不包含错误信息。

查找不存在的歌曲通常不是系统错误：

```rust
fn find_song(id: u64) -> Option<String> {
    if id == 123         Some(String::from("Song A"))
    } else {
        None
    }
}
```

而读取音乐文件失败，应当返回 Result：

```rust
fn read_song(path: &str) -> Result<Vec<u8>, std::io::Error> {
    std::fs::read(path)
}
```
