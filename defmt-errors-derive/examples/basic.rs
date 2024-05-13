use defmt_errors_derive::DefmtError;

#[allow(dead_code)]
#[derive(DefmtError)]
enum EmptyError {}

#[allow(dead_code)]
#[derive(DefmtError)]
enum ErrorUnitVariants {
    One,
}

fn main() {}
