use obi::{get_schema, OBIDecode, OBIEncode, OBISchema};
use owasm::{execute_entry_point, ext, oei, prepare_entry_point};

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
    oei::ask_external_data(1, 1, "".as_bytes());
}

#[no_mangle]
fn execute_impl(input: Input) -> Output {
    let avg = ext::load_average::<f64>(1);
    Output {
        price: (avg.unwrap() as f64 * input.multiplier as f64) as u64
    }
}

prepare_entry_point!(prepare_impl);
execute_entry_point!(execute_impl);