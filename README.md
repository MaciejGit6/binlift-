# binlift-

The main goal of this project (for me) is to understand differences between different executable formats on various OSes.

<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1280 820" width="1280" height="820" role="img" aria-label="Three operating systems and their executable formats: Windows uses PE, macOS uses Mach-O, Linux uses ELF">
  <title>What the loader reads first</title>
  <desc>Windows loads PE (magic 4D 5A), macOS loads Mach-O (magic CF FA ED FE), Linux loads ELF (magic 7F 45 4C 46).</desc>

  <style>
    .m    { font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, "DejaVu Sans Mono", monospace; }
    .eyebrow { font-size: 11px; letter-spacing: 3px; fill: #6E7870; }
    .title   { font-size: 30px; font-weight: 700; fill: #1B2320; letter-spacing: -0.5px; }
    .os      { font-size: 13px; letter-spacing: 2.6px; font-weight: 700; }
    .fmt     { font-size: 54px; font-weight: 700; fill: #1B2320; letter-spacing: -1.5px; }
    .expand  { font-size: 13.5px; fill: #6E7870; }
    .grouplbl{ font-size: 10.5px; letter-spacing: 2px; fill: #6E7870; }
    .ruler   { font-size: 9px; letter-spacing: 0.5px; fill: #9AA39A; }
    .hex     { font-size: 19px; font-weight: 700; fill: #1B2320; letter-spacing: 0.5px; }
    .ascii   { font-size: 12px; }
    .caption { font-size: 10.5px; fill: #6E7870; }
    .term    { font-size: 10px; letter-spacing: 1.4px; fill: #9AA39A; }
    .val     { font-size: 12px; fill: #1B2320; }
    .foot    { font-size: 12px; fill: #6E7870; letter-spacing: 0.3px; }
    .cell    { fill: #FAFBF8; }
  </style>

  <rect width="1280" height="820" fill="#E4E7E2"/>

  <!-- header -->
  <text class="m eyebrow" x="64" y="64">EXECUTABLE FORMATS · THREE PLATFORMS</text>
  <text class="m title" x="64" y="100">What the loader reads first</text>
  <line x1="64" y1="128" x2="1216" y2="128" stroke="#C3C9C0" stroke-width="1"/>

  <!-- ============ WINDOWS / PE ============ -->
  <g>
    <rect x="64" y="164" width="362" height="560" fill="#EFF1ED" stroke="#3D5A80" stroke-opacity="0.3"/>
    <rect x="64" y="164" width="362" height="5" fill="#3D5A80"/>

    <text class="m os" x="88" y="212" fill="#3D5A80">WINDOWS</text>
    <text class="m fmt" x="88" y="282">PE</text>
    <text class="m expand" x="88" y="310">Portable Executable</text>
    <line x1="88" y1="340" x2="402" y2="340" stroke="#C3C9C0"/>

    <text class="m grouplbl" x="88" y="370">MAGIC — OFFSET 0x00</text>
    <text class="m ruler" x="115" y="396" text-anchor="middle">00</text>
    <text class="m ruler" x="176" y="396" text-anchor="middle">01</text>

    <rect class="cell" x="88"  y="404" width="54" height="62" stroke="#3D5A80" stroke-opacity="0.45"/>
    <rect class="cell" x="149" y="404" width="54" height="62" stroke="#3D5A80" stroke-opacity="0.45"/>
    <text class="m hex" x="115" y="434" text-anchor="middle">4D</text>
    <text class="m hex" x="176" y="434" text-anchor="middle">5A</text>
    <line x1="98"  y1="443" x2="132" y2="443" stroke="#C3C9C0"/>
    <line x1="159" y1="443" x2="193" y2="443" stroke="#C3C9C0"/>
    <text class="m ascii" x="115" y="459" text-anchor="middle" fill="#3D5A80">M</text>
    <text class="m ascii" x="176" y="459" text-anchor="middle" fill="#3D5A80">Z</text>

    <text class="m grouplbl" x="88" y="502">PE SIGNATURE — AT [0x3C]</text>
    <text class="m ruler" x="115" y="528" text-anchor="middle">+0</text>
    <text class="m ruler" x="176" y="528" text-anchor="middle">+1</text>
    <text class="m ruler" x="237" y="528" text-anchor="middle">+2</text>
    <text class="m ruler" x="298" y="528" text-anchor="middle">+3</text>

    <rect class="cell" x="88"  y="536" width="54" height="62" stroke="#3D5A80" stroke-opacity="0.45"/>
    <rect class="cell" x="149" y="536" width="54" height="62" stroke="#3D5A80" stroke-opacity="0.45"/>
    <rect class="cell" x="210" y="536" width="54" height="62" stroke="#3D5A80" stroke-opacity="0.45"/>
    <rect class="cell" x="271" y="536" width="54" height="62" stroke="#3D5A80" stroke-opacity="0.45"/>
    <text class="m hex" x="115" y="566" text-anchor="middle">50</text>
    <text class="m hex" x="176" y="566" text-anchor="middle">45</text>
    <text class="m hex" x="237" y="566" text-anchor="middle">00</text>
    <text class="m hex" x="298" y="566" text-anchor="middle">00</text>
    <line x1="98"  y1="575" x2="132" y2="575" stroke="#C3C9C0"/>
    <line x1="159" y1="575" x2="193" y2="575" stroke="#C3C9C0"/>
    <line x1="220" y1="575" x2="254" y2="575" stroke="#C3C9C0"/>
    <line x1="281" y1="575" x2="315" y2="575" stroke="#C3C9C0"/>
    <text class="m ascii" x="115" y="591" text-anchor="middle" fill="#3D5A80">P</text>
    <text class="m ascii" x="176" y="591" text-anchor="middle" fill="#3D5A80">E</text>
    <text class="m ascii" x="237" y="591" text-anchor="middle" fill="#9AA39A">·</text>
    <text class="m ascii" x="298" y="591" text-anchor="middle" fill="#9AA39A">·</text>

    <text class="m caption" x="88" y="620">DOS stub jumps here via the 4-byte offset at 0x3C</text>
    <line x1="88" y1="636" x2="402" y2="636" stroke="#C3C9C0"/>

    <text class="m term" x="88" y="660">EXT</text>
    <text class="m val"  x="182" y="660">.exe .dll .sys</text>
    <text class="m term" x="88" y="684">SECTIONS</text>
    <text class="m val"  x="182" y="684">.text .data .rdata</text>
    <text class="m term" x="88" y="708">SYMBOLS</text>
    <text class="m val"  x="182" y="708">COFF table, IAT/EAT</text>
  </g>

  <!-- ============ MACOS / MACH-O ============ -->
  <g>
    <rect x="459" y="164" width="362" height="560" fill="#EFF1ED" stroke="#7A4E7E" stroke-opacity="0.3"/>
    <rect x="459" y="164" width="362" height="5" fill="#7A4E7E"/>

    <text class="m os" x="483" y="212" fill="#7A4E7E">MACOS</text>
    <text class="m fmt" x="483" y="282">MACH-O</text>
    <text class="m expand" x="483" y="310">Mach Object</text>
    <line x1="483" y1="340" x2="797" y2="340" stroke="#C3C9C0"/>

    <text class="m grouplbl" x="483" y="370">MAGIC — OFFSET 0x00</text>
    <text class="m ruler" x="510" y="396" text-anchor="middle">00</text>
    <text class="m ruler" x="571" y="396" text-anchor="middle">01</text>
    <text class="m ruler" x="632" y="396" text-anchor="middle">02</text>
    <text class="m ruler" x="693" y="396" text-anchor="middle">03</text>

    <rect class="cell" x="483" y="404" width="54" height="62" stroke="#7A4E7E" stroke-opacity="0.45"/>
    <rect class="cell" x="544" y="404" width="54" height="62" stroke="#7A4E7E" stroke-opacity="0.45"/>
    <rect class="cell" x="605" y="404" width="54" height="62" stroke="#7A4E7E" stroke-opacity="0.45"/>
    <rect class="cell" x="666" y="404" width="54" height="62" stroke="#7A4E7E" stroke-opacity="0.45"/>
    <text class="m hex" x="510" y="434" text-anchor="middle">CF</text>
    <text class="m hex" x="571" y="434" text-anchor="middle">FA</text>
    <text class="m hex" x="632" y="434" text-anchor="middle">ED</text>
    <text class="m hex" x="693" y="434" text-anchor="middle">FE</text>
    <line x1="493" y1="443" x2="527" y2="443" stroke="#C3C9C0"/>
    <line x1="554" y1="443" x2="588" y2="443" stroke="#C3C9C0"/>
    <line x1="615" y1="443" x2="649" y2="443" stroke="#C3C9C0"/>
    <line x1="676" y1="443" x2="710" y2="443" stroke="#C3C9C0"/>
    <text class="m ascii" x="510" y="459" text-anchor="middle" fill="#9AA39A">·</text>
    <text class="m ascii" x="571" y="459" text-anchor="middle" fill="#9AA39A">·</text>
    <text class="m ascii" x="632" y="459" text-anchor="middle" fill="#9AA39A">·</text>
    <text class="m ascii" x="693" y="459" text-anchor="middle" fill="#9AA39A">·</text>

    <text class="m grouplbl" x="483" y="502">FAT WRAPPER — OFFSET 0x00</text>
    <text class="m ruler" x="510" y="528" text-anchor="middle">00</text>
    <text class="m ruler" x="571" y="528" text-anchor="middle">01</text>
    <text class="m ruler" x="632" y="528" text-anchor="middle">02</text>
    <text class="m ruler" x="693" y="528" text-anchor="middle">03</text>

    <rect class="cell" x="483" y="536" width="54" height="62" stroke="#7A4E7E" stroke-opacity="0.45"/>
    <rect class="cell" x="544" y="536" width="54" height="62" stroke="#7A4E7E" stroke-opacity="0.45"/>
    <rect class="cell" x="605" y="536" width="54" height="62" stroke="#7A4E7E" stroke-opacity="0.45"/>
    <rect class="cell" x="666" y="536" width="54" height="62" stroke="#7A4E7E" stroke-opacity="0.45"/>
    <text class="m hex" x="510" y="566" text-anchor="middle">CA</text>
    <text class="m hex" x="571" y="566" text-anchor="middle">FE</text>
    <text class="m hex" x="632" y="566" text-anchor="middle">BA</text>
    <text class="m hex" x="693" y="566" text-anchor="middle">BE</text>
    <line x1="493" y1="575" x2="527" y2="575" stroke="#C3C9C0"/>
    <line x1="554" y1="575" x2="588" y2="575" stroke="#C3C9C0"/>
    <line x1="615" y1="575" x2="649" y2="575" stroke="#C3C9C0"/>
    <line x1="676" y1="575" x2="710" y2="575" stroke="#C3C9C0"/>
    <text class="m ascii" x="510" y="591" text-anchor="middle" fill="#9AA39A">·</text>
    <text class="m ascii" x="571" y="591" text-anchor="middle" fill="#9AA39A">·</text>
    <text class="m ascii" x="632" y="591" text-anchor="middle" fill="#9AA39A">·</text>
    <text class="m ascii" x="693" y="591" text-anchor="middle" fill="#9AA39A">·</text>

    <text class="m caption" x="483" y="620">Big-endian header wrapping one slice per architecture</text>
    <line x1="483" y1="636" x2="797" y2="636" stroke="#C3C9C0"/>

    <text class="m term" x="483" y="660">EXT</text>
    <text class="m val"  x="577" y="660">(none) .dylib .bundle</text>
    <text class="m term" x="483" y="684">SECTIONS</text>
    <text class="m val"  x="577" y="684">__TEXT,__text</text>
    <text class="m term" x="483" y="708">SYMBOLS</text>
    <text class="m val"  x="577" y="708">LC_SYMTAB, dyld info</text>
  </g>

  <!-- ============ LINUX / ELF ============ -->
  <g>
    <rect x="854" y="164" width="362" height="560" fill="#EFF1ED" stroke="#6B7F3A" stroke-opacity="0.3"/>
    <rect x="854" y="164" width="362" height="5" fill="#6B7F3A"/>

    <text class="m os" x="878" y="212" fill="#6B7F3A">LINUX</text>
    <text class="m fmt" x="878" y="282">ELF</text>
    <text class="m expand" x="878" y="310">Executable and Linkable Format</text>
    <line x1="878" y1="340" x2="1192" y2="340" stroke="#C3C9C0"/>

    <text class="m grouplbl" x="878" y="370">MAGIC — OFFSET 0x00</text>
    <text class="m ruler" x="905" y="396" text-anchor="middle">00</text>
    <text class="m ruler" x="966" y="396" text-anchor="middle">01</text>
    <text class="m ruler" x="1027" y="396" text-anchor="middle">02</text>
    <text class="m ruler" x="1088" y="396" text-anchor="middle">03</text>

    <rect class="cell" x="878" y="404" width="54" height="62" stroke="#6B7F3A" stroke-opacity="0.45"/>
    <rect class="cell" x="939" y="404" width="54" height="62" stroke="#6B7F3A" stroke-opacity="0.45"/>
    <rect class="cell" x="1000" y="404" width="54" height="62" stroke="#6B7F3A" stroke-opacity="0.45"/>
    <rect class="cell" x="1061" y="404" width="54" height="62" stroke="#6B7F3A" stroke-opacity="0.45"/>
    <text class="m hex" x="905" y="434" text-anchor="middle">7F</text>
    <text class="m hex" x="966" y="434" text-anchor="middle">45</text>
    <text class="m hex" x="1027" y="434" text-anchor="middle">4C</text>
    <text class="m hex" x="1088" y="434" text-anchor="middle">46</text>
    <line x1="888" y1="443" x2="922" y2="443" stroke="#C3C9C0"/>
    <line x1="949" y1="443" x2="983" y2="443" stroke="#C3C9C0"/>
    <line x1="1010" y1="443" x2="1044" y2="443" stroke="#C3C9C0"/>
    <line x1="1071" y1="443" x2="1105" y2="443" stroke="#C3C9C0"/>
    <text class="m ascii" x="905" y="459" text-anchor="middle" fill="#9AA39A">·</text>
    <text class="m ascii" x="966" y="459" text-anchor="middle" fill="#6B7F3A">E</text>
    <text class="m ascii" x="1027" y="459" text-anchor="middle" fill="#6B7F3A">L</text>
    <text class="m ascii" x="1088" y="459" text-anchor="middle" fill="#6B7F3A">F</text>

    <text class="m grouplbl" x="878" y="502">E_IDENT — OFFSET 0x04</text>
    <text class="m ruler" x="905" y="528" text-anchor="middle">04</text>
    <text class="m ruler" x="966" y="528" text-anchor="middle">05</text>
    <text class="m ruler" x="1027" y="528" text-anchor="middle">06</text>
    <text class="m ruler" x="1088" y="528" text-anchor="middle">07</text>

    <rect class="cell" x="878" y="536" width="54" height="62" stroke="#6B7F3A" stroke-opacity="0.45"/>
    <rect class="cell" x="939" y="536" width="54" height="62" stroke="#6B7F3A" stroke-opacity="0.45"/>
    <rect class="cell" x="1000" y="536" width="54" height="62" stroke="#6B7F3A" stroke-opacity="0.45"/>
    <rect class="cell" x="1061" y="536" width="54" height="62" stroke="#6B7F3A" stroke-opacity="0.45"/>
    <text class="m hex" x="905" y="566" text-anchor="middle">02</text>
    <text class="m hex" x="966" y="566" text-anchor="middle">01</text>
    <text class="m hex" x="1027" y="566" text-anchor="middle">01</text>
    <text class="m hex" x="1088" y="566" text-anchor="middle">00</text>
    <line x1="888" y1="575" x2="922" y2="575" stroke="#C3C9C0"/>
    <line x1="949" y1="575" x2="983" y2="575" stroke="#C3C9C0"/>
    <line x1="1010" y1="575" x2="1044" y2="575" stroke="#C3C9C0"/>
    <line x1="1071" y1="575" x2="1105" y2="575" stroke="#C3C9C0"/>
    <text class="m ascii" x="905" y="591" text-anchor="middle" fill="#9AA39A">·</text>
    <text class="m ascii" x="966" y="591" text-anchor="middle" fill="#9AA39A">·</text>
    <text class="m ascii" x="1027" y="591" text-anchor="middle" fill="#9AA39A">·</text>
    <text class="m ascii" x="1088" y="591" text-anchor="middle" fill="#9AA39A">·</text>

    <text class="m caption" x="878" y="620">Class 64-bit · little-endian · version 1 · System V ABI</text>
    <line x1="878" y1="636" x2="1192" y2="636" stroke="#C3C9C0"/>

    <text class="m term" x="878" y="660">EXT</text>
    <text class="m val"  x="972" y="660">(none) .so .ko</text>
    <text class="m term" x="878" y="684">SECTIONS</text>
    <text class="m val"  x="972" y="684">.text .data .bss</text>
    <text class="m term" x="878" y="708">SYMBOLS</text>
    <text class="m val"  x="972" y="708">.symtab / .dynsym</text>
  </g>

  <!-- footer -->
  <line x1="64" y1="756" x2="1216" y2="756" stroke="#C3C9C0"/>
  <text class="m foot" x="64" y="784">Different headers, same shape underneath — sections, symbols, entry point.</text>
</svg>
