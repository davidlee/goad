# Goad 

What could be more deserving of truly personalised software than forming habits, or nudging your own behaviour?

Goad is a starter kit for something so unique to your needs, nobody else need even be able to understand it.

Existing software errs in one of two ways. Either it's too simple to be useful; or, bent crooked by every edge case, becomes bureaucratic and ill-suited to any individual. Goad is different.

A thin GUI wrapper, a flexible, simple protocol, and some clear guidance to get your agent cooking. Goad doesn't model the general case; it just exposes a few simple interactions. When they happen, what they mean, what gets recorded where - these are not its concern, and invariably much simpler and more flexibly described in a general purpose programming language (pick any one you want).

Whatever it is you want to track, remember, or subtly intervene in, Goad provides the scaffolding to cook it up. So go ahead, and make something weirdly personal.

## Try it

```zsh
just demo
```

That starts goad against `examples/demo.toml` and the ten-line shell backend in
`examples/shell/backend.sh`, which prompts every time it is asked — so a window
is there at once. Answer it and the window goes; the backend has said there is
nothing more to show. `just run <config>` does the same with a configuration of
your own.

## GUI 

The lovely [Slint](https://github.com/slint-ui/slint) builds the GUI.

[Install the plugin](https://docs.slint.dev/latest/docs/slint/guide/tooling/ai-coding-assistants/) for your coding assistant of choice.

## Development Methodology

This repository employs a brutally minimalistic abbreviation of [Doctrine](https://doctrinal.systems), using pure markdown instruction and templates.


