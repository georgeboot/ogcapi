use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Hyperlink to enable Hypermedia Access
#[derive(Serialize, Deserialize, ToSchema, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    /// Supplies the URI to a remote resource (or resource fragment).
    pub href: String,
    /// The type or semantics of the relation.
    pub rel: String,
    /// A hint indicating what the media type of the result of dereferencing
    /// the link should be.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable = false)]
    pub r#type: Option<String>,
    /// This flag set to true if the link is a URL template.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable = false)]
    pub templated: Option<bool>,
    /// A base path to retrieve semantic information about the variables used in URL template.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable = false)]
    pub var_base: Option<String>,
    /// A hint indicating what the language of the result of dereferencing the
    /// link should be.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable = false)]
    pub hreflang: Option<String>,
    /// Used to label the destination of a link such that it can be used as a
    /// human-readable identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable = false)]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable = false)]
    pub length: Option<i64>,
    /// The HTTP method to use when following this link, when it is not `GET`.
    ///
    /// Added by STAC API - Item Search, which pages a `POST /search` with a
    /// `next` link. A URL alone cannot express that continuation, because the
    /// filters live in the request body.
    #[cfg(feature = "stac")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable = false)]
    pub method: Option<String>,
    /// Headers to send when following this link.
    #[cfg(feature = "stac")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable = false)]
    pub headers: Option<serde_json::Map<String, serde_json::Value>>,
    /// Body to send when following this link with a method that takes one.
    #[cfg(feature = "stac")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable = false)]
    pub body: Option<serde_json::Map<String, serde_json::Value>>,
    /// Whether [`body`](Link::body) extends the original request body rather
    /// than replacing it. A paging link carries only the token, so it merges.
    #[cfg(feature = "stac")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable = false)]
    pub merge: Option<bool>,
}

impl Link {
    /// Constructs a new Link with the given href and link relation
    pub fn new(href: impl ToString, rel: impl ToString) -> Link {
        Link {
            href: href.to_string(),
            rel: rel.to_string(),
            r#type: None,
            templated: None,
            var_base: None,
            hreflang: None,
            title: None,
            length: None,
            #[cfg(feature = "stac")]
            method: None,
            #[cfg(feature = "stac")]
            headers: None,
            #[cfg(feature = "stac")]
            body: None,
            #[cfg(feature = "stac")]
            merge: None,
        }
    }

    /// Sets the HTTP method to use when following this link.
    #[cfg(feature = "stac")]
    pub fn method(mut self, method: impl ToString) -> Link {
        self.method = Some(method.to_string());
        self
    }

    /// Sets a body to merge into the original request when following this
    /// link, as a `POST` paging link does.
    #[cfg(feature = "stac")]
    pub fn merge_body(mut self, body: serde_json::Map<String, serde_json::Value>) -> Link {
        self.body = Some(body);
        self.merge = Some(true);
        self
    }

    /// Sets the media type of the Link and returns the Value
    pub fn mediatype(mut self, mime: impl ToString) -> Link {
        self.r#type = Some(mime.to_string());
        self
    }

    /// Sets whether the link is templated
    pub fn templated(mut self, templated: bool) -> Link {
        self.templated = Some(templated);
        self
    }

    /// Sets the base path to retreive semantic information about the variables used in URL template
    pub fn var_base(mut self, var_base: impl ToString) -> Link {
        self.var_base = Some(var_base.to_string());
        self
    }

    /// Sets the language of the Link and returns the Value
    pub fn language(mut self, language: impl ToString) -> Link {
        self.hreflang = Some(language.to_string());
        self
    }

    /// Sets the title of the Link and returns the Value
    pub fn title(mut self, title: impl ToString) -> Link {
        self.title = Some(title.to_string());
        self
    }

    /// Sets the length of the reference resource by the Link and returns the Value
    pub fn length(mut self, length: i64) -> Link {
        self.length = Some(length);
        self
    }
}
