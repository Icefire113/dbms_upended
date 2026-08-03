import type.magic;

struct Header {
    type::Magic<"BM"> magic;
    u16 version;
    u32 num_pages;
    u64 num_keys;
    u64 num_elements;
    u64 root_node_id;
    u64 page_size;
};

Header h @ 0x00;