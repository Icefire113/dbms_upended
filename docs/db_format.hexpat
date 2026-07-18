struct header {
    u8 magic[4];
    // Little endian
    u32 ver;
    // Big endian
    u64 hash;
    // Little endian
    u64 nbytes;
    u8 data[nbytes];
};

header h @ 0x00;