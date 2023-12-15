#![allow(non_camel_case_types)]
//! raw bindings to the libmac library from [Monkey's Audio SDK](https://www.monkeysaudio.com/developers.html).
//!
//! > The terminology "Sample" refers to a single sample value, and "Block" refers
//! > to a collection of "Channel" samples.  For simplicity, MAC typically uses blocks
//! > everywhere so that channel mis-alignment cannot happen. (i.e. on a CD, a sample is
//! > 2 bytes and a block is 4 bytes ([2 bytes per sample] * [2 channels] = 4 bytes))
//!
//! Not all functions or defintions are present. If there's something you need an don't have,
//! consider opening a PR.
//!
//! ## NOTE
//! This crate ships a patched version of the SDK with fixes for how it handles UTF-16 strings.
//!
//! UTF-16 surrogate pairs were not being encoded as UTF-8 correctly causing some paths to fail
//! to resolve. That code has been modified to correct that behaviour.

#[repr(C)]
#[derive(Debug)]
pub struct APEDecompress {
	/// Don't use this.
	///
	/// It's a -sys crate and so not making it public feels wrong, but don't use this.
	pub vtable: *const APEDecompress_VTable,
}

#[link(name = "mac")]
extern "C" {
	/// Create the decompressor interface.
	///
	/// Returns a pointer to [APEDecompress] on success, or NULL on failure
	/// (and fills `pErrorCode` if it was passed in)
	///
	/// `pFilename` is ASCII only; (see [c_APEDecompress_CreateW] if you need UTF-16)
	pub fn c_APEDecompress_Create(
		pFilename: *const std::ffi::c_char,
		pErrorCode: *mut std::ffi::c_int,
	) -> *mut APEDecompress;

	/// Create the decompressor interface.
	///
	/// Returns a pointer to [APEDecompress] on success, or NULL on failure
	/// (and fills `pErrorCode` if it was passed in)
	///
	/// `pFilename` is a UTF-16 string (see crate-level docs about patched string handling)
	pub fn c_APEDecompress_CreateW(
		pFilename: *const u32,
		pErrorCode: *mut std::ffi::c_int,
	) -> *mut APEDecompress;

	/// Delete [APEDecompress]
	pub fn c_APEDecompress_Destroy(hAPEDecompress: *mut APEDecompress);

	/// gets raw decompressed audio
	///
	/// # Parameters:
	/// - `pBuffer`: a pointer to a buffer to put the data into
	/// - `nBlocks`: the number of audio blocks desired (see note at intro about blocks vs. samples)
	/// - `pBlocksRetrieved`: the number of blocks actually retrieved (could be less at end of file or on critical failure)
	pub fn c_APEDecompress_GetData(
		hAPEDecompress: *mut APEDecompress,
		pBuffer: *mut std::ffi::c_uchar,
		nBlocks: std::ffi::c_longlong,
		pBlocksRetrieved: *mut std::ffi::c_longlong,
	) -> std::ffi::c_int;

	/// seeks
	///
	/// # Parameters:
	/// - `nBlockOffset`: the block to seek to (see note at intro about blocks vs. samples)
	pub fn c_APEDecompress_Seek(
		hAPEDecompress: *mut APEDecompress,
		nBlockOffset: std::ffi::c_longlong,
	) -> std::ffi::c_int;

	/// get information about the APE file or the state of the decompressor
	///
	/// # Parameters:
	/// - `Field`: the field we're querying (see [APE_DECOMPRESS_FIELDS] for more info)
	/// - `nParam1`: generic parameter... usage is listed in [APE_DECOMPRESS_FIELDS]
	/// - `nParam2`: generic parameter... usage is listed in [APE_DECOMPRESS_FIELDS]
	pub fn c_APEDecompress_GetInfo(
		hAPEDecompress: *mut APEDecompress,
		Field: APE_DECOMPRESS_FIELDS,
		nParam1: std::ffi::c_longlong,
		nParam2: std::ffi::c_longlong,
	) -> std::ffi::c_longlong;

}

