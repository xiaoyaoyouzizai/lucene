use crate::store::ChecksumByteInput;
use crate::store::DataInput;

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
        println!("filename: {}", filename);
        let data = fs::read(filename).unwrap();
        let mut input = DataInput::new(ChecksumByteInput::new(&data[..]));
        let actual_header: u32 = input.read_int();
        println!("actualHeader: {}", actual_header);
        let actual_codec = input.read_string();
        println!("actualCodec: {}", actual_codec);
        let actual_version: u32 = input.read_int();
        println!("actualVersion: {}", actual_version);

        let index_header_id = input.read_bytes(16);
        println!("indexHeaderID: {:02X?}", index_header_id);

        let index_header_suffix = input.read_short_string();
        println!("indexHeaderSuffix: {}", index_header_suffix);

        let lucene_version = Version {
            major: input.read_int(),
            minor: input.read_int(),
            bugfix: input.read_int(),
            prerelease: 0,
        };
        println!("version: {:#?}", lucene_version);

        let index_created_version = input.read_vint();
        println!("index_created_version: {:#?}", index_created_version);

        let version = input.read_long();
        println!("version: {}", version);

        let counter = input.read_vlong(false);
        println!("counter: {}", counter);

        let num_segments = input.read_int();
        println!("num_segments: {}", num_segments);
        let mut min_version = Version {
            major: 0,
            minor: 0,
            bugfix: 0,
            prerelease: 0,
        };
        if num_segments > 0 {
            min_version.major = input.read_vint();
            min_version.minor = input.read_vint();
            min_version.bugfix = input.read_vint();
        }
        println!("min_version: {:#?}", min_version);
        Ok(())
    }

    pub fn read(filename: &str) -> crate::Result<Segment> {
        println!("filename: {}", filename);
        let data = fs::read(filename).unwrap();
        let mut input = DataInput::new(ChecksumByteInput::new(&data[..]));

        let actual_header: u32 = input.read_int();
        println!("actualHeader: {}", actual_header);
        if actual_header != CODEC_MAGIC {
            return Err(format!(
                "codec header mismatch: actual header={} vs expected header={}",
                actual_header, CODEC_MAGIC
            )
            .into());
        }

        let actual_codec = input.read_string();
        println!("actualCodec: {}", actual_codec);

        let actual_version: u32 = input.read_int();
        println!("actualVersion: {}", actual_version);

        let index_header_id = input.read_bytes(16);
        println!("indexHeaderID: {:02X?}", index_header_id);

        let index_header_suffix = input.read_short_string();
        println!("indexHeaderSuffix: {}", index_header_suffix);

        let lucene_version = Version {
            major: input.read_int(),
            minor: input.read_int(),
            bugfix: input.read_int(),
            prerelease: 0,
        };
        println!("version: {:#?}", lucene_version);

        let has_min_version = input.read_byte();
        println!("hasMinVersion: {}", has_min_version);

        let mut min_version = Version {
            major: 0,
            minor: 0,
            bugfix: 0,
            prerelease: 0,
        };
        if has_min_version == 1 {
            min_version.major = input.read_int();
            min_version.minor = input.read_int();
            min_version.bugfix = input.read_int();
        }
        println!("minVersion: {:#?}", min_version);

        let doc_count = input.read_int();
        println!("docCount: {}", doc_count);

        let is_compound_file = input.read_byte() == 1u8;
        println!("isCompoundFile: {}", is_compound_file);

        let diagnostics = input.read_string_map();
        println!("diagnostics: {:#?}", diagnostics);

        let files = input.read_string_set();
        println!("files: {:#?}", files);

        let attributes = input.read_string_map();
        println!("attributes: {:#?}", attributes);

        let num_sort_fields = input.read_vint();
        println!("numSortFields: {}", num_sort_fields);

        Ok(Segment {
            version: lucene_version,
        })
    }
}
