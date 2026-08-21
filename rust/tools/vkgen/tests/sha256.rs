// Pention Engine - vkgen sha256 tests
// Requirement: PN-RND-001
//
// The vectors below are the published examples in FIPS PUB 180-4 (Appendix B
// and the NIST example documents). They are the reason to trust this
// implementation: a hash that is merely self-consistent verifies nothing.

use vkgen::sha256;

#[test]
fn the_empty_message_matches_the_published_digest() {
    assert_eq!(
        sha256::hex(b""),
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
}

#[test]
fn the_one_block_vector_matches_the_published_digest() {
    // FIPS 180-4 Appendix B.1: "abc".
    assert_eq!(
        sha256::hex(b"abc"),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
}

#[test]
fn the_two_block_vector_matches_the_published_digest() {
    // FIPS 180-4 Appendix B.2: 448 bits, so padding spills into a second block.
    let message = b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq";
    assert_eq!(
        sha256::hex(message),
        "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1"
    );
}

#[test]
fn the_long_vector_matches_the_published_digest() {
    // FIPS 180-4 Appendix B.3: one million 'a'. Exercises the streaming path
    // over whole blocks, which the short vectors never reach.
    let message = vec![b'a'; 1_000_000];
    assert_eq!(
        sha256::hex(&message),
        "cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0"
    );
}

#[test]
fn a_message_that_is_exactly_one_block_is_padded_into_a_second() {
    // 64 bytes leaves no room for the 0x80 plus the length, so the padding
    // must occupy a whole extra block. This is the boundary the length
    // arithmetic gets wrong when it is wrong.
    let message = vec![0u8; 64];
    let digest = sha256::hex(&message);
    assert_eq!(digest.len(), 64);
    assert_eq!(
        digest,
        "f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b"
    );
}

#[test]
fn a_message_of_fifty_five_bytes_fits_in_a_single_block() {
    // 55 bytes is the largest message whose padding still fits alongside it:
    // 55 + 1 + 8 == 64. One byte more needs a second block.
    let fifty_five = vec![b'a'; 55];
    let fifty_six = vec![b'a'; 56];
    assert_ne!(sha256::hex(&fifty_five), sha256::hex(&fifty_six));
    assert_eq!(sha256::hex(&fifty_five).len(), 64);
    assert_eq!(sha256::hex(&fifty_six).len(), 64);
}
