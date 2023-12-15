#![allow(non_camel_case_types)]

use std::{
	ffi::CString,
	fs::File,
	io::Read,
	mem::size_of,
	os::fd::{AsRawFd, FromRawFd},
	thread::sleep,
	u8::MAX,
};

#[repr(C)]
#[derive(Debug)]
pub struct APEDecompress {
	vtable: *const APEDecompress_VTable,
}

#[link(name = "mac")]
extern "C" {
	pub fn c_APEDecompress_Create(
		pFilename: *const std::ffi::c_char,
		pErrorCode: *mut std::ffi::c_int,
	) -> *mut APEDecompress;

	pub fn c_APEDecompress_CreateW(
		pFilename: *const u32,
		pErrorCode: *mut std::ffi::c_int,
	) -> *mut APEDecompress;

	pub fn c_APEDecompress_Destroy(hAPEDecompress: *mut APEDecompress);

	pub fn c_APEDecompress_GetData(
		hAPEDecompress: *mut APEDecompress,
		pBuffer: *mut std::ffi::c_uchar,
		nBlocks: std::ffi::c_longlong,
		pBlocksRetrieved: *mut std::ffi::c_longlong,
	) -> std::ffi::c_int;

	pub fn c_APEDecompress_Seek(
		hAPEDecompress: *mut APEDecompress,
		nBlockOffset: std::ffi::c_longlong,
	) -> std::ffi::c_int;

	pub fn c_APEDecompress_GetInfo(
		hAPEDecompress: *mut APEDecompress,
		Field: std::ffi::c_int,
		nParam1: std::ffi::c_longlong,
		nParam2: std::ffi::c_longlong,
	) -> std::ffi::c_longlong;

}

// lmao: https://stackoverflow.com/a/73941622
#[repr(C)]
pub struct APEDecompress_VTable {
	//pub offset_destructor: unsafe extern "system" fn(this: *mut decompress),
	pub destructor: unsafe extern "system" fn(this: *mut APEDecompress),
	// there is a struct with three bool in the class here.
	bp: [bool; 3],
	pub get_data: unsafe extern "system" fn(
		this: *mut APEDecompress,
		pbuffer: *mut std::ffi::c_uchar,
		nblocks: std::ffi::c_ulonglong,
		nblocks_retreived: *mut std::ffi::c_ulonglong,
		ape_void: *const std::ffi::c_void,
	) -> std::ffi::c_int,
	//virtual int Seek(int64 nBlockOffset) = 0;
	pub seek: unsafe extern "system" fn(
		this: *mut APEDecompress,
		nblock_offset: std::ffi::c_longlong,
	) -> std::ffi::c_int,
	//virtual int64 GetInfo(APE_DECOMPRESS_FIELDS Field, int64 nParam1 = 0, int64 nParam2 = 0) = 0;
	pub get_info: unsafe extern "system" fn(
		this: *mut APEDecompress,
		field: std::ffi::c_int,
		param1: std::ffi::c_longlong,
		param2: std::ffi::c_longlong,
	) -> std::ffi::c_longlong,
}

