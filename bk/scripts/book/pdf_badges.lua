local resources = {
    ["crates.io"] = "crates.io",
    ["docs"] = "docs.rs",
    ["lib.rs"] = "lib.rs",
    ["repo"] = "repository",
    ["website"] = "website",
}

local function badge(image)
    if not image.src:match("^https://img%.shields%.io/") then return end
    local label = pandoc.utils.stringify(image.caption)
    local crate, resource = label:match("^([^~]+)~([^~]+)$")
    if resources[resource] and crate ~= "cat" then
        return pandoc.Span(
            { pandoc.Str(crate), pandoc.Space(), pandoc.Str("(" .. resources[resource] .. ")") },
            pandoc.Attr("", {}, {})
        )
    end
    return pandoc.Span(image.caption, pandoc.Attr("", { "pdf-badge" }, {}))
end

local function resource_name(target)
    if target:match("^https?://docs%.rs/") then return "docs.rs" end
    if target:match("^https?://crates%.io/") then return "crates.io" end
    if target:match("^https?://lib%.rs/") then return "lib.rs" end
    if target:match("^https?://github%.com/")
        or target:match("^https?://gitlab%.com/")
        or target:match("^https?://bitbucket%.org/") then
        return "repository"
    end
    return target:match("^#") and "category" or "website"
end

local function label_resource_badge(link)
    local resource = resource_name(link.target)
    for index, inline in ipairs(link.content) do
        if inline.t == "Span" and inline.classes:includes("pdf-badge") then
            local crate = pandoc.utils.stringify(inline.content)
            link.content[index] = pandoc.Span(
                { pandoc.Str(crate), pandoc.Space(), pandoc.Str("(" .. resource .. ")") },
                pandoc.Attr("", {}, {})
            )
        end
    end
end

return { Image = badge, Link = label_resource_badge }
