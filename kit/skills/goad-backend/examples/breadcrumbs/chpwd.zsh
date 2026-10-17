# Tell goad where the shell has been. Source this from ~/.zshrc:
#
#   source /path/to/breadcrumbs/chpwd.zsh
#
# Each directory change emits a `changed-directory` event carrying
# `{"from": ..., "to": ...}`, in the background, so `cd` is never held up.
# `goad-emit` finds a host started on the default configuration. For a host
# started on this example's config.toml, add `--socket` with the absolute path
# of its goad.sock to the `goad-emit` line below.

autoload -Uz add-zsh-hook

# A path as a JSON string: backslash, quote and every control character
# escaped, so no directory name can break the document.
_breadcrumbs_json_string() {
  local text=$1 control
  text=${text//\\/\\\\}
  text=${text//\"/\\\"}
  while [[ $text =~ [[:cntrl:]] ]]; do
    control=${MATCH}
    text=${text//$control/$(printf '\\u%04x' "'$control")}
  done
  print -rn -- "\"$text\""
}

_breadcrumbs_chpwd() {
  [[ -n $OLDPWD && $OLDPWD != "$PWD" ]] || return 0
  local from to
  from=$(_breadcrumbs_json_string "$OLDPWD")
  to=$(_breadcrumbs_json_string "$PWD")
  goad-emit --source breadcrumbs --kind changed-directory \
    --data "{\"from\":$from,\"to\":$to}" >/dev/null 2>&1 &!
}

add-zsh-hook chpwd _breadcrumbs_chpwd
