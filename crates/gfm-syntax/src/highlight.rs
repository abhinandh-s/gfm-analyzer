//! # SemanticTokenTypes
//!
//! this module exports only one thing
//!
//! const LEGEND_TYPE

#[macro_export]
macro_rules! SemanticTokenType {
    ($tag:literal) => {
        tower_lsp::lsp_types::SemanticTokenType::new($tag)
    };
}

macro_rules! SemanticTokenTypes {
    ($($tag:literal),* $(,)?) => {
        pub const LEGEND_TYPE: &[tower_lsp::lsp_types::SemanticTokenType] = &[
            $(
                tower_lsp::lsp_types::SemanticTokenType::new($tag),
            )*
        ];
    }
}

SemanticTokenTypes! {
    "gfm.text",
    "gfm.heading",
    "gfm.quote",
    "gfm.bold",
    "gfm.italic",
    "gfm.underline",
    "gfm.strikethrough",
    "gfm.spoiler",
    "gfm.superscript",
    "gfm.subscript",
    "gfm.inlinecode",
    "gfm.nullmodifier",
    "gfm.inlinemath",
    "gfm.variable",
}
