local function printable_text(value)
    local label = value.text:gsub("↗", ""):gsub("↳", "—")
    label = label:gsub("cat~([%w:_-]+)", function(slug)
        slug = slug:gsub("::", " / "):gsub("[-_]", " ")
        return (slug:gsub("%f[%a]%l", string.upper))
    end)
    if label == "" then return {} end
    if label ~= value.text then return pandoc.Str(label) end
end

return { Str = printable_text }
