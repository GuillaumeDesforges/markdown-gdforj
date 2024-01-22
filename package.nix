{ rustPlatform, lib }:

rustPlatform.buildRustPackage rec {
  pname = "markdown-rs-cli";
  version = "0.1.0";

  src = ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  meta = with lib; {
    description = "A light CLI tool to convert markdown files to HTML using markdown-rs.";
    homepage = "https://github.com/GuillaumeDesforges/markdown-rs-cli";
    license = licenses.mit;
    maintainers = [ maintainers.GuillaumeDesforges ];
  };
}
