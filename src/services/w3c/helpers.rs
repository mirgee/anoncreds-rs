use crate::data_types::w3c::credential::{CredentialAttributeValue, W3CCredential};
use crate::error::Result;
use crate::helpers::attr_common_view;
use std::collections::HashMap;

impl W3CCredential {
    pub(crate) fn get_case_insensitive_attribute(
        &self,
        requested_attribute: &str,
    ) -> Result<(String, CredentialAttributeValue)> {
        let requested_attribute = attr_common_view(requested_attribute);
        self.credential_subject
            .attributes
            .0
            .iter()
            .find(|(attribute, _)| attr_common_view(attribute) == requested_attribute)
            .map(|(attribute, value)| (attribute.to_owned(), value.to_owned()))
            .ok_or_else(|| err_msg!("Credential attribute {} not found", requested_attribute))
    }

    pub(crate) fn get_attributes(&self) -> HashMap<String, String> {
        let mut attributes: HashMap<String, String> = HashMap::new();
        for (name, attribute) in self.credential_subject.attributes.0.iter() {
            if let CredentialAttributeValue::Attribute(attribute) = attribute {
                attributes.insert(name.to_string(), attribute.to_string());
            }
        }
        attributes
    }
}