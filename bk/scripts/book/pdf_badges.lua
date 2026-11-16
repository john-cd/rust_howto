-- Use the badge label in PDFs while retaining its outer link.
function Image(image)
    if FORMAT == "latex" and image.src:match("^https://img%.shields%.io/") then
        return image.caption
    end
end
