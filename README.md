# \[WIP\] Neorg LSP Server

This repository hosts the LSP implementation for the Neorg file format.

## Features

## Code action

1. We can replace words with there synonyms. Uses `api.dictionaryapi.dev` to get synonyms.

### Hover action 

1. Shows meaning, definitions, examples of words under the cursor via `api.dictionaryapi.dev`.

### Syntax highlighting

#### Neovim 

```lua 
    ["@lsp.type.gfm.heading"] =  { fg = colors.red, bold = true },
    ["@lsp.type.gfm.quote"] =  { fg = colors.red, bold = true },
```

### Code Diagnosis

1. basic syntax errors

## FAQ

