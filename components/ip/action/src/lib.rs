#[macro_use]
extern crate rustfbp;
extern crate capnp;

component! {
    ip_action, contracts(generic_text)
    inputs(input: any),
    inputs_array(),
    outputs(output: any),
    outputs_array(),
    option(generic_text),
    acc(),
    fn run(&mut self) -> Result<()> {
        let mut opt = self.recv_option();
        let mut ip_input = try!(self.ports.recv("input"));
        let mut reader: generic_text::Reader = try!(opt.get_root());
        ip_input.action = try!(reader.get_text()).into();
        try!(self.ports.send("output", ip_input));
        Ok(())
    }
}
