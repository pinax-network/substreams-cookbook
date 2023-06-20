use substreams::Hex;

pub fn input_to_method(input: Vec<u8>) -> String {
    Hex::encode(&input[0..4])
}

pub fn method_to_name(method: &str) -> &str {
    let method_name = match method {
        "a9059cbb" => "transfer",
        "23b872dd" => "transferFrom",
        "095ea7b3" => "approve",
        "40c10f19" => "mint",
        "d505accf" => "permit",
        "39509351" => "increaseAllowance",
        "42966c68" => "burn",
        _ => "unknown"
    };
    method_name
}