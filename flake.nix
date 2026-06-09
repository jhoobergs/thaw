{
  description = "A basic Rust devshell for NixOS users developing eTeacher";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      {
        devShells.default = mkShell {
	  env = {
		PATH="$PATH:~/Programming/sejda-console-3.2.85/bin";
          };
          buildInputs =
            [
	    bash
	    git-lfs
	    python3
	    docker
	    nodejs
	    wasm-pack
	    pkg-config
	    tectonic
	    poppler-utils
	    imagemagick
	    python313Packages.pygments
	    diesel-cli
	    mariadb
	    libmysqlclient
	    sqlx-cli
	    cargo-generate
	    cargo-leptos
	    leptosfmt
	    dart-sass
	    binaryen
	    jdk21_headless
	    dbeaver-bin
      trunk
	      (vscode-with-extensions.override {
                vscode=vscodium;
		vscodeExtensions = with vscode-extensions; [
                  mkhl.direnv
		  rust-lang.rust-analyzer
		  redhat.vscode-yaml
		];
	      })
              (rust-bin.fromRustupToolchainFile ./rust-toolchain)            ]
            ++ pkgs.lib.optionals pkg.stdenv.isDarwin [
              darwin.apple_sdk.frameworks.SystemConfiguration
            ];

          shellHook = ''
            export PATH=$PATH:~/Programming/sejda-console-3.2.85/bin
          '';
        };
      }
    );
}
