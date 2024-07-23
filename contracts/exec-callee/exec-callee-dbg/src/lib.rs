ckb_std::entry_simulator!(program_entry);

/// program entry
fn program_entry() -> i8 {
    // Call main function and return error code
    match exec_callee::entry::main() {
        Ok(_) => 0,
        Err(err) => err as i8,
    }
}
