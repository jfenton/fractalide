#[macro_use]
extern crate rustfbp;
extern crate capnp;

use std::fs;
use std::process::Command;

component! {
    nucleus_find_contract, contracts(path, option_path)
    inputs(input: path),
    inputs_array(),
    outputs(output: option_path),
    outputs_array(),
    option(),
    acc(),
    fn run(&mut self) -> Result<()> {
        let mut ip = try!(self.ports.recv("input"));
        let name: path::Reader = try!(ip.get_root());
        let is_path = try!(name.get_path());
        let new_path = if fs::metadata(format!("{}", is_path)).is_ok() {
            Some(is_path)
        } else {
            //lookup_path(is_path)
            Some(is_path)
        };
        let mut new_ip = IP::new();
        {
            let mut ip = new_ip.init_root::<option_path::Builder>();
            match new_path {
                None => { ip.set_none(());},
                Some(p) => { ip.set_path(p);}
            };
        }
        let _ = self.ports.send("output", new_ip);
        Ok(())
    }

}

// fn build_component(name: &str) //-> Option<&str>
// {
//     let output = Command::new("nix-build")
//                      .arg("--argstr debug true")
//                      .arg("--argstr cache $(./support/buildCache.sh)")
//                      .arg("-I nixpkgs=/home/stewart/dev/fractalide/nixpkgs/")
//                      .arg(format!("--argstr subnet {}", name))
//                       .output()
//                      .expect("failed to execute process");
//
//     println!("status: {}", output.status);
//     println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
//     println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
// }
