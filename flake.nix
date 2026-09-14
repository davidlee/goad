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

    # fontconfig discovers fonts through a configuration file, not through
    # PATH or buildInputs — a font package in `guiLibs` alone does nothing
    # (D12, design.md §5.5 D12). `makeFontsConf` writes a `fonts.conf` naming
    # this store path explicitly, and the Slint cheap test tier needs a real
    # font to construct a component at all.
    fontsConf = pkgs.makeFontsConf {fontDirectories = [pkgs.dejavu_fonts];};

    # A nested, headless Wayland compositor, so an agent can run the GUI and
    # photograph it **without the host's compositor being in the jail at all**.
    # Handing over `$XDG_RUNTIME_DIR/wayland-1` would work and is what the
    # `wayland` combinator does; on wlroots it also hands over wlr-screencopy
    # and the clipboard, so a jailed agent could photograph the whole desktop.
    # A compositor of its own costs one process and gives a fixed-size output,
    # which is what makes a screenshot comparable between runs.
    #
    # No device access: wlroots runs on its headless backend with the pixman
    # (CPU) renderer, and Slint renders in software for the same reason, so
    # nothing here needs /dev/dri. If it turns out too slow, binding /dev/dri
    # is the upgrade — still far short of exposing the host's session.
    headlessEnv = ''
      export XDG_RUNTIME_DIR="''${XDG_RUNTIME_DIR:-/tmp/xdg-runtime}"
      mkdir -p "$XDG_RUNTIME_DIR" && chmod 700 "$XDG_RUNTIME_DIR"
      export WLR_BACKENDS=headless
      export WLR_LIBINPUT_NO_DEVICES=1
      export WLR_RENDERER=pixman
      export SLINT_BACKEND=winit-software
    '';

    # Run a command inside the nested compositor and leave it running. For a
    # session an agent drives from outside — `goad-emit` over the ingress
    # socket, say — rather than for a screenshot.
    goadHeadless = pkgs.writeShellScriptBin "goad-headless" ''
      set -euo pipefail
      ${headlessEnv}
      exec ${pkgs.cage}/bin/cage -- "$@"
    '';

    # The agent's tool: start the command, let it settle, photograph the output,
    # stop. `grim` inherits WAYLAND_DISPLAY from cage, so it photographs the
    # nested output and never the host's.
    #
    #   goad-shot [-o out.png] [-s seconds] -- cargo run -- examples/demo.toml
    goadShot = pkgs.writeShellScriptBin "goad-shot" ''
      set -euo pipefail
      out=shot.png
      settle=3
      while [ $# -gt 0 ]; do
        case "$1" in
          -o) out="$2"; shift 2 ;;
          -s) settle="$2"; shift 2 ;;
          --) shift; break ;;
          *) echo "goad-shot: unexpected argument: $1" >&2; exit 2 ;;
        esac
      done
      if [ $# -eq 0 ]; then
        echo "goad-shot: nothing to run; usage: goad-shot [-o out.png] [-s secs] -- CMD..." >&2
        exit 2
      fi
      ${headlessEnv}
      # Absolute, because cage's child starts wherever cage does, and exported
      # because the inner script is single-quoted so that "$@" reaches it whole.
      export GOAD_SHOT_OUT="$(${pkgs.coreutils}/bin/realpath -m "$out")"
      export GOAD_SHOT_SETTLE="$settle"
      exec ${pkgs.cage}/bin/cage -- ${pkgs.bashInteractive}/bin/bash -c '
        set -uo pipefail
        "$@" &
        app=$!
        sleep "$GOAD_SHOT_SETTLE"
        ${pkgs.grim}/bin/grim "$GOAD_SHOT_OUT" || echo "goad-shot: grim failed" >&2
        kill "$app" 2>/dev/null || true
        wait "$app" 2>/dev/null || true
      ' bash "$@"
    '';

    devToolPkgs = with pkgs; [
      rust-bin.beta.latest.default
      rust-analyzer
      pkg-config
      just

      # Claude Code runs Bash-tool commands under bash or zsh only; give it a
      # real bash (see CLAUDE_CODE_SHELL below).
      bashInteractive
      stdenv.cc # cc/ld on PATH — cargo's linker
      git
    ];

    projectPkgs =
      devToolPkgs
      ++ [pkgs.deno pkgs.socat]
      ++ [pkgs.nodejs_latest] # for pi
      ++ [goadHeadless goadShot pkgs.cage pkgs.grim]
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
      # slint sources
      (ro-bind "/home/david/.local/src/slint" "/workspace/slint")

      # Fonts. The devShell sets both of these; the jail set only
      # LD_LIBRARY_PATH, and fontconfig finds nothing without a config file —
      # there is no /etc/fonts inside. Measured, not assumed: with a fontless
      # config, 58 of the 156 `-p goad --test renderer` cases panic in
      # `fontique-…/backend/fontconfig.rs` with `NoMatch`. The font files ride
      # in on fontconfig's own closure; only the conf file needs binding.
      (set-env "FONTCONFIG_FILE" "${fontsConf}")
      (ro-bind "${fontsConf}" "${fontsConf}")

      # A place for a nested compositor to put its socket. bwrap starts from a
      # clean environment, so nothing sets this; `--tmpfs /tmp` is where it can
      # go. Deliberately **not** the host's /run/user/1000: the host's Wayland
      # socket stays out of the jail (see `headlessGui`).
      (set-env "XDG_RUNTIME_DIR" "/tmp/xdg-runtime")
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
      jailed-pi = mkJail jailLib.makeJailedPi {
        allowSelfAsSubagent = true;
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
      FONTCONFIG_FILE = fontsConf;

      shellHook = ''
        alias jcl='jailed-claude --dangerously-skip-permissions'
        alias jcx='jailed-codex'
        alias jpi='jailed-pi'
      '';
    };
  };
}
