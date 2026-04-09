# Basic Mining

This is the current Rust rewrite flow.

## Initialize Project Config

```bash
cargo run -p multipass-cli -- init /path/to/project
```

This writes `multipass.yaml` in the project root.

## Mine The Project Into A Ship

```bash
cargo run -p multipass-cli -- --ship /path/to/ship mine /path/to/project
```

## Search Records

```bash
cargo run -p multipass-cli -- --ship /path/to/ship search Auth --limit 5
```

## Render A Wake-Up Brief

```bash
cargo run -p multipass-cli -- --ship /path/to/ship wake-up
```

## Notes

- this reflects the Rust rewrite, not the legacy Python/Chroma flow
- the project contract is `multipass.yaml`
- the local store is a SQLite-backed ship
