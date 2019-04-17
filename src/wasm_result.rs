use std::fmt;

use std::ffi::CString;
use std::os::raw::c_char;

pub type RawCString = *mut c_char;

fn export_string(s: String) -> RawCString {
    let s = CString::new(s).unwrap();
    s.into_raw()
}

pub struct WasmResult {
    pub main_result: Option<u32>,
    pub info_lines: Vec<String>,
}

impl fmt::Display for WasmResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write the main results on the first line
        write!(f, "{}\n", match self.main_result {
            None => "".to_string(),
            Some(x) => x.to_string(),
        })?;

        write!(f, "{}", self.info_lines.join("\n"))
    }
}

impl WasmResult {
    pub fn new() -> WasmResult {
        WasmResult {
            main_result: None,
            info_lines: vec![],
        }
    }

    pub fn push_line(&mut self, s: String) {
        self.info_lines.push(s);
    }

    pub fn flush_lines(&mut self) {
        self.info_lines.clear();
    }

    pub fn solve(&mut self, res: u32) -> RawCString {
        self.main_result = Some(res);
        self.export()
    }

    pub fn export(&self) -> RawCString {
        let s = self.to_string();
        export_string(s)
    }
}