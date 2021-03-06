{ stdenv, buildFractalideComponent, genName, upkeepers
  , file_list
  , path
  , value_string
  , ...}:

buildFractalideComponent rec {
  name = genName ./.;
  src = ./.;
  contracts = [ file_list path value_string ];
  depsSha256 = "0n5l21kfp58j453jmcqsfc5xs195rrpchabmjjha6cp39khynf5b";

  meta = with stdenv.lib; {
    description = "Component: Iterate over a list of 1000 file paths";
    homepage = https://github.com/fractalide/fractalide/tree/master/components/example/wrangler/iterate_paths;
    license = with licenses; [ mpl20 ];
    maintainers = with upkeepers; [ dmichiels sjmackenzie];
  };
}
