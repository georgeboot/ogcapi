use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use utoipa::ToSchema;

use crate::common::Link;

/// A STAC Catalog object represents a logical group of other `Catalog`,
/// `Collection`, and `Item` objects.
#[derive(Serialize, Deserialize, ToSchema, Debug, PartialEq, Eq, Clone)]
pub struct Catalog {
    /// Set to `Catalog` if this Catalog only implements the Catalog spec.
    #[serde(default = "crate::stac::catalog")]
    pub r#type: String,
    /// The STAC version the Catalog implements.
    #[serde(default = "crate::stac::stac_version")]
    pub stac_version: String,
    /// A list of extension identifiers the Catalog implements.
    #[serde(default)]
    pub stac_extensions: Vec<String>,
    /// Identifier for the Catalog.
    pub id: String,
    /// A short descriptive one-line title for the Catalog.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Detailed multi-line description to fully explain the Catalog.
    /// CommonMark 0.29 syntax MAY be used for rich text representation.
    #[serde(default)]
    pub description: String,
    /// The conformance classes this API implements.
    ///
    /// STAC puts these on the landing page rather than behind a second
    /// request, so a client learns what the API can do in one round trip. Only
    /// a Catalog serving as an API landing page has them; a static catalog
    /// does not.
    #[serde(rename = "conformsTo", default, skip_serializing_if = "Vec::is_empty")]
    pub conforms_to: Vec<String>,
    /// A list of references to other documents.
    #[serde(default)]
    pub links: Vec<Link>,
    #[serde(flatten, default, skip_serializing_if = "Map::is_empty")]
    pub additional_properties: Map<String, Value>,
}

impl Catalog {
    pub fn new(id: impl ToString, description: impl ToString) -> Self {
        Catalog {
            r#type: super::catalog(),
            stac_version: super::stac_version(),
            stac_extensions: Default::default(),
            id: id.to_string(),
            title: Default::default(),
            description: description.to_string(),
            conforms_to: Default::default(),
            links: Default::default(),
            additional_properties: Default::default(),
        }
    }

    /// Declare the conformance classes this API implements.
    pub fn conforms_to<I, S>(mut self, classes: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: ToString,
    {
        self.conforms_to = classes.into_iter().map(|c| c.to_string()).collect();
        self
    }

    pub fn title(mut self, title: impl ToString) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn links(mut self, links: Vec<Link>) -> Self {
        self.links = links;
        self
    }
}
