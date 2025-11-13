# UI System - Records-Based Architecture

## Vision

Create a declarative UI system for Achronyme using records as the foundation. Components are records with properties and methods, enabling reactive state management, event handling, and rendering across multiple backends (Web Canvas, Native GUI, Terminal UI).

## Core Principles

1. **Records as Components**: UI components are records with properties (data) and methods (behavior)
2. **Declarative Syntax**: Define what you want, not how to render it
3. **Reactive State**: Changes to state automatically trigger re-renders
4. **Tailwind-like Styling**: Utility-first CSS with v4-style syntax
5. **Type Safety**: Leverage Achronyme's type system for compile-time checks
6. **Multi-Backend**: Same code renders to Web, Native, or Terminal

## Architecture Overview

```
┌─────────────────────────────────────────────────────────┐
│                    User Application                      │
│         (Declarative UI with Records & Functions)        │
└─────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────┐
│                   UI Framework Layer                     │
│  • Component Factory Functions                           │
│  • State Management (Signals/Computed)                   │
│  • Event System                                          │
│  • Style System (Tailwind-like)                          │
└─────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────┐
│                  Virtual DOM / Tree                      │
│  • Component Tree Representation                         │
│  • Diffing Algorithm                                     │
│  • Update Reconciliation                                 │
└─────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────┐
│                   Renderer Backends                      │
│  • Web (Canvas API)                                      │
│  • Native (egui/iced)                                    │
│  • Terminal (ratatui)                                    │
└─────────────────────────────────────────────────────────┘
```

## Component API Design

### Basic Component Structure

```javascript
// Component is a record with:
// - props: Configuration and data
// - state: Local mutable state
// - render: Function that returns UI description
// - methods: Lifecycle and custom functions

let Button = (props) => {
    // Local state
    mut hovered = false
    mut pressed = false

    // Component record
    {
        // Properties (immutable from parent)
        label: props.label,
        onClick: props.onClick,
        disabled: props.disabled || false,

        // Mutable state (local)
        mut hovered: hovered,
        mut pressed: pressed,

        // Style
        style: {
            padding: "12px 24px",
            backgroundColor: if(self.disabled, "#ccc",
                             if(self.pressed, "#0056b3",
                             if(self.hovered, "#0069d9", "#007bff"))),
            color: "#fff",
            border: "none",
            borderRadius: "4px",
            cursor: if(self.disabled, "not-allowed", "pointer")
        },

        // Event handlers
        onMouseEnter: () => do {
            if(!self.disabled) {
                self.hovered = true
            }
        },

        onMouseLeave: () => do {
            self.hovered = false;
            self.pressed = false
        },

        onMouseDown: () => do {
            if(!self.disabled) {
                self.pressed = true
            }
        },

        onMouseUp: () => do {
            if(!self.disabled && self.pressed) {
                self.pressed = false;
                if(self.onClick) {
                    self.onClick()
                }
            }
        },

        // Render method
        render: () => {
            type: "button",
            text: self.label,
            style: self.style
        }
    }
}
```

### Factory Functions for Built-in Components

```javascript
// Text Input
let TextInput = (props) => {
    mut value = props.value || ""
    mut focused = false

    {
        mut value: value,
        mut focused: focused,
        placeholder: props.placeholder || "",

        style: {
            padding: "8px 12px",
            border: if(self.focused, "2px solid #007bff", "1px solid #ccc"),
            borderRadius: "4px",
            fontSize: "14px"
        },

        onFocus: () => do { self.focused = true },
        onBlur: () => do { self.focused = false },
        onChange: (newValue) => do {
            self.value = newValue;
            if(props.onChange) {
                props.onChange(newValue)
            }
        },

        render: () => {
            type: "input",
            inputType: "text",
            value: self.value,
            placeholder: self.placeholder,
            style: self.style
        }
    }
}

// Container/Box
let Box = (props) => {
    {
        children: props.children || [],
        style: props.style || {},

        render: () => {
            type: "container",
            children: map(child => child.render(), self.children),
            style: self.style
        }
    }
}

// Text Label
let Text = (props) => {
    {
        content: props.content,
        style: props.style || {},

        render: () => {
            type: "text",
            content: self.content,
            style: self.style
        }
    }
}
```

## Tailwind-like Style System

### Style Utilities as Records

