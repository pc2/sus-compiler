pub mod ariadne_interface;

#[cfg(feature = "dot")]
pub mod dot_graphs;

#[cfg(not(feature = "dot"))]
pub mod dot_graphs {
    pub fn display_generated_hardware_structure(
        _md_instance: &crate::instantiation::InstantiatedModule,
        _linker: &crate::linker::Linker,
    ) {
        panic!("sus_compiler was not compiled with the 'dot' feature!");
    }
}

#[cfg(feature = "lsp")]
pub mod lsp;

#[cfg(not(feature = "lsp"))]
pub mod lsp {
    pub fn lsp_main() {
        panic!("LSP not enabled!")
    }
}
