
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ModState {
    installed_version: crate::ModVersion,
    dll_name: String,
}
