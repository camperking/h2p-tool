<script lang="ts">
    import { open } from "@tauri-apps/api/dialog";
    import { invoke } from "@tauri-apps/api/tauri";
    import { exit } from "@tauri-apps/api/process";
    import { open as openShell } from "@tauri-apps/api/shell";
    import Icon from "./lib/Icon.svelte";
    import Options from "./lib/Options.svelte";

    let step = 0;
    let outputDir = "";
    let files: string[] = [];

    let loading = false;
    let success = false;
    let error = false;

    let options;

    async function chooseOutDir() {
        const selected = await open({
            multiple: false,
            directory: true,
        });

        if (selected === null) return;

        if (!Array.isArray(selected)) {
            // user selected a directory
            outputDir = selected;
            step = 1;
        }        
    }

    async function chooseFile() {
        const selected = await open({
            multiple: true,
            filters: [
                {
                    name: "Hypertext Markup Language",
                    extensions: ["htm", "html"],
                },
            ],
        });

        if (Array.isArray(selected)) {
            // user selected multiple files
            files = selected;
        } else if (selected === null) {
            // user cancelled the selection
            return;
        } else {
            // user selected a single file
            files = [selected];
        }

        step = 2;
    }

    async function convert() {
        step = -1;
        loading = true;
        try {
            await invoke("convert", { outputDir, files, options });
            loading = false;
            success = true;
        } catch (error) {
            loading = false;
            error = true;
            console.error(error);
        }
    }

    function reset() {
        step = 0;
        outputDir = "";
        files = [];
        loading = false;
        success = false;
        error = false;
    }

    function exitApp() {
        exit(0);
    }
</script>

<main class="flex flex-col justify-between h-screen items-center">
    
    <div class="flex flex-col items-center text-center pt-[10vh]">
        <div class="space-y-8">
            {#if step === 0}
                <h1>1. Output directory</h1>
                <p>Choose the directory where the pdf file(s) will be saved.</p>
                <button class="btn btn-primary" on:click={chooseOutDir}>
                    <span>Choose directory</span>
                </button>
            {:else if step === 1}
                <h1>2. Choose File(s)</h1>
                <p>Choose one or more html file to convert to pdf.</p>
                <button class="btn btn-primary" on:click={chooseFile}>
                    File(s)
                </button>
            {:else if step === 2}
                <h1>3. Convert</h1>
                <div class="badge badge-success">Output directory:</div>
                <br />{outputDir}
                <div>
                    <div class="overflow-x-auto">
                        <table class="table table-xs">
                            <thead>
                                <tr>
                                    <th>File(s)</th>
                                </tr>
                            </thead>
                            <tbody>
                                {#each files as file}
                                    <tr>
                                        <td>{file}</td>
                                    </tr>
                                {/each}
                            </tbody>
                        </table>
                    </div>
                </div>
                <button class="btn btn-primary" on:click={convert}>
                    Convert
                </button>
                <Options bind:PdfOptions={options} />
            {/if}
    
            {#if loading}
                <h1>Converting</h1>
                <span class="loading loading-spinner loading-lg" />
            {/if}
            {#if success}
                <h1>Success</h1>
                <p>Files converted successfully</p>
                <button
                    class="btn link link-hover link-success"
                    on:click={() => openShell(outputDir)}
                >
                    <span>Open directory</span>
                    <Icon icon="external-link" size="16" />
                </button>
                <div>
                    <button class="btn btn-primary" on:click={reset}> New </button>
                    <button class="btn btn-secondary" on:click={exitApp}>
                        Exit
                    </button>
                </div>
            {/if}
            {#if error}
                <h1>Error</h1>
                <p>Something went wrong</p>
                <div>
                    <button class="btn btn-primary" on:click={reset}> New </button>
                    <button class="btn btn-secondary" on:click={exitApp}>
                        Exit
                    </button>
                </div>
            {/if}
        </div>
    </div>

    <footer>
        <button
            on:click={() => openShell("https://github.com/camperking/h2p-tool")}
        >
            <Icon icon="github" />
        </button>
    </footer>

</main>
