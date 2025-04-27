use hex::{decode, encode};
use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};
use std::io::Write;
use num_bigint::BigUint;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64;
    let difficulty_bits = 0x1e007fff;

    let coinbase_payload = vec![
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Blue score
        0x00, 0xE1, 0xF5, 0x05, 0x00, 0x00, 0x00, 0x00, // Subsidy
        0x00, 0x00, // Script version
        0x01, // Varint
        0x00, // OP-FALSE
        0x65, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x6c, 0x79, 0x2c, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x65, 0x76, 0x65, 0x72, // vecno-testnet
    ];

    let coinbase_tx = DomainTransaction { payload: coinbase_payload.clone() };

    let merkle_root = calculate_merkle_root(vec![coinbase_tx.clone()]);

    let (nonce, block_hash) = mine_genesis_block(&merkle_root, difficulty_bits, timestamp)?;

    println!("Genesis Block Hash: {}", block_hash);

    println!("Genesis block has been successfully mined!");

    print_genesis_block_config(&block_hash, &merkle_root, timestamp, difficulty_bits, nonce, &coinbase_payload);

    Ok(())
}

fn print_byte_array(s: &str) {
    let bytes = decode(s).unwrap();
    for (i, &b) in bytes.iter().enumerate() {
        if i == bytes.len() - 1 {
            print!("0x{:02x}", b);
        } else if (i + 1) % 16 == 0 {
            println!("0x{:02x},", b);
        } else {
            print!("0x{:02x}, ", b);
        }
    }
    println!();
}

fn calculate_merkle_root(transactions: Vec<DomainTransaction>) -> String {
    if transactions.is_empty() {
        return String::new();
    }

    let mut hashes: Vec<Vec<u8>> = transactions.into_iter().map(|tx| {
        let mut hasher = Sha256::new();
        hasher.update(&tx.payload);
        hasher.finalize().to_vec()
    }).collect();

    while hashes.len() > 1 {
        let mut next_level = Vec::new();
        for i in (0..hashes.len()).step_by(2) {
            if i + 1 < hashes.len() {
                let mut combined = hashes[i].clone();
                combined.extend(&hashes[i + 1]);
                let mut hasher = Sha256::new();
                hasher.update(&combined);
                next_level.push(hasher.finalize().to_vec());
            } else {
                next_level.push(hashes[i].clone());
            }
        }
        hashes = next_level;
    }

    encode(&hashes[0])
}

fn mine_genesis_block(merkle_root: &str, difficulty_bits: u32, timestamp: u64) -> Result<(u64, String), Box<dyn std::error::Error>> {
    let target = calculate_target(difficulty_bits);
    let mut nonce = 0;
    let merkle_root_bytes = decode(merkle_root)?;

    println!("Mining genesis block...");
    loop {
        let header = BlockHeader {
            version: 0,
            hash_merkle_root: DomainHash { bytes: merkle_root_bytes.clone() },
            timestamp,
            bits: difficulty_bits,
            nonce,
        };

        let mut serialized_header = Vec::new();
        serialize_block_header(&header, &mut serialized_header)?;

        let mut hasher = Sha256::new();
        hasher.update(&serialized_header);
        let hash = hasher.finalize();
        let hash_bigint = BigUint::from_bytes_be(&hash);

        if hash_bigint < target {
            return Ok((nonce, encode(hash)));
        }

        nonce += 1;
    }
}

fn calculate_target(bits: u32) -> BigUint {
    let exponent = ((bits >> 24) & 0xff) as u32;
    let mantissa = bits & 0xffffff;
    BigUint::from(mantissa) << (8 * (exponent - 3))
}

fn serialize_block_header(header: &BlockHeader, buffer: &mut Vec<u8>) -> std::io::Result<()> {
    buffer.write_all(&header.version.to_le_bytes())?;
    buffer.write_all(&header.hash_merkle_root.bytes)?;
    buffer.write_all(&header.timestamp.to_le_bytes())?;
    buffer.write_all(&header.bits.to_le_bytes())?;
    buffer.write_all(&header.nonce.to_le_bytes())?;
    Ok(())
}

fn print_genesis_block_config(hash: &str, merkle_root: &str, timestamp: u64, bits: u32, nonce: u64, coinbase_payload: &[u8]) {
    println!("pub const GENESIS: GenesisBlock = GenesisBlock {{");
    println!("    hash: Hash::from_bytes([");
    print_byte_array(hash);
    println!("    ]),");
    println!("    version: {},", 0);
    println!("    hash_merkle_root: Hash::from_bytes([");
    print_byte_array(merkle_root);
    println!("    ]),");
    println!("    utxo_commitment: EMPTY_MUHASH,");
    println!("    timestamp: {},", timestamp / 1000); // Convert milliseconds to seconds
    println!("    bits: 0x{:08x},", bits);
    println!("    nonce: 0x{:08x},", nonce);
    println!("    daa_score: 0, // Checkpoint DAA score");
    println!("    #[rustfmt::skip]");
    println!("    coinbase_payload: &[");
    for (i, &b) in coinbase_payload.iter().enumerate() {
        let comment = match i {
            7 => " // Blue score",
            15 => " // Subsidy",
            17 => " // Script version",
            18 => " // Varint",
            19 => " // OP-FALSE",
            38 => " // Eternally, for ever",
            _ => "",
        };
        if i == coinbase_payload.len() - 1 {
            println!(" 0x{:02x}{}", b, comment);
        } else if i % 8 == 7 || comment != "" {
            print!(" 0x{:02x},", b);
            if comment != "" {
                println!("{}", comment);
            } else {
                println!();
            }
        } else {
            print!(" 0x{:02x},", b);
        }
    }
    println!("    ],");
    println!("}};");
}

#[derive(Clone, Debug)]
struct DomainTransaction {
    payload: Vec<u8>,
}

#[derive(Clone, Debug)]
struct DomainHash {
    bytes: Vec<u8>,
}

#[derive(Clone, Debug)]
struct BlockHeader {
    version: u16,
    hash_merkle_root: DomainHash,
    timestamp: u64,
    bits: u32,
    nonce: u64,
}