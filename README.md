To create the SeaORM Rust structs automatically with Serde macros included
sea-orm-cli generate entity -o src/models/entities -t <TABLENAME> -s <SCHEME_WITH_TABLE> --with-serde both (meaning serialize & deser.)
