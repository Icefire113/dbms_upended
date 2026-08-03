use std::{
    fmt::Debug,
    fs::{self, File},
    io::{BufReader, Write},
    marker::PhantomData,
    path::{Path, PathBuf},
};

use rkyv::{
    Archive, Deserialize, Serialize, api::high::HighValidator, bytecheck::CheckBytes,
    rancor::Error, seal::Seal,
};
use xxhash_rust::xxh3::xxh3_64;

use crate::{
    dbms::bplus_tree::error::{BPlusTreeError, PageError},
    util::{self},
};

pub mod error;

const PAGE_SIZE: usize = 4096;

const BP_META_FILENAME: &str = "bp_meta";
const BP_META_MAGIC: [u8; 2] = *b"BM";
const BP_META_VERSION: u16 = 0;

const BP_PAGE_MAGIC: [u8; 2] = *b"BN";
const BP_PAGE_VERSION: u16 = 0;

/// A type that represents the header for a page file on disk
#[allow(unused)]
struct PageFileHeader {
    pub magic: [u8; 2],
    pub version: u16,
    pub data_size: u32,
    pub checksum: u64,
}

/// Size in bytes of the header for a node
const BP_PAGE_HEADER_SIZE: usize = std::mem::size_of::<PageFileHeader>();

type PageId = u64;

/// A composite key that stores the acutal key as well as a unique id to allow for duplicate keys
type CompositeKey<K> = (u64, K);

/// A B+ Tree that is stored on disk
#[derive(Debug)]
pub struct BPlusTree<K, V>
where
    K: Ord + Debug,
    V: Debug,
{
    root_path: PathBuf,
    root_node_id: PageId,
    num_keys: u64,
    num_elements: u64,
    num_pages: u32,
    // Markers so the compiler won't complain
    _k: PhantomData<K>,
    _v: PhantomData<V>,
}

#[derive(Debug, Archive, Serialize, Deserialize, PartialEq)]
struct LeafNode<K, V>
where
    K: Ord,
{
    pub items: Vec<(CompositeKey<K>, V)>,
    pub next: Option<PageId>,
    pub prev: Option<PageId>,
}

#[derive(Debug, Archive, Serialize, Deserialize, PartialEq)]
struct InternalNode<K>
where
    K: Ord,
{
    pub keys: Vec<CompositeKey<K>>,
    pub children: Vec<PageId>,
}

#[derive(Debug, Archive, Serialize, Deserialize, PartialEq)]
enum Node<K, V>
where
    K: Ord,
{
    Leaf(LeafNode<K, V>),
    Internal(InternalNode<K>),
}

