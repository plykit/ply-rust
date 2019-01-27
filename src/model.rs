#[derive(Serialize, Deserialize, Debug, Default)]
pub struct System {
    pub name: String,
    pub components: Vec<Component>,
    pub dependencies: Vec<Dependency>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Dependency {
    pub name: String,
    pub service: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Component {
    pub name: String,
    #[serde(rename = "type")]
    pub tipe: String,
    pub implements: Vec<String>,
}
