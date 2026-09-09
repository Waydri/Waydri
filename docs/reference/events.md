# IPC Events Reference

Events are broadcast to all connected IPC clients when compositor state changes.

## window_created

Emitted when a new window is mapped.

```json
{"type":"event","event":"window_created","data":{"window_id":42,"title":"Alacritty","app_id":"org.alacritty","workspace":0},"timestamp":1700000000000}
```

## window_destroyed

Emitted when a window is unmapped.

```json
{"type":"event","event":"window_destroyed","data":{"window_id":42},"timestamp":1700000000000}
```

## window_focus_changed

Emitted when focus moves to a different window.

```json
{"type":"event","event":"window_focus_changed","data":{"window_id":43,"previous_window_id":42},"timestamp":1700000000000}
```

## workspace_changed

Emitted when the active workspace changes.

```json
{"type":"event","event":"workspace_changed","data":{"workspace":2,"previous_workspace":0},"timestamp":1700000000000}
```

## layout_changed

Emitted when the layout changes on a workspace.

```json
{"type":"event","event":"layout_changed","data":{"workspace":0,"layout":"dwindle","previous_layout":"tile"},"timestamp":1700000000000}
```

## output_added

Emitted when a new display output is connected.

```json
{"type":"event","event":"output_added","data":{"name":"DP-1","width":1920,"height":1080,"refresh_rate":60},"timestamp":1700000000000}
```

## output_removed

Emitted when a display output is disconnected.

```json
{"type":"event","event":"output_removed","data":{"name":"DP-1"},"timestamp":1700000000000}
```

## frame_completed

Emitted after each compositor frame is rendered.

```json
{"type":"event","event":"frame_completed","data":{"frame_time_ms":8.2,"damage_regions":3},"timestamp":1700000000000}
```

## damaged_region

Emitted when a screen region is marked as damaged.

```json
{"type":"event","event":"damaged_region","data":{"x":0,"y":0,"width":1920,"height":30},"timestamp":1700000000000}
```
