local chapter
local in_word_index = false

local function header(heading)
    local prefix, slug = heading.identifier:match("^(.-%.md__)(.+)$")
    if prefix and prefix ~= chapter then
        chapter = prefix
        in_word_index = slug == "index" and prefix:match("word_index%.md__$") ~= nil
    end
    if in_word_index and slug == "index" then
        return { heading, pandoc.Para("Each number links to an occurrence of the term in the book.") }
    end
end

-- Split the plugin's hard-break-separated occurrences into individual lines.
local function split_lines(inlines, unwrap_emphasis)
    local lines = pandoc.List()
    local line = pandoc.Inlines({})

    local function append(inline)
        if unwrap_emphasis and inline.t == "Emph" then
            for _, child in ipairs(inline.content) do append(child) end
            return
        end
        local html_break = inline.t == "RawInline"
            and inline.format == "html"
            and inline.text:match("^%s*<br%s*/?>%s*$")
        if inline.t == "LineBreak" or html_break then
            if #line > 0 then lines:insert(line) end
            line = pandoc.Inlines({})
        elseif inline.t ~= "SoftBreak" or #line > 0 then
            line:insert(inline)
        end
    end

    for _, inline in ipairs(inlines) do append(inline) end
    if #line > 0 then lines:insert(line) end
    return lines
end

local function word_index(list)
    if not in_word_index then return end
    local blocks = pandoc.Blocks({})
    for _, item in ipairs(list.content) do
        for _, line in ipairs(split_lines(item[1], true)) do
            blocks:insert(pandoc.Para(line))
        end
        for _, definition in ipairs(item[2]) do
            blocks:extend(definition)
        end
    end
    return blocks
end

local function word_index_paragraph(para)
    if not in_word_index then return end
    local blocks = pandoc.Blocks({})
    for _, line in ipairs(split_lines(para.content, true)) do
        blocks:insert(pandoc.Para(line))
    end
    return blocks
end

return {
    { traverse = "topdown", Header = header, Para = word_index_paragraph, DefinitionList = word_index },
}
