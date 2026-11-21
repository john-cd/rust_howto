local function improve_pdf_table_widths(table)
    if FORMAT ~= "typst" or #table.colspecs ~= 3 or #table.head.rows == 0 then return end
    local header = table.head.rows[1].cells
    if #header ~= 3 then return end
    local labels = {}
    for index, cell in ipairs(header) do
        labels[index] = pandoc.utils.stringify(cell.contents):lower():gsub("%s+", " ")
    end
    if labels[1] ~= "type family" or labels[2] ~= "types" or labels[3] ~= "examples" then return end

    local widths = { 0.22, 0.52, 0.26 }
    local colspecs = pandoc.List()
    for index, spec in ipairs(table.colspecs) do
        colspecs:insert({ spec[1], widths[index] })
    end
    table.colspecs = colspecs
end

local function normalize_table(table)
    if FORMAT ~= "typst" then return end
    -- mdBook emits empty header rows for headerless tables; Pandoc otherwise
    -- serializes those rows as invalid Typst calls.
    local head = table.head
    head.rows = head.rows:filter(function(row) return #row.cells > 0 end)
    table.head = head
    local bodies = table.bodies
    for _, body in ipairs(bodies) do
        body.head = body.head:filter(function(row) return #row.cells > 0 end)
    end
    table.bodies = bodies
    local foot = table.foot
    foot.rows = foot.rows:filter(function(row) return #row.cells > 0 end)
    table.foot = foot
    improve_pdf_table_widths(table)
    return table
end

return { Table = normalize_table }
