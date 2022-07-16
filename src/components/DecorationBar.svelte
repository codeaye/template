<script>
  import Decorations from "./Decorations.svelte";
</script>

<body>
  <div class="w-full bg-zinc-900 text-white flex h-fit select-none">
    <div class="grow">
      <div
        class="flex items-center justify-center h-full text-lg font-mono subpixel-antialiased"
        data-tauri-drag-region
      >
        Tauri App
      </div>
    </div>
    <div class="flex-none"><Decorations /></div>
  </div>
</body>
