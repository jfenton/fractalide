{ stdenv, buildFractalideComponent, genName, upkeepers
  , fbp_semantic_error
  , fbp_graph
  , fbp_lexical
  , ...}:

buildFractalideComponent rec {
  name = genName ./.;
  src = ./.;
  contracts = [ fbp_semantic_error fbp_graph fbp_lexical ];
  depsSha256 = "12pqii2amnxb1b686ixrr9znv8k8hdika3nzm6gmw03qfmi804vh";

  meta = with stdenv.lib; {
    description = "Component: Flow-based programming semantics";
    homepage = https://github.com/fractalide/fractalide/tree/master/components/development/parser/fbp/semantic;
    license = with licenses; [ mpl20 ];
    maintainers = with upkeepers; [ dmichiels sjmackenzie];
  };
}
