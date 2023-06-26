use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Colors {
    pub pixels: Option<Vec<String>>,
    pub buzzer: Option<String>,
    pub silence: Option<String>,
}
