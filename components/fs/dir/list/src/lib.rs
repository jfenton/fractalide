#[macro_use]
extern crate rustfbp;
extern crate capnp;

use std::fs;
use std::io::BufReader;
use std::io::BufRead;

component! {
    fs_dir_list, contracts(file_list, path)
    inputs(input: path),
    inputs_array(),
    outputs(output: file_list),
    outputs_array(),
    option(),
    acc(),
    fn run(&mut self) -> Result<()> {

        let mut ip = try!(self.ports.recv("input"));
        let path: path::Reader = try!(ip.get_root());
        let paths = try!(fs::read_dir(try!(path.get_path())));

        let mut new_ip = IP::new();
        {
            let ip = new_ip.init_root::<file_list::Builder>();
            let mut files = ip.init_files(try!(fs::read_dir(try!(path.get_path()))).count() as u32);
            let mut i: u32 = 0;
            for path in try!(fs::read_dir(try!(path.get_path()))) {
                files.borrow().set(i, try!(path).path().to_str().unwrap());
                i += 1;
            }
        }
        try!(self.ports.send("output", new_ip));
        Ok(())
    }
}
