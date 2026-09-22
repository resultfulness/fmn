{
    inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";

    outputs = { self, nixpkgs, ... }:
    let
        system = "x86_64-linux";
        pkgs = import nixpkgs { inherit system; };
    in {
        devShells.${system}.default = pkgs.mkShell {
            packages = with pkgs; [
                watchexec

                cargo
                rustc
                rust-analyzer
                rustfmt

                nodejs-slim
                pnpm
                prettier
                typescript-language-server
                emmet-language-server
                svelte-language-server
                vscode-css-languageserver
            ];
        };
    };
}
