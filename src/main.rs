mod filemanager;

fn main() {
	if filemanager::cgi::is_cgi() { // run as cgi

		let cgi_vars = filemanager::cgi::CgiInputVars::populate();
		filemanager::handle_request(&cgi_vars);

	} else { // command line

	}
}
