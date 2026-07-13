//! Confirms the `json-fg` feature re-exports the `jsonfg` crate and defines the media
//! type, so consumers can build JSON-FG responses through `ogcapi_types`.
#![cfg(feature = "json-fg")]

use ogcapi_types::common::media_type::JSON_FG;
use ogcapi_types::jsonfg::{self, CoordRefSys, Feature, Geometry};

#[test]
fn media_type_and_conformance_are_available() {
    assert_eq!(JSON_FG, "application/vnd.ogc.fg+json");
    assert_eq!(
        jsonfg::conformance::CORE,
        "http://www.opengis.net/spec/json-fg-1/1.0/conf/core"
    );
}

#[test]
fn build_a_native_crs_feature() {
    // Geometry in a projected CRS goes in `place`; `geometry` (GeoJSON) stays null.
    let mut feature = Feature::new();
    feature.coord_ref_sys = Some(CoordRefSys::from_epsg(3857));
    feature.place = Some(Geometry::point(vec![100.0, 200.0]));

    assert!(feature.geometry.is_none());
    assert!(!feature.coord_ref_sys.as_ref().unwrap().is_wgs84());
}
