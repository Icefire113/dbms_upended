# Details About the B+ Tree implementation

The B+ tree has a file that stores some metadata about the tree including the root node, number of keys, and number of elements, it will always be stored at `{root_path}/bp_meta`.

## Node File Format

The format for the nodes (pages) can be found [in the docs](page_header_format.hexpat).
