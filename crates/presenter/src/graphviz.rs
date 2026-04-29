use graphviz_rust::{
    cmd::{CommandArg, Format, Layout},
    dot_generator::{attr, edge, id, node, node_id},
    dot_structures::{
        Attribute, Edge, EdgeTy, Graph as GraphvizGraph, Id, Node, NodeId, Stmt, Vertex,
    },
    printer::PrinterContext,
};
use qsimplify::{EdgeType, EdgeView, GateType, Graph, NodeView, PiFormat, formatter};
use std::{fs, io};

const WHITE: &str = "#FFFFFF";
const RED: &str = "#EF9A9A";
const DARK_RED: &str = "#B71C1C";
const GREEN: &str = "#A5D6A7";
const DARK_GREEN: &str = "#1B5E20";
const BLUE: &str = "#90CAF9";
const DARK_BLUE: &str = "#0D47A1";
const ORANGE: &str = "#FFCC80";
const PURPLE: &str = "#CE93D8";
const GRAY: &str = "#EEEEEE";
const DARK_GRAY: &str = "#424242";

#[derive(Debug, Clone, Copy)]
pub enum GraphvizFormat {
    Gv,
    Png,
    Svg,
}

impl From<GraphvizFormat> for Format {
    fn from(format: GraphvizFormat) -> Self {
        use GraphvizFormat::*;

        match format {
            Gv => Self::Gv,
            Png => Self::Png,
            Svg => Self::Svg,
        }
    }
}

pub(crate) fn save_graph_png(graph: &Graph, file_name: &str) -> Result<(), io::Error> {
    let bytes = graph_to_graphviz(graph, GraphvizFormat::Png, Some(150))?;
    fs::write(format!("{file_name}.png"), bytes)
}

pub(crate) fn save_graph_svg(graph: &Graph, file_name: &str) -> Result<(), io::Error> {
    let bytes = graph_to_graphviz(graph, GraphvizFormat::Svg, None)?;
    fs::write(format!("{file_name}.svg"), bytes)
}

pub fn graph_to_graphviz(
    graph: &Graph,
    format: GraphvizFormat,
    dpi: Option<u32>,
) -> Result<Vec<u8>, io::Error> {
    use GraphvizFormat::*;

    let mut statements: Vec<Stmt> = Vec::new();

    apply_graph_attributes(&mut statements, dpi);
    draw_nodes(graph, &mut statements);
    draw_edges(graph, &mut statements);

    let graphviz = GraphvizGraph::DiGraph {
        id: id!(),
        strict: true,
        stmts: statements,
    };

    let mut context = PrinterContext::default();

    match format {
        Gv => {
            let content = graphviz_rust::print(graphviz, &mut context);
            Ok(content.as_bytes().to_vec())
        }
        Png | Svg => graphviz_rust::exec(
            graphviz,
            &mut context,
            vec![
                CommandArg::Layout(Layout::Neato),
                CommandArg::Format(Format::from(format)),
            ],
        ),
    }
}

fn apply_graph_attributes(statements: &mut Vec<Stmt>, dpi: Option<u32>) {
    let mut attributes = vec![
        attr!("scale", "2.75"),
        attr!("nodesep", "0.75"),
        attr!("splines", "ortho"),
    ];

    if let Some(dpi_value) = dpi {
        attributes.push(attr!("dpi", dpi_value.to_string()));
    }

    for attribute in attributes {
        statements.push(Stmt::Attribute(attribute));
    }
}

fn draw_nodes(graph: &Graph, statements: &mut Vec<Stmt>) {
    for node in graph.iter_nodes_ordered_by_row() {
        let (x, y) = find_draw_position(graph, &node);

        let node_id = node_id(&node);
        let node_label = find_node_label(&node);
        let node_color = find_node_color(&node);
        let position = format!("\"{x},{y}!\"");

        let graphviz_node = Stmt::Node(node!(
            node_id;
            attr!("label", node_label),
            attr!("fillcolor", node_color),
            attr!("style", "filled"),
            attr!("shape", "circle"),
            attr!("width", "1.6"),
            attr!("pos", position),
            attr!("pin", "true")
        ));

        statements.push(graphviz_node);
    }
}

fn draw_edges(graph: &Graph, statements: &mut Vec<Stmt>) {
    for edge in graph.iter_edges() {
        let start_id = node_id(edge.start());
        let end_id = node_id(edge.end());
        let label = edge_label(&edge);
        let color = find_edge_color(&edge);

        let graphviz_edge = Stmt::Edge(edge!(
            node_id!(start_id) => node_id!(end_id);
            attr!("taillabel", label),
            attr!("fontcolor", color),
            attr!("labeldistance", "2.0"),
            attr!("style", "dashed"),
            attr!("color", color)
        ));

        statements.push(graphviz_edge);
    }
}

fn node_id(node: &NodeView) -> String {
    format!("\"{}_{}\"", node.position().row(), node.position().column())
}

fn find_draw_position(graph: &Graph, node: &NodeView) -> (usize, usize) {
    (
        node.position().column(),
        graph.height() - node.position().row() - 1,
    )
}

fn find_node_label(node: &NodeView) -> String {
    use GateType::*;

    let type_data = node.r#type().to_string().to_ascii_uppercase();

    let top_label = match node.r#type() {
        P | RX | RY | RZ | CP => {
            let angle = formatter::format_angle(node.angle().unwrap_or_default(), PiFormat::Fancy);
            format!("{type_data}({angle})")
        }
        Measure => {
            format!("M({})", node.bit().unwrap_or_default())
        }
        _ => type_data,
    };

    format!(
        "<<font point-size=\"18\"><b>{}</b></font><br/><font point-size=\"14\" color=\"{}\">{}</font>>",
        top_label,
        DARK_GRAY,
        node.position()
    )
}

fn find_node_color(node: &NodeView) -> String {
    use GateType::*;

    let color = match node.r#type() {
        ID => WHITE,
        H | CH => RED,
        X | RX | SX | CX | CCX => BLUE,
        Y | RY | SY | CY => ORANGE,
        Z | P | RZ | S | SDG | T | TDG | CZ | CP | CCZ => GREEN,
        Swap | CSwap => PURPLE,
        Measure => GRAY,
    };

    format!("\"{color}\"")
}

fn edge_label(edge: &EdgeView) -> String {
    format!("\"{}\"", edge.r#type().to_string().replace('_', " "))
}

fn find_edge_color(edge: &EdgeView) -> String {
    use EdgeType::*;

    let color = match edge.r#type() {
        Right => DARK_GRAY,
        Targets => DARK_BLUE,
        SwapsWith => DARK_RED,
        WorksWith => DARK_GREEN,
    };

    format!("\"{color}\"")
}
