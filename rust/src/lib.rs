// SFS_PORT_STUB: stub kosong. HAPUS baris ini setelah port selesai dan diuji.
//
// Titik masuk Zygisk. Stub ini sengaja tidak mendaftarkan modul apa pun, sehingga
// Zygisk melepas library ini dan TIDAK ADA hook yang terpasang (fitur modul mati).
use core::ffi::c_void;

#[no_mangle]
pub extern "C" fn zygisk_module_entry(_api_table: *mut c_void, _env: *mut c_void) {}
