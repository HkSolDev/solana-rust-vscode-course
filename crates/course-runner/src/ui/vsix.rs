use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};

use super::ExtensionPackage;

struct VsixFile {
    name: String,
    data: Vec<u8>,
}

pub(crate) fn build_vsix(
    root: &Path,
    extension: &ExtensionPackage,
    dry_run: bool,
) -> Result<PathBuf> {
    let extension_dir = root.join("vscode-extension");
    let out_dir = root.join(".course-vsix");
    let vsix_path = out_dir.join(format!("{}-{}.vsix", extension.name, extension.version));
    let mut files = vec![
        VsixFile {
            name: "[Content_Types].xml".to_string(),
            data: content_types_xml().into_bytes(),
        },
        VsixFile {
            name: "extension.vsixmanifest".to_string(),
            data: vsix_manifest_xml(extension).into_bytes(),
        },
    ];

    collect_extension_files(&extension_dir, &extension_dir, &mut files)?;
    let zip = create_zip(&files)?;

    if dry_run {
        println!("Would prepare VSIX at {}", vsix_path.display());
    } else {
        fs::create_dir_all(&out_dir)
            .with_context(|| format!("failed to create {}", out_dir.display()))?;
        fs::write(&vsix_path, zip)
            .with_context(|| format!("failed to write {}", vsix_path.display()))?;
        println!("Prepared VSIX at {}", vsix_path.display());
    }

    Ok(vsix_path)
}

fn collect_extension_files(base: &Path, dir: &Path, files: &mut Vec<VsixFile>) -> Result<()> {
    for entry in fs::read_dir(dir).with_context(|| format!("failed to read {}", dir.display()))? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            collect_extension_files(base, &path, files)?;
        } else if path.is_file() {
            let relative = path
                .strip_prefix(base)
                .context("failed to calculate extension package path")?
                .to_string_lossy()
                .replace('\\', "/");

            files.push(VsixFile {
                name: format!("extension/{relative}"),
                data: fs::read(&path)
                    .with_context(|| format!("failed to read {}", path.display()))?,
            });
        }
    }

    Ok(())
}

fn content_types_xml() -> String {
    r#"<?xml version="1.0" encoding="utf-8"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="json" ContentType="application/json"/>
  <Default Extension="js" ContentType="application/javascript"/>
  <Default Extension="md" ContentType="text/markdown"/>
  <Default Extension="svg" ContentType="image/svg+xml"/>
  <Default Extension="vsixmanifest" ContentType="text/xml"/>
</Types>"#
        .to_string()
}

fn vsix_manifest_xml(extension: &ExtensionPackage) -> String {
    format!(
        r#"<?xml version="1.0" encoding="utf-8"?>
<PackageManifest Version="2.0.0" xmlns="http://schemas.microsoft.com/developer/vsx-schema/2011">
  <Metadata>
    <Identity Language="en-US" Id="{}" Version="{}" Publisher="{}"/>
    <DisplayName>{}</DisplayName>
    <Description xml:space="preserve">{}</Description>
    <Tags>Education</Tags>
    <Categories>Education</Categories>
  </Metadata>
  <Installation>
    <InstallationTarget Id="Microsoft.VisualStudio.Code"/>
  </Installation>
  <Dependencies/>
  <Assets>
    <Asset Type="Microsoft.VisualStudio.Code.Manifest" Path="extension/package.json" Addressable="true"/>
  </Assets>
</PackageManifest>"#,
        escape_xml(&extension.name),
        escape_xml(&extension.version),
        escape_xml(&extension.publisher),
        escape_xml(extension.display_name.as_deref().unwrap_or(&extension.name)),
        escape_xml(extension.description.as_deref().unwrap_or_default())
    )
}

fn escape_xml(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn create_zip(files: &[VsixFile]) -> Result<Vec<u8>> {
    let mut local = Vec::new();
    let mut central = Vec::new();
    let mut offset = 0u32;

    for file in files {
        let name = file.name.as_bytes();
        let data = &file.data;
        let name_len = u16::try_from(name.len()).context("zip file name is too long")?;
        let data_len = u32::try_from(data.len()).context("zip file is too large")?;
        let crc = crc32(data);

        write_u32(&mut local, 0x0403_4b50);
        write_u16(&mut local, 20);
        write_u16(&mut local, 0x0800);
        write_u16(&mut local, 0);
        write_u16(&mut local, 0);
        write_u16(&mut local, 0);
        write_u32(&mut local, crc);
        write_u32(&mut local, data_len);
        write_u32(&mut local, data_len);
        write_u16(&mut local, name_len);
        write_u16(&mut local, 0);
        local.extend_from_slice(name);
        local.extend_from_slice(data);

        write_u32(&mut central, 0x0201_4b50);
        write_u16(&mut central, 20);
        write_u16(&mut central, 20);
        write_u16(&mut central, 0x0800);
        write_u16(&mut central, 0);
        write_u16(&mut central, 0);
        write_u16(&mut central, 0);
        write_u32(&mut central, crc);
        write_u32(&mut central, data_len);
        write_u32(&mut central, data_len);
        write_u16(&mut central, name_len);
        write_u16(&mut central, 0);
        write_u16(&mut central, 0);
        write_u16(&mut central, 0);
        write_u16(&mut central, 0);
        write_u32(&mut central, 0);
        write_u32(&mut central, offset);
        central.extend_from_slice(name);

        offset = u32::try_from(local.len()).context("zip archive is too large")?;
    }

    let central_size =
        u32::try_from(central.len()).context("zip central directory is too large")?;
    let file_count = u16::try_from(files.len()).context("too many files in zip archive")?;
    let mut zip = local;
    zip.extend_from_slice(&central);
    write_u32(&mut zip, 0x0605_4b50);
    write_u16(&mut zip, 0);
    write_u16(&mut zip, 0);
    write_u16(&mut zip, file_count);
    write_u16(&mut zip, file_count);
    write_u32(&mut zip, central_size);
    write_u32(&mut zip, offset);
    write_u16(&mut zip, 0);

    Ok(zip)
}

fn write_u16(out: &mut Vec<u8>, value: u16) {
    out.extend_from_slice(&value.to_le_bytes());
}

fn write_u32(out: &mut Vec<u8>, value: u32) {
    out.extend_from_slice(&value.to_le_bytes());
}

fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xffff_ffffu32;

    for byte in data {
        let mut value = (crc ^ u32::from(*byte)) & 0xff;
        for _ in 0..8 {
            value = if value & 1 == 1 {
                0xedb8_8320 ^ (value >> 1)
            } else {
                value >> 1
            };
        }
        crc = (crc >> 8) ^ value;
    }

    crc ^ 0xffff_ffff
}