fn main() {
	let file = "test.ape";
	let cstr = CString::new(file).unwrap();
	let wstr_raw = "💀.ape";
	let mut wstr: Vec<u32> = String::from(wstr_raw)
		.encode_utf16()
		.map(|c| c as u32)
		.collect(); //.map(|c| c as u32).collect();
	wstr.push(0x00);

	{
		let mut cl = wstr.clone();
		cl.pop();
		let mut cl16: Vec<u16> = cl.into_iter().map(|c| c as u16).collect();
		let rt = String::from_utf16(cl16.as_slice()).unwrap();

		println!("roundtrip {rt}");

		/*let hh: Vec<u8> = wstr
		.iter()
		.flat_map(|b| b.to_le_bytes().into_iter())
		.collect();*/
		print!("utf-16: ");
		for b in &wstr {
			print!("{:02X} ", b);
		}
		println!();

		print!("\nutf-8: ");
		for b in wstr_raw.bytes() {
			print!("{:02X} ", b);
		}
		println!();

		println!("Rust can open?");
		let f = std::fs::File::open(wstr_raw).unwrap();
		let len = f.metadata().unwrap().len() / 1024;
		println!("rawfd is {}", f.as_raw_fd());
		println!("file is {len}KB");
	}

	let mut errcode = 0;

	let decomp = unsafe { c_APEDecompress_CreateW(wstr.as_ptr(), &mut errcode as *mut i32) };
	println!("returned / errcode {errcode}");
	println!("{}", std::io::Error::last_os_error());

	if decomp.is_null() {
		println!("decomp is null");
		return;
	}

	println!("trying to get ape version");
	let ret = unsafe { ((*(*decomp).vtable).get_info)(decomp as *mut _, 1000, 0, 0) };
	println!("version: {ret}");

	println!("trying to get ape block alignment");
	let ret = unsafe { ((*(*decomp).vtable).get_info)(decomp as *mut _, 1007, 0, 0) };
	println!("alignment: {ret}");

	println!("trying to current block");
	let ret = unsafe { ((*(*decomp).vtable).get_info)(decomp as *mut _, 2000, 0, 0) };
	println!("block: {ret}");

	println!("trying to total blocks");
	let ret = unsafe { ((*(*decomp).vtable).get_info)(decomp as *mut _, 2002, 0, 0) };
	println!("total: {ret}");

	println!("trying to seek to 2048 with cAPI");
	let ret = unsafe { c_APEDecompress_Seek(decomp as *mut _, 2048) };
	println!("seek ret: {ret}");

	println!("trying to current block");
	let ret = unsafe { ((*(*decomp).vtable).get_info)(decomp as *mut _, 2000, 0, 0) };
	println!("block: {ret}");

	println!("trying to get io handle");
	let ret = unsafe { ((*(*decomp).vtable).get_info)(decomp as *mut _, 1027, 0, 0) };
	println!("handle: {ret}");

	/*let cio_ptr = unsafe { ret as *mut u32 };
	let mut idx = 0;
	'outer: loop {
		for wstr_idx in 0..wstr.len() {
			let cio = unsafe { *cio_ptr.add(idx + wstr_idx) };

			if cio != wstr[wstr_idx] {
				idx += 1;
				continue 'outer;
			}
		}

		println!("found filepath at {idx}");
		break;
	}

	println!(
		"sizeof bool: {}\nalign: bool={}, i32={}",
		size_of::<bool>(),
		std::mem::align_of::<bool>(),
		std::mem::align_of::<i32>()
	);

	for x in 0..MAX_PATH as usize + 64 {
		let fd = unsafe { *((cio_ptr.add(idx) as *const char).add(x) as *const u8) };

		if x == 1000000 {
			let fd_ptr: *const u32 = unsafe {
				(cio_ptr.add(idx + MAX_PATH as usize) as *const char).add(x) as *const u32
			};
			println!("pointer to fd: {}", fd_ptr as u32);
			let ptr = unsafe { *fd_ptr as *const u32 };
			println!("\tdata there: {}", ptr as u32);
			let fd = unsafe { *ptr };
			println!("double deref: {fd}");
		}

		print!("{fd:02X} ");

		if x == MAX_PATH as usize - 1 {
			println!("\n--");
		}

		if x % 16 == 15 {
			println!()
		}
	}

	let fd_ptr: *const u32 = unsafe { cio_ptr.add(idx + MAX_PATH as usize) };
	println!("pointer to cio: {:?}", cio_ptr);
	println!("pointer to fd: {:?}", fd_ptr);
	let ptr = unsafe { *fd_ptr as *const u32 };
	println!("\tdata there: {}", ptr as u32);*/

	let pid = std::process::id();
	println!("sleeping for five minutes; PID {pid}");
	sleep(std::time::Duration::from_secs(60 * 5));

	println!("trying to seek to 8096");
	let ret = unsafe { ((*(*decomp).vtable).seek)(decomp as *mut _, 8096) };
	println!("seek ret: {ret}");

	println!("trying to current block");
	let ret = unsafe { ((*(*decomp).vtable).get_info)(decomp as *mut _, 2000, 0, 0) };
	println!("block: {ret}");

	println!("trying to get data...");
	// 256MB to be commically large just in case
	let mut buffer: Vec<u8> = vec![0; 1024 * 1024 * 256];
	let mut retreived: u64 = 0;
	let ret = unsafe {
		((*(*decomp).vtable).get_data)(
			decomp as *mut _,
			buffer.as_mut_ptr(),
			1024,
			&mut retreived as *mut u64,
			std::ptr::null(),
		)
	};
	if ret != 0 {
		println!("error getting data {ret}");
		return;
	}
	println!("returned: {ret}");
	println!(
		"sum of data: {}",
		buffer.iter().fold(0usize, |acc, c| acc + *c as usize)
	);
	println!("slice of first 16 bytes:\n{:?}", &buffer[0..16]);

	unsafe { ((*(*decomp).vtable).destructor)(decomp as *mut _) };
}

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

pub const MAX_PATH: i32 = 4096;

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
