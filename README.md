# jsonld-crawler
Search for json-ld entities on a website and collect them

This utility produces a newline delimited list of jsonld entities matching the given object type from all pages of a website.

To extract 'Product's from 'exmpl.store' you can invoke the utility like so.

```sh
cargo run -- --url http://exmpl.store
```

For more options consult the help page with the `--help` option.

    USAGE:
        crawler [OPTIONS] --url <URL>

    OPTIONS:
        -h, --help                         Print help information
        -o, --output <OUTPUT>              [default: entities.ndjson]
        -T, --object-type <OBJECT_TYPE>    Object @type to search for (e.g. Product)
        -u, --url <URL>                    The domain name to search
        -V, --version                      Print version information
