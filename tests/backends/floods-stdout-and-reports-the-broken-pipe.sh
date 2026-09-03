# The same flood, arranged so the **backend** can say whether the host closed
# the stream — which is what R-43's verification row asks for and what no
# in-band channel can carry, since the host kills this shell the moment the
# bound is hit.
#
# Two grandchildren, one job each. Both must be grandchildren: a child dies with
# the kill and reports nothing.
#
#   1. a sleep holding **stderr**, so disposal stalls for the whole cleanup
#      budget. That is what turns "did the marker arrive before the exchange
#      returned?" from a race into a 500 ms window. Measured: with the handle
#      dropped at the bound the marker lands 500 ms before the return, and with
#      it dropped at the return it lands 1.8 ms after.
#   2. the flooder, with its **stderr closed** so it does not hold the drain
#      open, writing a marker file when its write fails.
#
# $1 is the marker path. It arrives as an argument rather than an environment
# variable because `std::env::set_var` is `unsafe` in edition 2024 and
# `unsafe_code` is denied; `command` is an argv vector anyway (R-36).
cat >/dev/null
echo "$$" >&2
sleep 2 &
( yes xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx || echo "the host closed the stream" > "$1" ) 2>/dev/null &
wait
