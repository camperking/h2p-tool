<script lang="ts">
    import feather, { icons } from 'feather-icons';

    export let icon: keyof typeof icons;

    export let size = "24";
    export let strokeWidth = 2;
    let customClass = "";
    export { customClass as class };
  
    if (size !== "100%") {
        size = size.slice(-1) === 'x' 
            ? size.slice(0, size.length -1) + 'em'
            : parseInt(size) + 'px';
    }
</script>

<svg
    xmlns="http://www.w3.org/2000/svg"
    width={size}
    height={size}
    fill="none"
    viewBox="0 0 24 24"
    stroke="currentColor"
    stroke-width="{strokeWidth}"
    stroke-linecap="round"
    stroke-linejoin="round"
    class="feather feather-{icon} {customClass}"
>
    {@html feather.icons[icon].toString()}
</svg>