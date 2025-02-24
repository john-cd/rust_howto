## Geoscience

[![cat-science::geo][cat-science::geo-badge]][cat-science::geo]{{hi:Geoscience}}

Processing of spatial information, maps, navigation data, and geographic information systems.

{{#include geo.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[P2 write](https://github.com/john-cd/rust_howto/issues/958)

- Crates like [`geo`][c-geo]⮳{{hi:geo}}, or [`proj`][c-proj]⮳{{hi:proj}} for tasks such as:
  - Calculating distances and bearings between locations. [`geo`][c-geo]⮳{{hi:geo}}
  - Converting between coordinate systems (e.g., latitude/longitude to UTM). [`proj`][c-proj]⮳{{hi:proj}}
  - Working with geometric shapes (points, lines, polygons).

The Rust geoscience ecosystem is developing, and while not as mature as Python's, it offers promising options, especially for performance-sensitive applications.

| Topic | Rust Crates (Examples) | Notes |
|---|---|---|
| Geospatial Data Structures | [`geo`][c-geo]⮳{{hi:geo}}, [`geos`][c-geos]⮳{{hi:geos}}, [`wkt`][c-wkt]⮳{{hi:wkt}} | [`geo`][c-geo]⮳{{hi:geo}} provides fundamental geospatial types and algorithms. [`geos`][c-geos]⮳{{hi:geos}} offers bindings to the GEOS library for more advanced operations. [`wkt`][c-wkt]⮳{{hi:wkt}} handles Well-Known Text (WKT) parsing and serialization. |
| Geospatial Data Formats | [`gdal`][c-gdal]⮳{{hi:gdal}}, [`raster`][c-raster]⮳{{hi:raster}}, [`geotiff`][c-geotiff]⮳{{hi:geotiff}} | [`gdal`][c-gdal]⮳{{hi:gdal}} provides bindings to the GDAL library for reading and writing various geospatial raster and vector formats. [`raster`][c-raster]⮳{{hi:raster}} provides a pure Rust alternative for working with raster data. [`geotiff`][c-geotiff]⮳{{hi:geotiff}} specifically focuses on GeoTIFF files. |
| Coordinate Reference Systems (CRS) & Projections | [`proj`][c-proj]⮳{{hi:proj}}, `crs` | [`proj`][c-proj]⮳{{hi:proj}} offers bindings to the PROJ library for coordinate transformations. [`crs`][c-crs]⮳{{hi:crs}} is a newer crate focused on CRS management in Rust. |
| Spatial Analysis | (Often built upon [`geo`][c-geo]⮳{{hi:geo}} and [`geos`][c-geos]⮳{{hi:geos}}) | Many spatial analysis operations can be implemented using the core geospatial crates. |
| Geostatistics | (Developing area) | This area is still developing in Rust. Some crates might emerge or be based on numerical computation crates. |
| Remote Sensing | (Often uses [`gdal`][c-gdal]⮳{{hi:gdal}} or [`raster`][c-raster]⮳{{hi:raster}}) | Remote sensing applications often leverage raster processing capabilities provided by crates like [`gdal`][c-gdal]⮳{{hi:gdal}} or [`raster`][c-raster]⮳{{hi:raster}}. |
| GIS (Geographic Information Systems) | (Developing area) | Full-fledged GIS functionality is still under development. Crates are emerging to address specific aspects. |
| 3D Geospatial Data | (Developing area) | Work is ongoing in this area. Integration with 3D graphics libraries could be a direction. |
| Visualization | [`plotters`][c-plotters]⮳{{hi:plotters}}, [`iced`][c-iced]⮳{{hi:iced}} (general purpose) | General-purpose plotting libraries can be used to visualize geospatial data. Specialized geospatial visualization crates are less common. |
| Terrain Analysis | (Often uses [`raster`][c-raster]⮳{{hi:raster}} and numerical computation crates) | Terrain analysis can be performed using raster processing and numerical algorithms. |
| Data Processing & Manipulation | [`polars`][c-polars]⮳{{hi:polars}}, [`dataframe`][c-dataframe]⮳{{hi:dataframe}} | These crates are useful for tabular data processing and are often used in conjunction with geospatial data. |

## Key Considerations

- Maturity: The Rust geoscience ecosystem is evolving. Some areas might have fewer mature options compared to Python's GDAL/OGR or other GIS libraries.
- Performance: Rust's performance can be a significant advantage for geospatial processing, especially for large datasets.
- Interoperability: Bindings to established libraries like GDAL and PROJ are crucial for accessing a wide range of geospatial functionalities.
- Community: The Rust geoscience community is growing, and more libraries and resources are becoming available.

</div>
