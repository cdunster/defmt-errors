use defmt_errors_derive::DefmtError;

#[allow(dead_code)]
#[derive(DefmtError)]
enum EmptyError {}

fn main() {}
