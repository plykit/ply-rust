#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Assembly {
    pub name: String,
    pub description: String,
}

///
///
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Element {
    pub id: String,
    pub name: String,
    pub description: String,
}
