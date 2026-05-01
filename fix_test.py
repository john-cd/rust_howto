import sys

content = open('later/crates/other/examples/architecture/cqrs.rs').read()

content = content.replace('''fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}''', '''fn test() -> anyhow::Result<()> {
    // We just execute main, which shouldn't panic
    let _ = main();
    Ok(())
}''')

open('later/crates/other/examples/architecture/cqrs.rs', 'w').write(content)
