# File Specs

## `db_fmt` file spec

```raw
dbfm: u8[4]
version: u32
// The big endian xxhash3_64 hash of the data_blob
data_hash: u64
data_size: u64
// bitcode encoded DBFormat struct, note that the format of this struct will change between versions
data_blob: u8[data_size]
```
