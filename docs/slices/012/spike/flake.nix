{
  description = "012 spike: R1 (plugin load from a store path), R2 ($HOME bind shadows the shared home)";
  inputs.goad.url = "git+file:///home/david/dev/goad";
  outputs = {goad, ...}: let
    system = "x86_64-linux";
    pkgs = import goad.inputs.nixpkgs {inherit system;};
    jailLib = goad.inputs.pub.lib.${system}.mkJailedAgents {};
    c = jailLib.combinators;
    # kit/ only: the shape design.md §5.2.8 gives packages.goad-kit
    kitOnly = pkgs.runCommand "goad-kit-stub" {} "cp -r ${./stub/kit} $out";
    # the repo shape: root marketplaces + kit/
    kitRepo = pkgs.runCommand "goad-kit-repo-stub" {} "cp -r ${./stub} $out";
    consumerOptions = [
      (c.set-env "GOAD_KIT" "${kitOnly}")
      (c.set-env "GOAD_KIT_REPO" "${kitRepo}")
      # the walk-home bind: after the profile's persist-home, so it shadows it
      (c.unsafe-add-raw-args "--bind \"$GOAD_WALK_HOME\" \"$HOME\"")
    ];
    mk = maker: args:
      maker ({
          profile = "specDev";
          extraPkgs = [pkgs.jq kitOnly kitRepo]; # set-env alone does not bind a store path
          extraOptions = consumerOptions;
          useOpEnv = false;
          passApiKeysFromEnv = false;
        }
        // args);
  in {
    packages.${system} = {
      claude = mk jailLib.makeJailedClaude {};
      codex = mk jailLib.makeJailedCodex {};
      shell = mk jailLib.makeJailedAgent {
        name = "shell";
        agent = pkgs.writeShellScriptBin "shell" ''exec "$@"'';
        extraPkgs = [pkgs.jq kitOnly kitRepo jailLib.agentsByName.claude jailLib.agentsByName.codex];
      };
    };
  };
}
