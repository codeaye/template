<script lang="ts">
  export let name: string;

  interface IconType {
    box: number;
    name: string;
    svg: string;
  }

  let icons: IconType[] = [
    {
      box: 24,
      name: "x",
      svg: `<path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12"/>`,
    },
    {
      box: 24,
      name: "maximize",
      svg: `<path stroke-linecap="round" stroke-linejoin="round" d="M4 8V4m0 0h4M4 4l5 5m11-1V4m0 0h-4m4 0l-5 5M4 16v4m0 0h4m-4 0l5-5m11 5l-5-5m5 5v-4m0 4h-4"/>`,
    },
    {
      box: 24,
      name: "minimize",
      svg: `<path stroke-linecap="round" stroke-linejoin="round" d="M20 12H4" />`,
    },
    {
      box: 24,
      name: "cog",
      svg: `<path stroke-linecap="round" stroke-linejoin="round" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" /><path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>`,
    },
    {
      box: 24,
      name: "sun",
      svg: `<path stroke-linecap="round"stroke-linejoin="round"d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"/>`,
    },
    {
      box: 24,
      name: "moon",
      svg: `<path stroke-linecap="round"stroke-linejoin="round"d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"/>`,
    },
  ];

  let displayIcon = icons.find((e: IconType) => e.name === name);
</script>

<svg
  xmlns="http://www.w3.org/2000/svg"
  class={$$props.class}
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  viewBox="0 0 {displayIcon.box} {displayIcon.box}">{@html displayIcon.svg}</svg
>
