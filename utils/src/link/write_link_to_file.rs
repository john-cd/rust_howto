/// Write links and reference definitions to file(s)
use std::borrow::Cow;
use std::io::Write;

use anyhow::Result;

use super::LinkBuilder;

/// Write a reference definition and link to two separate writers /
/// files
pub(crate) fn write_ref_def_and_link_to<W1, W2>(
    lbl: &str,
    image_url: Cow<'_, str>,
    ref_def_writer: &mut W1,
    link_writer: &mut W2,
) -> Result<()>
where
    W1: Write,
    W2: Write,
{
    let link = LinkBuilder::default()
        .set_label(lbl.into())
        .set_image_url(image_url.into())
        .build();
    // writeln!(w, "{}", link.to_reference_definition())?;
    writeln!(ref_def_writer, "{}", link.to_badge_reference_definition())?;

    // writeln!(&mut buf, "{}", link.to_reference_link())?;
    writeln!(link_writer, "{}", link.to_link_with_badge())?;
    Ok(())
}

pub(crate) fn write_links_to<W: Write>(links: Vec<super::Link>, w: &mut W) -> Result<()>
{
    if !links.is_empty() {
            for l in links {
                writeln!(
                    w,
                    "{:?}\n{}\n{}\n{}\n{}\n{}\n",
                    l,
                    l.to_inline_link(),
                    l.to_reference_link(),
                    l.to_reference_definition(),
                    l.to_link_with_badge(),
                    l.to_badge_reference_definition()
                )?;
            }
        }
    Ok(())
}

    // if !links.is_empty() {
    //     for l in links {
    //         writeln!(
    //             &mut f,
    //             "{:?}\n{}\n{}\n",
    //             l,
    //             l.to_reference_link(),
    //             l.to_reference_definition()
    //         )?;
    //     }
    // }
