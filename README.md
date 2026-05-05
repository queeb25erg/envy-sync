# envy-sync

> Sync `.env` files across machines using encrypted remote storage backends.

---

## Installation

**Via Cargo:**

```bash
cargo install envy-sync
```

Or download a pre-built binary from the [releases page](https://github.com/your-username/envy-sync/releases).

---

## Usage

Initialize a new sync configuration in your project:

```bash
envy-sync init
```

Push your local `.env` file to the remote backend:

```bash
envy-sync push
```

Pull the latest `.env` from the remote on another machine:

```bash
envy-sync pull
```

All data is encrypted client-side before leaving your machine. Supported backends include S3, GCS, and a self-hosted option.

**Example workflow:**

```bash
# First machine
envy-sync init --backend s3 --bucket my-env-store
envy-sync push --file .env

# Second machine
envy-sync pull --file .env
```

---

## Configuration

`envy-sync` reads from a `.envy-sync.toml` file in your project root. Run `envy-sync init` to generate one interactively.

---

## Contributing

Pull requests are welcome. Please open an issue first to discuss any major changes.

---

## License

[MIT](LICENSE) © your-username