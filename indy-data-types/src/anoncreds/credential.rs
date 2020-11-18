use std::collections::HashMap;

use zeroize::Zeroize;

use crate::identifiers::cred_def::CredentialDefinitionId;
use crate::identifiers::rev_reg::RevocationRegistryId;
use crate::identifiers::schema::SchemaId;
use crate::{Validatable, ValidationError};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Credential {
    pub schema_id: SchemaId,
    pub cred_def_id: CredentialDefinitionId,
    pub rev_reg_id: Option<RevocationRegistryId>,
    pub values: CredentialValues,
    pub signature: ursa_cl!(CredentialSignature),
    pub signature_correctness_proof: ursa_cl!(SignatureCorrectnessProof),
    pub rev_reg: Option<ursa_cl!(RevocationRegistry)>,
    pub witness: Option<ursa_cl!(Witness)>,
}

impl Credential {
    pub const QUALIFIABLE_TAGS: [&'static str; 5] = [
        "issuer_did",
        "cred_def_id",
        "schema_id",
        "schema_issuer_did",
        "rev_reg_id",
    ];
}

impl Validatable for Credential {
    fn validate(&self) -> Result<(), ValidationError> {
        self.schema_id.validate()?;
        self.cred_def_id.validate()?;
        self.values.validate()?;

        if self.rev_reg_id.is_some() && (self.witness.is_none() || self.rev_reg.is_none()) {
            return Err("Credential validation failed: `witness` and `rev_reg` must be passed for revocable Credential".into());
        }

        if self.values.0.is_empty() {
            return Err("Credential validation failed: `values` is empty".into());
        }

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct CredentialInfo {
    pub referent: String,
    pub attrs: ShortCredentialValues,
    pub schema_id: SchemaId,
    pub cred_def_id: CredentialDefinitionId,
    pub rev_reg_id: Option<RevocationRegistryId>,
    pub cred_rev_id: Option<String>,
}

pub type ShortCredentialValues = HashMap<String, String>;

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct CredentialValues(pub HashMap<String, AttributeValues>);

impl Drop for CredentialValues {
    fn drop(&mut self) {
        self.zeroize();
    }
}

impl Validatable for CredentialValues {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.0.is_empty() {
            return Err("CredentialValues validation failed: empty list has been passed".into());
        }

        Ok(())
    }
}

impl Zeroize for CredentialValues {
    fn zeroize(&mut self) {
        for attr in self.0.values_mut() {
            attr.zeroize();
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Zeroize)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AttributeValues {
    pub raw: String,
    pub encoded: String,
}
