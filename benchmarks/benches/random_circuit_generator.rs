use std::f64;

use rand::rngs::StdRng;
use rand::{RngExt, SeedableRng};

use qctidy::{Graph, GraphBuilder};

const GATE_TYPES: usize = 25;

pub fn generate(seed: u64, width: usize, height: usize) -> Graph {
    assert!(width > 0);
    assert!(height > 0);

    let mut generator = StdRng::seed_from_u64(seed);
    let mut builder = GraphBuilder::new(height);

    for _ in 0..width {
        let mut available_qubits = (0..height).collect::<Vec<_>>();

        while !available_qubits.is_empty() {
            match generator.random_range(0..GATE_TYPES) {
                0 => {
                    let qubit = generator.take_qubit(&mut available_qubits);
                    builder.push_id(qubit);
                }
                1 => {
                    let qubit = generator.take_qubit(&mut available_qubits);
                    builder.push_h(qubit);
                }
                2 => {
                    let qubit = generator.take_qubit(&mut available_qubits);
                    builder.push_x(qubit);
                }
                3 => {
                    let qubit = generator.take_qubit(&mut available_qubits);
                    builder.push_y(qubit);
                }
                4 => {
                    let qubit = generator.take_qubit(&mut available_qubits);
                    builder.push_z(qubit);
                }
                5 => {
                    let qubit = generator.take_qubit(&mut available_qubits);
                    builder.push_p(generator.angle(), qubit).unwrap();
                }
                6 => {
                    let qubit = generator.take_qubit(&mut available_qubits);
                    builder.push_rx(generator.angle(), qubit).unwrap();
                }
                7 => {
                    let qubit = generator.take_qubit(&mut available_qubits);
                    builder.push_ry(generator.angle(), qubit).unwrap();
                }
                8 => {
                    let qubit = generator.take_qubit(&mut available_qubits);
                    builder.push_rz(generator.angle(), qubit).unwrap();
                }
                9 => {
                    let qubit = generator.take_qubit(&mut available_qubits);
                    builder.push_s(qubit);
                }
                10 => {
                    let qubit = generator.take_qubit(&mut available_qubits);
                    builder.push_sdg(qubit);
                }
                11 => {
                    let qubit = generator.take_qubit(&mut available_qubits);
                    builder.push_sx(qubit);
                }
                12 => {
                    let qubit = generator.take_qubit(&mut available_qubits);
                    builder.push_sy(qubit);
                }
                13 => {
                    let qubit = generator.take_qubit(&mut available_qubits);
                    builder.push_t(qubit);
                }
                14 => {
                    let qubit = generator.take_qubit(&mut available_qubits);
                    builder.push_tdg(qubit);
                }
                15 => {
                    let qubit = generator.take_qubit(&mut available_qubits);
                    builder
                        .push_u(
                            generator.angle(),
                            generator.angle(),
                            generator.angle(),
                            qubit,
                        )
                        .unwrap();
                }
                16 => {
                    if available_qubits.len() < 2 {
                        break;
                    }

                    let (qubit1, qubit2) = generator.take_distinct_pair(&mut available_qubits);
                    builder.push_swap(qubit1, qubit2).unwrap();
                }
                17 => {
                    if available_qubits.len() < 2 {
                        break;
                    }

                    let (control, target) = generator.take_distinct_pair(&mut available_qubits);
                    builder.push_ch(control, target).unwrap();
                }
                18 => {
                    if available_qubits.len() < 2 {
                        break;
                    }

                    let (control, target) = generator.take_distinct_pair(&mut available_qubits);
                    builder.push_cx(control, target).unwrap();
                }
                19 => {
                    if available_qubits.len() < 2 {
                        break;
                    }

                    let (control, target) = generator.take_distinct_pair(&mut available_qubits);
                    builder.push_cy(control, target).unwrap();
                }
                20 => {
                    if available_qubits.len() < 2 {
                        break;
                    }

                    let (qubit1, qubit2) = generator.take_distinct_pair(&mut available_qubits);
                    builder.push_cz(qubit1, qubit2).unwrap();
                }
                21 => {
                    if available_qubits.len() < 2 {
                        break;
                    }

                    let (qubit1, qubit2) = generator.take_distinct_pair(&mut available_qubits);
                    builder.push_cp(generator.angle(), qubit1, qubit2).unwrap();
                }
                22 => {
                    if available_qubits.len() < 3 {
                        break;
                    }

                    let [control, target1, target2] =
                        generator.take_distinct_three(&mut available_qubits);
                    builder.push_cswap(control, target1, target2).unwrap();
                }
                23 => {
                    if available_qubits.len() < 3 {
                        break;
                    }

                    let [control1, control2, target] =
                        generator.take_distinct_three(&mut available_qubits);
                    builder.push_ccx(control1, control2, target).unwrap();
                }
                24 => {
                    if available_qubits.len() < 3 {
                        break;
                    }

                    let [qubit1, qubit2, qubit3] =
                        generator.take_distinct_three(&mut available_qubits);
                    builder.push_ccz(qubit1, qubit2, qubit3).unwrap();
                }
                _ => unreachable!(),
            }
        }
    }

    builder.build()
}

trait Generator {
    fn angle(&mut self) -> f64;

    fn take_qubit(&mut self, available_qubits: &mut Vec<usize>) -> usize;

    fn take_distinct_pair(&mut self, available_qubits: &mut Vec<usize>) -> (usize, usize);

    fn take_distinct_three(&mut self, available_qubits: &mut Vec<usize>) -> [usize; 3];
}

impl Generator for StdRng {
    fn angle(&mut self) -> f64 {
        self.random_range(-f64::consts::PI..f64::consts::PI)
    }

    fn take_qubit(&mut self, available_qubits: &mut Vec<usize>) -> usize {
        let index = self.random_range(0..available_qubits.len());
        available_qubits.swap_remove(index)
    }

    fn take_distinct_pair(&mut self, available_qubits: &mut Vec<usize>) -> (usize, usize) {
        let qubit1 = self.take_qubit(available_qubits);
        let qubit2 = self.take_qubit(available_qubits);

        (qubit1, qubit2)
    }

    fn take_distinct_three(&mut self, available_qubits: &mut Vec<usize>) -> [usize; 3] {
        [
            self.take_qubit(available_qubits),
            self.take_qubit(available_qubits),
            self.take_qubit(available_qubits),
        ]
    }
}