```javascript
// Padding utilities
let p = {
    _type: "padding",
    0: { padding: "0" },
    1: { padding: "0.25rem" },
    2: { padding: "0.5rem" },
    3: { padding: "0.75rem" },
    4: { padding: "1rem" },
    5: { padding: "1.25rem" },
    6: { padding: "1.5rem" },
    8: { padding: "2rem" }
}

// Margin utilities
let m = {
    _type: "margin",
    0: { margin: "0" },
    auto: { margin: "auto" },
    1: { margin: "0.25rem" },
    2: { margin: "0.5rem" },
    4: { margin: "1rem" }
}

// Background color utilities
let bg = {
    _type: "background",
    white: { backgroundColor: "#ffffff" },
    black: { backgroundColor: "#000000" },
    blue: {
        50: { backgroundColor: "#eff6ff" },
        100: { backgroundColor: "#dbeafe" },
        500: { backgroundColor: "#3b82f6" },
        600: { backgroundColor: "#2563eb" },
        700: { backgroundColor: "#1d4ed8" }
    },
    red: {
        500: { backgroundColor: "#ef4444" },
        600: { backgroundColor: "#dc2626" }
    }
}

// Text color utilities
let text = {
    _type: "color",
    white: { color: "#ffffff" },
    black: { color: "#000000" },
    blue: {
        500: { color: "#3b82f6" },
        700: { color: "#1d4ed8" }
    }
}

// Flexbox utilities
let flex = {
    _type: "display",
    flex: { display: "flex" },
    col: { flexDirection: "column" },
    row: { flexDirection: "row" },
    center: {
        justifyContent: "center",
        alignItems: "center"
    }
}

// Border radius
let rounded = {
    _type: "border",
    none: { borderRadius: "0" },
    sm: { borderRadius: "0.125rem" },
    md: { borderRadius: "0.375rem" },
    lg: { borderRadius: "0.5rem" },
    full: { borderRadius: "9999px" }
}
```

### Using Style Utilities

```javascript
// Option 1: Direct style records
let myButton = Button({
    label: "Click me",
    style: {
        ...p.4,           // padding: 1rem
        ...m.2,           // margin: 0.5rem
        ...bg.blue.500,   // backgroundColor: #3b82f6
        ...text.white,    // color: #ffffff
        ...rounded.md     // borderRadius: 0.375rem
    }
})

// Option 2: Helper function to merge styles
let styles = (styleList) => {
    reduce((acc, style) => { ...acc, ...style }, {}, styleList)
}

let myButton2 = Button({
    label: "Click me",
    style: styles([p.4, m.2, bg.blue.500, text.white, rounded.md])
})

// Option 3: Class array (if we implement a preprocessor)
let myButton3 = Button({
    label: "Click me",
    class: [p.4, m.2, bg.blue.500, text.white, rounded.md]
})
```

## State Management

### Signals (Reactive State)

```javascript
// Signal: Reactive primitive value
let createSignal = (initialValue) => {
    mut value = initialValue
    mut subscribers = []

    {
        // Get current value
        get: () => value,

        // Set new value and notify subscribers
        set: (newValue) => do {
            value = newValue;
            // Notify all subscribers
            map(fn => fn(newValue), subscribers)
        },

        // Subscribe to changes
        subscribe: (fn) => do {
            subscribers = [...subscribers, fn];
            // Return unsubscribe function
            () => do {
                subscribers = filter(f => f != fn, subscribers)
            }
        }
    }
}

// Usage
let counter = createSignal(0)
counter.set(counter.get() + 1)  // Increment

// Subscribe to changes
let unsubscribe = counter.subscribe((newVal) => {
    print("Counter changed to:", newVal)
})
```

### Computed Values

```javascript
// Computed: Derived reactive value
let createComputed = (computation, dependencies) => {
    mut value = computation()
    mut subscribers = []

    // Subscribe to all dependencies
    let unsubscribers = map(dep => {
        dep.subscribe(() => do {
            let newValue = computation();
            if(newValue != value) {
                value = newValue;
                map(fn => fn(newValue), subscribers)
            }
        })
    }, dependencies)

    {
        get: () => value,
        subscribe: (fn) => do {
            subscribers = [...subscribers, fn];
            () => do {
                subscribers = filter(f => f != fn, subscribers)
            }
        },
        dispose: () => do {
            map(unsub => unsub(), unsubscribers)
        }
    }
}

// Usage
let count = createSignal(5)
let doubled = createComputed(
    () => count.get() * 2,
    [count]
)

print(doubled.get())  // 10
count.set(10)
print(doubled.get())  // 20
```

### Component with State

