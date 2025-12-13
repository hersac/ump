use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProyectoUmp {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub umbral: String,
    pub main: String,
    pub scripts: Option<HashMap<String, String>>,
    pub dependencies: Option<HashMap<String, String>>,
}

impl Default for ProyectoUmp {
    fn default() -> Self {
        ProyectoUmp {
            name: "mi-proyecto".to_string(),
            version: "1.0.0".to_string(),
            description: Some("Un nuevo proyecto Umbral".to_string()),
            umbral: ">=0.1.0".to_string(),
            main: "src/main.um".to_string(),
            scripts: Some(HashMap::from([
                ("start".to_string(), "umbral src/main.um".to_string()),
                ("dev".to_string(), "umbral src/main.um --watch".to_string()),
                ("test".to_string(), "umbral pruebas/main.um".to_string()),
            ])),
            dependencies: Some(HashMap::new()),
        }
    }
}
