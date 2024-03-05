use risc0_zkvm::Journal;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

// include this file instead of handling as a crate because `core` library
// and risc-v based `guest` binary should use different `serde` configs
include!("../../core/methods/interface.rs");

use interface::HashResults;

#[wasm_bindgen]
pub fn json_obj_from_journal_bytes(journal: Vec<u8>) -> Result<JsValue, JsValue> {
    let hash_results = hash_results_from_journal_bytes(journal)?;

    let json_obj = serde_wasm_bindgen::to_value(&hash_results).map_err(|e| {
        JsValue::from_str(&format!(
            "Failed to serialize decoded journal to JsValue: {}",
            e
        ))
    })?;

    Ok(json_obj)
}

#[wasm_bindgen]
pub fn statement_from_journal_bytes(journal: Vec<u8>) -> Result<JsValue, JsValue> {
    let hash_results = hash_results_from_journal_bytes(journal)?;

    let sha256_hex = hex::encode(hash_results.sha256);
    let blake3_hex = hex::encode(hash_results.blake3);

    let statement = format!("There exists a binary blob with a **Sha256** hash of **`{sha256_hex}`** and a **Blake3** hash of **`{blake3_hex}`**.");
    let statement = JsValue::from_str(&statement);

    Ok(statement)
}

fn hash_results_from_journal_bytes(journal: Vec<u8>) -> Result<HashResults, JsValue> {
    let journal = Journal::new(journal);

    let journal = journal
        .decode()
        .map_err(|e| JsValue::from_str(&format!("Failed to decode journal: {e}")))?;

    Ok(journal)
}
