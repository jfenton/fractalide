{stdenv, buildFractalideContract, upkeepers, ...}:

buildFractalideContract rec {
  src = ./.;
  contract = ''
  @0x9deaa106a95c1af8;

  struct FileError {
      notFound @0 :Text;
  }
  '';

  meta = with stdenv.lib; {
    description = "Contract: Describes file errors";
    homepage = https://github.com/fractalide/fractalide/tree/master/contracts/file_error;
    license = with licenses; [ mpl20 ];
    maintainers = with upkeepers; [ dmichiels sjmackenzie];
  };
}
