{ rustPlatform, lib }:

rustPlatform.buildRustPackage rec {
  pname = "markdown-gdforj";
  version = "0.1.0";

  src = ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  meta = with lib; {
    description = "A light CLI tool to convert markdown files to HTML using markdown-rs.";
    homepage = "https://github.com/GuillaumeDesforges/markdown-gdforj";
    license = licenses.mit;
    maintainers = [ maintainers.GuillaumeDesforges ];
  };
}
