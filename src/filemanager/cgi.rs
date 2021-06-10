use std::collections::HashMap;
use std::{env, io};

pub struct CgiInputVars {
	pub env: HashMap<String, String>,
	pub stdin: Box<dyn io::Read>,
}

impl CgiInputVars {
	pub fn populate() -> CgiInputVars {
		let mut environ: HashMap<String, String> = HashMap::new();

		for var in env::vars() {
			environ.insert(var.0, var.1);
		}

		CgiInputVars {
			env: environ,
			stdin: Box::new(io::stdin())
		}
	}
}

pub fn is_cgi() -> bool {
	env::var("GATEWAY_INTERFACE").is_ok()
}
