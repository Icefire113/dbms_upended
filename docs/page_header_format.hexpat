import type.magic;

struct Header {
    // Should just be the bytes: `BN`
    type::Magic<"BN"> magic;
    u16 version;
    u32 data_size;
    // The checksum (xxh3) of the data
    u64 checksum;


    u8 data[data_size];
};

Header h @ 0x00;