# DataSet Manager

The objective of this project is to create a cli tool to manage the [clothing-dataset-small](https://github.com/alexeygrigorev/clothing-dataset-small) and insert it into a postgres database.

All the commands are strongly attached to the directory structure of the repository. So, if you want to use this tool, you should clone the repository and follow the instructions below.

## Pre-requisites

- Clone the clothing-dataset-small from the original [repository](https://github.com/alexeygrigorev/clothing-dataset-small).

    ```bash
    git clone https://github.com/alexeygrigorev/clothing-dataset-small
    ```

- Have a postgres database running.

## Development

Run the cli tool with the following command:

```shell
cargo run --
```

After the `--` you can pass the arguments to the cli tool. For example:

```shell
cargo run -- --help
```

## Release

To build the release version of the cli tool, run the following command:

```shell
cargo build --release
```

## More information

<!-- TODO: Link the postres initialization documentation here -->
An actual use case explanation of this tool can be found in the app-php-apache project [here]().


## References

- [Random value from enum](https://stackoverflow.com/a/48491021)
- [OsString in Rust](https://doc.rust-lang.org/std/ffi/struct.OsString.html)
