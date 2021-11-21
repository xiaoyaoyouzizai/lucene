use crate::store::input::DataInput;

use std::fs;
use std::str;

const CODEC_MAGIC: u32 = 0x3fd76c17;

#[derive(Debug)]
struct Version {
    major: u32,
    /** Minor version, incremented within the stable branch */
    minor: u32,
    /** Bugfix number, incremented on release branches */
    bugfix: u32,
    /** Prerelease version, currently 0 (alpha), 1 (beta), or 2 (final) */
    prerelease: u32,
}

pub struct Segment {
    version: Version,
}

impl Segment {
    pub fn read_latest_commit(filename: &str) -> crate::Result<()> {
        Ok(())
    }

    pub fn read(filename: &str) -> crate::Result<Segment> {
        println!("filename: {}", filename);
        let data = fs::read(filename).unwrap();
        let mut data_input = DataInput::new(&data[..]);

        let actual_header: u32 = data_input.read_int();
        println!("actualHeader: {}", actual_header);
        if actual_header != CODEC_MAGIC {
            return Err(format!(
                "codec header mismatch: actual header={} vs expected header={}",
                actual_header, CODEC_MAGIC
            )
            .into());
        }

        let actual_codec = data_input.read_string();
        println!("actualCodec: {}", actual_codec);

        let actual_version: u32 = data_input.read_int();
        println!("actualVersion: {}", actual_version);

        let index_header_id = data_input.read_bytes(16);
        println!("indexHeaderID: {:02X?}", index_header_id);

        let index_header_suffix = data_input.read_short_string();
        println!("indexHeaderSuffix: {}", index_header_suffix);

        let version = Version {
            major: data_input.read_int(),
            minor: data_input.read_int(),
            bugfix: data_input.read_int(),
            prerelease: 0,
        };
        println!("version: {:#?}", version);

        let mut min_version = Version {
            major: 0,
            minor: 0,
            bugfix: 0,
            prerelease: 0,
        };
        let has_min_version = data_input.read_byte();
        println!("hasMinVersion: {}", has_min_version);

        if has_min_version == 1 {
            min_version.major = data_input.read_int();
            min_version.minor = data_input.read_int();
            min_version.bugfix = data_input.read_int();
        }

        println!("minVersion: {:#?}", min_version);

        let doc_count = data_input.read_int();
        println!("docCount: {}", doc_count);

        let is_compound_file = data_input.read_byte() == 1u8;
        println!("isCompoundFile: {}", is_compound_file);

        let diagnostics = data_input.read_string_map();
        println!("diagnostics: {:#?}", diagnostics);

        let files = data_input.read_string_set();
        println!("files: {:#?}", files);

        let attributes = data_input.read_string_map();
        println!("attributes: {:#?}", attributes);

        let num_sort_fields = data_input.read_vint();
        println!("numSortFields: {}", num_sort_fields);

        Ok(Segment { version: version })
    }
}
