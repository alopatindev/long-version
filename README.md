# long-version
Minimalistic version parsing and comparison. Supports non-semver versions, ignores postfixes like alpha/rc/etc.

```rust
assert!("2.0.0".parse::<Version>()? > "1.1.0.0".parse::<Version>()?);
```

## License
MIT/Apache-2.0
