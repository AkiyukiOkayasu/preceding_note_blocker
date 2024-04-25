# Preceding Note Blocker

MIDI effect supporting VST3 and CLAP.  
This plug-in does not allow re-triggering when a MIDI note is being played and there is a note-on of the same MIDI note. This is not musical, but may be useful in some specific use cases.  

## Building

After installing [Rust](https://rustup.rs/), you can compile Preceding Note Blocker as follows:

```shell
cargo xtask bundle preceding_note_blocker --release
```

### macOS universal binary

```shell
cargo xtask bundle-universal preceding_note_blocker --release
```

## Install

### macOS

```shell
rsync -ahv --delete target/bundled/Preceding\ Note\ Blocker.clap/ ~/Library/Audio/Plug-Ins/CLAP/Preceding\ Note\ Blocker.clap
rsync -ahv --delete target/bundled/Preceding\ Note\ Blocker.vst3/ ~/Library/Audio/Plug-Ins/VST3/Preceding\ Note\ Blocker.vst3
```

## Validation

### CLAP

```shell
clap-validator validate target/bundled/Preceding\ Note\ Blocker.clap
```

### VST3

```shell
pluginval --verbose --strictness-level 5 target/bundled/Preceding\ Note\ Blocker.vst3
```

## Debug

### Mac

#### AudioPluginHost.app of JUCE

Install JUCE and build AudioPluginHost.app  

```shell
lldb /Applications/JUCE/extras/AudioPluginHost/Builds/MacOSX/build/Release/AudioPluginHost.app/Contents/MacOS/AudioPluginHost
(lldb) run
```

Then, scan VST3 plugins and test them.  

#### Reaper

Install [REAPER](https://www.reaper.fm/).  

```shell
lldb /Applications/REAPER.app/Contents/MacOS/REAPER
(lldb) run
```

Then, scan VST3 or Clap plugins and test them.  
