struct header {
    u8 magic[4];
    u32 ver;
    u64 hash;
    u64 nbytes;
    u8 data[nbytes];
};

header h @ 0x00;