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
    pub fn display_latency_count_graph(
        _lc_problem: &crate::latency::LatencyCountingProblem,
        _wires: &crate::alloc::FlatAlloc<
            crate::instantiation::RealWire,
            crate::prelude::WireIDMarker,
        >,
        _submodules: &crate::alloc::FlatAlloc<
            crate::instantiation::ConcreteSubModule,
            crate::prelude::SubModuleIDMarker,
        >,
        _linker: &crate::linker::Linker,
        _solution: Option<&[i64]>,
        _filename: &str,
    ) {
        panic!("sus_compiler was not compiled with the 'dot' feature!");
    }
}

#[cfg(feature = "lsp")]
pub mod lsp;

#[cfg(not(feature = "lsp"))]
pub mod lsp {
    pub fn lsp_main() -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
        panic!("LSP not enabled!")
    }
}
