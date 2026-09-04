// A goad backend, in about eighty lines.
//
// One process per exchange: the host spawns this, writes one JSON request to
// stdin, closes it, and reads one JSON response from stdout. Nothing else on
// stdout is part of the protocol — a stray `console.log` would make the
// response two documents and the host would reject the pair. Diagnostics go to
// stderr, which the host captures and reports either way.
//
// Copy this file. It is meant to be edited, and the host does not know what
// this backend is for: it carries the interaction, and every decision below —
// whether to prompt, what to ask, when to look again — belongs here.
//
// `deno run -A` gives this script the full authority of the user running goad,
// and that is deliberate: a backend is the user's own program, not a sandboxed
// plugin. deno's default-deny permissions are a nuisance here rather than a
// security boundary, and `-A` switches them off. Do not read `-A` as a claim
// that anything is contained; nothing is.
//
// `deno run` does not typecheck. `deno check examples/typescript/backend.ts`
// does, and goad's own `just check` runs it.

/** RFC 3339, always UTC-normalized by the host. */
type Instant = string;

/** Whatever emitted the event, and whatever it chose to attach. */
interface Event {
  source: string;
  kind: string;
  timestamp: Instant;
  /** Opaque to the host. Its meaning is this backend's, and nobody else's. */
  data: unknown;
}

interface EvaluateRequest {
  protocol: number;
  type: "evaluate";
  now: Instant;
  event: Event;
}

interface RespondRequest {
  protocol: number;
  type: "respond";
  now: Instant;
  /** The interaction being answered. The host mints it; echo nothing back. */
  view_id: string;
  response: {
    option: string;
    /** Field id to value, for whichever fields the chosen option carried. */
    values: Record<string, unknown>;
  };
}

type Request = EvaluateRequest | RespondRequest;

/**
 * What a backend may answer.
 *
 * `view: null` means "nothing to show" — a positive answer, and not the same as
 * leaving `view` out, which asserts nothing. `next_check` is when to ask again:
 * a duration like `"45 minutes"` or an absolute instant with a UTC offset.
 * Both are optional.
 *
 * The types below are the whole of what the host accepts, not the subset this
 * file happens to use — so extending this backend is a matter of writing more
 * of them, never of loosening them. The normative statement is the protocol
 * spec; these follow it.
 */
interface Response {
  view?: View | null;
  next_check?: string;
}

/** A bare string is text; an object names its own kind. */
type Content =
  | string
  | { kind: "text" | "markdown" | "html" | "uri"; value: string };

interface View {
  kind: "choice";
  title: string;
  /** Optional context. */
  body?: Content;
  options: Option[];
}

interface Option {
  id: string;
  label: string;
  /** What to collect if this option is chosen. Answers arrive in `values`. */
  fields?: Field[];
}

/**
 * A field on an option. `min`, `max` and `options` belong to the kinds that
 * give them meaning and are refused on any other; every key the protocol does
 * not name is a presentation hint, carried flat on the field — `multiline`,
 * say — and passed through to the renderer untouched. A nested `hints` object
 * is the other spelling of the same thing and is refused.
 *
 * The `never` members are what make the refusals typecheck: the index
 * signature admits any key, and `never` narrows the ones the host would
 * reject on this kind, so `deno check` refuses them where the host would.
 */
type Field = {
  id: string;
  label: string;
  hints?: never;
  [hint: string]: unknown;
} & (
  | { kind: "text" | "boolean" | "datetime"; min?: never; max?: never; options?: never }
  | { kind: "number"; min?: number; max?: number; options?: never }
  | { kind: "choice"; options: Alternative[]; min?: never; max?: never }
);

/** A value a `choice` field may take. Not an option: it carries no fields. */
interface Alternative {
  id: string;
  label: string;
}

/**
 * How long the user has gone without an entry, as this backend reads it.
 *
 * `event.data` is opaque to the host, so it arrives as `unknown` and this is
 * where it acquires meaning. A real backend would more likely read its own
 * state here — a file, a database, another process — and would not need the
 * emitter to tell it anything.
 */
function minutesSinceEntry(data: unknown): number {
  if (typeof data === "object" && data !== null && "minutes_since_entry" in data) {
    const minutes = (data as Record<string, unknown>).minutes_since_entry;
    if (typeof minutes === "number") return minutes;
  }
  return 0;
}

const PROMPT_AFTER_MINUTES = 45;

function evaluate(request: EvaluateRequest): Response {
  if (minutesSinceEntry(request.event.data) < PROMPT_AFTER_MINUTES) {
    // Nothing to show, and look again in three quarters of an hour.
    return { view: null, next_check: "45 minutes" };
  }
  return {
    view: {
      kind: "choice",
      title: "Fill in your interstitial journal?",
      body: "The last entry was a while ago.",
      options: [
        {
          id: "yes",
          label: "Yeah",
          fields: [
            { id: "entry", kind: "text", label: "What happened?", multiline: true },
          ],
        },
        { id: "no", label: "Nah" },
      ],
    },
    next_check: "45 minutes",
  };
}

function respond(request: RespondRequest): Response {
  // A real backend does its work here: write the entry, launch an editor,
  // record that it asked. Whatever it does is invisible to the host, which
  // learns only that there is nothing further to show.
  console.error(`answered ${request.view_id} with ${request.response.option}`);
  return { view: null, next_check: "60 minutes" };
}

function answer(request: Request): Response {
  switch (request.type) {
    case "evaluate":
      return evaluate(request);
    case "respond":
      return respond(request);
  }
}

// One document in, one document out. `Deno.stdin.readable` resolves when the
// host closes stdin, which it does as soon as the request is written.
const request: Request = JSON.parse(
  await new Response(Deno.stdin.readable).text(),
);
console.log(JSON.stringify(answer(request)));