```javascript
let Counter = () => {
    // Local reactive state
    let count = createSignal(0)
    let doubled = createComputed(() => count.get() * 2, [count])

    {
        count: count,
        doubled: doubled,

        increment: () => count.set(count.get() + 1),
        decrement: () => count.set(count.get() - 1),
        reset: () => count.set(0),

        render: () => {
            type: "container",
            style: styles([p.4, flex.col, flex.center]),
            children: [
                Text({
                    content: "Count: " + str(count.get()),
                    style: styles([text.blue.700, p.2])
                }),
                Text({
                    content: "Doubled: " + str(doubled.get()),
                    style: styles([text.blue.500, p.2])
                }),
                Box({
                    style: styles([flex.row, p.2]),
                    children: [
                        Button({
                            label: "-",
                            onClick: self.decrement,
                            style: styles([m.1, p.2, bg.red.500, text.white])
                        }),
                        Button({
                            label: "Reset",
                            onClick: self.reset,
                            style: styles([m.1, p.2, bg.blue.500, text.white])
                        }),
                        Button({
                            label: "+",
                            onClick: self.increment,
                            style: styles([m.1, p.2, bg.blue.500, text.white])
                        })
                    ]
                })
            ]
        }
    }
}
```

## Application Structure

### Simple Application

```javascript
import { createSignal, createComputed } from "ui/state"
import { Button, Text, Box, TextInput } from "ui/components"
import { styles, p, m, bg, text, flex, rounded } from "ui/styles"

let App = () => {
    // Application state
    let name = createSignal("")
    let greeting = createComputed(
        () => if(name.get() == "", "Hello!", "Hello, " + name.get() + "!"),
        [name]
    )

    {
        name: name,
        greeting: greeting,

        render: () => {
            type: "container",
            style: styles([p.8, flex.col, flex.center, bg.white]),
            children: [
                Text({
                    content: greeting.get(),
                    style: styles([text.blue.700, p.4, { fontSize: "24px" }])
                }),
                TextInput({
                    value: name.get(),
                    placeholder: "Enter your name...",
                    onChange: (newValue) => name.set(newValue),
                    style: styles([p.2, m.2, rounded.md])
                }),
                Button({
                    label: "Clear",
                    onClick: () => name.set(""),
                    style: styles([p.3, m.2, bg.blue.500, text.white, rounded.md])
                })
            ]
        }
    }
}

// Mount and render
let app = App()
render(app, "#root")  // Built-in render function
```

### Todo List Application

