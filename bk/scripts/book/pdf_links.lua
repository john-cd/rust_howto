local chapter_targets = {}
local heading_targets = {}
local chapter_sources = {}
local source_targets = {}
local link_chapter

local function decode_percent_escapes(value)
    return (value:gsub("%%(%x%x)", function(hex)
        return string.char(tonumber(hex, 16))
    end))
end

local function source_path_from_target(path)
    local normalized = decode_percent_escapes(path):gsub("\\", "/")
    normalized = normalized:gsub("^file:/+", "")
    local source = normalized:match("/src/(.+%.md)$")
    if source then return source:lower() end
end

local function source_path(prefix)
    local encoded = prefix:match("__src__(.+)%.md__$")
    if encoded then return (encoded:gsub("__", "/") .. ".md"):lower() end
    local normalized = decode_percent_escapes(prefix):gsub("\\", "/"):gsub("__$", "")
    local source = normalized:match("/src/(.+%.md)$")
    if source then return source:lower() end
end

local function header(heading)
    local prefix = heading.identifier:match("^(.-%.md__)")
    if prefix and prefix ~= link_chapter then
        link_chapter = prefix
        chapter_targets[prefix] = heading.identifier
        local path = source_path(prefix)
        if path then
            chapter_sources[prefix] = path
            source_targets[path] = prefix
        end
    end
    if prefix then
        if FORMAT == "typst" and heading_targets[heading.identifier] then
            local base = heading.identifier
            local suffix = 2
            repeat
                heading.identifier = base .. "-pdf-" .. suffix
                suffix = suffix + 1
            until not heading_targets[heading.identifier]
        end
        heading_targets[heading.identifier] = true
    end
end

local function anchor(span)
    if not link_chapter or span.identifier == "" then return end
    local destination = span.identifier
    if destination:sub(1, #link_chapter) ~= link_chapter then
        destination = link_chapter .. destination
    end
    if FORMAT == "typst" and heading_targets[destination] then
        local base = destination
        local suffix = 2
        repeat
            destination = base .. "-pdf-" .. suffix
            suffix = suffix + 1
        until not heading_targets[destination]
    end
    heading_targets[destination] = true
    if FORMAT == "typst" then
        return pandoc.RawInline("typst", "#box(width: 0pt)[]<" .. destination .. ">")
    end
    span.identifier = destination
    return span
end

local function resolve_source_path(current_prefix, path)
    local current = chapter_sources[current_prefix]
    if not current then return end
    path = path:gsub("\\", "/")
    local directory = current:match("^(.*)/[^/]+$") or ""
    local joined = path:sub(1, 1) == "/" and path:sub(2) or (directory .. "/" .. path)
    local parts = {}
    for part in joined:gmatch("[^/]+") do
        if part == ".." then
            if #parts > 0 then table.remove(parts) end
        elseif part ~= "." then
            parts[#parts + 1] = part
        end
    end
    return table.concat(parts, "/"):lower()
end

local function link_destination(source, fragment)
    local prefix = source_targets[source]
    if not prefix then return end
    local destination = fragment == "" and chapter_targets[prefix] or prefix .. fragment
    if destination and heading_targets[destination] then return destination end
    local pandoc_fragment = fragment:gsub("^%d+", "")
    if pandoc_fragment ~= fragment then
        destination = prefix .. pandoc_fragment
        if heading_targets[destination] then return destination end
    end
end

local function resolve_link(link)
    local scoped_prefix, scoped_fragment = link.target:match("^#(.+%.md__)(.+)$")
    if scoped_prefix then
        local destination = scoped_prefix .. scoped_fragment
        if heading_targets[destination] then return link end
        local pandoc_fragment = scoped_fragment:gsub("^%d+", "")
        destination = scoped_prefix .. pandoc_fragment
        if heading_targets[destination] then
            link.target = "#" .. destination
            return link
        end
    end
    local file_label = link.target:match("^#(.+%.md)$")
    local first_heading = file_label and chapter_targets[file_label .. "__"]
    if first_heading then
        link.target = "#" .. first_heading
        return link
    end
    local path, fragment = link.target:match("^https?://john%-cd%.com/rust_howto/(.-)%.html#?(.*)$")
    if path then
        local destination = link_destination((path .. ".md"):lower(), fragment)
        if destination then
            link.target = "#" .. destination
            return link
        end
    end
    if not link.target:match("^https?://") then
        local relative_path, relative_fragment = link.target:match("^(.-%.md)#?(.*)$")
        if relative_path then
            local source = source_path_from_target(relative_path)
                or resolve_source_path(link_chapter, relative_path)
            local destination = source and link_destination(source, relative_fragment)
            if destination then
                link.target = "#" .. destination
                return link
            end
        end
    end
end

return {
    { traverse = "topdown", Header = header, Span = anchor },
    { traverse = "topdown", Header = function(heading)
          link_chapter = heading.identifier:match("^(.-%.md__)") or link_chapter
      end, Link = resolve_link },
}
