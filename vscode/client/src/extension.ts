import * as path from "path";
import * as fs from "fs";
import * as url from "url";

import {
    workspace,
    EventEmitter,
    ExtensionContext,
    window,
    TextDocumentChangeEvent,
    commands,
} from "vscode";

import {
    Disposable,
    Executable,
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
    TransportKind,
} from "vscode-languageclient/node";

let client: LanguageClient;

export async function activate(context: ExtensionContext) {
    let disposable = commands.registerCommand(
        "dpscript.helloWorld",
        async (uri) => {
            window.showInformationMessage("Hello, world!");

            // const url = Uri.parse(
            //     "/home/victor/Documents/test-dir/nrs/another.nrs"
            // );

            // let document = await workspace.openTextDocument(uri);
            // await window.showTextDocument(document);

            // window.activeTextEditor.document;
            // let editor = window.activeTextEditor;
            // let range = new Range(1, 1, 1, 1);
            // editor.selection = new Selection(range.start, range.end);
        }
    );

    context.subscriptions.push(disposable);

    const traceOutputChannel = window.createOutputChannel(
        "DPScript Language Server trace"
    );

    const folder = workspace.workspaceFolders[0];

    if (!folder) return;

    const root = url.fileURLToPath(folder.uri.toString());
    const devPath = path.resolve(path.join(root, "target", "debug", "dscls"));
    const devPathWin = path.resolve(
        path.join(root, "target", "debug", "dscls")
    );

    let command = process.env.SERVER_PATH || "dscls";

    if (fs.existsSync(devPath)) {
        command = devPath;
    }

    if (fs.existsSync(devPathWin)) {
        command = devPathWin;
    }

    const run: Executable = {
        command,

        transport: TransportKind.stdio,

        options: {
            env: {
                ...process.env,
                RUST_LOG: "debug",
            },
        },
    };

    const serverOptions: ServerOptions = {
        run,
        debug: run,
    };

    const clientOptions: LanguageClientOptions = {
        documentSelector: [
            { scheme: "file", language: "dpscript" },
            { scheme: "file", language: "dps" },
        ],

        synchronize: {
            fileEvents: workspace.createFileSystemWatcher("**/.clientrc"),
        },

        traceOutputChannel,
    };

    client = new LanguageClient(
        "dscls",
        "dpscript language server",
        serverOptions,
        clientOptions
    );

    // activateInlayHints(context);
    client.start();
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }

    return client.stop();
}

export function activateInlayHints(ctx: ExtensionContext) {
    const maybeUpdater = {
        hintsProvider: null as Disposable | null,
        updateHintsEventEmitter: new EventEmitter<void>(),

        async onConfigChange() {
            this.dispose();

            const event = this.updateHintsEventEmitter.event;

            // this.hintsProvider = languages.registerInlayHintsProvider(
            //     { scheme: "file", language: "nrs" },
            //     // new (class implements InlayHintsProvider {
            //     //     onDidChangeInlayHints = event;
            //     //     resolveInlayHint(
            //     //         hint: InlayHint,
            //     //         token: CancellationToken
            //     //     ): ProviderResult<InlayHint> {
            //     //         const ret = {
            //     //             label: hint.label,
            //     //             ...hint,
            //     //         };
            //     //         return ret;
            //     //     }
            //     //     async provideInlayHints(
            //     //         document: TextDocument,
            //     //         range: Range,
            //     //         token: CancellationToken
            //     //     ): Promise<InlayHint[]> {
            //     //         const hints = (await client
            //     //             .sendRequest("custom/inlay_hint", {
            //     //                 path: document.uri.toString(),
            //     //             })
            //     //             .catch((err) => null)) as [
            //     //             number,
            //     //             number,
            //     //             string,
            //     //         ][];
            //     //         if (hints == null) {
            //     //             return [];
            //     //         } else {
            //     //             return hints.map((item) => {
            //     //                 const [start, end, label] = item;
            //     //                 let startPosition = document.positionAt(start);
            //     //                 let endPosition = document.positionAt(end);
            //     //                 return {
            //     //                     position: endPosition,
            //     //                     paddingLeft: true,
            //     //                     label: [
            //     //                         {
            //     //                             value: `${label}`,
            //     //                             // location: {
            //     //                             //   uri: document.uri,
            //     //                             //   range: new Range(1, 0, 1, 0)
            //     //                             // }
            //     //                             command: {
            //     //                                 title: "hello world",
            //     //                                 command:
            //     //                                     "helloworld.helloWorld",
            //     //                                 arguments: [document.uri],
            //     //                             },
            //     //                         },
            //     //                     ],
            //     //                 };
            //     //             });
            //     //         }
            //     //     }
            //     // })()
            // );
        },

        onDidChangeTextDocument({
            contentChanges,
            document,
        }: TextDocumentChangeEvent) {
            // debugger
            // this.updateHintsEventEmitter.fire();
        },

        dispose() {
            this.hintsProvider?.dispose();
            this.hintsProvider = null;
            this.updateHintsEventEmitter.dispose();
        },
    };

    workspace.onDidChangeConfiguration(
        maybeUpdater.onConfigChange,
        maybeUpdater,
        ctx.subscriptions
    );

    workspace.onDidChangeTextDocument(
        maybeUpdater.onDidChangeTextDocument,
        maybeUpdater,
        ctx.subscriptions
    );

    maybeUpdater.onConfigChange().catch(console.error);
}
