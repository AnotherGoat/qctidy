import type { GateData, GateState, Grid } from "~/types/circuit";

/**
 * Drop empty columns and push the nodes of every multi-qubit gate to the
 * rightmost column they occupy, so the grid stays compact and aligned.
 */
export function alignGrid(grid: Grid): Grid {
  const newGrid = grid.map((row) => [...row]);
  const multiQubits = new Map<string, { row: number; col: number }[]>();

  for (let row = 0; row < newGrid.length; row++) {
    for (let column = 0; column < newGrid[row].length; column++) {
      const cell = newGrid[row][column];
      if (cell?.props.multiQubitId) {
        if (!multiQubits.has(cell.props.multiQubitId)) {
          multiQubits.set(cell.props.multiQubitId, []);
        }
        multiQubits.get(cell.props.multiQubitId)!.push({ row, col: column });
      }
    }
  }

  let changed = true;
  let iterations = 0;
  while (changed && iterations < 100) {
    changed = false;
    iterations++;
    for (const [id, nodes] of multiQubits.entries()) {
      const currentNodes = nodes.map((node) => {
        const column = newGrid[node.row].findIndex(
          (gate) => gate?.props.multiQubitId === id,
        );
        return { row: node.row, col: column };
      });

      const maxColumn = Math.max(...currentNodes.map((node) => node.col));

      for (const node of currentNodes) {
        if (node.col < maxColumn && node.col !== -1) {
          const difference = maxColumn - node.col;
          for (let i = 0; i < difference; i++) {
            newGrid[node.row].splice(node.col, 0, null);
          }
          changed = true;
        }
      }
    }
  }

  const maxColumns = Math.max(...newGrid.map((row) => row.length), 0);
  for (let column = maxColumns - 1; column >= 0; column--) {
    let isEmpty = true;
    for (let row = 0; row < newGrid.length; row++) {
      if (newGrid[row][column] != null) {
        isEmpty = false;
        break;
      }
    }
    if (isEmpty) {
      for (let row = 0; row < newGrid.length; row++) {
        if (newGrid[row].length > column) {
          newGrid[row].splice(column, 1);
        }
      }
    }
  }

  return newGrid;
}

/** Remove a qubit row, along with any multi-qubit gate that touched it. */
export function removeRow(grid: Grid, rowIndex: number): Grid {
  const rowToDelete = grid[rowIndex];
  if (!rowToDelete) return grid;

  const multiQubitIdsToRemove = new Set(
    rowToDelete
      .map((cell) => cell?.props.multiQubitId)
      .filter((id): id is string => id !== undefined),
  );

  // Replace the removed gates with `null` instead of filtering them out, so the
  // remaining rows keep their columns aligned; `alignGrid` then drops any column
  // that became empty.
  const remainingRows = grid
    .filter((_, index) => index !== rowIndex)
    .map((row) =>
      row.map((cell) =>
        cell?.props.multiQubitId &&
        multiQubitIdsToRemove.has(cell.props.multiQubitId)
          ? null
          : cell,
      ),
    );

  return alignGrid(remainingRows);
}

export interface GateEdition {
  row: number;
  column: number;
  gate: GateState;
}

/**
 * Apply edited properties to a gate, moving the nodes of a multi-qubit gate to
 * the rows given by `moveInstructions` (keyed by role).
 */
export function saveGateProps(
  grid: Grid,
  { row, column, gate }: GateEdition,
  newProps: GateData,
  moveInstructions?: Record<string, number>,
): Grid {
  const newGrid = grid.map((gridRow) => [...gridRow]);

  if (moveInstructions && gate.props.multiQubitId) {
    const group: { r: number; c: number; cell: GateState }[] = [];

    for (let r = 0; r < newGrid.length; r++) {
      for (let c = 0; c < newGrid[r].length; c++) {
        const cell = newGrid[r][c];
        if (cell?.props.multiQubitId === gate.props.multiQubitId) {
          group.push({ r, c, cell });
        }
      }
    }

    if (group.length > 0) {
      const maxRowNeeded = Math.max(...Object.values(moveInstructions));
      while (maxRowNeeded >= newGrid.length) {
        newGrid.push([]);
      }

      for (const item of group) {
        newGrid[item.r][item.c] = null;
      }

      let collision = false;
      for (const item of group) {
        const role = item.cell.props.multiQubitRole;
        const newRow =
          role && moveInstructions[role] !== undefined
            ? moveInstructions[role]
            : item.r;
        if (newGrid[newRow]?.[column] != null) {
          collision = true;
        }
      }

      if (collision) {
        for (let r = 0; r < newGrid.length; r++) {
          newGrid[r].splice(column, 0, null);
        }
      }

      for (const item of group) {
        const role = item.cell.props.multiQubitRole;
        const newRow =
          role && moveInstructions[role] !== undefined
            ? moveInstructions[role]
            : item.r;

        item.cell.props = {
          ...item.cell.props,
          ...newProps,
          multiQubitRole: role,
        };
        newGrid[newRow][column] = item.cell;
      }

      return alignGrid(newGrid);
    }
  }

  const existingCell = newGrid[row]?.[column];

  if (!existingCell) {
    return newGrid;
  }

  newGrid[row][column] = {
    ...existingCell,
    props: newProps,
  };

  return alignGrid(newGrid);
}
