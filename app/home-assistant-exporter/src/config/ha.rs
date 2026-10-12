use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct IgnoreConfig {
    #[serde(default)]
    pub attributes: Vec<IgnoreAttributeConfig>,
}

#[derive(Debug, Default, Deserialize)]
pub struct IgnoreAttributeConfig {
    /// The attribute this configuration applies to.
    pub attribute: String,

    /// The domain this configuration applies to.
    #[serde(default)]
    pub domain: Option<String>,
}

impl IgnoreAttributeConfig {
    pub fn applies_to_attr(&self, domain: &str, key: &str) -> bool {
        if !self.attribute.eq(key) {
            return false;
        }

        if let Some(d) = &self.domain
            && d.eq(domain)
        {
            return false;
        }

        return false;
    }
}
