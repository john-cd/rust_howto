// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Builds and tessellates a small vector path with Lyon.

use lyon::math::point;
use lyon::path::Path;
use lyon::tessellation::BuffersBuilder;
use lyon::tessellation::FillOptions;
use lyon::tessellation::FillTessellator;
use lyon::tessellation::FillVertex;
use lyon::tessellation::VertexBuffers;

/// Builds a path and returns the number of generated vertices and indices.
fn tessellate_triangle() -> Result<(usize, usize), lyon::tessellation::TessellationError> {
    let mut builder = Path::builder();
    builder.begin(point(12.0, 12.0));
    builder.line_to(point(72.0, 12.0));
    builder.line_to(point(42.0, 60.0));
    builder.close();
    let path = builder.build();

    let mut geometry = VertexBuffers::<lyon::math::Point, u32>::new();
    FillTessellator::new().tessellate_path(
        &path,
        &FillOptions::default(),
        &mut BuffersBuilder::new(&mut geometry, |vertex: FillVertex<'_>| vertex.position()),
    )?;

    Ok((geometry.vertices.len(), geometry.indices.len()))
}

/// Prints the mesh produced by tessellating a Lyon path.
fn main() -> anyhow::Result<()> {
    run()
}
// ANCHOR_END: example

/// Verifies the example entry point tessellates and reports its path.
#[cfg(test)]
mod tests {
    use super::*;
    /// Produces drawable vertices and indices for a closed triangle fill.
    #[test]
    fn tessellates_a_closed_triangle() -> anyhow::Result<()> {
        let (vertices, indices) = tessellate_triangle()?;
        assert!(vertices >= 3);
        assert!(indices >= 3);
        Ok(())
    }
}
