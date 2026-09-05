# goad

## niri

```kdl
window-rule {
    match app-id="^goad$"
    open-floating true
    open-focused true
}
```

goad cannot place, raise or focus its own window — on Wayland all three are
the compositor's, and a client asking for them is a no-op. A rule like the
one above is how a prompt reaches you; the only part goad guarantees is the
app id, which is `goad` and will not change.

This is niri's syntax. Another compositor matches the same app id its own
way.
