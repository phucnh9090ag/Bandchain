se obi::{get_schema, OBIDecode, OBIEncode, OBISchema};
use owasm2::{execute_entry_point, ext, oei, prepare_entry_point};

#[derive(OBIDecode, OBISchema)]
struct Input {
    multiplier: u64,
}

#[derive(OBIEncode, OBISchema)]
struct Output {
    price: u64,
}

#[no_mangle]
fn prepare_impl(_input: Input) {
    // Coingecko price data source
    oei::ask_external_data(1, 1, "".as_bytes());
}

#[no_mangle]
fn execute_impl(input: Input) -> Output {
    let median = ext::load_median::<f64>(1);
    Output {
        price: (median.unwrap() * input.multiplier as f64) as u64,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use obi::get_schema;
    use std::collections::*;

    #[test]
    fn test_get_schema() {
        let mut schema = HashMap::new();
        Input::add_definitions_recursively(&mut schema);
        Output::add_definitions_recursively(&mut schema);
        let input_schema = get_schema(String::from("Input"), &schema);
        let output_schema = get_schema(String::from("Output"), &schema);
        println!("{}/{}", input_schema, output_schema);
        assert_eq!(
            "{multiplier:u64}/{price:u64}",
            format!("{}/{}", input_schema, output_schema),
        );
    }
}

prepare_entry_point!(prepare_impl);
execute_entry_point!(execute_impl);