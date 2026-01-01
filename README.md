# \[WIP\] Neorg LSP Server

This repository hosts the LSP implementation for the Neorg file format.

## Features

- code action
- goto definitions
- hover capability
- semantic tokens

## Code action

1. We can replace words with there synonyms. Uses `api.dictionaryapi.dev` to get synonyms.

### Hover action 

1. Shows meaning, definitions, examples of words under the cursor via `api.dictionaryapi.dev`.

### Syntax highlighting

#### Neovim 

```lua 
local links = {
  ["markdownH1"] = "@lsp.type.gfm.heading",
  ["markdownH2"] = "@lsp.type.gfm.heading",
  ["markdownH3"] = "@lsp.type.gfm.heading",
  ["markdownH4"] = "@lsp.type.gfm.heading",
  ["markdownH5"] = "@lsp.type.gfm.heading",
  ["markdownH6"] = "@lsp.type.gfm.heading",
  ["markdownH1Delimiter"] = "@lsp.type.gfm.heading",
  ["markdownH2Delimiter"] = "@lsp.type.gfm.heading",
  ["markdownH3Delimiter"] = "@lsp.type.gfm.heading",
  ["markdownH4Delimiter"] = "@lsp.type.gfm.heading",
  ["markdownH5Delimiter"] = "@lsp.type.gfm.heading",
  ["markdownH6Delimiter"] = "@lsp.type.gfm.heading",
}

for from, to in pairs(links) do
 vim.cmd(string.format("highlight! link %s %s", from, to)) 
end

vim.api.nvim_set_hl(0, "@lsp.type.gfm.heading", { fg = "#e52e71", bold = true })
```

### Code Diagnosis

1. basic syntax errors

## FAQ

