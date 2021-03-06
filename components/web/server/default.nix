{ stdenv, buildFractalideComponent, genName, openssl
  , path
  , domain_port
  , url
  , ...}:

buildFractalideComponent rec {
  name = genName ./.;
  src = ./.;
  contracts = [ path domain_port url ];
  buildInputs = [ openssl ];
  depsSha256 = "091z6ikpyvj8r3p05iga4m37sc7jlbpmvz4ynvinxl0np8qqp4j2";

  meta = with stdenv.lib; {
    description = "Component:  web_server";
    homepage = https://github.com/fractalide/fractalide/tree/master/components/web/server;
    longDescription = "Component: web_server
    example usage:
    'path:(path="/path/to/some/index.html")' -> www_dir www(web_server)
    'domain_port:(domainPort="localhost:8080")' -> domain_port www()
    'url:(url="/docs")' -> url www()
    ";
    license = with licenses; [ mpl20 ];
    maintainers = with upkeepers; [ dmichiels sjmackenzie];
  };
}