```javascript
let TodoApp = () => {
    // State
    let todos = createSignal([])
    let input = createSignal("")
    let filter = createSignal("all")  // "all", "active", "completed"

    // Computed
    let filteredTodos = createComputed(() => {
        let allTodos = todos.get()
        let filterType = filter.get()

        if(filterType == "active") {
            filter(todo => !todo.completed, allTodos)
        } else if(filterType == "completed") {
            filter(todo => todo.completed, allTodos)
        } else {
            allTodos
        }
    }, [todos, filter])

    let activeCount = createComputed(
        () => len(filter(todo => !todo.completed, todos.get())),
        [todos]
    )

    {
        // Methods
        addTodo: () => do {
            let text = input.get();
            if(text != "") {
                let newTodo = {
                    id: len(todos.get()) + 1,
                    text: text,
                    mut completed: false
                };
                todos.set([...todos.get(), newTodo]);
                input.set("")
            }
        },

        toggleTodo: (id) => do {
            let updated = map(todo =>
                if(todo.id == id,
                   { ...todo, completed: !todo.completed },
                   todo
                ),
                todos.get()
            );
            todos.set(updated)
        },

        deleteTodo: (id) => do {
            todos.set(filter(todo => todo.id != id, todos.get()))
        },

        clearCompleted: () => do {
            todos.set(filter(todo => !todo.completed, todos.get()))
        },

        // Render
        render: () => {
            type: "container",
            style: styles([p.8, flex.col, bg.white, { width: "600px", margin: "0 auto" }]),
            children: [
                // Header
                Text({
                    content: "Todos",
                    style: styles([text.blue.700, p.4, { fontSize: "32px", textAlign: "center" }])
                }),

                // Input section
                Box({
                    style: styles([flex.row, p.2]),
                    children: [
                        TextInput({
                            value: input.get(),
                            placeholder: "What needs to be done?",
                            onChange: (val) => input.set(val),
                            style: styles([p.3, flex.flex, { flex: 1 }, rounded.md])
                        }),
                        Button({
                            label: "Add",
                            onClick: self.addTodo,
                            style: styles([p.3, m.1, bg.blue.500, text.white, rounded.md])
                        })
                    ]
                }),

                // Filter buttons
                Box({
                    style: styles([flex.row, flex.center, p.2]),
                    children: [
                        Button({
                            label: "All",
                            onClick: () => filter.set("all"),
                            style: styles([
                                p.2, m.1, rounded.md,
                                if(filter.get() == "all", bg.blue.500, bg.blue.100),
                                if(filter.get() == "all", text.white, text.blue.700)
                            ])
                        }),
                        Button({
                            label: "Active",
                            onClick: () => filter.set("active"),
                            style: styles([
                                p.2, m.1, rounded.md,
                                if(filter.get() == "active", bg.blue.500, bg.blue.100),
                                if(filter.get() == "active", text.white, text.blue.700)
                            ])
                        }),
                        Button({
                            label: "Completed",
                            onClick: () => filter.set("completed"),
                            style: styles([
                                p.2, m.1, rounded.md,
                                if(filter.get() == "completed", bg.blue.500, bg.blue.100),
                                if(filter.get() == "completed", text.white, text.blue.700)
                            ])
                        })
                    ]
                }),

                // Todo list
                Box({
                    style: styles([p.2]),
                    children: map(todo => {
                        Box({
                            style: styles([flex.row, p.2, m.1, rounded.md, bg.blue.50]),
                            children: [
                                // Checkbox
                                Button({
                                    label: if(todo.completed, "✓", "○"),
                                    onClick: () => self.toggleTodo(todo.id),
                                    style: styles([p.2, m.1, rounded.full])
                                }),
                                // Text
                                Text({
                                    content: todo.text,
                                    style: styles([
                                        p.2, flex.flex, { flex: 1 },
                                        if(todo.completed, { textDecoration: "line-through", color: "#999" }, {})
                                    ])
                                }),
                                // Delete button
                                Button({
                                    label: "×",
                                    onClick: () => self.deleteTodo(todo.id),
                                    style: styles([p.2, m.1, bg.red.500, text.white, rounded.md])
                                })
                            ]
                        })
                    }, filteredTodos.get())
                }),

                // Footer
                Box({
                    style: styles([flex.row, flex.center, p.2]),
                    children: [
                        Text({
                            content: str(activeCount.get()) + " items left",
                            style: styles([p.2, text.blue.700])
                        }),
                        Button({
                            label: "Clear completed",
                            onClick: self.clearCompleted,
                            style: styles([p.2, m.2, bg.red.500, text.white, rounded.md])
                        })
                    ]
                })
            ]
        }
    }
}
```

## Implementation Phases

### Phase 1: Core Infrastructure (2-3 weeks)
- [ ] Basic component system with records
- [ ] Virtual DOM representation
- [ ] Simple diffing algorithm
- [ ] Canvas renderer (web backend)
- [ ] Basic event system (click, input)

**Deliverables:**
- Simple button that responds to clicks
- Text input that updates on change
- Box container for layout

### Phase 2: State Management (1-2 weeks)
- [ ] Signal implementation
- [ ] Computed values
- [ ] Subscription system
- [ ] Automatic re-rendering on state changes

**Deliverables:**
- Counter app with reactive state
- Todo list with add/remove

### Phase 3: Style System (2 weeks)
- [ ] Tailwind-like utility functions
- [ ] Style merging helper
- [ ] Responsive utilities
- [ ] Theme system

**Deliverables:**
- Complete style utility library
- Styled components examples
- Theme switching demo

### Phase 4: Advanced Components (2-3 weeks)
- [ ] Form components (checkbox, radio, select)
- [ ] Layout components (Grid, Flex)
- [ ] Modal/Dialog
- [ ] Charts integration
- [ ] Table component

### Phase 5: Additional Backends (3-4 weeks)
- [ ] Terminal UI renderer (ratatui)
- [ ] Native GUI renderer (egui)
- [ ] Backend abstraction layer

### Phase 6: Developer Experience (2 weeks)
- [ ] Component inspector
- [ ] Hot reload
- [ ] Error boundaries
- [ ] Performance profiler

## Built-in Components API Reference

### Layout Components

```javascript
// Box - Generic container
Box({
    children: [...],
    style: {...},
    onClick: () => {...}
})

// VStack - Vertical stack
VStack({
    children: [...],
    spacing: 4,  // Gap between children
    align: "center"  // "start", "center", "end"
})

// HStack - Horizontal stack
HStack({
    children: [...],
    spacing: 2,
    align: "center"
})

// Grid - CSS Grid layout
Grid({
    children: [...],
    columns: 3,
    gap: 4
})
```

### Form Components

