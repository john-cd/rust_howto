//! Tessellates a path with Lyon and rasterizes the mesh with tiny-skia.
//!
//! TODO Reviewed ??-??-2026.

// ANCHOR: example
use lyon::math::point;
use lyon::path::Path;
use lyon::tessellation::BuffersBuilder;
use lyon::tessellation::FillOptions;
use lyon::tessellation::FillTessellator;
use lyon::tessellation::FillVertex;
use lyon::tessellation::VertexBuffers;
use tiny_skia::Color;
use tiny_skia::FillRule;
use tiny_skia::Paint;
use tiny_skia::PathBuilder;
use tiny_skia::Pixmap;
use tiny_skia::Transform;

/// Tessellates a triangle: Lyon produces geometry, not pixels.
fn tessellate() -> anyhow::Result<VertexBuffers<[f32; 2], u16>> {
    let mut builder = Path::builder();
    builder.begin(point(8.0, 8.0));
    builder.line_to(point(56.0, 8.0));
    builder.line_to(point(32.0, 56.0));
    builder.close();
    let path = builder.build();

    let mut mesh = VertexBuffers::<[f32; 2], u16>::new();
    FillTessellator::new().tessellate_path(
        &path,
        &FillOptions::default(),
        &mut BuffersBuilder::new(&mut mesh, |vertex: FillVertex<'_>| {
            vertex.position().to_array()
        }),
    )?;
    Ok(mesh)
}

/// Draws each mesh triangle into a CPU pixmap: the application, not Lyon,
/// chooses the renderer.
fn rasterize(mesh: &VertexBuffers<[f32; 2], u16>) -> anyhow::Result<Pixmap> {
    let mut pixmap = Pixmap::new(64, 64)
        .ok_or_else(|| anyhow::anyhow!("could not allocate the pixmap"))?;
    pixmap.fill(Color::WHITE);

    let mut paint = Paint::default();
    paint.set_color_rgba8(30, 110, 200, 255);

    let (triangles, _) = mesh.indices.as_chunks::<3>();
    for [a, b, c] in triangles {
        let [a, b, c] =
            [*a, *b, *c].map(|index| mesh.vertices[usize::from(index)]);
        let mut path = PathBuilder::new();
        path.move_to(a[0], a[1]);
        path.line_to(b[0], b[1]);
        path.line_to(c[0], c[1]);
        path.close();
        let path = path
            .finish()
            .ok_or_else(|| anyhow::anyhow!("degenerate triangle"))?;
        pixmap.fill_path(
            &path,
            &paint,
            FillRule::Winding,
            Transform::identity(),
            None,
        );
    }
    Ok(pixmap)
}

/// Runs the tessellation-then-raster pipeline and reports the result.
fn main() -> anyhow::Result<()> {
    let mesh = tessellate()?;
    let pixmap = rasterize(&mesh)?;
    let painted = pixmap
        .pixels()
        .iter()
        .filter(|pixel| pixel.red() < 255)
        .count();
    println!(
        "{} triangles painted {painted} of {} pixels",
        mesh.indices.len() / 3,
        pixmap.pixels().len()
    );
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point.
    #[test]
    fn test_main_entry_point() -> anyhow::Result<()> {
        main()
    }

    /// Verifies that the triangle's centroid is painted and a corner is not.
    #[test]
    fn rasterizes_the_tessellated_triangle() -> anyhow::Result<()> {
        let pixmap = rasterize(&tessellate()?)?;
        let inside = pixmap.pixel(32, 24).expect("pixel is in bounds");
        let outside = pixmap.pixel(2, 60).expect("pixel is in bounds");
        assert_eq!((inside.red(), inside.blue()), (30, 200));
        assert_eq!(outside.red(), 255);
        Ok(())
    }
}
