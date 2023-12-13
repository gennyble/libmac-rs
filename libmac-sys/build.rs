fn main() {
	let dst = cmake::Config::new("MAC_1029_SDK")
		.define("BUILD_UTIL", "OFF")
		.define("BUILD_SHARED", "OFF")
		.build_target("install")
		.build();

	println!("cargo:rustc-link-search=native={}/lib", dst.display());
	println!("cargo:rustc-link-lib=static=mac");
}
