# Decorations

AtomicWM's decoration model is logical only. It describes rectangles that later rendering code can draw.

It does not draw borders or titlebars. It does not use a server-side decoration protocol yet.

## Modes

`none`

No compositor-side decoration. The outer rectangle is the same as the content rectangle.

`border`

Adds four border rectangles around the content rectangle.

`titlebar`

Adds a titlebar above the content rectangle and keeps border rectangles around the outer rectangle. A close button rectangle is placed inside the titlebar on the right.

## Geometry

The content rectangle is the logical client area.

The outer rectangle includes any border and titlebar.

`DecorationGeometry` contains:

- outer rectangle
- content rectangle
- top, right, bottom, and left border rectangles
- optional titlebar rectangle
- optional close button rectangle

## Hit Testing

Decoration hit testing is logical. It returns which decoration region contains a point.

Priority order:

1. close button
2. titlebar
3. borders
4. content

Points outside the outer rectangle return `None`.

## Limits

- no actual rendering
- no server-side decoration protocol yet
- no rounded corners
- no shadows
- no real pointer event integration
