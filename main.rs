fn generating_the_mnemonic() {
    println!("| Entropy | Checksum | Entropy + Checksum | Mnemonic |");
    for idx in (32..256+1).step_by(32) {
        let entropy = idx;
        let checksum = entropy / 32;
        let entropy_checksum = entropy + checksum; 
        let mnemonic = entropy_checksum / 11;
        println!("| {entropy:^7} | {checksum:^8} | {entropy_checksum:^18} | {mnemonic:^8} |");
    }
}

fn main() {
    generating_the_mnemonic();
}
