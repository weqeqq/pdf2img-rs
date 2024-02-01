Program that allows you to export pages of a pdf file as images.

# Build
You need rust to build from source code.
```
git clone https://github.com/Weqeqq/pdf2img-rs.git
cd pdf2img-rs
cargo build --release
```
The executable will be in `target/release/`.

# Usage 
Pdfium is required for the program to work.
download it here: https://github.com/bblanchon/pdfium-binaries/releases
afterwards just move pdfium.dll from bin/ to the directory where the pdf2img executable file is located.

Exporting all pages from a single pdf.
```
pdf2img -i path/to/file.pdf -o path/to/output/dir
```

Exporting all pages from a directory with pdf files.
```
pdf2img -i path/to/input/dir -o path/to/output/dir
```

Write `pdf2img.exe --help` to get more information about using it.
