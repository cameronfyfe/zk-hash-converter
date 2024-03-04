use risc0_zkvm::Journal;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use zk_hash_converter_core::HashResults;

// include this file instead of handling as a crate because `core` library
// and risc5 based `guest` binary should use different `serde` configs
// include!("../../interface/interface.rs");

#[wasm_bindgen]
pub fn json_obj_from_journal_bytes(journal: Vec<u8>) -> Result<JsValue, JsValue> {
    let journal = Journal::new(journal);

    let journal: HashResults = journal
        .decode()
        .map_err(|e| JsValue::from_str(&format!("Failed to decode journal: {e}")))?;

    let journal = serde_wasm_bindgen::to_value(&journal).map_err(|e| {
        JsValue::from_str(&format!(
            "Failed to serialize decoded journal to JsValue: {}",
            e
        ))
    })?;

    Ok(journal)
}
