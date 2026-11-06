{
  description = "goad: dev shell and packages";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    crane.url = "github:ipetkov/crane";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    crane,
    ...
  }: let
    system = "x86_64-linux";

    pkgs = import nixpkgs {
      inherit system;
      overlays = [rust-overlay.overlays.default];
    };
    inherit (pkgs) lib stdenv;

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
    #
    # A jail sees only the closure of `dev-tools`, and there is no /etc/fonts
    # inside, so the conf file must be in that closure: `fontsConfPkg`. Measured,
    # not assumed: with a fontless config, 58 of the 156 `-p goad --test renderer`
    # cases panic in `fontique-…/backend/fontconfig.rs` with `NoMatch`.
    fontsConf = pkgs.makeFontsConf {fontDirectories = [pkgs.dejavu_fonts];};
    fontsConfPkg = pkgs.linkFarm "goad-fonts-conf" [
      {
        name = "etc/goad/fonts.conf";
        path = fontsConf;
      }
    ];

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
    #
    # A runtime directory of its own, whatever the caller's says: a pi jail
    # forwards the host's path, which it cannot write.
    headlessEnv = ''
      export XDG_RUNTIME_DIR=/tmp/xdg-runtime
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
    #   goad-shot [-o out.png] [-s seconds] -- cargo run -- exercisers/demo.toml
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

    # One toolchain binding, shared by the devShell and crane. crane's own
    # default is nixpkgs-stable rustc, which is older than this workspace's
    # `edition = "2024"`; it would also lint against a different compiler than
    # the gate does.
    rust = pkgs.rust-bin.beta.latest.default;

    devToolPkgs = with pkgs; [
      rust
      rust-analyzer
      pkg-config
      just

      # Claude Code runs Bash-tool commands under bash or zsh only; give it a
      # real bash.
      bashInteractive
      stdenv.cc # cc/ld on PATH — cargo's linker
      git
    ];

    # What a build and test run needs with no display: the oubliette room's
    # tool set (`packages.capsule-tools`).
    headlessPkgs =
      devToolPkgs
      ++ [pkgs.deno pkgs.socat]
      ++ [pkgs.python3 pkgs.jq]; # the kit examples' interpreters, spawned by the `kit` test tier

    # What `yeslogic-fontconfig-sys` finds through pkg-config at build time: a
    # buildEnv, unlike mkShell, collects no dev outputs on its own. freetype and
    # expat are fontconfig.pc's own requirements.
    guiDevLibs = map lib.getDev (guiLibs ++ [pkgs.freetype pkgs.expat]);

    # The room's set plus the GUI: what a jail and the devshell need to run the
    # app and photograph it (`packages.dev-tools`). The room stays headless.
    projectPkgs =
      headlessPkgs
      ++ [goadHeadless goadShot pkgs.cage pkgs.grim]
      ++ guiLibs
      ++ guiDevLibs
      ++ [fontsConfPkg pkgs.tzdata];

    # What those tools need beyond packages, in every jail, form and shell.
    projectEnv = {
      LD_LIBRARY_PATH = lib.makeLibraryPath guiLibs;
      FONTCONFIG_FILE = "${fontsConf}";
      PKG_CONFIG_PATH = lib.makeSearchPath "lib/pkgconfig" guiDevLibs;
      # jiff reads zones from disk, and a jail has no /usr/share/zoneinfo.
      TZDIR = "${pkgs.tzdata}/share/zoneinfo";
    };

    # ---- the packages ----

    craneLib = (crane.mkLib pkgs).overrideToolchain rust;

    # One version, read from the manifest rather than repeated here as a
    # literal: what the binaries print is what cargo compiled them with.
    #
    # It enforces nothing, and must not be read as though it did. `Cargo.toml`
    # must stay TOML 1.0-parseable — see the `tokio` entry's own comment — and
    # what holds that is crane, which parses every manifest in `cleanCargoToml`
    # whether or not this line exists. Deleting this read would loosen nothing.
    workspaceVersion =
      (builtins.fromTOML (builtins.readFile ./Cargo.toml)).workspace.package.version;

    # crane's stock filter keeps `.rs`, `.toml` and `.lock` and nothing else,
    # which drops both halves of what `build.rs` reads: `crates/goad/ui/app.slint`,
    # and the two faces it imports out of `assets/`.
    #
    # Spelled by **directory, not by extension** — a future non-font asset then
    # needs no flake edit. Both halves are load-bearing, and both fail loudly:
    # without `.slint`, `build.rs` cannot open `app.slint`; with `.slint` but
    # without `assets/`, the Slint compiler cannot resolve the font imports.
    # Measured twice — once at the spike (S-2), once as PHASE-01's VA-3, each
    # time as a derivation that was built and observed to fail.
    src = let
      # A prefix and not a basename: `hasSuffix "assets"` would also admit a
      # file called `assets` anywhere in the tree.
      assetsDir = "${toString ./assets}/";
    in
      lib.cleanSourceWith {
        src = ./.;
        filter = path: type:
          (craneLib.filterCargoSources path type)
          || (lib.hasSuffix ".slint" path)
          || (lib.hasPrefix assetsDir path);
        name = "goad-source";
      };

    # One dependency layer over `--workspace`, shared by every binary package:
    # it holds every workspace member's dependencies, so a second layer would
    # rebuild what this one already holds.
    cargoArtifacts = craneLib.buildDepsOnly {
      # `pname` and `version` are explicit on every crane derivation here: the
      # workspace root is a virtual manifest with no `[package]`, so crane's
      # `crateNameFromCargoToml` has nothing to read.
      pname = "goad-deps";
      version = workspaceVersion;
      inherit src;
      cargoExtraArgs = "--locked --workspace";
      nativeBuildInputs = [pkgs.pkg-config];
      buildInputs = guiLibs;
      doCheck = false;
    };

    # The revision the binaries print, from PHASE-03 on. Set-but-empty is
    # unset, the rule `crates/goad/build.rs` already states for `SLINT_STYLE`:
    # a tarball fetch has neither attribute and prints a bare version rather
    # than a guess (design.md §5.2(c), A1).
    revision = self.shortRev or self.dirtyShortRev or "";

    # `doCheck = false` on every crane derivation above and below, and not as an
    # economy — do not reach for `doCheck = true` here. The nix sandbox has no
    # tzdb (six `instant.rs` cases fail on it), no session bus and no writable
    # font cache, and `event_loop_schedule` still fails on timing once both are
    # supplied. Worse, crane's `checkPhaseCargoCommand` inherits
    # `cargoExtraArgs`, so the obvious spelling — `doCheck = true` beside a
    # `--bin` selector — runs `cargo test -p goad --bin goad`, which is the
    # binary target's own unit tests, of which `main.rs` has none: it **reports
    # green having run nothing** (S-3, OQ-5, D5). What holds the tests is the
    # gate, `just check`.
    goadPackages = {
      goad = craneLib.buildPackage {
        pname = "goad";
        version = workspaceVersion;
        inherit src cargoArtifacts;
        cargoExtraArgs = "--locked -p goad --bin goad";
        nativeBuildInputs = [pkgs.pkg-config pkgs.makeWrapper];
        buildInputs = guiLibs;
        doCheck = false;
        GOAD_REVISION = revision;
        # The whole point of the package: the binary carries its environment,
        # so there is no second thing to install and nothing to go stale.
        # `--prefix` because the libraries are dlopen'd, so `ldd` resolves
        # clean on a binary that still opens no window without them;
        # `--set-default` and not `--set` so a caller's own fontconfig
        # configuration still wins.
        postInstall = ''
          wrapProgram $out/bin/goad \
            --prefix LD_LIBRARY_PATH : "${lib.makeLibraryPath guiLibs}" \
            --set-default FONTCONFIG_FILE "${fontsConf}"
        '';
        meta.mainProgram = "goad";
      };

      # No wrapper and no `guiLibs`: emit writes a line to a socket and has no
      # renderer to dlopen anything for (S-4).
      goad-emit = craneLib.buildPackage {
        pname = "goad-emit";
        version = workspaceVersion;
        inherit src cargoArtifacts;
        cargoExtraArgs = "--locked -p goad-emit --bin goad-emit";
        nativeBuildInputs = [pkgs.pkg-config];
        doCheck = false;
        GOAD_REVISION = revision;
        meta.mainProgram = "goad-emit";
      };

      # No wrapper and no `guiLibs`, as `goad-emit`: the checker drives a
      # backend over pipes and has no renderer to dlopen anything for.
      goad-check = craneLib.buildPackage {
        pname = "goad-check";
        version = workspaceVersion;
        inherit src cargoArtifacts;
        cargoExtraArgs = "--locked -p goad-check --bin goad-check";
        nativeBuildInputs = [pkgs.pkg-config];
        doCheck = false;
        GOAD_REVISION = revision;
        meta.mainProgram = "goad-check";
      };
    };

    # The plugin's marketplace root, not its plugin root: Codex installs only
    # from a marketplace and copies only `kit/` into its cache, and Claude loads
    # `$out/kit` (design.md §5.2.8). A fileset, not `cleanSourceWith`: a listed
    # path that is missing, or untracked in a git-input flake, fails evaluation
    # rather than being dropped in silence. Not a crane derivation.
    #
    # Copied into a derivation: `toSource` yields a source path, and a flake's
    # `packages` holds derivations (`nix flake check` refuses the bare path).
    goad-kit = pkgs.runCommandLocal "goad-kit" {} ''
      cp -r ${lib.fileset.toSource {
        root = ./.;
        fileset = lib.fileset.unions [
          ./.claude-plugin/marketplace.json
          ./.agents/plugins/marketplace.json
          ./kit
        ];
      }} $out
    '';
  in {
    packages.${system} =
      goadPackages
      // {inherit goad-kit;}
      // {
        default = goadPackages.goad;
        # What dev-envs builds the jails, forms and shell from. No agents: the
        # consumer supplies them.
        dev-tools = pkgs.buildEnv {
          name = "goad-dev-tools";
          paths = projectPkgs;
          ignoreCollisions = true;
          passthru.devEnv.env = projectEnv;
        };
        # The room's tool set, without the GUI. No agents: the room supplies
        # them.
        capsule-tools = pkgs.buildEnv {
          name = "goad-capsule-tools";
          paths = headlessPkgs;
          ignoreCollisions = true;
        };
      };

    # Not under `${system}`, unlike `packages` and `devShells` above: a
    # home-manager module is a function of the consumer's own module arguments,
    # so it is system-agnostic, and it is the consumer that reaches back into
    # `packages.${pkgs.system}` to say which build the unit runs.
    homeManagerModules.default = import ./nix/module.nix;

    devShells.${system}.default = pkgs.mkShell ({packages = projectPkgs;} // projectEnv);
  };
}
