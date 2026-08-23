{
  description = "goad: dev shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    pub.url = "github:davidlee/nix-config?dir=flakes/pub";
    llm-agents.url = "github:numtide/llm-agents.nix";
  };

  outputs = inputs @ {
    self,
    nixpkgs,
    rust-overlay,
    ...
  }: let
    system = "x86_64-linux";

    pkgs = import nixpkgs {
      inherit system;
      overlays = [rust-overlay.overlays.default];
    };
    inherit (pkgs) lib stdenv;

    jailLib = inputs.pub.lib.${system}.mkJailedAgents {inherit (inputs) llm-agents;};

    # Shared libraries a Slint binary dlopen()s at runtime. Not build inputs —
    # they must be on LD_LIBRARY_PATH inside and outside the jail or the window
    # never opens.
    guiLibs = with pkgs; [
      wayland
      libxkbcommon
      libGL
      fontconfig
      stdenv.cc.cc.lib
    ];

    devToolPkgs = with pkgs; [
      rust-bin.beta.latest.default
      rust-analyzer
      pkg-config

      # Claude Code runs Bash-tool commands under bash or zsh only; give it a
      # real bash (see CLAUDE_CODE_SHELL below).
      bashInteractive
      stdenv.cc # cc/ld on PATH — cargo's linker
      git
    ];

    projectPkgs =
      devToolPkgs
      ++ guiLibs
      ++ [jailLib.agentsByName.claude jailLib.agentsByName.codex];

    # API keys reach the jail over a file descriptor, never over argv.
    #
    # jail.nix's stock forwarding (`passApiKeysFromEnv`, on by default) expands
    # `--setenv VAR "$VAR"` onto the bwrap command line, and /proc/<pid>/cmdline
    # is world-readable — every local process, including the nixbld uids running
    # arbitrary upstream build scripts, could read the keys in plaintext for as
    # long as a jail ran. `bwrap --args FD` parses NUL-separated arguments from a
    # descriptor instead, so the keys travel down an anonymous pipe.
    #
    # The fd number is a literal, not `{FD}<`-allocated: bash expands a command's
    # words before performing its redirections, so `--args "$FD"` on the same
    # line as `{FD}< <(…)` would expand to empty. 21 clears bash's floor of 10.
    apiKeyNames = ["OPENAI_API_KEY" "ANTHROPIC_API_KEY"];
    apiKeyArgsFd = "21";
    apiKeysViaFd = jailLib.combinators.unsafe-add-raw-args (
      "--args ${apiKeyArgsFd} ${apiKeyArgsFd}< <(printf '%s\\0'"
      + lib.concatMapStrings (var: " --setenv ${var} \"\${${var}:-}\"") apiKeyNames
      + ")"
    );

    jailEnvOptions = with jailLib.combinators; [
      apiKeysViaFd
      (set-env "LD_LIBRARY_PATH" (lib.makeLibraryPath guiLibs))
      # Claude Code auto-detects its Bash-tool shell from $SHELL and falls back
      # to "first working zsh, then bash" — so a jailed agent lands on the jail's
      # zsh and gets zsh globbing semantics it does not expect (`grep
      # --include=*.md` dies with `no matches found`). Pin the store path:
      # /bin/bash and /run/current-system are not jail-visible. A bad value is
      # ignored, so this cannot hard-break.
      (set-env "CLAUDE_CODE_SHELL" "${pkgs.bashInteractive}/bin/bash")
    ];

    mkJail = maker: args:
      maker ({
          profile = "specDev";
          extraPkgs = projectPkgs;
          extraOptions = jailEnvOptions;
          passApiKeysFromEnv = false; # replaced by apiKeysViaFd above
        }
        // args);

    jailPkgs = {
      jailed-claude = mkJail jailLib.makeJailedClaude {
        allowSelfAsSubagent = true;
        subagents = ["codex"];
        maxSubagentDepth = 2;
      };
      jailed-codex = mkJail jailLib.makeJailedCodex {
        subagents = ["claude" "codex"];
      };
    };
  in {
    packages.${system} = jailPkgs;

    devShells.${system}.default = pkgs.mkShell {
      packages =
        projectPkgs
        ++ (with jailLib.unjailed; [claude codex])
        ++ lib.attrValues jailPkgs;

      LD_LIBRARY_PATH = lib.makeLibraryPath guiLibs;

      shellHook = ''
        alias jcl='jailed-claude --dangerously-skip-permissions'
        alias jcx='jailed-codex'
      '';
    };
  };
}
