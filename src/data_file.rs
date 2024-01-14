use std::fs::File;
use std::error::Error;
use std::io::{Seek, Read};
use std::mem::{size_of, transmute};

/// The number of unicode characters in a value blob
const VALUE_SIZE: usize = 20;

/// A representation of one record
#[repr(C)]
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct Record 
{
  pub key: u32,
  pub value: [char; VALUE_SIZE]
}

/// Encodes the runtime metadata for a data file
pub struct DataFile
{
  file: File,
  number_of_records: usize,
  min_key: u32,
  max_key: u32,
}

/// Transmute a raw byte buffer into a record
fn buffer_to_record(buffer: [u8; size_of::<Record>()]) -> Record
{
   unsafe { transmute::<[u8; size_of::<Record>()], Record>(buffer) }
}

impl DataFile
{
  /// Opens a data file and initializes its metadata
  ///
  /// # Arguments
  ///
  /// * `filename`: The path to the file to be opened
  /// 
  /// The file size must be a multiple of the number of records
  /// 
  /// # Complexity
  /// - Runtime: O(1)
  /// - Memory: O(1)
  /// - IO: O(1)
  ///
  pub fn open(path: &String) 
    -> Result<DataFile,Box<dyn Error>>
  {
    todo!()
  }

  /// Returns the `idx`th record from the file.
  ///
  /// # Arguments
  ///
  /// * `idx`: The index of the record.
  ///
  /// The record to be loaded will begin at byte `idx * size_of::<Record>()`
  /// 
  /// # Complexity
  /// - Runtime: O(1)
  /// - Memory: O(1)
  /// - IO: O(1)
  ///
  pub fn get(&mut self, idx: usize) -> Result<Record,Box<dyn Error>>
  {
    assert!(idx < self.number_of_records);
    todo!()
  }

  /// Returns the number of records in the file
  pub fn len(&self) -> usize
  {
    return self.number_of_records;
  }

  /// Retrieves the record with key `key`, or the immediately following record, if one exists.
  ///
  /// # Arguments
  ///
  /// * `key`: The key of the record to retrieve
  ///
  /// If `key` is present in the data file, the corresponding record should be returned. If
  /// not, then the next highest key in the file should be returned.  If key > file.max_key
  /// then this function should return None.
  ///
  /// # Complexity
  /// - Runtime: O(log_2(N))
  /// - Memory: O(1)
  /// - IO: O(log_2(N))
  ///
  pub fn find(&mut self, key: u32) -> Result<Option<Record>,Box<dyn Error>> 
  {
    todo!()
  }
}

