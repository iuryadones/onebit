mod binary_to_hex;
mod hex_to_binary;
mod sha256;

fn generating_the_mnemonic() {
    println!("| Entropy | Checksum | Entropy + Checksum | Mnemonic |");
    for idx in (32..256 + 1).step_by(32) {
        let entropy = idx;
        let checksum = entropy / 32;
        let entropy_checksum = entropy + checksum;
        let mnemonic = entropy_checksum / 11;
        println!("| {entropy:^7} | {checksum:^8} | {entropy_checksum:^18} | {mnemonic:^8} |");
    }
}

fn main() {
    generating_the_mnemonic();
    println!();

    let binary = "00110010001 01010011000 0011001011";
    let checksum = "0";
    println!("Raw binary: {}", binary);
    println!("Binary checksum: {}", checksum);
    let binary_splited: Vec<&str> = binary.split(" ").collect();
    println!("{:?}", binary_splited);
    println!();

    // for idx in 0..binary_splited.len() {
    //     let str_binary = binary_splited[idx];
    //     println!("{:?}", str_binary);
    // }

    let binary_value = "00110010001010100110000011001011";

    let checksum_cut = binary_value.len() / 32;
    println!("Checksum: {}", checksum_cut);
    println!("Entropy: {}", &binary_value);
    println!("Entropy length: {}", binary_value.len());
    let hexadecimal_value = binary_to_hex::convert_to_hex_from_binary(&binary_value);
    println!("Hexdecimal: {}", &hexadecimal_value);

    let mut hash256 = sha256::Sha256::default();
    hash256.update(&hexadecimal_value[..].as_bytes());
    let binding = sha256::to_hex(&hash256.finish());
    let hex_str = binding.as_str();
    let last_hex = hex_str.len();
    let checksum_hex = &hex_str[last_hex - 2..];
    let checksum_binary = hex_to_binary::convert_to_binary_from_hex(&format!("0x{}", &checksum_hex));
    // let checksum_first = checksum_binary[..checksum_cut]

    println!("sha256 (little endian): {}", hex_str);
    println!("checksum (sha256-last): {}", checksum_hex);
    println!("checksum binary: {}", checksum_binary);
    // println!("checksum: {}", checksum_first);
}
