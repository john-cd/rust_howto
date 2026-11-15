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
    let (vertices, indices) = tessellate_triangle()?;
    println!("Generated mesh with {vertices} vertices and {indices} indices.");
    Ok(())
}
// ANCHOR_END: example

/// Runs the Lyon vector-graphics example.
pub fn run() -> anyhow::Result<()> {
    main()
}

/// Exercises the entry point and tessellation helper.
#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its private entry point.
    #[test]
    fn example_runs() -> anyhow::Result<()> {
        main()
    }

    /// Produces drawable vertices and indices for a closed triangle fill.
    #[test]
    fn tessellates_a_closed_triangle() -> anyhow::Result<()> {
        let (vertices, indices) = tessellate_triangle()?;
        assert!(vertices >= 3);
        assert!(indices >= 3);
        Ok(())
    }
}
