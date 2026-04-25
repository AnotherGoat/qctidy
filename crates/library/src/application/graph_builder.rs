use std::fmt;

use crate::{
    dto::GateOperation,
    {EdgeType, GateType, Graph, Position},
};

/// Provides an interface for easily building `Graphs`.
///
/// This is the recommended way to build a `Graph`, because it automatically builds the required nodes and edges.
// Note: Prefer using the `push_*` methods over the `put_*` methods, since the last ones may break the graph when used incorrectly.
#[derive(Default, Debug)]
#[must_use]
pub struct GraphBuilder {
    graph: Graph,
}

impl fmt::Display for GraphBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.graph)
    }
}

impl GraphBuilder {
    /// Create an empty graph builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Push a gate operation at the end of the graph.
    pub fn push_operation(&mut self, operation: &GateOperation) -> &mut Self {
        use GateOperation::*;

        match *operation {
            ID { qubit } => self.push_id(qubit),
            H { qubit } => self.push_h(qubit),
            X { qubit } => self.push_x(qubit),
            Y { qubit } => self.push_y(qubit),
            Z { qubit } => self.push_z(qubit),
            P { angle, qubit } => self.push_p(angle, qubit),
            RX { angle, qubit } => self.push_rx(angle, qubit),
            RY { angle, qubit } => self.push_ry(angle, qubit),
            RZ { angle, qubit } => self.push_rz(angle, qubit),
            S { qubit } => self.push_s(qubit),
            SDG { qubit } => self.push_sdg(qubit),
            SX { qubit } => self.push_sx(qubit),
            SY { qubit } => self.push_sy(qubit),
            T { qubit } => self.push_t(qubit),
            TDG { qubit } => self.push_tdg(qubit),
            Measure { qubit, bit } => self.push_measure(qubit, bit),
            Swap { qubit1, qubit2 } => self.push_swap(qubit1, qubit2),
            CH { control, target } => self.push_ch(control, target),
            CX { control, target } => self.push_cx(control, target),
            CY { control, target } => self.push_cy(control, target),
            CZ { qubit1, qubit2 } => self.push_cz(qubit1, qubit2),
            CP {
                angle,
                qubit1,
                qubit2,
            } => self.push_cp(angle, qubit1, qubit2),
            CSwap {
                control,
                target1,
                target2,
            } => self.push_cswap(control, target1, target2),
            CCX {
                control1,
                control2,
                target,
            } => self.push_ccx(control1, control2, target),
            CCZ {
                qubit1,
                qubit2,
                qubit3,
            } => self.push_ccz(qubit1, qubit2, qubit3),
        }
    }

    /// Push a ID gate at the end of the graph, which effectively does nothing.
    pub fn push_id(&mut self, _qubit: usize) -> &mut Self {
        self
    }

    /// Push a H gate at the end of the graph.
    pub fn push_h(&mut self, qubit: usize) -> &mut Self {
        self.put_h(qubit, self.find_push_column(&[qubit]))
    }

    /// Push a X gate at the end of the graph.
    pub fn push_x(&mut self, qubit: usize) -> &mut Self {
        self.put_x(qubit, self.find_push_column(&[qubit]))
    }

    /// Push a Y gate at the end of the graph.
    pub fn push_y(&mut self, qubit: usize) -> &mut Self {
        self.put_y(qubit, self.find_push_column(&[qubit]))
    }

    /// Push a Z gate at the end of the graph.
    pub fn push_z(&mut self, qubit: usize) -> &mut Self {
        self.put_z(qubit, self.find_push_column(&[qubit]))
    }

    /// Push a P gate at the end of the graph.
    pub fn push_p(&mut self, angle: f64, qubit: usize) -> &mut Self {
        self.put_p(angle, qubit, self.find_push_column(&[qubit]))
    }

    /// Push a RX gate at the end of the graph.
    pub fn push_rx(&mut self, angle: f64, qubit: usize) -> &mut Self {
        self.put_rx(angle, qubit, self.find_push_column(&[qubit]))
    }

    /// Push a RY gate at the end of the graph.
    pub fn push_ry(&mut self, angle: f64, qubit: usize) -> &mut Self {
        self.put_ry(angle, qubit, self.find_push_column(&[qubit]))
    }

    /// Push a RZ gate at the end of the graph.
    pub fn push_rz(&mut self, angle: f64, qubit: usize) -> &mut Self {
        self.put_rz(angle, qubit, self.find_push_column(&[qubit]))
    }

