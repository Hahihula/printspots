# PrintSpots

**PrintSpots** is an open-source toolkit for creating multi-layered 3D-printed halftone images. It enables makers, designers, and researchers to turn standard images into printable 3MF objects, calibrated for grayscale and color 3D printing.

👉 Learn more at **[printspots.org](https://printspots.org/)**

---

## Features

* 🎨 Convert images into 3D-printable halftone models
* 🖤 Generate grayscale calibration patterns for printer tuning
* ⚙️ Flexible command-line interface with interactive configuration wizard
* 🔧 Cross-platform configuration management
* 📦 Distributed as binaries for easy installation (source release coming soon)
* 🖨️ Output in **3MF format**, ready for slicing and printing

---

## Installation

Prebuilt binaries will be available soon in the [**Releases**](../../releases) section of this repository.
Download the version for your platform and run it.

Example (Linux/macOS):

```bash
chmod +x printspots
printspots --help
```

---

## Quick Start Guide

### Step 1: Configure Your Printer Settings

Before creating any prints, configure PrintSpots with your printer's specifications:

```bash
printspots configure
```

This interactive wizard will guide you through setting up:
- **Base thickness**: The thickness of the base layer (typically 1.0mm)
- **Layer thickness**: Your printer's layer height (e.g., 0.05mm)
- **Image size**: Default size for the larger dimension of generated images (e.g., 100mm)
- **Maximum layers**: Maximum number of layers to stack (e.g., 19)

The configuration is saved and will be used as defaults for all subsequent commands. You can re-run `configure` anytime to update your settings.

> **Note:** Other commands will work with built-in defaults even without configuration, but for best results, configure your printer first.

---

### Step 2: Generate Calibration Pattern

Create a calibration object to tune your printer for accurate grayscale halftones:

```bash
printspots calibration
```

This generates `calibration.3mf` with test squares at varying thickness levels.

**Printing the calibration:**
1. Open `calibration.3mf` in your slicer
2. The object contains two parts:
   - **Bottom part**: Print in **black** filament
   - **Top part**: Print in **white** filament
3. Slice and print the calibration object

---

### Step 3: Photograph and Process Calibration

After printing the calibration pattern:

1. **Photograph** the printed calibration:
   - Use a white paper background for proper white balance
   - Ensure even lighting
   - Photograph from directly above

2. **Crop** the image to include only the calibration squares

3. **Generate palette file**:
   - Go to the [**Palette Generator Tool**](https://printspots-vuejs-components-cc348b.gitlab.io/)
   - Upload your cropped calibration photo
   - **Align the image** so that:
     - The **darkest square** is labeled **1**
     - Squares get progressively lighter as numbers increase
   - Use flip/rotate tools as needed
   - Download the generated palette file

4. **Edit the palette file**:
   - Open the downloaded `.toml` palette file in a text editor
   - **Remove duplicate entries** where colors are basically the same
   - Keep only distinct grayscale levels
   
   > **Note:** Automatic cutoff will be added to the web tool in a future update.

---

### Step 4: Generate Your Halftone Image

Convert your image into a 3D-printable halftone object:

```bash
printspots generate \
  --input your-image.jpg \
  --palette palette.toml \
  --output result.3mf
```

**Review the preview:**
- A `prediction.png` file is generated in the current directory
- Review this preview to see how your image will look
- If the result doesn't look good:
  - Try a different source image
  - Adjust the contrast and brightness of your original image
  - Re-run the generate command

**Print your object:**
1. Open the output file (e.g., `result.3mf` or `out.3mf`) in your slicer
2. The object contains parts named **"black"** and **"white"**
3. Assign filament colors accordingly:
   - **black** part → black filament
   - **white** part → white filament
4. Slice and print!

---

## Command Reference

### `configure`
Set up your printer configuration interactively.

```bash
printspots configure
```

### `calibration`
Generate a calibration pattern for printer tuning.

```bash
printspots calibration [OPTIONS]

Options:
  -s, --size <SIZE>        Size of single square in mm [default: 10]
      --flat-top           Create calibration with flat top
  -f, --filename <FILE>    Output filename [default: calibration.3mf]
```

### `generate`
Convert an image to a 3D halftone object.

```bash
printspots generate [OPTIONS] --input <IMAGE> --palette <PALETTE> --output <OUTPUT>

Options:
  -i, --input <IMAGE>      Input image file
  -s, --size <SIZE>        Size in mm (larger dimension) [default: 100]
      --flat-top           Create output with flat top
  -p, --palette <PALETTE>  Path to palette file (.toml)
  -o, --output <OUTPUT>    Output 3MF filename
```

---

## Configuration File Location

PrintSpots stores configuration in a platform-specific location:

- **Linux/macOS**: `~/.config/printspots/config.json`
- **Windows**: `%APPDATA%\printspots\config.json`

You can manually edit this file if needed, though using `printspots configure` is recommended.

---

## Documentation

For more information and examples, visit **[printspots.org](https://printspots.org/)**.

Full documentation coming soon.

---

## Roadmap

* ✅ Interactive configuration wizard
* ✅ Website
* 🔜 Binary releases
* 🔜 Automatic palette optimization in web tool
* 🔜 Comprehensive documentation
* 🔜 Open-source release of the full codebase
* 🔜 GUI application
* 🔜 Additional calibration utilities
* 🔜 Color halftone support

---

## Troubleshooting

**Q: The preview looks wrong or pixelated**
- Try adjusting the contrast and brightness of your source image
- Use a higher resolution source image
- Ensure your source image has good tonal range

**Q: Calibration squares look the same**
- Check your printer's layer height settings
- Verify base and layer thickness in your configuration
- Ensure proper filament contrast (pure black and pure white work best)

**Q: Output file parts aren't labeled**
- Open in a slicer that supports 3MF part names (PrusaSlicer, OrcaSlicer, etc.)
- Check that you're using the latest version of PrintSpots

---

## License

MIT

---

## Contributing

Contributions, bug reports, and feature requests are welcome! Please check back after the source code release for contribution guidelines.

---

## Support

- 🌐 Website: [printspots.org](https://printspots.org/)
- 🐛 Issues: [GitLab Issues](../../issues)

---

Made with ❤️ for the 3D printing community