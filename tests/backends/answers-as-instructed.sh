# A backend that does what it was told, one instruction per invocation.
#
# Every other script here is one behaviour. This one is "obey the list", and it
# exists because a `Host` is built around **one** command while EX-2 runs the
# whole misbehaving suite through **one** `Host`. The transport spawns a fresh
# process per exchange and keeps nothing between them, so a backend that varies
# by invocation has to count them itself — which is what the log is for.
#
# argv[2] is the invocation log and argv[3…] are the instructions, in order;
# `bash` is argv[0] and this script is argv[1], so they are $1 and $2… here
# (R-36). Past the end of the list it behaves, which is the positive control
# every suite case needs.
#
# An instruction is a response body, or one of four sentinels naming a
# transport-level misbehaviour. Each sentinel is the behaviour of the script
# named beside it, quoted rather than re-derived — in particular the `exec`s,
# without which bash forks and the case silently becomes PHASE-06's grandchild
# case instead of the one it claims to be.
log="${1:?the invocation log path must be argv[2]}"
shift
instructions=("$@")

index=0
if [[ -f $log ]]; then
  readarray -t seen <"$log"
  index=${#seen[@]}
fi
printf 'invoked\n' >>"$log"

instruction='{"view":null,"next_check":"45 minutes"}'
if ((index < ${#instructions[@]})); then
  instruction="${instructions[index]}"
fi

case "$instruction" in
@hang) # hangs-past-the-timeout.sh
  echo "$$" >&2
  exec sleep 30
  ;;
@flood) # floods-stdout-past-the-cap.sh
  cat >/dev/null
  echo "$$" >&2
  exec yes xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
  ;;
@exit1) # answers-then-exits-non-zero.sh
  cat >/dev/null
  printf '{"view":null}\n'
  echo "that answer is not to be trusted" >&2
  exit 1
  ;;
@garbage) # exits-zero-with-unparseable-stdout.sh
  cat >/dev/null
  echo "config is missing, so this is all you get" >&2
  printf 'not JSON at all\n'
  ;;
*)
  cat >/dev/null
  printf '%s\n' "$instruction"
  ;;
esac