// lmao: https://stackoverflow.com/a/73941622
/// APEDecompress vtable. Should probably be considered extremely unsafe, dangerous,
/// and likely to cause emotional distress.
///
/// Mostly here because I didn't want to get
/// rid of it. Do you? Open a PR but I like it so I might fight for it's continued
/// existence a little.
#[repr(C)]
pub struct APEDecompress_VTable {
	//pub offset_destructor: unsafe extern "system" fn(this: *mut decompress),
	/// Use [c_APEDecompress_Destroy] instead
	pub destructor: unsafe extern "system" fn(this: *mut APEDecompress),
	/// there is a struct with three bool in the class here and if i don't
	/// put this here things get mad. i do not understand, but these functions
	/// aren't meant to be called anyway they just exist. this is how i was trying
	/// to do things before i found MACDll.h and thus the C API. but i needed a
	/// struct to act as the thing-to-be-pointed-at anyway, so i might as
	/// well keep it here, right? what's the harm. - genny
	bp: [bool; 3],
	/// Use [c_APEDecompress_GetData] instead
	pub get_data: unsafe extern "system" fn(
		this: *mut APEDecompress,
		pbuffer: *mut std::ffi::c_uchar,
		nblocks: std::ffi::c_ulonglong,
		nblocks_retreived: *mut std::ffi::c_ulonglong,
		ape_void: *const std::ffi::c_void,
	) -> std::ffi::c_int,
	//virtual int Seek(int64 nBlockOffset) = 0;
	/// Use [c_APEDecompress_Seek] instead
	pub seek: unsafe extern "system" fn(
		this: *mut APEDecompress,
		nblock_offset: std::ffi::c_longlong,
	) -> std::ffi::c_int,
	//virtual int64 GetInfo(APE_DECOMPRESS_FIELDS Field, int64 nParam1 = 0, int64 nParam2 = 0) = 0;
	/// Use [c_APEDecompress_GetInfo] instead
	pub get_info: unsafe extern "system" fn(
		this: *mut APEDecompress,
		field: APE_DECOMPRESS_FIELDS,
		param1: std::ffi::c_longlong,
		param2: std::ffi::c_longlong,
	) -> std::ffi::c_longlong,
}

/// Give this to [c_APEDecompress_GetInfo]. The two items in square
/// brackets are [param1, param2].
#[repr(C)]
pub enum APE_DECOMPRESS_FIELDS {
	/// version of the APE file * 1000 (3.93 = 3930) [ignored, ignored]
	APE_INFO_FILE_VERSION = 1000,
	/// compression level of the APE file [ignored, ignored]
	APE_INFO_COMPRESSION_LEVEL = 1001,
	/// format flags of the APE file [ignored, ignored]
	APE_INFO_FORMAT_FLAGS = 1002,
	/// sample rate (Hz) [ignored, ignored]
	APE_INFO_SAMPLE_RATE = 1003,
	/// bits per sample [ignored, ignored]
	APE_INFO_BITS_PER_SAMPLE = 1004,
	/// number of bytes per sample [ignored, ignored]
	APE_INFO_BYTES_PER_SAMPLE = 1005,
	/// channels [ignored, ignored]
	APE_INFO_CHANNELS = 1006,
	/// block alignment [ignored, ignored
	APE_INFO_BLOCK_ALIGN = 1007,
	/// number of blocks in a frame (frames are used internally)  [ignored, ignored]
	APE_INFO_BLOCKS_PER_FRAME = 1008,
	/// blocks in the final frame (frames are used internally) [ignored, ignored]
	APE_INFO_FINAL_FRAME_BLOCKS = 1009,
	/// total number frames (frames are used internally) [ignored, ignored]
	APE_INFO_TOTAL_FRAMES = 1010,
	/// header bytes of the decompressed WAV [ignored, ignored]
	APE_INFO_WAV_HEADER_BYTES = 1011,
	/// terminating bytes of the decompressed WAV [ignored, ignored]
	APE_INFO_WAV_TERMINATING_BYTES = 1012,
	/// data bytes of the decompressed WAV [ignored, ignored]
	APE_INFO_WAV_DATA_BYTES = 1013,
	/// total bytes of the decompressed WAV [ignored, ignored]
	APE_INFO_WAV_TOTAL_BYTES = 1014,
	/// total bytes of the APE file [ignored, ignored]
	APE_INFO_APE_TOTAL_BYTES = 1015,
	/// total blocks of audio data [ignored, ignored]
	APE_INFO_TOTAL_BLOCKS = 1016,
	/// length in ms (1 sec = 1000 ms) [ignored, ignored]
	APE_INFO_LENGTH_MS = 1017,
	/// average bitrate of the APE [ignored, ignored]
	APE_INFO_AVERAGE_BITRATE = 1018,
	/// bitrate of specified APE frame [frame index, ignored]
	APE_INFO_FRAME_BITRATE = 1019,
	/// bitrate of the decompressed WAV [ignored, ignored]
	APE_INFO_DECOMPRESSED_BITRATE = 1020,
	/// peak audio level (obsolete) (-1 is unknown) [ignored, ignored]
	APE_INFO_PEAK_LEVEL = 1021,
	/// bit offset [frame index, ignored]
	APE_INFO_SEEK_BIT = 1022,
	/// byte offset [frame index, ignored]
	APE_INFO_SEEK_BYTE = 1023,
	/// error code [buffer *, max bytes]
	APE_INFO_WAV_HEADER_DATA = 1024,
	/// error code [buffer *, max bytes]
	APE_INFO_WAV_TERMINATING_DATA = 1025,
	/// error code [waveformatex *, ignored]
	APE_INFO_WAVEFORMATEX = 1026,
	/// I/O source (CIO *) [ignored, ignored]
	APE_INFO_IO_SOURCE = 1027,
	/// bytes (compressed) of the frame [frame index, ignored]
	APE_INFO_FRAME_BYTES = 1028,
	/// blocks in a given frame [frame index, ignored]
	APE_INFO_FRAME_BLOCKS = 1029,
	/// point to tag (CAPETag *) [ignored, ignored]
	APE_INFO_TAG = 1030,
	/// whether it's an APL file
	APE_INFO_APL = 1031,
	/// the MD5 checksum [buffer *, ignored]
	APE_INFO_MD5 = 1032,
	/// an MD5 checksum to test (returns ERROR_INVALID_CHECKSUM or ERROR_SUCCESS) [buffer *, ignored]
	APE_INFO_MD5_MATCHES = 1033,

