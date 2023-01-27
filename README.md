# Interface

Valera's data interfacing between all services and languages. It uses the
[FlatBuffer](https://github.com/google/flatbuffers) data codec which is more
efficient than JSON and more flexible than protobuf. It requires a bit more
thought to use, but runs 1.5-10x faster than JSON and is 0.7x the size. It also
allows us to have a language-agnostic type schema, which is incredibly useful for
interoperability between services.

## Development

The platform's schema is developed in the [schema](schema) directory. Each directory
is a different scope, such as the [auth](schema/auth) directory which contains:

- [api.fbs](schema/auth/api.fbs) - types for interacting with the auth service. clients and servers
  use these types to communicate with each other
- [kv.fbs](schema/auth/kv.fbs) - key-value types that the service uses for storage
- [webauthn.fbs](schema/auth/webauthn.fbs) - types for the webauthn protocol used for authentication

The schema is then compiled into the languages Valera supports. These are:

- **[Rust](dist/rs)** - most of the platform is Rust, and our hardware runs Rust
- **[TypeScript](dist/ts)** - for our web client
- **[Swift](dist/swift)** - for our iOS apps
- **[Kotlin](dist/kt)** - for our Android apps

It _is_ possible to use interfacing for other languages, but we don't use them
ourselves. You are welcome to try other languages that FlatBuffer supports, such
as C++, C#, Go, Java, Python, and more. To do this, run `flatc` with the language
you want to use, using the input file pattern `**/*.fbs` and the output directory
`dist/<lang>`.

### Compiling the interfaces

```zsh
zsh build.zsh
```

This will compile the schema into the languages listed above. Note that these can
be compiled separately, but the build script will compile them all.

## Usage

### Rust

You can view examples written in Rust in the [examples](examples) directory.

Open its documentation with `cargo doc -p interface --open`. **We recommend reading the Rust docs to understand how to use interface.**

Add as a dependency in your `Cargo.toml`:

```toml
interface = { git = "https://github.com/valeralabs/interface" }
```

Run `cargo update` to update the interface. We are currently not using stable,
versioned releases while in development. Sometimes, `rust-analyzer` will not
update the interface. If this happens, run `cargo clean` and click the `rust-analyzer`
button in the bottom of VSCode to reload the project.

### TypeScript

TypeScript packages are published on GitHub Packages. You'll need to login to
GitHub Packages from `pnpm` to install them using your GitHub credentials.

You can do this by running:

```zsh
pnpm login --registry=https://npm.pkg.github.com
```

Then, add the package to your `package.json`:

```zsh
pnpm add @valeralabs/interface
```
