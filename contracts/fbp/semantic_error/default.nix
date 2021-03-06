{stdenv, buildFractalideContract, upkeepers, ...}:

buildFractalideContract rec {
  src = ./.;
  contract = ''
  @0xf96c29a52799b766;

  struct FbpSemanticError {
    path @0 :Text;
    parsing @1 :List(Text);
  }
  '';
  meta = with stdenv.lib; {
    description = "Contract: Describes semantic errors in a Flow-based graph";
    homepage = https://github.com/fractalide/fractalide/tree/master/contracts/fbp/semantic_error;
    license = with licenses; [ mpl20 ];
    maintainers = with upkeepers; [ dmichiels sjmackenzie];
  };
}
