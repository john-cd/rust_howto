## Geoscience

[![cat~science::geo][cat~science::geo~badge]][cat~science::geo]{{hi:Geoscience}}

Processing of spatial information, maps, navigation data, and geographic information systems.

The Rust geoscience ecosystem is developing, and while not as mature as Python's GDAL/OGR or other GIS libraries, it offers promising options, especially for performance-sensitive applications and large datasets. Crates like [`geo`][c~geo~docs]↗{{hi:geo}}, or [`proj`][c~proj~docs]↗{{hi:proj}} can be used for tasks such as:

- Calculating distances and bearings between locations.
- Converting between coordinate systems (e.g., latitude/longitude to UTM).
- Working with geometric shapes (points, lines, polygons).

| Topic | Rust Crates (Examples) | Notes |
|---|---|---|
| Geospatial Data Structures | [`geo`][c~geo~docs]↗{{hi:geo}}, [`geos`][c~geos~docs]↗{{hi:geos}}, [`wkt`][c~wkt~docs]↗{{hi:wkt}} | [`geo`][c~geo~docs]↗{{hi:geo}} provides fundamental geospatial types and algorithms. [`geos`][c~geos~docs]↗{{hi:geos}} offers bindings to the GEOS library for more advanced operations. [`wkt`][c~wkt~docs]↗{{hi:wkt}} handles Well-Known Text (WKT) parsing and serialization. |
| Geospatial Data Formats | [`gdal`][c~gdal~docs]↗{{hi:gdal}}, [`raster`][c~raster~docs]↗{{hi:raster}}, [`geotiff`][c~geotiff~docs]↗{{hi:geotiff}} | [`gdal`][c~gdal~docs]↗{{hi:gdal}} provides bindings to the GDAL library for reading and writing various geospatial raster and vector formats. [`raster`][c~raster~docs]↗{{hi:raster}} provides a pure Rust alternative for working with raster data. [`geotiff`][c~geotiff~docs]↗{{hi:geotiff}} specifically focuses on GeoTIFF files. |
| Coordinate Reference Systems (CRS) & Projections | [`proj`][c~proj~docs]↗{{hi:proj}}, [`crs`][c~crs~docs]↗{{hi:crs}} | [`proj`][c~proj~docs]↗{{hi:proj}} offers bindings to the PROJ library for coordinate transformations. [`crs`][c~crs~docs]↗{{hi:crs}} is a newer crate focused on CRS management in Rust. |
| Spatial Analysis | (Often built upon [`geo`][c~geo~docs]↗{{hi:geo}} and [`geos`][c~geos~docs]↗{{hi:geos}}) | Many spatial analysis operations can be implemented using the core geospatial crates. |
| Geostatistics | | This area is still developing in Rust. Some crates might emerge or be based on numerical computation crates. |
| Remote Sensing | Often uses [`gdal`][c~gdal~docs]↗{{hi:gdal}} or [`raster`][c~raster~docs]↗{{hi:raster}} | Remote sensing applications often leverage raster processing capabilities provided by crates like [`gdal`][c~gdal~docs]↗{{hi:gdal}} or [`raster`][c~raster~docs]↗{{hi:raster}}. |
| GIS (Geographic Information Systems) | | Full-fledged GIS functionality is still under development. Crates are emerging to address specific aspects. |
| 3D Geospatial Data | | Work is ongoing in this area. Integration with 3D graphics libraries could be a direction. |
| Visualization | [`plotters`][c~plotters~docs]↗{{hi:plotters}}, [`iced`][c~iced~docs]↗{{hi:iced}} (general purpose) | General-purpose plotting libraries can be used to visualize geospatial data. Specialized geospatial visualization crates are less common. |
| Terrain Analysis | Often uses [`raster`][c~raster~docs]↗{{hi:raster}} and numerical computation crates | Terrain analysis can be performed using raster processing and numerical algorithms. |
| Data Processing & Manipulation | [`polars`][c~polars~docs]↗{{hi:polars}}, [`dataframe`][c~dataframe~docs]↗{{hi:dataframe}} | These crates are useful for tabular data processing and are often used in conjunction with geospatial data. |

## Code Examples

{{#include geo.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/958)
review in depth
</div>