```javascript
// Button
Button({
    label: "Click me",
    onClick: () => {...},
    disabled: false,
    variant: "primary"  // "primary", "secondary", "danger"
})

// TextInput
TextInput({
    value: "",
    placeholder: "Enter text...",
    onChange: (val) => {...},
    type: "text"  // "text", "password", "email"
})

// Checkbox
Checkbox({
    checked: false,
    label: "Accept terms",
    onChange: (checked) => {...}
})

// Radio
Radio({
    selected: false,
    label: "Option 1",
    name: "group1",
    value: "opt1",
    onChange: (val) => {...}
})

// Select
Select({
    value: "opt1",
    options: [{label: "Option 1", value: "opt1"}, ...],
    onChange: (val) => {...}
})
```

### Display Components

```javascript
// Text
Text({
    content: "Hello",
    style: {...}
})

// Heading
Heading({
    level: 1,  // 1-6
    content: "Title"
})

// Image
Image({
    src: "path/to/image.png",
    alt: "Description",
    width: 200,
    height: 150
})

// Chart (integration with existing chart functions)
Chart({
    type: "line",  // "line", "bar", "scatter"
    data: [...],
    options: {...}
})
```

### Advanced Components

```javascript
// Modal/Dialog
Modal({
    open: true,
    onClose: () => {...},
    children: [...]
})

// Table
Table({
    columns: [{header: "Name", key: "name"}, ...],
    data: [...],
    onRowClick: (row) => {...}
})

// Tabs
Tabs({
    tabs: [{label: "Tab 1", content: ...}, ...],
    activeTab: 0
})
```

## Event System

```javascript
// Supported events
let events = {
    // Mouse events
    onClick: (event) => {...},
    onDoubleClick: (event) => {...},
    onMouseDown: (event) => {...},
    onMouseUp: (event) => {...},
    onMouseEnter: (event) => {...},
    onMouseLeave: (event) => {...},
    onMouseMove: (event) => {...},

    // Keyboard events
    onKeyDown: (event) => {...},
    onKeyUp: (event) => {...},
    onKeyPress: (event) => {...},

    // Form events
    onChange: (value) => {...},
    onInput: (value) => {...},
    onSubmit: (event) => {...},
    onFocus: (event) => {...},
    onBlur: (event) => {...},

    // Touch events (for mobile)
    onTouchStart: (event) => {...},
    onTouchEnd: (event) => {...},
    onTouchMove: (event) => {...}
}
```

## Integration with Existing Features

### With Charts
```javascript
import { createSignal } from "ui/state"
import { Chart, Button } from "ui/components"

let ChartDemo = () => {
    let data = createSignal([[1, 2], [2, 4], [3, 6]])

    {
        addPoint: () => do {
            let current = data.get();
            let lastX = current[len(current)-1][0];
            data.set([...current, [lastX + 1, random() * 10]])
        },

        render: () => Box({
            children: [
                Chart({
                    type: "line",
                    data: data.get()
                }),
                Button({
                    label: "Add Point",
                    onClick: self.addPoint
                })
            ]
        })
    }
}
```

### With Graph Theory
```javascript
import { createSignal } from "ui/state"
import { GraphVisualization } from "ui/components"
import { bfs } from "graph"

let GraphDemo = () => {
    let graph = createSignal({
        nodes: ["A", "B", "C", "D"],
        edges: [["A", "B"], ["B", "C"], ["C", "D"]]
    })

    let startNode = createSignal("A")
    let visited = createSignal([])

    {
        runBFS: () => do {
            let result = bfs(graph.get(), startNode.get());
            visited.set(result)
        },

        render: () => Box({
            children: [
                GraphVisualization({
                    graph: graph.get(),
                    highlightNodes: visited.get()
                }),
                Button({
                    label: "Run BFS",
                    onClick: self.runBFS
                })
            ]
        })
    }
}
```

## Next Steps

1. **Start with Phase 1**: Implement basic component system
2. **Create prototype**: Build a simple counter app
3. **Iterate**: Get feedback and refine API
4. **Document**: Write comprehensive guides
5. **Test**: Create example applications

## Questions to Answer

1. How should we handle component lifecycle (mount, update, unmount)?
2. Should we support JSX-like syntax or stick with records?
3. How to optimize re-renders for large component trees?
4. What's the best way to handle animations?
5. How to integrate with existing Rust GUI libraries?

## References

- Tailwind CSS v4: https://tailwindcss.com/
- React Hooks: For signal/state inspiration
- Solid.js: Fine-grained reactivity model
- egui: Immediate mode GUI in Rust
- ratatui: Terminal UI library
