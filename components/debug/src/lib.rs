extern crate capnp;

#[macro_use]
extern crate rustfbp;

component! {
    debug, contracts(generic_text)
    inputs(input: any),
    inputs_array(),
    outputs(output: any),
    outputs_array(),
    option(generic_text),
    acc(),
    fn run(&mut self) -> Result<()> {

        // Get the path
        let mut ip = try!(self.ports.recv("input"));

        let mut opt = self.recv_option();
        let text: generic_text::Reader = try!(opt.get_root());

        println!("{}\naction: {}", try!(text.get_text()), ip.action);

        let _ = self.ports.send("output", ip);

        Ok(())

    }

}
