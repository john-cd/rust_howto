local chapter
local in_category_index = false

local function header(heading)
    local prefix, slug = heading.identifier:match("^(.-%.md__)(.+)$")
    if prefix and prefix ~= chapter then
        chapter = prefix
        in_category_index = slug == "crates-by-category"
    end
end

local function split_lines(inlines)
    local lines = pandoc.List()
    local line = pandoc.Inlines({})

    for _, inline in ipairs(inlines) do
        local html_break = inline.t == "RawInline"
            and inline.format == "html"
            and inline.text:match("^%s*<br%s*/?>%s*$")
        if inline.t == "LineBreak" or html_break then
            if #line > 0 then lines:insert(line) end
            line = pandoc.Inlines({})
        else
            line:insert(inline)
        end
    end
    if #line > 0 then lines:insert(line) end
    return lines
end

-- Turn each table row into a category label and a breakable list. A PDF table
-- cannot split a row containing hundreds of crate links across pages.
local function category_index(table)
    if not in_category_index then return end
    local blocks = pandoc.Blocks({})
    for _, body in ipairs(table.bodies) do
        for _, row in ipairs(body.body) do
            local category, description, entries = row.cells[1], row.cells[2], row.cells[3]
            blocks:insert(pandoc.Para({ pandoc.Strong(pandoc.utils.blocks_to_inlines(category.contents)) }))
            blocks:extend(description.contents)
            local items = pandoc.List()
            for _, entry in ipairs(entries.contents) do
                for _, line in ipairs(split_lines(entry.content)) do
                    items:insert({ pandoc.Plain(line) })
                end
            end
            if #items > 0 then blocks:insert(pandoc.BulletList(items)) end
        end
    end
    if table.caption.long then blocks:extend(table.caption.long) end
    return blocks
end

return {
    { traverse = "topdown", Header = header, Table = category_index },
}
