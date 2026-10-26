<!-- TODO Reviewed ??-??-2026 -->

# Geoscience

[![cat~science::geo][cat~science::geo~badge]][cat~science::geo]{{hi:Geoscience}}

Use geospatial types and algorithms to process coordinates, shapes, and spatial datasets. Keep coordinate reference systems explicit: a distance or area calculation is only meaningful when the input coordinates and units are understood.

| Task                          | Crates                                                                                                                                                                      | Notes                                                                                              |
| ----------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------- |
| Geometry and algorithms       | [`geo`][c~geo~docs]↗{{hi:geo}}, [`geo-types`][c~geo-types~docs]↗{{hi:geo-types}}                                                                                            | Points, lines, polygons, and common operations such as distance and area.                          |
| Geometry formats              | [`wkt`][c~wkt~docs]↗{{hi:wkt}}                                                                                                                                              | Parse and serialize Well-Known Text geometry.                                                      |
| Coordinate transformations    | [`proj`][c~proj~docs]↗{{hi:proj}}                                                                                                                                           | Transform coordinates using PROJ and a defined coordinate reference system.                        |
| Coordinate reference systems  | [`crs`][c~crs~docs]↗{{hi:crs}}                                                                                                                                              | Represent and work with coordinate reference systems; check supported formats and operations.      |
| Raster formats and processing | [`gdal`][c~gdal~docs]↗{{hi:gdal}}, [`raster`][c~raster~docs]↗{{hi:raster}}, [`geotiff`][c~geotiff~docs]↗{{hi:geotiff}}                                                      | Read and write raster data; GDAL may require native libraries, while `geotiff` focuses on GeoTIFF. |
| Vector formats and processing | [`geojson`][c~geojson~docs]↗{{hi:geojson}}, [`shapefile`][c~shapefile~docs]↗{{hi:shapefile}}, [`gdal`][c~gdal~docs]↗{{hi:gdal}}, [`geozero`][c~geozero~docs]↗{{hi:geozero}} | Read, write, and stream common GIS formats; GDAL may require native libraries.                     |
| Spatial indexing              | [`rstar`][c~rstar~docs]↗{{hi:rstar}}                                                                                                                                        | Index spatial objects for intersection and nearest-neighbor queries.                               |
| Hierarchical hexagonal cells  | [`h3o`][c~h3o~docs]↗{{hi:h3o}}                                                                                                                                              | Work with H3 cells and cell relationships.                                                         |

Python's geospatial stack has long used GDAL/OGR for raster and vector data. Rust offers GDAL bindings and specialized crates for common formats and operations; compare format support, native dependencies, and analysis features before choosing a stack.

## Example

{{#include geo.incl.md}}

## Related Topics

- [[data-processing | Data Processing]].
- [[visualization | Visualization]].
- [[coordinate-systems | Coordinate Systems and Projections]].
- [[environmental-science | Environmental Science]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  content audit: category index content removed by recent commits; review and restore
</div>