    /// Push a S gate at the end of the graph.
    pub fn push_s(&mut self, qubit: usize) -> &mut Self {
        self.put_s(qubit, self.find_push_column(&[qubit]))
    }

    /// Push a SDG gate at the end of the graph.
    pub fn push_sdg(&mut self, qubit: usize) -> &mut Self {
        self.put_sdg(qubit, self.find_push_column(&[qubit]))
    }

    /// Push a SX gate at the end of the graph.
    pub fn push_sx(&mut self, qubit: usize) -> &mut Self {
        self.put_sx(qubit, self.find_push_column(&[qubit]))
    }

    /// Push a SY gate at the end of the graph.
    pub fn push_sy(&mut self, qubit: usize) -> &mut Self {
        self.put_sy(qubit, self.find_push_column(&[qubit]))
    }

    /// Push a T gate at the end of the graph.
    pub fn push_t(&mut self, qubit: usize) -> &mut Self {
        self.put_t(qubit, self.find_push_column(&[qubit]))
    }

    /// Push a TDG gate at the end of the graph.
    pub fn push_tdg(&mut self, qubit: usize) -> &mut Self {
        self.put_tdg(qubit, self.find_push_column(&[qubit]))
    }

    /// Push a Measure gate at the end of the graph.
    pub fn push_measure(&mut self, qubit: usize, bit: usize) -> &mut Self {
        self.put_measure(qubit, bit, self.find_push_column(&[qubit]))
    }

    /// Push a Swap gate at the end of the graph.
    pub fn push_swap(&mut self, qubit1: usize, qubit2: usize) -> &mut Self {
        self.put_swap(qubit1, qubit2, self.find_push_column(&[qubit1, qubit2]))
    }

    /// Push a CH gate at the end of the graph.
    pub fn push_ch(&mut self, control_qubit: usize, target_qubit: usize) -> &mut Self {
        self.put_ch(
            control_qubit,
            target_qubit,
            self.find_push_column(&[control_qubit, target_qubit]),
        )
    }

    /// Push a CX gate at the end of the graph.
    pub fn push_cx(&mut self, control_qubit: usize, target_qubit: usize) -> &mut Self {
        self.put_cx(
            control_qubit,
            target_qubit,
            self.find_push_column(&[control_qubit, target_qubit]),
        )
    }

    /// Push a CY gate at the end of the graph.
    pub fn push_cy(&mut self, control_qubit: usize, target_qubit: usize) -> &mut Self {
        self.put_cy(
            control_qubit,
            target_qubit,
            self.find_push_column(&[control_qubit, target_qubit]),
        )
    }

    /// Push a CZ gate at the end of the graph.
    pub fn push_cz(&mut self, qubit1: usize, qubit2: usize) -> &mut Self {
        self.put_cz(qubit1, qubit2, self.find_push_column(&[qubit1, qubit2]))
    }

    /// Push a CP gate at the end of the graph.
    pub fn push_cp(&mut self, angle: f64, qubit1: usize, qubit2: usize) -> &mut Self {
        self.put_cp(
            angle,
            qubit1,
            qubit2,
            self.find_push_column(&[qubit1, qubit2]),
        )
    }

    /// Push a `CSwap` gate at the end of the graph.
    pub fn push_cswap(
        &mut self,
        control_qubit: usize,
        target_qubit1: usize,
        target_qubit2: usize,
    ) -> &mut Self {
        self.put_cswap(
            control_qubit,
            target_qubit1,
            target_qubit2,
            self.find_push_column(&[control_qubit, target_qubit1, target_qubit2]),
        )
    }

    /// Push a CCX gate at the end of the graph.
    pub fn push_ccx(
        &mut self,
        control_qubit1: usize,
        control_qubit2: usize,
        target_qubit: usize,
    ) -> &mut Self {
        self.put_ccx(
            control_qubit1,
            control_qubit2,
            target_qubit,
            self.find_push_column(&[control_qubit1, control_qubit2, target_qubit]),
        )
    }

    /// Push a CCZ gate at the end of the graph.
    pub fn push_ccz(&mut self, qubit1: usize, qubit2: usize, qubit3: usize) -> &mut Self {
        self.put_ccz(
            qubit1,
            qubit2,
            qubit3,
            self.find_push_column(&[qubit1, qubit2, qubit3]),
        )
    }

    /// Put a ID gate directly into the graph, which effectively does nothing.
    pub(crate) fn put_id(&mut self, _qubit: usize, _columnn: usize) -> &mut Self {
        self
    }

