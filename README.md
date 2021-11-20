# anyhoo

## About

Auto-complete your functions to use `?` expressions and return `anyhow::Result` with less boilerplate!

## Usage with cargo

For convenience, `anyhoo` reexports `anyhow` as `anyhoo::anyhow`:

```toml
[dependencies]
anyhoo = { git = "https://github.com/tranzystorek-io/anyhoo", features = ["reexport"] }
```

You can opt out of this feature if you're using a particular version of `anyhow`.

## Example

A function that reads a line and parses it to an integer.

Default rusty idiom:

```rs
fn parse_line() -> anyhow::Result<i32> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    let result = input.parse();
    Ok(result)
}
```

With `anyhoo`:

```rs
#[anyhoo::anyhoo]
fn parse_line() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    input.parse()?
}
```
