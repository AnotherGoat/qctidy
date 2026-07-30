use qctidy_ports::DisplayError;
use std::sync::Arc;

use getset::{CloneGetters, CopyGetters, Getters};
use inew::New;
use qctidy::{Circuit, DiracFormat, Graph, PiFormat};

#[derive(Debug, Clone, CloneGetters, CopyGetters, New)]
#[new(pub, const)]
#[must_use]
pub struct DisplayRequest {
    #[get_clone = "pub"]
    circuit: Arc<Circuit>,
    #[get_copy = "pub"]
    format: DisplayFormat,
    #[get_copy = "pub"]
    pi_format: Option<PiFormat>,
    #[get_copy = "pub"]
    dirac_format: Option<DiracFormat>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayFormat {
    Graph,
    Grid,
    Matrix,
    Circuit,
}

#[derive(Debug, Clone, Getters, New)]
#[new(pub, const)]
#[must_use]
pub struct DisplayResponse {
    #[get = "pub"]
    text: String,
}

pub fn display(request: &DisplayRequest) -> Result<DisplayResponse, DisplayError> {
    use DisplayFormat::{Grid, Matrix};

    let graph: Graph = request.circuit().as_ref().into();

    let pi_format = request.pi_format().unwrap_or(PiFormat::Fancy);
    let dirac_format = request.dirac_format().unwrap_or(DiracFormat::Fancy);

    let text = match request.format() {
        DisplayFormat::Graph => graph.display_nodes_and_edges(pi_format),
        Grid => graph.display_grid(pi_format),
        Matrix => graph.display_matrix(dirac_format),
        DisplayFormat::Circuit => request.circuit().display(pi_format),
    };

    Ok(DisplayResponse::new(text))
}
