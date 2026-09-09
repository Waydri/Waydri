# IPC Commands Reference

All commands are sent as JSON over the Waydri IPC socket.

## quit

Shuts down the compositor gracefully.

```json
{"type":"command","id":"1","command":"quit"}
```

## reload_config

Reloads configuration files without restarting.

```json
{"type":"command","id":"2","command":"reload_config"}
```

## set_layout

Sets the layout for the active workspace.

```json
{"type":"command","id":"3","command":"set_layout","args":{"layout":"dwindle"}}
```

## next_workspace

Switches to the next workspace.

```json
{"type":"command","id":"4","command":"next_workspace"}
```

## prev_workspace

Switches to the previous workspace.

```json
{"type":"command","id":"5","command":"prev_workspace"}
```

## switch_workspace

Switches to a specific workspace by index (0-based).

```json
{"type":"command","id":"6","command":"switch_workspace","args":{"index":2}}
```

## focus_window

Focuses a window by its ID.

```json
{"type":"command","id":"7","command":"focus_window","args":{"window_id":42}}
```

## close_window

Closes a window by its ID.

```json
{"type":"command","id":"8","command":"close_window","args":{"window_id":42}}
```

## set_opacity

Sets the opacity of a window (0.0 to 1.0).

```json
{"type":"command","id":"9","command":"set_opacity","args":{"window_id":42,"opacity":0.8}}
```

## toggle_fullscreen

Toggles fullscreen on the currently focused window.

```json
{"type":"command","id":"10","command":"toggle_fullscreen"}
```

## toggle_floating

Toggles floating mode on the currently focused window.

```json
{"type":"command","id":"11","command":"toggle_floating"}
```

## list_windows

Returns a list of all managed windows.

```json
{"type":"command","id":"12","command":"list_windows"}
```

## get_version

Returns the compositor version.

```json
{"type":"command","id":"13","command":"get_version"}
```
