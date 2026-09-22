//! The `stac` feature is crate-wide: enabling it for one surface must not
//! change what an OGC API Features collection serialises to.
#![cfg(feature = "stac")]

use ogcapi_types::common::Collection;

fn features_collection() -> Collection {
    Collection {
        id: "panorama".to_string(),
        ..Default::default()
    }
}

#[test]
fn a_features_collection_claims_nothing_about_stac() {
    let v = serde_json::to_value(features_collection()).unwrap();
    let object = v.as_object().unwrap();

    for member in [
        "type",
        "stac_version",
        "license",
        "stac_extensions",
        "item_assets",
        "itemAssets",
    ] {
        assert!(!object.contains_key(member), "unexpected `{member}` in {v}");
    }
}

#[test]
fn a_stac_collection_carries_its_required_members() {
    let mut collection = features_collection().as_stac_collection("other");
    collection.item_assets.insert(
        "image".to_string(),
        serde_json::json!({ "type": "image/jpeg" }),
    );
    let v = serde_json::to_value(collection).unwrap();

    assert_eq!(v["type"], "Collection");
    assert_eq!(v["stac_version"], "1.0.0");
    assert_eq!(v["license"], "other");
    assert_eq!(v["item_assets"]["image"]["type"], "image/jpeg");
    assert!(v.get("itemAssets").is_none());
}

#[test]
fn a_stac_collection_round_trips() {
    let json = serde_json::json!({
        "type": "Collection",
        "stac_version": "1.0.0",
        "id": "c",
        "license": "proprietary",
        "links": [],
        "item_assets": { "data": { "roles": ["data"] } }
    });
    let collection: Collection = serde_json::from_value(json).unwrap();

    assert_eq!(collection.r#type.as_deref(), Some("Collection"));
    assert_eq!(collection.license.as_deref(), Some("proprietary"));
    assert!(collection.item_assets.contains_key("data"));
}
