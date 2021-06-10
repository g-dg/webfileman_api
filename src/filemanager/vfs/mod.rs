pub enum FileType {
	File, // A basic file
	Directory, // A directory which may contain more files and directories
	Link, // A link to a file or directory
	Mountpoint, // A placeholder for a mountpoint. If the mountpoint doesn't exist or can't be mounted, this should be displayed as an inaccessible file
}

pub struct EntryInfo {

}

pub trait VfsDriver {
	// Creates the driver object
	// `name` contains the vfs name as stored in the mountpoint table in the database
	// `options` is a string directly from the database. Driver dependant, but recommended JSON
	// `external_path` is the external path of where the vfs is mounted, without a trailing slash if any
	fn new(name: &str, options: &str, external_path: &str) -> Self;

	// Sets up the filesystem
	// Returns success or error message
	// Only run when the VFS is about to be used for the first time in the request
	fn mount(&mut self) -> Result<(), String>;

	// Takes down the filesystem
	// Returns success or error message
	// Only runs when the VFS no longer needs to be used in the request
	fn unmount(&mut self) -> Result<(), String>;

}