    /// Put a H gate directly into the graph, which may break it when used incorrectly.
    pub(crate) fn put_h(&mut self, qubit: usize, column: usize) -> &mut Self {
        self.put_single(GateType::H, qubit, column)
    }

    /// Put a X gate directly into the graph, which may break it when used incorrectly.
    pub(crate) fn put_x(&mut self, qubit: usize, column: usize) -> &mut Self {
        self.put_single(GateType::X, qubit, column)
    }

    /// Put a Y gate directly into the graph, which may break it when used incorrectly.
    pub(crate) fn put_y(&mut self, qubit: usize, column: usize) -> &mut Self {
        self.put_single(GateType::Y, qubit, column)
    }

    /// Put a Z gate directly into the graph, which may break it when used incorrectly.
    pub(crate) fn put_z(&mut self, qubit: usize, column: usize) -> &mut Self {
        self.put_single(GateType::Z, qubit, column)
    }

    fn put_single(&mut self, gate: GateType, qubit: usize, column: usize) -> &mut Self {
        let position = Position::new(qubit, column);

        self.graph.replace_node(gate, position, None, None);
        self.graph
            .connect_row_neighbors(position)
            .expect("The added node should exist");

        #[cfg(debug_assertions)]
        self.graph.validate_internal();
        self
    }

    /// Put a P gate directly into the graph, which may break it when used incorrectly.
    pub(crate) fn put_p(&mut self, angle: f64, qubit: usize, column: usize) -> &mut Self {
        self.put_rotation(GateType::P, angle, qubit, column)
    }

    /// Put a RX gate directly into the graph, which may break it when used incorrectly.
    pub(crate) fn put_rx(&mut self, angle: f64, qubit: usize, column: usize) -> &mut Self {
        self.put_rotation(GateType::RX, angle, qubit, column)
    }

    /// Put a RY gate directly into the graph, which may break it when used incorrectly.
    pub(crate) fn put_ry(&mut self, angle: f64, qubit: usize, column: usize) -> &mut Self {
        self.put_rotation(GateType::RY, angle, qubit, column)
    }

    /// Put a RZ gate directly into the graph, which may break it when used incorrectly.
    pub(crate) fn put_rz(&mut self, angle: f64, qubit: usize, column: usize) -> &mut Self {
        self.put_rotation(GateType::RZ, angle, qubit, column)
    }

    fn put_rotation(
        &mut self,
        gate: GateType,
        angle: f64,
        qubit: usize,
        column: usize,
    ) -> &mut Self {
        let position = Position::new(qubit, column);

        self.graph.replace_node(gate, position, Some(angle), None);
        self.graph
            .connect_row_neighbors(position)
            .expect("The added node should exist");

        #[cfg(debug_assertions)]
        self.graph.validate_internal();
        self
    }

    /// Put a S gate directly into the graph, which may break it when used incorrectly.
    pub(crate) fn put_s(&mut self, qubit: usize, column: usize) -> &mut Self {
        self.put_single(GateType::S, qubit, column)
    }

    /// Put a SDG gate directly into the graph, which may break it when used incorrectly.
    pub(crate) fn put_sdg(&mut self, qubit: usize, column: usize) -> &mut Self {
        self.put_single(GateType::SDG, qubit, column)
    }

    /// Put a SX gate directly into the graph, which may break it when used incorrectly.
    pub(crate) fn put_sx(&mut self, qubit: usize, column: usize) -> &mut Self {
        self.put_single(GateType::SX, qubit, column)
    }

    /// Put a SY gate directly into the graph, which may break it when used incorrectly.
    pub(crate) fn put_sy(&mut self, qubit: usize, column: usize) -> &mut Self {
        self.put_single(GateType::SY, qubit, column)
    }

    /// Put a T gate directly into the graph, which may break it when used incorrectly.
    pub(crate) fn put_t(&mut self, qubit: usize, column: usize) -> &mut Self {
        self.put_single(GateType::T, qubit, column)
    }

    /// Put a TDG gate directly into the graph, which may break it when used incorrectly.
    pub(crate) fn put_tdg(&mut self, qubit: usize, column: usize) -> &mut Self {
        self.put_single(GateType::TDG, qubit, column)
    }

