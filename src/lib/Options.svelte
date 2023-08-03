<script lang="ts">
    import Icon from "./Icon.svelte";

    let showOptions = false;
    let selectedPaper = "a4";

    export let PdfOptions = {
        landscape: true,
        display_header_footer: true,
        print_background: true,
        scale: 1.0,
        paper_width: 8.5,
        paper_height: 11,
        margin_top: 0.4,
        margin_bottom: 0.4,
        margin_left: 0.4,
        margin_right: 0.4,
        page_ranges: null,
        ignore_invalid_page_ranges: null,
        header_template: "<div></div>",
        footer_template: "<div></div>",
        prefer_css_page_size: null,
        transfer_mode: null,
    };
</script>

<div>
    <button
        on:click={() => (showOptions = !showOptions)}
        class="btn btn-ghost btn-sm"
    >
        <span>Options</span>
        <Icon icon={showOptions ? "chevron-up" : "chevron-down"} size="24" />
    </button>
</div>
{#if showOptions}
    <div class="max-w-lg">
        <div class="join">
            <button
                class="btn btn-primary {selectedPaper === 'letter'
                    ? 'btn-active'
                    : ''}"
                on:click={() => {
                    PdfOptions.paper_width = 8.5;
                    PdfOptions.paper_height = 11;
                    selectedPaper = "letter";
                }}
            >
                Letter
            </button>
            <button
                class="btn btn-primary {selectedPaper === 'a4'
                    ? 'btn-active'
                    : ''}"
                on:click={() => {
                    PdfOptions.paper_width = 8.27;
                    PdfOptions.paper_height = 11.69;
                    selectedPaper = "a4";
                }}
            >
                DIN A4
            </button>
            <button
                class="btn btn-primary {selectedPaper === 'a3'
                    ? 'btn-active'
                    : ''}"
                on:click={() => {
                    PdfOptions.paper_width = 11.69;
                    PdfOptions.paper_height = 16.54;
                    selectedPaper = "a3";
                }}
            >
                DIN A3
            </button>
        </div>

        <label class="label">
            <span class="labeltext">Scale</span>
            <input
                type="number"
                bind:value={PdfOptions.scale}
                step="0.1"
                class="input input-bordered"
            />
        </label>

        <label class="label cursor-pointer">
            <span class="labeltext">Landscape</span>
            <input
                type="checkbox"
                bind:checked={PdfOptions.landscape}
                class="checkbox"
            />
        </label>

        <div class="label">
            <h2>Margins:</h2>
            <span />
        </div>

        <label class="label cursor-pointer">
            <span class="labeltext">Top</span>
            <input
                type="number"
                bind:value={PdfOptions.margin_top}
                class="input input-bordered"
            />
        </label>

        <label class="label cursor-pointer">
            <span class="labeltext">Bottom</span>
            <input
                type="number"
                bind:value={PdfOptions.margin_bottom}
                class="input input-bordered"
            />
        </label>

        <label class="label cursor-pointer">
            <span class="labeltext">Left</span>
            <input
                type="number"
                bind:value={PdfOptions.margin_left}
                class="input input-bordered"
            />
        </label>

        <label class="label cursor-pointer">
            <span class="labeltext">Right</span>
            <input
                type="number"
                bind:value={PdfOptions.margin_right}
                class="input input-bordered"
            />
        </label>
    </div>
{/if}
