# PrintSpots

**PrintSpots** is an open-source toolkit for creating multi-layered 3D-printed halftone images. It enables makers, designers, and researchers to turn standard images into printable 3MF objects, calibrated for grayscale and color 3D printing.

👉 Learn more at **[printspots.org](https://printspots.org/)**

---

## Features

* 🎨 Convert images into 3D-printable halftone models
* 🖤 Generate grayscale calibration patterns for printer tuning
* ⚙️ Flexible command-line interface with both defaults and interactive TUI (GUI comming soon)
* 📦 Distributed as binaries for easy installation (source release coming soon)
* 🖨️ Output in **3MF format**, ready for slicing and printing

---

## Installation

Prebuilt binaries are available in the [**Releases**](../../releases) section of this repository.
Download the version for your platform and run it.

Example (Linux/macOS):

```bash
chmod +x printspots
printspots --help
```

---

## Usage Overview

### Grayscale Calibration

Generate a calibration object to tune your printer for grayscale halftones:

```bash
printspots grayscale-calibration --default
```

This produces a 3MF file with test squares at varying thickness levels.

---

### Process an Image

Turn an image into a 3MF halftone object:

```bash
printspots process-image --image-path input.jpg --palette-path palette_gray.toml --default
```

More details and examples are available on the [webpage](https://printspots.org/).

---

## Documentation

For now information and examples can be found at **[printspots.org](https://printspots.org/)**.

Full documentation comming soon.

---

## Roadmap

* ✅ Initial binary releases
* ✅ Website
* 🔜 Documentation
* 🔜 Open-source release of the full codebase
* 🔜 Additional calibration utilities

---

## License

PrintSpots will be released under an open-source license (to be announced at source release).
For now, binaries are distributed for evaluation and use. But you are free to use resultion 3d objects as you wish, even comertially.
 