    /// Put a Measure gate directly into the graph, which may break it when used incorrectly.
    pub(crate) fn put_measure(&mut self, qubit: usize, bit: usize, column: usize) -> &mut Self {
        let position = Position::new(qubit, column);

        self.graph
            .replace_node(GateType::Measure, position, None, Some(bit));
        self.graph
            .connect_row_neighbors(position)
            .expect("The added node should exist");

        #[cfg(debug_assertions)]
        self.graph.validate_internal();
        self
    }

    /// Put a Swap gate directly into the graph, which may break it when used incorrectly.
    pub(crate) fn put_swap(&mut self, qubit1: usize, qubit2: usize, column: usize) -> &mut Self {
        let first = Position::new(qubit1, column);
        let second = Position::new(qubit2, column);

        self.graph.replace_node(GateType::Swap, first, None, None);
        self.graph.replace_node(GateType::Swap, second, None, None);

        self.graph
            .add_edge(EdgeType::SwapsWith, first, second)
            .expect("Both nodes should exist");

        self.graph
            .connect_row_neighbors(first)
            .expect("The added node should exist");
        self.graph
            .connect_row_neighbors(second)
            .expect("The added node should exist");

        #[cfg(debug_assertions)]
        self.graph.validate_internal();
        self
    }

    /// Put a CH gate directly into the graph, which may break it when used incorrectly.
    pub(crate) fn put_ch(
        &mut self,
        control_qubit: usize,
        target_qubit: usize,
        column: usize,
    ) -> &mut Self {
        self.put_control(GateType::CH, control_qubit, target_qubit, column)
    }

    /// Put a CX gate directly into the graph, which may break it when used incorrectly.
    pub(crate) fn put_cx(
        &mut self,
        control_qubit: usize,
        target_qubit: usize,
        column: usize,
    ) -> &mut Self {
        self.put_control(GateType::CX, control_qubit, target_qubit, column)
    }

    /// Put a CY gate directly into the graph, which may break it when used incorrectly.
    pub(crate) fn put_cy(
        &mut self,
        control_qubit: usize,
        target_qubit: usize,
        column: usize,
    ) -> &mut Self {
        self.put_control(GateType::CY, control_qubit, target_qubit, column)
    }

    fn put_control(
        &mut self,
        gate: GateType,
        control_qubit: usize,
        target_qubit: usize,
        column: usize,
    ) -> &mut Self {
        let control = Position::new(control_qubit, column);
        let target = Position::new(target_qubit, column);

        self.graph.replace_node(gate, control, None, None);
        self.graph.replace_node(gate, target, None, None);

        self.graph
            .add_edge(EdgeType::Targets, control, target)
            .expect("Both nodes should exist");

        self.graph
            .connect_row_neighbors(control)
            .expect("The added node should exist");
        self.graph
            .connect_row_neighbors(target)
            .expect("The added node should exist");

        #[cfg(debug_assertions)]
        self.graph.validate_internal();
        self
    }

    /// Put a CZ gate directly into the graph, which may break it when used incorrectly.
    pub(crate) fn put_cz(&mut self, qubit1: usize, qubit2: usize, column: usize) -> &mut Self {
        let first = Position::new(qubit1, column);
        let second = Position::new(qubit2, column);

        self.graph.replace_node(GateType::CZ, first, None, None);
        self.graph.replace_node(GateType::CZ, second, None, None);

        self.graph
            .add_edge(EdgeType::WorksWith, first, second)
            .expect("Both nodes should exist");

        self.graph
            .connect_row_neighbors(first)
            .expect("The added node should exist");
        self.graph
            .connect_row_neighbors(second)
            .expect("The added node should exist");

        #[cfg(debug_assertions)]
        self.graph.validate_internal();
        self
    }

    /// Put a CP gate directly into the graph, which may break it when used incorrectly.
    pub(crate) fn put_cp(
        &mut self,
        angle: f64,
        qubit1: usize,
        qubit2: usize,
        column: usize,
    ) -> &mut Self {
        let first = Position::new(qubit1, column);
        let second = Position::new(qubit2, column);

        self.graph
            .replace_node(GateType::CP, first, Some(angle), None);
        self.graph
            .replace_node(GateType::CP, second, Some(angle), None);

        self.graph
            .add_edge(EdgeType::WorksWith, first, second)
            .expect("Both nodes should exist");

        self.graph
            .connect_row_neighbors(first)
            .expect("The added node should exist");
        self.graph
            .connect_row_neighbors(second)
            .expect("The added node should exist");

        #[cfg(debug_assertions)]
        self.graph.validate_internal();
        self
    }

