# wp-primary-selection Protocol

## Overview

wp-primary-selection enables the "primary selection" clipboard mechanism. In X11, selecting text automatically copies it to the primary selection, which can be pasted with the middle mouse button. This protocol brings the same functionality to Wayland.

## Version

Waydri supports wp-primary-selection version 1.

## Interfaces

### zwp_primary_selection_device_manager_v1

Factory for creating primary selection devices. Bound as a global singleton.

#### get_device

```
get_device(id: new_id zwp_primary_selection_device_v1, seat: wl_seat)
```

Creates a primary selection device for the specified seat.

### zwp_primary_selection_device_v1

Manages the primary selection state for a seat.

#### set_selection

```
set_selection(source: zwp_primary_selection_source_v1, serial: uint32)
```

Sets the primary selection source. Replaces any existing source from this client.

#### destroy

```
destroy()
```

Destroys the device. The primary selection data is released.

### zwp_primary_selection_offer_v1

Offers primary selection data to a client.

#### receive

```
receive(mime_type: string, fd: fd)
```

Requests the primary selection data in the specified MIME type. Data is written to the provided file descriptor.

#### finish

```
finish()
```

Indicates the client is done reading the data.

#### destroy

```
destroy()
```

Destroys the offer object.

### zwp_primary_selection_source_v1

Provides primary selection data from a client.

#### send

```
send(mime_type: string, fd: fd)
```

Event: the client requests the data in the specified MIME type. The source writes the data to the file descriptor.

#### cancelled

```
cancelled()
```

Event: the source was replaced by another client or was deselected.

## Offer/Source Model

1. Client A selects text, creating a source.
2. Client A calls set_selection with its source.
3. Client B middle-clicks, requesting the selection.
4. Client B receives an offer event with the available MIME types.
5. Client B calls receive on the offer to get the data.
6. Client A receives a send event and writes the data to the fd.

## MIME Types

Common MIME types for text selections:

- `text/plain`: Plain text.
- `text/plain;charset=utf-8`: UTF-8 encoded text.
- `text/html`: HTML content.

## Integration

Waydri integrates primary selection with the Wayland seat. When a client sets a primary selection source, it is stored per-seat. Other clients can query the selection through the standard offer mechanism.

The primary selection is separate from the regular clipboard (wl_data_device). Both can exist simultaneously.
