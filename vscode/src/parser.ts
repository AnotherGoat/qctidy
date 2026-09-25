import * as vscode from 'vscode';
const wts = require('web-tree-sitter');

let parser: any = null;

export async function initParser(extensionUri: vscode.Uri): Promise<any> {
    if (parser) {
        return parser;
    }

    // Inicializamos el módulo WebAssembly
    await wts.Parser.init({
        locateFile(scriptName: string) {
            return vscode.Uri.joinPath(extensionUri, 'wasm', scriptName).fsPath;
        },
    });

    const p = new wts.Parser();
    
    // Cargamos las reglas de Python
    const lang = await wts.Language.load(
        vscode.Uri.joinPath(extensionUri, 'wasm', 'tree-sitter-python.wasm').fsPath
    );
    p.setLanguage(lang);
    
    parser = p;
    return p;
}

export function getParser(): any {
    if (!parser) {
        throw new Error('El parser no ha sido inicializado. Llama a initParser primero.');
    }
    return parser;
}