    /// Put a `CSwap` gate directly into the graph, which may break it when used incorrectly.
    pub(crate) fn put_cswap(
        &mut self,
        control_qubit: usize,
        target_qubit1: usize,
        target_qubit2: usize,
        column: usize,
    ) -> &mut Self {
        let control = Position::new(control_qubit, column);
        let target1 = Position::new(target_qubit1, column);
        let target2 = Position::new(target_qubit2, column);

        self.graph
            .replace_node(GateType::CSwap, control, None, None);
        self.graph
            .replace_node(GateType::CSwap, target1, None, None);
        self.graph
            .replace_node(GateType::CSwap, target2, None, None);

        self.graph
            .add_edge(EdgeType::Targets, control, target1)
            .expect("Both nodes should exist");
        self.graph
            .add_edge(EdgeType::Targets, control, target2)
            .expect("Both nodes should exist");
        self.graph
            .add_edge(EdgeType::SwapsWith, target1, target2)
            .expect("Both nodes should exist");

        self.graph
            .connect_row_neighbors(control)
            .expect("The added node should exist");
        self.graph
            .connect_row_neighbors(target1)
            .expect("The added node should exist");
        self.graph
            .connect_row_neighbors(target2)
            .expect("The added node should exist");

        #[cfg(debug_assertions)]
        self.graph.validate_internal();
        self
    }

    /// Put a CCX gate directly into the graph, which may break it when used incorrectly.
    pub(crate) fn put_ccx(
        &mut self,
        control_qubit1: usize,
        control_qubit2: usize,
        target_qubit: usize,
        column: usize,
    ) -> &mut Self {
        let control1 = Position::new(control_qubit1, column);
        let control2 = Position::new(control_qubit2, column);
        let target = Position::new(target_qubit, column);

        self.graph.replace_node(GateType::CCX, control1, None, None);
        self.graph.replace_node(GateType::CCX, control2, None, None);
        self.graph.replace_node(GateType::CCX, target, None, None);

        self.graph
            .add_edge(EdgeType::Targets, control1, target)
            .expect("Both nodes should exist");
        self.graph
            .add_edge(EdgeType::Targets, control2, target)
            .expect("Both nodes should exist");
        self.graph
            .add_edge(EdgeType::WorksWith, control1, control2)
            .expect("Both nodes should exist");

        self.graph
            .connect_row_neighbors(control1)
            .expect("The added node should exist");
        self.graph
            .connect_row_neighbors(control2)
            .expect("The added node should exist");
        self.graph
            .connect_row_neighbors(target)
            .expect("The added node should exist");

        #[cfg(debug_assertions)]
        self.graph.validate_internal();
        self
    }

    /// Put a CCZ gate directly into the graph, which may break it when used incorrectly.
    pub(crate) fn put_ccz(
        &mut self,
        qubit1: usize,
        qubit2: usize,
        qubit3: usize,
        column: usize,
    ) -> &mut Self {
        let first = Position::new(qubit1, column);
        let second = Position::new(qubit2, column);
        let third = Position::new(qubit3, column);

        self.graph.replace_node(GateType::CCZ, first, None, None);
        self.graph.replace_node(GateType::CCZ, second, None, None);
        self.graph.replace_node(GateType::CCZ, third, None, None);

        self.graph
            .add_edge(EdgeType::WorksWith, first, second)
            .expect("Both nodes should exist");
        self.graph
            .add_edge(EdgeType::WorksWith, first, third)
            .expect("Both nodes should exist");
        self.graph
            .add_edge(EdgeType::WorksWith, second, third)
            .expect("Both nodes should exist");

        self.graph
            .connect_row_neighbors(first)
            .expect("The added node should exist");
        self.graph
            .connect_row_neighbors(second)
            .expect("The added node should exist");
        self.graph
            .connect_row_neighbors(third)
            .expect("The added node should exist");

        #[cfg(debug_assertions)]
        self.graph.validate_internal();
        self
    }

    fn find_push_column(&self, qubits: &[usize]) -> usize {
        qubits
            .iter()
            .filter_map(|&row| {
                self.graph
                    .iter_positions_ordered_by_column()
                    .filter(|position| position.row() == row)
                    .max_by_key(Position::column)
                    .map(|position| position.column() + 1)
            })
            .max()
            .unwrap_or(0)
    }

    /// Build the graph to get a working result.
    ///
    /// The output is cloned, so the builder can be reused after this.
    pub fn build(&self) -> Graph {
        self.graph.clone()
    }
}
