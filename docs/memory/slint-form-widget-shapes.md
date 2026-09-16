# What shape a form of mixed field kinds has to take in Slint

Measured by the spike at commit `a698217`, which drew all five `SPEC-001/R-16`
field kinds in one repeater.

## A `PopupWindow` cannot be repeated or conditional

```
error: PopupWindow cannot be directly repeated or conditional
```

A hard compiler error, not a warning. `DatePickerPopup` and `TimePickerPopup`
are popups, so **neither can live inside a per-field repeater**. The shape that
works is one instance at the component root, with the row recording which field
opened it:

```slint
Button { clicked => { root.picking = field.id; root.open-picker(); } }
// …at the root:
picker := DatePickerPopup { accepted(d) => { root.edited({ id: root.picking, … }); } }
open-picker => { picker.show(); }
```

Both pickers being popups also means a `datetime` field has **no inline
control**: what sits in the form is a button showing the current value.

## One fat struct with a kind enum, not one model per kind

Slint structs have no sum types. Separate models per kind would lose
declaration order, which is what the protocol declares fields in. The shape
that compiles and keeps the order is one struct carrying a discriminant and a
payload slot per kind, with an `if` chain in the markup:

```slint
export enum Kind { boolean, text, number, choice, datetime }
export struct FieldRow {
    id: string, label: string, kind: Kind,
    checked: bool, text: string, number: float,
    minimum: float, maximum: float,
    alternatives: [string],
}
```

`[string]` inside a struct works, and a `ComboBox`'s `model` binds to it
directly.

## One callback taking the struct, not one per kind

```slint
callback edited(FieldRow);
…
root.edited({ id: field.id, kind: field.kind, checked: self.checked });
```

A struct literal in Slint may name a subset of the fields; the rest default. So
one callback carries every kind's edit, typed, and the host reads the slot the
`kind` names. The alternative — a single stringly value the host parses — puts
a parse failure in the path of a value the host itself produced.

## The widget inventory

`std-widgets` gives `CheckBox`, `Switch`, `LineEdit`, `TextEdit`, `Slider`,
`SpinBox`, `ComboBox`, `RadioGroup`, `DatePickerPopup`/`Date`,
`TimePickerPopup`/`Time`. `Slider` carries `minimum`, `maximum`, `step`,
`value`, and fires both `changed` (continuously, while dragging) and `released`.

**`LineEdit` exposes no readable cursor offset** — only
`set-selection-offsets()`. The builtin `TextInput` does expose
`out property <int> cursor-position-byte-offset`. So any approach that destroys
and recreates a text field cannot restore the caret where it was through
`LineEdit`; only to the end of the string. Preferring an approach that never
destroys the element avoids needing to read it at all
(`a-present-destroys-the-widget-it-writes.md`).

Related: `a-present-destroys-the-widget-it-writes.md`, `slint-styling-facts.md`,
`slint-build-mechanics.md`.
