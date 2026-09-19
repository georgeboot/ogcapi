use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use utoipa::ToSchema;

#[cfg(feature = "edr")]
use crate::edr::{Contact, Provider};

use super::Link;

/// The Landing page is the entry point of a OGC API.
///
/// The Landing page provides links to:
/// * the API definition (link relations `service-desc` and `service-doc`),
/// * the Conformance declaration (path `/conformance`, link relation `conformance`), and
/// * the Collections (path `/collections`, link relation `data`).
#[derive(Serialize, Deserialize, ToSchema, Debug, PartialEq, Eq, Clone)]
pub struct LandingPage {
    /// `Catalog`, when this landing page is also a STAC Catalog.
    ///
    /// Optional, and omitted when absent, for the same reason as
    /// [`Feature::stac_version`](crate::features::Feature): the `stac` feature
    /// is crate-wide, so a landing page that serves only OGC API Features must
    /// not announce itself as a STAC Catalog.
    #[cfg(feature = "stac")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The STAC version the Catalog implements, when it is one.
    #[cfg(feature = "stac")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stac_version: Option<String>,
    /// A list of extension identifiers the Catalog implements.
    #[cfg(feature = "stac")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stac_extensions: Vec<String>,
    /// Identifier for the Catalog, required of a STAC one.
    #[cfg(feature = "stac")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The title of the API.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// A textual description of the API.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Attribution for the API.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attribution: Option<String>,
    /// Links to the resources exposed through this API
    #[serde(default)]
    pub links: Vec<Link>,
    #[cfg(feature = "edr")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keywords: Vec<String>,
    #[cfg(feature = "edr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<Provider>,
    #[cfg(feature = "edr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    #[cfg(feature = "stac")]
    #[serde(default, rename = "conformsTo", skip_serializing_if = "Vec::is_empty")]
    pub conforms_to: Vec<String>,
    #[serde(flatten, default, skip_serializing_if = "Map::is_empty")]
    pub additional_properties: Map<String, Value>,
}

#[allow(clippy::derivable_impls)]
impl Default for LandingPage {
    fn default() -> Self {
        Self {
            #[cfg(feature = "stac")]
            r#type: None,
            #[cfg(feature = "stac")]
            stac_version: None,
            #[cfg(feature = "stac")]
            stac_extensions: Default::default(),
            #[cfg(feature = "stac")]
            id: Default::default(),
            title: Default::default(),
            description: Default::default(),
            attribution: Default::default(),
            links: Default::default(),
            #[cfg(feature = "edr")]
            keywords: Default::default(),
            #[cfg(feature = "edr")]
            provider: Default::default(),
            #[cfg(feature = "edr")]
            contact: Default::default(),
            #[cfg(feature = "stac")]
            conforms_to: Default::default(),
            additional_properties: Default::default(),
        }
    }
}

impl LandingPage {
    pub fn new(name: impl ToString) -> Self {
        let landing_page = LandingPage::default();
        #[cfg(feature = "stac")]
        let landing_page = landing_page.id(name.to_string());
        landing_page.title(name)
    }

    #[cfg(feature = "stac")]
    pub fn id(mut self, id: impl ToString) -> Self {
        self.id = Some(id.to_string());
        self
    }

    /// Make this landing page a STAC Catalog: `type`, `stac_version` and the
    /// conformance classes a STAC client reads from it.
    #[cfg(feature = "stac")]
    pub fn as_stac_catalog(mut self, id: impl ToString) -> Self {
        self.r#type = Some(crate::stac::catalog());
        self.stac_version = Some(crate::stac::stac_version());
        self.id = Some(id.to_string());
        self
    }

    pub fn title(mut self, title: impl ToString) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn description(mut self, description: impl ToString) -> Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn links(mut self, links: Vec<Link>) -> Self {
        self.links = links;
        self
    }

    #[cfg(feature = "stac")]
    pub fn conforms_to(mut self, classes: &[impl ToString]) -> Self {
        self.conforms_to = classes.iter().map(|c| c.to_string()).collect();
        self
    }
}
