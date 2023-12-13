// lmao: https://stackoverflow.com/a/73941622
#[link(name = "mac")]
extern "C" {
	fn CreateIAPEDecompress(
		filename: *const u32,
		errcode: *mut std::os::raw::c_int,
		readonly: bool,
		analyzetagnow: bool,
		readwholefile: bool,
	) -> *const decompress;

	fn c_APEDecompress_Destroy(this: *mut decompress);

	fn c_APEDecompress_Seek(
		this: *mut decompress,
		nblock_offset: std::ffi::c_longlong,
	) -> std::ffi::c_int;
}

#[repr(C)]
pub struct decompress_vtbl {
	//pub offset_destructor: unsafe extern "system" fn(this: *mut decompress),
	pub destructor: unsafe extern "system" fn(this: *mut decompress),
	// there is a struct with three bool in the class here.
	bp: [bool; 3],
	pub get_data: unsafe extern "system" fn(
		this: *mut decompress,
		pbuffer: *mut std::ffi::c_uchar,
		nblocks: std::ffi::c_ulonglong,
		nblocks_retreived: *mut std::ffi::c_ulonglong,
		ape_void: *const std::ffi::c_void,
	) -> std::ffi::c_int,
	//virtual int Seek(int64 nBlockOffset) = 0;
	pub seek: unsafe extern "system" fn(
		this: *mut decompress,
		nblock_offset: std::ffi::c_longlong,
	) -> std::ffi::c_int,
	//virtual int64 GetInfo(APE_DECOMPRESS_FIELDS Field, int64 nParam1 = 0, int64 nParam2 = 0) = 0;
	pub get_info: unsafe extern "system" fn(
		this: *mut decompress,
		field: std::ffi::c_int,
		param1: std::ffi::c_longlong,
		param2: std::ffi::c_longlong,
	) -> std::ffi::c_longlong,
}

#[repr(C)]
pub struct decompress {
	pub vtable: *const decompress_vtbl,
}

fn main() {
	let file = "test.ape";
	let mut wchars: Vec<u32> = file.encode_utf16().map(|s| s as u32).collect();
	wchars.push(0x00);

	let mut errcode = 0;

	let decomp = unsafe {
		CreateIAPEDecompress(wchars.as_ptr(), &mut errcode as *mut i32, true, true, true)
	};
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
