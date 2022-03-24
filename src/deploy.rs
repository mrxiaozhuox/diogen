//! this file will check deploy enviroment.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeployInfo {
    pub scheme: Vec<DeployScheme>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeployScheme {
    #[serde(default)]
    pub name: String,

    pub domain: serde_json::Value,

    pub request: String,
}

impl DeployInfo {
    pub fn current_scheme(&self) -> Option<DeployScheme> {
        let doc = web_sys::window()?.document()?;
        let curr_url = doc.url().ok()?;
        let curr_url = url::Url::parse(&curr_url).ok()?;
        let domain = curr_url.host()?;
        let domain = match domain {
            url::Host::Domain(v) => v.to_string(),
            url::Host::Ipv4(v) => v.to_string(),
            url::Host::Ipv6(v) => v.to_string(),
        };
        let scheme = self.scheme.iter().find(|scheme| {
            let temp = match &scheme.domain {
                serde_json::Value::String(v) => v.to_string(),
                serde_json::Value::Array(arr) => {
                    for v in arr {
                        if let serde_json::Value::String(v) = v {
                            if v == &domain {
                                return true;
                            }
                        }
                    }
                    return false;
                }
                _ => return false,
            };
            domain == temp
        });
        scheme.cloned()
    }
}
