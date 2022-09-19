pub fn base_inst_from(inst: u32) -> String {
    format!("db {:b}", inst)
}