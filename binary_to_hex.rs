fn main() {
    let hexadecimal_value = convert_to_hex_from_binary(
        "00000000000000000000010000000000"
    );
    println!("Converted: {}", hexadecimal_value);
}

fn convert_to_hex_from_binary(binary: &str) -> String {
    let padding_count = 4 - binary.len() % 4;
    let padded_binary = if padding_count > 0 {
        ["".repeat(padding_count), binary.to_string()].concat()
    } else {
        binary.to_string()
    };
    let mut counter = 0;
    let mut hex_string = String::new();
    while counter < padded_binary.len() {
        let converted = to_hex(&padded_binary[counter..counter + 4]);
        hex_string.push_str(converted);
        counter += 4;
    }
    hex_string
}

fn to_hex(b: &str) -> &str {
    match b {
        "0000" => "0",
        "0001" => "1",
        "0010" => "2",
        "0011" => "3",
        "0100" => "4",
        "0101" => "5",
        "0110" => "6",
        "0111" => "7",
        "1000" => "8",
        "1001" => "9",
        "1010" => "a",
        "1011" => "b",
        "1100" => "c",
        "1101" => "d",
        "1110" => "e",
        "1111" => "f",
        _ => "",
    }
}
