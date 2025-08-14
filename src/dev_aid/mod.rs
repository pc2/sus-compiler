pub mod ariadne_interface;

pub mod dot_graphs;

#[cfg(feature = "lsp")]
pub mod lsp;

#[cfg(not(feature = "lsp"))]
pub mod lsp {
    pub fn lsp_main() -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
        panic!("LSP not enabled!")
    }
}
