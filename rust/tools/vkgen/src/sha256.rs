// Pention Engine - vkgen/sha256.rs
// Requirement: PN-RND-001
// Decision:    ADR-0009, ADR-0010
//
// SHA-256, implemented from the published algorithm description in
// FIPS PUB 180-4 (Secure Hash Standard), NIST, August 2015, sections 4.1.2,
// 4.2.2, 5.3.3 and 6.2. No third-party crate and no copied implementation;
// see PROVENANCE_LEDGER.md entry S-008.
//
// Why the generator needs a hash at all: the emitted bindings carry the
// sha256 of the registry they were produced from. Without it, "regenerated
// from vk.xml" is an unverifiable claim - the file could have come from any
// registry, or from an edited one.

/// Round constants: the first 32 bits of the fractional parts of the cube
/// roots of the first 64 primes (FIPS 180-4 section 4.2.2).
const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

/// Initial hash value: the first 32 bits of the fractional parts of the
/// square roots of the first 8 primes (FIPS 180-4 section 5.3.3).
const H0: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

/// Returns the SHA-256 digest of `message` as lowercase hexadecimal.
pub fn hex(message: &[u8]) -> String {
    let digest = digest(message);
    let mut out = String::with_capacity(64);
    for byte in digest {
        // Written by hand rather than with a format specifier so the two
        // nibbles are unmistakably in the right order.
        const DIGITS: &[u8; 16] = b"0123456789abcdef";
        out.push(DIGITS[usize::from(byte >> 4)] as char);
        out.push(DIGITS[usize::from(byte & 0x0f)] as char);
    }
    out
}

/// Returns the 32-byte SHA-256 digest of `message`.
pub fn digest(message: &[u8]) -> [u8; 32] {
    let mut state = H0;

    // Padding (FIPS 180-4 section 5.1.1): append 0x80, then zeroes until the
    // length is 56 mod 64, then the message length in bits as a big-endian
    // 64-bit integer.
    let bit_length = (message.len() as u64).wrapping_mul(8);
    let mut tail = Vec::with_capacity(128);
    tail.push(0x80u8);
    while (message.len() + tail.len()) % 64 != 56 {
        tail.push(0);
    }
    tail.extend_from_slice(&bit_length.to_be_bytes());

    // Walk the whole padded message as 64-byte blocks without materialising a
    // copy of the registry, which is several megabytes.
    let whole_blocks = message.len() / 64;
    for index in 0..whole_blocks {
        let block = &message[index * 64..(index + 1) * 64];
        compress(&mut state, block.try_into().expect("64-byte block"));
    }

    let mut remainder = Vec::with_capacity(128);
    remainder.extend_from_slice(&message[whole_blocks * 64..]);
    remainder.extend_from_slice(&tail);
    debug_assert!(remainder.len() % 64 == 0);
    for chunk in remainder.chunks_exact(64) {
        compress(&mut state, chunk.try_into().expect("64-byte block"));
    }

    let mut out = [0u8; 32];
    for (index, word) in state.iter().enumerate() {
        out[index * 4..index * 4 + 4].copy_from_slice(&word.to_be_bytes());
    }
    out
}

/// One application of the compression function (FIPS 180-4 section 6.2.2).
fn compress(state: &mut [u32; 8], block: &[u8; 64]) {
    let mut w = [0u32; 64];
    for index in 0..16 {
        w[index] = u32::from_be_bytes([
            block[index * 4],
            block[index * 4 + 1],
            block[index * 4 + 2],
            block[index * 4 + 3],
        ]);
    }
    for index in 16..64 {
        let s0 = w[index - 15].rotate_right(7)
            ^ w[index - 15].rotate_right(18)
            ^ (w[index - 15] >> 3);
        let s1 = w[index - 2].rotate_right(17)
            ^ w[index - 2].rotate_right(19)
            ^ (w[index - 2] >> 10);
        w[index] = w[index - 16]
            .wrapping_add(s0)
            .wrapping_add(w[index - 7])
            .wrapping_add(s1);
    }

    let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = *state;

    for index in 0..64 {
        let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
        let ch = (e & f) ^ ((!e) & g);
        let temp1 = h
            .wrapping_add(s1)
            .wrapping_add(ch)
            .wrapping_add(K[index])
            .wrapping_add(w[index]);
        let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
        let maj = (a & b) ^ (a & c) ^ (b & c);
        let temp2 = s0.wrapping_add(maj);

        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(temp1);
        d = c;
        c = b;
        b = a;
        a = temp1.wrapping_add(temp2);
    }

    state[0] = state[0].wrapping_add(a);
    state[1] = state[1].wrapping_add(b);
    state[2] = state[2].wrapping_add(c);
    state[3] = state[3].wrapping_add(d);
    state[4] = state[4].wrapping_add(e);
    state[5] = state[5].wrapping_add(f);
    state[6] = state[6].wrapping_add(g);
    state[7] = state[7].wrapping_add(h);
}
