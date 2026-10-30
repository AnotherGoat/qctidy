import * as vscode from 'vscode';

import { fixesFor, type FixEdit } from './diagnostics';

/** The context value that marks tree items with an applicable fix. */
export const FIXABLE_CONTEXT = 'qctidyFixable';

const QUICK_FIX = vscode.CodeActionKind.QuickFix;

let analyzed: { uri: string; source: string } | null = null;

/** Remember the source that the latest report was computed from. */
export function rememberAnalyzedSource(uri: vscode.Uri, source: string): void {
    analyzed = { uri: uri.toString(), source };
}

/** Whether the document still matches the source the fixes were computed from. */
function isCurrent(document: vscode.TextDocument): boolean {
    return analyzed !== null
        && analyzed.uri === document.uri.toString()
        && analyzed.source === document.getText();
}

function toRange(edit: FixEdit): vscode.Range {
    return new vscode.Range(edit.line, edit.column, edit.end_line, edit.end_column);
}

/** Keep the first edit of each overlapping group, so a workspace edit stays valid. */
function nonOverlapping(edits: readonly FixEdit[]): FixEdit[] {
    const sorted = [...edits].sort(
        (left, right) => left.line - right.line || left.column - right.column
    );
    const result: FixEdit[] = [];
    let endLine = -1;
    let endColumn = -1;

    for (const edit of sorted) {
        const overlaps = edit.line < endLine
            || (edit.line === endLine && edit.column < endColumn);

        if (overlaps) {
            continue;
        }

        result.push(edit);
        endLine = edit.end_line;
        endColumn = edit.end_column;
    }

    return result;
}

/** Apply fixes to a document, refusing when the document changed since the check. */
export async function applyFixes(
    document: vscode.TextDocument,
    edits: readonly FixEdit[]
): Promise<void> {
    if (edits.length === 0) {
        return;
    }

    if (!isCurrent(document)) {
        void vscode.window.showWarningMessage(
            'QCTidy: the document changed since the last check. Run the check again before applying fixes.'
        );
        return;
    }

    const workspaceEdit = new vscode.WorkspaceEdit();

    for (const edit of nonOverlapping(edits)) {
        workspaceEdit.replace(document.uri, toRange(edit), edit.new_text);
    }

    await vscode.workspace.applyEdit(workspaceEdit);
}

/** Offers the fixes computed by QCTidy as quick fixes (Ctrl+.). */
class QctidyCodeActionProvider implements vscode.CodeActionProvider {
    provideCodeActions(
        document: vscode.TextDocument,
        _range: vscode.Range | vscode.Selection,
        context: vscode.CodeActionContext
    ): vscode.CodeAction[] {
        if (!isCurrent(document)) {
            return [];
        }

        const actions: vscode.CodeAction[] = [];

        for (const diagnostic of context.diagnostics) {
            if (diagnostic.source !== 'qctidy') {
                continue;
            }

            const edits = fixesFor(diagnostic);

            if (edits.length === 0) {
                continue;
            }

            const action = new vscode.CodeAction(`Fix: ${diagnostic.message}`, QUICK_FIX);
            action.diagnostics = [diagnostic];
            action.isPreferred = true;
            action.edit = new vscode.WorkspaceEdit();

            for (const edit of nonOverlapping(edits)) {
                action.edit.replace(document.uri, toRange(edit), edit.new_text);
            }

            actions.push(action);
        }

        return actions;
    }
}

/** Register the quick fix provider for Python documents. */
export function registerFixActions(context: vscode.ExtensionContext): void {
    context.subscriptions.push(
        vscode.languages.registerCodeActionsProvider('python', new QctidyCodeActionProvider(), {
            providedCodeActionKinds: [QUICK_FIX],
        })
    );
}
