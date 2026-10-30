import * as vscode from 'vscode';

import type { AstAnalysis } from './ast';

/**
 * A source code range in 0-based coordinates, matching `vscode.Range`.
 */
export interface JsonSourceLocation {
    line: number;
    column: number;
    end_line: number;
    end_column: number;
}

export interface JsonCircuitPosition {
    qubit: number;
    time_step: number;
}

/**
 * A source edit that applies a fix, in UTF-16 code unit columns.
 */
export interface FixEdit {
    line: number;
    column: number;
    end_line: number;
    end_column: number;
    new_text: string;
}

const fixesByDiagnostic = new WeakMap<vscode.Diagnostic, FixEdit[]>();

/** The fixes attached to a QCTidy diagnostic, empty when it has none. */
export function fixesFor(diagnostic: vscode.Diagnostic): FixEdit[] {
    return fixesByDiagnostic.get(diagnostic) ?? [];
}

/**
 * A single diagnostic emitted by `qctidy check --output-format json`.
 */
export interface CliDiagnostic {
    filename: string;
    circuit: string | null;
    code: string;
    category: string;
    severity: string;
    message: string;
    graph_positions: JsonCircuitPosition[];
    operation_indices: number[];
    source_locations: JsonSourceLocation[];
    fixes?: FixEdit[] | null;
}

/**
 * An uncheckable circuit or global error emitted by `qctidy check --output-format json`.
 */
export interface CliError {
    filename: string;
    circuit: string | null;
    message: string;
}

/**
 * The JSON report structure returned by `qctidy check --output-format json`.
 */
export interface CheckReport {
    diagnostics: CliDiagnostic[];
    errors: CliError[];
    analyses: AstAnalysis[];
}

/**
 * Maps CLI severity string ("error", "warn") to `vscode.DiagnosticSeverity`.
 */
export function mapSeverity(severity: string): vscode.DiagnosticSeverity {
    switch (severity.toLowerCase()) {
        case 'error':
            return vscode.DiagnosticSeverity.Error;
        case 'warn':
        case 'warning':
            return vscode.DiagnosticSeverity.Warning;
        case 'info':
        case 'information':
            return vscode.DiagnosticSeverity.Information;
        case 'hint':
            return vscode.DiagnosticSeverity.Hint;
        default:
            return vscode.DiagnosticSeverity.Warning;
    }
}

/**
 * Converts a CLI diagnostic and a specific source location into a `vscode.Diagnostic`.
 *
 * Mappings applied:
 * - range: `new vscode.Range(loc.line, loc.column, loc.end_line, loc.end_column)`
 * - severity: mapped via `mapSeverity`
 * - code: `diag.code`
 * - source: `'qctidy'`
 * - message: `diag.message`
 * - fixes: attached out of band, so code actions can apply them
 */
export function toVsCodeDiagnostic(
    diag: CliDiagnostic,
    loc: JsonSourceLocation
): vscode.Diagnostic {
    const range = new vscode.Range(loc.line, loc.column, loc.end_line, loc.end_column);
    const severity = mapSeverity(diag.severity);

    const vsDiagnostic = new vscode.Diagnostic(range, diag.message, severity);
    vsDiagnostic.code = diag.code;
    vsDiagnostic.source = 'qctidy';
    fixesByDiagnostic.set(vsDiagnostic, diag.fixes ?? []);
    return vsDiagnostic;
}

/**
 * Converts all diagnostics in a `CheckReport` to an array of `vscode.Diagnostic`.
 */
export function convertReportToVsCodeDiagnostics(report: CheckReport): vscode.Diagnostic[] {
    const result: vscode.Diagnostic[] = [];

    for (const diag of report.diagnostics) {
        if (diag.source_locations && diag.source_locations.length > 0) {
            for (const loc of diag.source_locations) {
                result.push(toVsCodeDiagnostic(diag, loc));
            }
        }
    }

    return result;
}
