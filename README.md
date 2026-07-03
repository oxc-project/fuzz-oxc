# Fuzz oxc parser

## Using `shift`

```bash
pnpm install
pnpm run start
```

## Using `cargo fuzz`

```bash
cargo install cargo-fuzz
```
### Run

Run fuzzer for the parser, for 15 minutes.

```bash
cd fuzz
rustup default nightly

# JavaScript Parser
cargo +nightly fuzz run --sanitizer none parser -- -only_ascii=1 -max_total_time=900 -timeout=5

# Regular Expression Parser
cargo +nightly fuzz run --sanitizer none regex -- -only_ascii=1 -max_total_time=900 -timeout=5
```

## ❤ Who's [Sponsoring Oxc](https://github.com/sponsors/Boshen)?

<p align="center">
  <a href="https://github.com/sponsors/Boshen">
    <img src="https://raw.githubusercontent.com/Boshen/sponsors/main/sponsors.svg" alt="Our sponsors" />
  </a>
</p>
