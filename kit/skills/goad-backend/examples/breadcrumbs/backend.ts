// Breadcrumbs: a goad backend in TypeScript, for deno, with no imports.
//
// When the shell changes directory (chpwd.zsh forwards a `changed-directory`
// event carrying `{from, to}`), this asks "Where were you in <from>?" and
// stores the note against that directory. If the directory just entered has a
// note, it shows it, with *Thanks* to keep it and *Clear* to delete it.
//
// One process per exchange: one JSON request on stdin, one JSON response on
// stdout, nothing else on stdout. Diagnostics go to stderr.
//
// State is under $XDG_STATE_HOME/breadcrumbs/ (default ~/.local/state):
//   notes.json    note per directory
//   pending.json  what the person is being asked, so a respond knows
//
// An untouched text field arrives as "" (SPEC-001/R-57, R-58), so "" stores no
// note.
//
// `deno run` does not typecheck; `deno check backend.ts` does.

const EXAMPLE = "breadcrumbs";

type Notes = Record<string, string>;

/** What the outstanding view asks, kept because a respond carries no context. */
type Pending =
  | { asking: { from: string; to: string } }
  | { showing: string };

interface Request {
  type: string;
  event?: { source: string; kind: string; data: unknown };
  response?: { option: string; values?: Record<string, unknown> };
}

interface Option {
  id: string;
  label: string;
  fields?: { id: string; kind: "text"; label: string }[];
}

interface Response {
  view: { kind: "choice"; title: string; body?: string; options: Option[] } | null;
}

const NOTHING: Response = { view: null };

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null;
}

function stateDir(): string {
  const base = Deno.env.get("XDG_STATE_HOME") ?? `${Deno.env.get("HOME")}/.local/state`;
  return `${base}/${EXAMPLE}`;
}

async function read<T>(file: string, fallback: T): Promise<T> {
  try {
    return JSON.parse(await Deno.readTextFile(`${stateDir()}/${file}`)) as T;
  } catch (error) {
    if (!(error instanceof Deno.errors.NotFound)) {
      console.error(`${EXAMPLE}: ignoring unreadable ${file}: ${error}`);
    }
    return fallback;
  }
}

async function write(file: string, value: unknown): Promise<void> {
  await Deno.mkdir(stateDir(), { recursive: true });
  await Deno.writeTextFile(`${stateDir()}/${file}`, JSON.stringify(value));
}

function ask(from: string): Response {
  return {
    view: {
      kind: "choice",
      title: `Where were you in ${from}?`,
      options: [
        { id: "note", label: "Leave a note", fields: [{ id: "note", kind: "text", label: "Note" }] },
        { id: "skip", label: "Nothing to note" },
      ],
    },
  };
}

function show(directory: string, note: string): Response {
  return {
    view: {
      kind: "choice",
      title: `Back in ${directory}`,
      body: note,
      options: [{ id: "thanks", label: "Thanks" }, { id: "clear", label: "Clear" }],
    },
  };
}

/** Show the note for `directory` if there is one, remembering that we did. */
async function recall(directory: string): Promise<Response> {
  const note = (await read<Notes>("notes.json", {}))[directory];
  if (note === undefined) {
    await Deno.remove(`${stateDir()}/pending.json`).catch(() => {});
    return NOTHING;
  }
  await write("pending.json", { showing: directory } satisfies Pending);
  return show(directory, note);
}

async function arrived(data: unknown): Promise<Response> {
  if (!isRecord(data) || typeof data.from !== "string" || typeof data.to !== "string") {
    console.error(`${EXAMPLE}: ignoring a changed-directory event without from and to`);
    return NOTHING;
  }
  await write("pending.json", { asking: { from: data.from, to: data.to } } satisfies Pending);
  return ask(data.from);
}

async function answered(response: NonNullable<Request["response"]>): Promise<Response> {
  const pending = await read<Pending | null>("pending.json", null);
  if (pending === null) return NOTHING;
  if ("showing" in pending) {
    if (response.option === "clear") {
      const notes = await read<Notes>("notes.json", {});
      delete notes[pending.showing];
      await write("notes.json", notes);
    }
    await Deno.remove(`${stateDir()}/pending.json`).catch(() => {});
    return NOTHING;
  }
  const note = response.values?.note;
  if (response.option === "note" && typeof note === "string" && note.trim() !== "") {
    const notes = await read<Notes>("notes.json", {});
    notes[pending.asking.from] = note.trim();
    await write("notes.json", notes);
  }
  return await recall(pending.asking.to);
}

async function main(): Promise<Response> {
  const request = JSON.parse(await new Response(Deno.stdin.readable).text()) as Request;
  if (request.type === "respond" && request.response) return await answered(request.response);
  const event = request.event;
  if (event?.source === EXAMPLE && event.kind === "changed-directory") {
    return await arrived(event.data);
  }
  // The host's own kinds, and any event this backend does not know.
  return NOTHING;
}

console.log(JSON.stringify(await main()));