impl<K, V> BPlusTree<K, V>
where
    K: Ord
        + Debug
        + rkyv::Archive
        + for<'a> rkyv::Serialize<
            rkyv::rancor::Strategy<
                rkyv::ser::Serializer<
                    rkyv::util::AlignedVec,
                    rkyv::ser::allocator::ArenaHandle<'a>,
                    rkyv::ser::sharing::Share,
                >,
                rkyv::rancor::Error,
            >,
        >,
    V: Debug
        + rkyv::Archive
        + for<'a> rkyv::Serialize<
            rkyv::rancor::Strategy<
                rkyv::ser::Serializer<
                    rkyv::util::AlignedVec,
                    rkyv::ser::allocator::ArenaHandle<'a>,
                    rkyv::ser::sharing::Share,
                >,
                rkyv::rancor::Error,
            >,
        >,
    for<'a> <K as Archive>::Archived: CheckBytes<HighValidator<'a, Error>>,
    for<'a> <V as Archive>::Archived: CheckBytes<HighValidator<'a, Error>>,
{
    /// Creates a new B+ Tree in a given directory
    pub fn new(root_path: impl AsRef<Path>) -> Result<Self, BPlusTreeError> {
        let root_path: PathBuf = root_path.as_ref().to_path_buf();

        // yes this has time of check time of use problems, but it's better to tell the user early if something looks wrong
        let meta = fs::metadata(&root_path)?;
        if !meta.is_dir() {
            return Err(BPlusTreeError::RootPathNotDirectory(root_path));
        }

        if let Ok(tree_meta) = File::options()
            .read(true)
            .open(root_path.join(BP_META_FILENAME))
        {
            let mut reader = BufReader::new(tree_meta);
            if util::read_n_bytes(&mut reader, 2)? != BP_META_MAGIC {
                return Err(BPlusTreeError::MetaDataFileInvalidMagic);
            }
            let ver: u16 = util::read_u16_le(&mut reader)?;
            if ver != BP_META_VERSION {
                return Err(BPlusTreeError::MetaDataFileVersionMismatch(ver));
            }
            let num_pages: u32 = util::read_u32_le(&mut reader)?;
            let num_keys: u64 = util::read_u64_le(&mut reader)?;
            let num_elements: u64 = util::read_u64_le(&mut reader)?;
            let root_node_id: u64 = util::read_u64_le(&mut reader)?;
            let page_size: u64 = util::read_u64_le(&mut reader)?;
            if page_size != PAGE_SIZE as u64 {
                return Err(BPlusTreeError::MetaDataInvalidPageSize(
                    PAGE_SIZE as u64,
                    page_size,
                ));
            }

            Ok(Self {
                root_path,
                root_node_id,
                num_keys,
                num_elements,
                num_pages,
                _k: PhantomData,
                _v: PhantomData,
            })
        } else {
            // meta doesn't exist, so we assume that the tree is empty and should create one
            let mut tree_meta = File::options()
                .create_new(true)
                .write(true)
                .open(root_path.join(BP_META_FILENAME))
                .map_err(BPlusTreeError::MetaDataFileIo)?;
            let mut tree_meta_bytes: Vec<u8> = Vec::new();
            tree_meta_bytes.extend(BP_META_MAGIC);
            tree_meta_bytes.extend(BP_META_VERSION.to_le_bytes());
            tree_meta_bytes.extend(1u32.to_le_bytes()); // num pages
            tree_meta_bytes.extend(0u64.to_le_bytes()); // num keys
            tree_meta_bytes.extend(0u64.to_le_bytes()); // num elements
            tree_meta_bytes.extend(0u64.to_le_bytes()); // root node id
            tree_meta
                .write_all(&tree_meta_bytes)
                .map_err(BPlusTreeError::MetaDataFileIo)?;
            tree_meta.flush()?;

            // Construct a new empty root
            let mut root_page: [u8; PAGE_SIZE] = [0u8; _];
            root_page[0..2].copy_from_slice(&BP_PAGE_MAGIC);
            root_page[2..4].copy_from_slice(&BP_PAGE_VERSION.to_le_bytes());
            let empty_leaf_node: Node<K, V> = Node::Leaf(LeafNode {
                items: Vec::new(),
                next: None,
                prev: None,
            });
            let node_bytes = rkyv::to_bytes(&empty_leaf_node)?;
            if node_bytes.len() > u32::MAX as usize {
                panic!("empty node is > u32::MAX")
            }

            root_page[4..8].copy_from_slice(&(node_bytes.len() as u32).to_le_bytes()); // data size
            root_page[8..16].copy_from_slice(&xxh3_64(&node_bytes).to_le_bytes()); // checksum
            root_page[16..16 + node_bytes.len()].copy_from_slice(&node_bytes); // data

            let mut root_page_file = File::options()
                .read(true)
                .write(true)
                .create_new(true)
                .open(root_path.join("page_0"))?;
            root_page_file.write(&root_page)?;
            root_page_file.flush()?;

            Ok(Self {
                root_path,
                root_node_id: 0,
                num_keys: 0,
                num_elements: 0,
                num_pages: 1,
                _k: PhantomData,
                _v: PhantomData,
            })
        }
    }

    /// Gets a mutable reference to a node, note that the file data is read into `data_buff`
    fn get_node_mut<'a>(
        &self,
        id: PageId,
        data_buff: &'a mut Vec<u8>,
    ) -> Result<Seal<'a, ArchivedNode<K, V>>, PageError> {
        let page = self.get_writable_page_handle(id)?;
        let mut reader = BufReader::new(page);

        let (data_size, checksum) = self.verify_page_header(id, &mut reader)?;

        util::read_n_bytes_to_buff(&mut reader, data_buff, data_size as usize)?;

        if checksum != xxh3_64(&data_buff) {
            return Err(PageError::InvalidHash(id));
        }

        Ok(rkyv::access_mut(&mut data_buff[0..(data_size as usize)])?)
    }

    /// Gets a refrence to a node, note that the file data is read into `data_buff`
    fn get_node<'a>(
        &self,
        id: PageId,
        data_buff: &'a mut Vec<u8>,
    ) -> Result<&'a ArchivedNode<K, V>, PageError> {
        let page = self.get_page_handle(id)?;
        let mut reader = BufReader::new(page);

        let (data_size, checksum) = self.verify_page_header(id, &mut reader)?;
        debug_assert!(
            data_size as usize <= PAGE_SIZE,
            "page data exceeds page size"
        );

        util::read_n_bytes_to_buff(&mut reader, data_buff, data_size as usize)?;

        if checksum != xxh3_64(&data_buff) {
            return Err(PageError::InvalidHash(id));
        }

        Ok(rkyv::access(&data_buff[0..(data_size as usize)])?)
    }

    /// Verifies that the header is valid and returns the data size and checksum
    fn verify_page_header(
        &self,
        id: PageId,
        page_reader: &mut BufReader<File>,
    ) -> Result<(u32, u64), PageError> {
        let magic: [u8; 2] = util::read_n_bytes_const(page_reader)?;
        if magic != BP_PAGE_MAGIC {
            return Err(PageError::InvalidMagic(id));
        }

        let ver: u16 = util::read_u16_le(page_reader)?;
        if ver != BP_PAGE_VERSION {
            return Err(PageError::UnknownVersion(ver, id));
        }

        let data_size: u32 = util::read_u32_le(page_reader)?;
        let checksum: u64 = util::read_u64_le(page_reader)?;
        Ok((data_size, checksum))
    }

    fn get_page_handle(&self, id: PageId) -> Result<File, PageError> {
        let page_path: PathBuf = self.root_path.join(String::from("page_") + &id.to_string());
        Ok(File::options().read(true).open(page_path)?)
    }

    fn get_writable_page_handle(&self, id: PageId) -> Result<File, PageError> {
        let page_path: PathBuf = self.root_path.join(String::from("page_") + &id.to_string());
        Ok(File::options().read(true).write(true).open(page_path)?)
    }

    /// Calculates the balance factor for our page size, returns `(min key count, max key count)`
    const fn calc_balance_factor() -> (usize, usize) {
        // TODO: adjust to be better representative of the average size of an entry
        let entry_size: usize = 28;
        let max: usize = (PAGE_SIZE - BP_PAGE_HEADER_SIZE) / entry_size;
        let min: usize = max.div_ceil(2);
        (min, max)
    }
}

/// The size of a page in bytes
pub const fn page_size() -> usize {
    PAGE_SIZE
}
