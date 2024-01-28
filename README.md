Program that allows you to export pages of a pdf file as images.

# Build
You need rust to build from source code.
```
cargo build --release
```
The executable will be in `target/release/`.

# Usage 
Exporting all pages from a single pdf.
```
img2psd -i path/to/file.pdf -o path/to/output/dir
```

Exporting all pages from a dirictory with pdf files.
```
img2psd -i path/to/input/dir -o path/to/output/dir
```

Write `img2psd.exe --help` to get more information about using it.
