/// Implements the standardized payload format for
/// ply nodes.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Node {
    /// Globally unique ID of this element
    pub id: String,
    /// Display name
    pub name: String,
    /// Longer description of the meaning and significance
    /// of this element.
    #[serde(default)]
    pub description: String,
}