	/// current block location [ignored, ignored]
	APE_DECOMPRESS_CURRENT_BLOCK = 2000,
	/// current millisecond location [ignored, ignored]
	APE_DECOMPRESS_CURRENT_MS = 2001,
	/// total blocks in the decompressors range [ignored, ignored]
	APE_DECOMPRESS_TOTAL_BLOCKS = 2002,
	/// length of the decompressors range in milliseconds [ignored, ignored]
	APE_DECOMPRESS_LENGTH_MS = 2003,
	/// current bitrate [ignored, ignored]
	APE_DECOMPRESS_CURRENT_BITRATE = 2004,
	/// average bitrate (works with ranges) [ignored, ignored]
	APE_DECOMPRESS_AVERAGE_BITRATE = 2005,
	/// current frame
	APE_DECOMPRESS_CURRENT_FRAME = 2006,

	/// for internal use -- don't use (returns APE_FILE_INFO *) [ignored, ignored]
	APE_INTERNAL_INFO = 3000,
}

//pub const MAX_PATH: i32 = 4096;

pub const APE_COMPRESSION_LEVEL_FAST: i32 = 1000;
pub const APE_COMPRESSION_LEVEL_NORMAL: i32 = 2000;
pub const APE_COMPRESSION_LEVEL_HIGH: i32 = 3000;
pub const APE_COMPRESSION_LEVEL_EXTRA_HIGH: i32 = 4000;
pub const APE_COMPRESSION_LEVEL_INSANE: i32 = 5000;

pub const ERROR_SUCCESS: i32 = 0;

// file and i/o errors (1000's)
pub const ERROR_IO_READ: i32 = 1000;
pub const ERROR_IO_WRITE: i32 = 1001;
pub const ERROR_INVALID_INPUT_FILE: i32 = 1002;
pub const ERROR_INVALID_OUTPUT_FILE: i32 = 1003;
pub const ERROR_INPUT_FILE_TOO_LARGE: i32 = 1004;
pub const ERROR_INPUT_FILE_UNSUPPORTED_BIT_DEPTH: i32 = 1005;
pub const ERROR_INPUT_FILE_UNSUPPORTED_SAMPLE_RATE: i32 = 1006;
pub const ERROR_INPUT_FILE_UNSUPPORTED_CHANNEL_COUNT: i32 = 1007;
pub const ERROR_INPUT_FILE_TOO_SMALL: i32 = 1008;
pub const ERROR_INVALID_CHECKSUM: i32 = 1009;
pub const ERROR_DECOMPRESSING_FRAME: i32 = 1010;
pub const ERROR_INITIALIZING_UNMAC: i32 = 1011;
pub const ERROR_INVALID_FUNCTION_PARAMETER: i32 = 1012;
pub const ERROR_UNSUPPORTED_FILE_TYPE: i32 = 1013;
pub const ERROR_UNSUPPORTED_FILE_VERSION: i32 = 1014;
pub const ERROR_OPENING_FILE_IN_USE: i32 = 1015;

// memory errors (2000's)
pub const ERROR_INSUFFICIENT_MEMORY: i32 = 2000;

// dll errors (3000's)
pub const ERROR_LOADING_APE_DLL: i32 = 3000;
pub const ERROR_LOADING_APE_INFO_DLL: i32 = 3001;
pub const ERROR_LOADING_UNMAC_DLL: i32 = 3002;

// general and misc errors
pub const ERROR_USER_STOPPED_PROCESSING: i32 = 4000;
pub const ERROR_SKIPPED: i32 = 4001;

// programmer errors
pub const ERROR_BAD_PARAMETER: i32 = 5000;

// IAPECompress errors
pub const ERROR_APE_COMPRESS_TOO_MUCH_DATA: i32 = 6000;

// unknown error
pub const ERROR_UNDEFINED: i32 = -1;
