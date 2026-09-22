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
        # The text `systemctl --user status goad` shows. A constant and not a
        # default: `extraConfig` merges over `Service` alone, so a consumer
        # cannot override this one. It is the string the hand-written unit this
        # module replaces carried, kept so the cutover changes nothing a person
        # reads.
        Description = "goad — personal intervention shell";

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
          # held by a live host. Those do not succeed on a retry, and
          # `Restart = "always"` would turn each into a restart loop that ends
          # in systemd's rate limiter. Exit 0 is the window being closed, which
          # was meant.
          #
          # **`Platform` is the exception, and this directive suppresses the
          # one restart that would work.** `start` ends
          # `run_event_loop_until_quit().map_err(StartupError::Platform)`, so a
          # compositor that goes away under a host which has been running for
          # hours exits 2 exactly as a host that never started does, and
          # systemd is told not to bring it back. Measured on this machine at
          # 006's audit: four such exits in two days, two of them leaving the
          # host down for around two hours until a person noticed. The repair
          # is an exit-code taxonomy that separates *never started* from
          # *stopped running*, which is a change to the startup surface and to
          # `SPEC-003`'s failure vocabulary rather than to this file —
          # `slice-006.md` §Follow-ups carries it. Until then the directive is
          # knowingly wrong in that one case, and saying so here is what keeps
          # the next reader from re-deriving the same conclusion from the same
          # three variants.
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
