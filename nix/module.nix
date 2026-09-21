# Home-manager module: goad's systemd user service, generated from a store
# path. Exported as `homeManagerModules.default`.
#
# The consumer is four lines:
#
#   { inputs, pkgs, ... }: {
#     imports = [inputs.goad.homeManagerModules.default];
#     services.goad = {
#       enable = true;
#       package = inputs.goad.packages.${pkgs.system}.goad;
#     };
#   }
#
# There is no `EnvironmentFile` and nothing to install alongside the binary: the
# package's wrapper carries LD_LIBRARY_PATH and FONTCONFIG_FILE itself, so
# there is no second file to drift against the build the unit runs.
#
# **Fetch goad as a git input, not as a tarball.** The revision the binary
# prints comes from the flake's own `self.shortRev` / `self.dirtyShortRev`, and
# a tarball fetch has neither attribute: over a tarball URL the package still
# builds and still runs, and `goad --version` quietly prints a bare version with
# no revision in it. Nothing reports that — it is visible only as an absent
# parenthetical (`design.md` §8 R1).
#
# This module asserts nothing about how `package` was built. It takes a
# derivation and names `bin/goad` in it.
#
# Smoke:
#   systemctl --user status goad
#   journalctl --user -u goad -f
{
  config,
  lib,
  ...
}: let
  cfg = config.services.goad;

  # The session the window belongs to: goad follows it up, stops with it, and
  # is wanted by it.
  session = "graphical-session.target";
in {
  options.services.goad = {
    enable = lib.mkEnableOption "goad, the personal intervention shell";

    package = lib.mkOption {
      type = lib.types.package;
      description = ''
        The package to run. Deliberately without a default: the consumer
        passes the one it built, so nothing here decides which goad runs.
      '';
      example = lib.literalExpression "inputs.goad.packages.\${pkgs.system}.goad";
    };

    extraConfig = lib.mkOption {
      type = lib.types.attrsOf lib.types.anything;
      default = {};
      description = ''
        Systemd directives merged over the generated `Service` block, and over
        no other block. Every `Service` field this module sets is a default a
        consumer may override this way.
      '';
      example = lib.literalExpression "{ RestartSec = 10; }";
    };
  };

  config = lib.mkIf cfg.enable {
    # On PATH as well as in the unit, so a hand-run `goad` is the same build
    # the service runs rather than whatever else is on PATH.
    home.packages = [cfg.package];

    systemd.user.services.goad = {
      Unit = {
        After = [session];
        PartOf = [session];
      };

      Service =
        {
          ExecStart = "${cfg.package}/bin/goad";

          # Restart a crash, not a refusal. Exit 2 is any `StartupError` —
          # `main` in `crates/goad/src/main.rs` maps every one of them to it,
          # and each new variant inherits that — so it covers a bad
          # configuration, an unreadable clock, and an ingress socket already
          # held by a live host. None of those succeeds on a retry, and
          # `Restart = "always"` would turn each into a restart loop that ends
          # in systemd's rate limiter. Exit 0 is the window being closed, which
          # was meant.
          Restart = "on-failure";
          RestartPreventExitStatus = 2;
          RestartSec = 2;

          # No `EnvironmentFile`. See the header: the wrapper carries the
          # environment the renderer needs.
        }
        // cfg.extraConfig;

      Install.WantedBy = [session];
    };
  };
}
