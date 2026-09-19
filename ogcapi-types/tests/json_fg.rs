//! JSON-FG support on the OGC API Features types, behind the `json-fg` feature: the extra
//! members on `Feature`/`FeatureCollection` and the `into_json_fg` conversion helpers.
#![cfg(feature = "json-fg")]

use ogcapi_types::common::media_type::JSON_FG;
use ogcapi_types::features::{Feature, FeatureCollection};
use ogcapi_types::jsonfg::CoordRefSys;

fn geojson_point_feature() -> Feature {
    serde_json::from_value(serde_json::json!({
        "type": "Feature",
        "id": "f1",
        "geometry": { "type": "Point", "coordinates": [155000.0, 463000.0] },
        "properties": {}
    }))
    .unwrap()
}

#[test]
fn media_type_available() {
    assert_eq!(JSON_FG, "application/vnd.ogc.fg+json");
}

#[test]
fn feature_native_crs_goes_to_place() {
    let v =
        serde_json::to_value(geojson_point_feature().into_json_fg(CoordRefSys::from_epsg(28992)))
            .unwrap();
    assert_eq!(v["place"]["type"], "Point");
    assert_eq!(
        v["place"]["coordinates"],
        serde_json::json!([155000.0, 463000.0])
    );
    assert!(v["geometry"].is_null());
    assert_eq!(
        v["coordRefSys"],
        "http://www.opengis.net/def/crs/EPSG/0/28992"
    );
    assert!(
        v["conformsTo"]
            .as_array()
            .unwrap()
            .iter()
            .any(|u| u.as_str().unwrap().contains("json-fg-1/1.0/conf/core"))
    );
}

#[test]
fn feature_wgs84_stays_in_geometry() {
    let v =
        serde_json::to_value(geojson_point_feature().into_json_fg(CoordRefSys::crs84())).unwrap();
    assert_eq!(v["geometry"]["type"], "Point");
    assert!(v.get("place").is_none());
    assert!(v.get("coordRefSys").is_none());
}

/// `coordRefSys` describes `place`, so a document may carry a WGS 84
/// `geometry` beside it. That is what lets one response serve a JSON-FG reader
/// and a plain GeoJSON reader at once, and what keeps it a valid GeoJSON
/// Feature, which a STAC Item has to be.
#[test]
fn feature_can_carry_both_geometries() {
    let wgs84 = geojson::Geometry::new(geojson::GeometryValue::Point {
        coordinates: vec![5.387, 52.155].into(),
    });

    let v = serde_json::to_value(
        geojson_point_feature()
            .into_json_fg_with_geometry(CoordRefSys::from_epsg(28992), Some(wgs84)),
    )
    .unwrap();

    assert_eq!(
        v["place"]["coordinates"],
        serde_json::json!([155000.0, 463000.0])
    );
    assert_eq!(
        v["geometry"]["coordinates"],
        serde_json::json!([5.387, 52.155])
    );
    assert_eq!(
        v["coordRefSys"],
        "http://www.opengis.net/def/crs/EPSG/0/28992"
    );
}

/// Passing no WGS 84 geometry keeps the original shape, so the new entry point
/// is a superset of the old one rather than a replacement.
#[test]
fn feature_without_a_wgs84_geometry_nulls_it() {
    let v = serde_json::to_value(
        geojson_point_feature().into_json_fg_with_geometry(CoordRefSys::from_epsg(28992), None),
    )
    .unwrap();

    assert_eq!(v["place"]["type"], "Point");
    assert!(v["geometry"].is_null());
}

#[test]
fn collection_can_carry_both_geometries() {
    let fc = FeatureCollection::new(vec![geojson_point_feature()]);

    let v = serde_json::to_value(fc.into_json_fg_with(CoordRefSys::from_epsg(28992), |f| {
        // The native geometry is still in place when the closure runs, which is
        // what lets a caller derive the WGS 84 one from it.
        assert!(f.geometry.is_some());
        Some(geojson::Geometry::new(geojson::GeometryValue::Point {
            coordinates: vec![5.387, 52.155].into(),
        }))
    }))
    .unwrap();

    assert_eq!(
        v["coordRefSys"],
        "http://www.opengis.net/def/crs/EPSG/0/28992"
    );
    assert_eq!(
        v["features"][0]["place"]["coordinates"],
        serde_json::json!([155000.0, 463000.0])
    );
    assert_eq!(
        v["features"][0]["geometry"]["coordinates"],
        serde_json::json!([5.387, 52.155])
    );
}

#[test]
fn collection_sets_crs_once_and_features_omit_it() {
    let fc = FeatureCollection::new(vec![geojson_point_feature()]);
    let v = serde_json::to_value(fc.into_json_fg(CoordRefSys::from_epsg(28992))).unwrap();
    assert_eq!(
        v["coordRefSys"],
        "http://www.opengis.net/def/crs/EPSG/0/28992"
    );
    // The contained feature inherits the collection CRS and omits its own.
    assert!(v["features"][0].get("coordRefSys").is_none());
    assert!(v["features"][0].get("conformsTo").is_none());
    assert_eq!(v["features"][0]["place"]["type"], "Point");
    assert!(v["features"][0]["geometry"].is_null());
}
