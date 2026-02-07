# Blickfang

`Blickfang` is a modern, cross-platform image viewer application built with SvelteKit and Tauri. It offers a simple interface for browsing images, as well as a few tools for image analysis and transformation.

While the main goal of this app is to provide a simple way to view local images with only the essential features and a minimal UI, it also includes a few fun extras—like converting images to ASCII art—because every image looks best as ASCII art.

This app was mainly built with macOS as the target in mind, but also offers binaries for Windows and Linux.

## Features

- **Comprehensive Image Viewing:** Open and view various image formats including PNG, JPEG, GIF, BMP, and WEBP.
- **Detailed Image Information:** Access and display EXIF metadata and other relevant details embedded within the image.
- **Image Conversion:** Convert images to different formats, such as JPEG, PNG, or WEBP.
- **ASCII Art Conversion:** Transform images into unique ASCII art representations.
- **AI Image Analysis:** Includes functionality check images for hints of AI-generated content. This is done via a simple metadata analysis and is therefore quite unreliable and should be considered a gimmick rather than a serious tool.
- **Customizable User Interface:** Customizable themes and UI elements.
- **Composition Tools:** Grid overlays and edge indicators to assist with image composition and alignment.
- **Configurable Hotkeys:** All actions have configurable hotkeys.
- **Cross-Platform:** Works on Windows, Linux, and macOS.

## Compatibility

- **macOS:** macOS 11.0 (Big Sur) or later. Binaries are built for Apple Silicon (arm64) and are compatible with Intel Macs via Rosetta.
- **Windows:** Windows 10 (1809) or later (x86_64).
- **Linux:** Most modern distributions (x86_64). AppImage¹, `.deb`, and `.rpm` packages are provided.

¹To associate image files with Blickfang when using [AppImageLauncher](https://github.com/TheAssassin/AppImageLauncher), add %U to the Exec line in the .desktop file (e.g., Exec=/home/{yourUserName}/Applications/{BlickfangAppImageName} %U). Alternatively, the app offers automatic launcher integration on first start.

Binaries for all supported platforms are available on the [Releases](https://github.com/erynder-z/blickfang/releases) page.

## Contributing

Contributions are welcome! Please feel free to open issues or submit pull requests.

## License

This project is licensed under the MIT License. See the `LICENSE` file for more details.
