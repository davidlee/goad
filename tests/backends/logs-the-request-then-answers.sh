# Like `answers-as-instructed.sh` — one instruction per invocation, past the
# end of the list it behaves — except the invocation log holds each raw
# request rather than the literal string `invoked`, so a case can read off
# `event.kind` (slice 003 PHASE-02, VT-3/VT-7: `event.kind` cannot be
# observed through `answers-as-instructed.sh`, which never reads its stdin).
#
# argv[2] is the invocation log and argv[3…] are the instructions, in order;
# `bash` is argv[0] and this script is argv[1] (R-36).
log="${1:?the invocation log path must be argv[2]}"
shift
instructions=("$@")

request="$(cat)"

index=0
if [[ -f $log ]]; then
  readarray -t seen <"$log"
  index=${#seen[@]}
fi
printf '%s\n' "$request" >>"$log"

instruction='{"view":null}'
if ((index < ${#instructions[@]})); then
  instruction="${instructions[index]}"
fi
printf '%s\n' "$instruction"
