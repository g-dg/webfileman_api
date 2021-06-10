mod api;
pub mod cgi;
pub mod conf;
mod db;
mod vfs;

pub fn handle_request(cgi_input: &cgi::CgiInputVars) {
	let request_method = cgi_input.env
		.get("REQUEST_METHOD")
		.expect("CGI environment variable \"REQUEST_METHOD\" not set.")
		.to_uppercase()
		.trim();

	let request_url = String::from("/");
	let request_url = cgi_input.env
		.get("PATH_INFO")
		.unwrap_or(&request_url)
		.trim_matches('/');

	let request_split: Vec<&str> = request_url.split("/")
		.filter(|x| x != &"")
		.collect();

	print!("Status: 200\r\n");
	print!("Content-Type: text/html\r\n");
	print!("\r\n");

	println!("{:?}", request_split);
}
