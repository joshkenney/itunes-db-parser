/**
 * File: photo_database.rs
 *
 * Provides functionality around working with the Photo Database internals file. Photo analogue of 'itunesdb.rs'
 * http://www.ipodlinux.org/ITunesDB/#Photo_Database
 */

use crate::helpers::*;

pub struct Image {
    pub filename : String,
    pub file_size_bytes: u64,
    pub file_size_human_readable : String,
    pub original_date_epoch: u64,
    pub original_date_ts : chrono::DateTime<chrono::Utc>,
    pub digitized_date_epoch : u64,
    pub digitized_date_ts : chrono::DateTime<chrono::Utc>
}

/// Allows instantiation of a "default" Image,
/// since each property/field of the image struct will be populated
/// at a different time
impl Default for Image {

    fn default() -> Image {

        return Image{filename: "".to_string(), file_size_bytes: 0, file_size_human_readable: "".to_string(), original_date_epoch: 0, original_date_ts: get_timestamp_as_mac(0), digitized_date_epoch : 0, digitized_date_ts : super::helpers::get_timestamp_as_mac(0)};
    }
}

impl Image {

    pub fn set_original_date(&mut self, orig_date_epoch : u64) { 

        self.original_date_epoch = orig_date_epoch;
        self.original_date_ts = super::helpers::get_timestamp_as_mac(orig_date_epoch);
    }

    pub fn set_filesize(&mut self, filesize_in_bytes : u64) {

        self.file_size_bytes = filesize_in_bytes;

        const ONE_MB_AS_BYTES : f64 = 1000000_f64;
        const ONE_KB_AS_BYTES : f64 = 1000_f64;

        // Shows the size of the size of the image, in whatever the most
        // appropriate is, specifically, chooses the largest possible unit
        // that still results in a greater-than-1 value
        // ie. "1245916" will be shown as "1.245 MB", because
        // with KB, the value would be 1245.916 KB, but with GB
        // it would be smaller than 1
        let human_readable_size : String;

        let size_in_kb = filesize_in_bytes as f64 / ONE_KB_AS_BYTES;
        let size_in_mb = filesize_in_bytes as f64 / ONE_MB_AS_BYTES;

        if size_in_mb < 1.0f64 {

            // Megabytes was too small, choose the next smallest unit

            human_readable_size = format!("{:.2} KB", size_in_kb);
        } else {
            human_readable_size = format!("{:.2} MB", size_in_mb);
        }

        self.file_size_human_readable = human_readable_size;

    }


    pub fn set_digitized_date(&mut self, digitized_date_epoch : u64) {

        self.digitized_date_epoch = digitized_date_epoch;
        self.digitized_date_ts = super::helpers::get_timestamp_as_mac(digitized_date_epoch);
    }

    pub fn set_filename(&mut self, filename : String) {

        self.filename = super::itunesdb_helpers::get_canonical_path(filename);
    }

    pub fn are_dates_valid(&mut self) -> bool {

        return (self.original_date_epoch > 0) && (self.digitized_date_epoch > 0);
    }
}

    pub enum MhodType {
        ALBUM_NAME = 1,
        THUMBNAIL_IMAGE = 2,
        FILE_NAME = 3,
        CONTAINER = 5
    }

    // impl std::fmt::Display for MhodType {

    //     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    //         write!(f, "{:?}", self);
    //         // or, alternatively:
    //         //std::fmt::Debug::fmt(self, f);
    //     }

    // }

    /// See "MHOD types" table in Photos Database section
    pub fn decode_mhod_type(mhod_type : u16) -> String {

        let mhod_type_name : String;
        
        if mhod_type == 1 {
            mhod_type_name = String::from("Album Name");
        }

        else if mhod_type == 2 {
            mhod_type_name = String::from("Thumbnail image");
        }

        else if mhod_type == 3 {
            mhod_type_name = String::from("File name");
        }

        else if mhod_type == 5 {
            mhod_type_name = String::from("Container (unused)");
        }

        else {
            // panic!("{} is not a supported mhod type", mhod_type);
            // I would normally have panicked here, since the wiki doesn't mention any other valid mhod types,
            // but in my testing I found that for some reason, I was seeing mhod type "6" in the photo database file,
            // which shouldn't be possible...
            mhod_type_name = format!("Unsupported ({})", mhod_type);
        }

        return mhod_type_name;
    }

