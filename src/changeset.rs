struct Head {
    old_len: u32,
    new_len: u32,
}

struct Opertion(String);

struct Changeset {
    head: Head,
    ops: Opertion,
    char_bank: String,
}
