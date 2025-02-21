# rusty-delay

A simple delay plugin (VST3, CLAP) to explore different delay styles and implementations. Programmed in Rust using the [nih-plug](https://github.com/robbert-vdh/nih-plug.git) framework.

## Build Instructions

1. Simply run:
```bash
cargo xtask bundle rusty_delay --release
```

2. Locate VST3 and CLAP files in `target/bundled/`

3. Copy these files to your system's plugin directories:

### MacOS
- VST3: `~/Library/Audio/Plug-Ins/VST3/`
- CLAP: `~/Library/Audio/Plug-Ins/CLAP/`

### Windows
- VST3: `C:\Program Files\Common Files\VST3\`
- CLAP: `C:\Program Files\Common Files\CLAP\`

## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.

Note: This plugin uses the VST3 bindings from nih-plug which are licensed under GPL-3.0, requiring this project to also be GPL-3.0 compliant.