use proc_macros_derive::EnumFromStr;
use proc_macros_derive::MimeTypeFromEnum;

/// `EnumFromStr` -> Adds "from_str(s: &str) -> Result<Self, ()>" function. Ignores case.
/// `MimeTypeFromEnum` -> Adds "mime_type(&self) -> Result<&str, ()>" function.

/// A enum that is used to map file extensions to mime types.
#[derive(PartialEq, Debug, EnumFromStr, MimeTypeFromEnum)]
#[english_number_prefix_to_numerical(true)]
pub enum StaticFileExt {
    AAC, ABW, ARC, AVI, AZW, BIN, BMP, BZ, BZ2, CSH, CSS, CSV, DOC, DOCX, EOT, EPUB, GZ, GIF, HTML,
    ICO, ICS, JAR, JPEG, JPG, JS, JSON, JSONLD, MIDI, MJS, MP3, MPEG, MPKG, ODP, ODS, ODT, OGA, OGV,
    OGX, OPUS, OTF, PNG, PDF, PHP, PPT, PPTX, RAR, RTF, SH, SVG, SWF, TAR, TIFF, TS, TTF, TXT, VSD,
    WAV, WEBA, WEBM, WEBP, WOFF, WOFF2, XHTML, XLS, XLSX, XML, XUL, ZIP, THREEGP, THREEG2, SEVENZ,
    IgnoreThisVariant
}

#[cfg(test)]
mod tests {
    use crate::web::server::data::enums::static_file_ext_enum::StaticFileExt;

    #[test]
    fn it_works() {
        println!("{:#?}", StaticFileExt::from_str("AAC"));
        println!("{:#?}", StaticFileExt::AAC.mime_type());
    }
}