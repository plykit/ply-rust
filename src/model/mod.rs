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


/// A string that is a valid solution ID.
#[derive(Debug, Clone)]
pub struct SolutionId(pub String);

/// Generic error for turning strings into more constrained string types.
#[derive(Clone, Debug)]
pub struct StringError { cause:String }

// TODO Implement real validity check and develop consistent error strategy for these things.
impl std::str::FromStr for SolutionId {
    type Err = StringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //println!("parse Solution ID {}",s);
        // TODO This is just dummy code to illustrate the validity check
        if s.len() > 10 {
            return Err(StringError{cause:"String to long".into()})
        }
        Ok(SolutionId(s.into()))
    }
}