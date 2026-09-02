# The case stderr exists for (F-24): a clean exit, a body that will not parse,
# and the reason already written to stderr. This transport returns the bytes
# unparsed — the rejection happens where `from_slice` runs, in PHASE-07 — so
# what is asserted here is that the explanation arrived.
cat >/dev/null
echo "config is missing, so this is all you get" >&2
printf 'not JSON at all\n'
