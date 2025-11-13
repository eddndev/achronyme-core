# UI System Architecture for Achronyme

## Overview

Sistema de UI declarativo para Achronyme con:
- Componentes como records con métodos
- Sistema de estilos tipo Tailwind v4
- Reactividad automática
- Backend multiplataforma (web, desktop, terminal)

## 1. Arquitectura General

```
┌─────────────────────────────────────────┐
│   Achronyme UI DSL (User Code)         │
│   - Declarative component creation      │
│   - Functional composition              │
└────────────────┬────────────────────────┘
                 │
┌────────────────▼────────────────────────┐
│   UI Module (Built-in)                  │
│   - Component constructors              │
│   - Style system                        │
│   - Event handlers                      │
└────────────────┬────────────────────────┘
                 │
┌────────────────▼────────────────────────┐
│   UI Runtime (Rust)                     │
│   - Virtual DOM                         │
│   - Reactive state management           │
│   - Layout engine                       │
└────────────────┬────────────────────────┘
                 │
        ┌────────┴────────┬────────────┐
        │                 │            │
┌───────▼──────┐  ┌──────▼─────┐  ┌──▼─────┐
│ Web Renderer │  │   Native   │  │  TUI   │
│   (Canvas/   │  │  (egui/    │  │ (ratatui│
│    WebGL)    │  │   tauri)   │  │  /tui)  │
└──────────────┘  └────────────┘  └────────┘
```

## 2. Core Components API

### 2.1 Component Record Structure

Cada componente es un record con:

```rust
pub struct Component {
    pub id: String,
    pub type_name: String,
    pub props: HashMap<String, Value>,
    pub children: Vec<Component>,
    pub state: HashMap<String, Value>,
    pub methods: HashMap<String, Function>,
}
```

En Achronyme:

```javascript
{
    id: "input-1",
    type: "Input",
    props: {
        value: "",
        placeholder: "Enter text",
        disabled: false
    },
    state: {
        focused: false,
        touched: false
    },
    methods: {
        focus: () => updateState("focused", true),
        blur: () => updateState("focused", false),
        setValue: value => updateState("value", value)
    }
}
```

### 2.2 Component Constructors

```rust
// crates/achronyme-eval/src/function_modules/ui/components.rs

pub fn input(args: &[Value]) -> Result<Value, String> {
    // args[0] = props record
    let props = extract_record(&args[0])?;

    let id = generate_id();
    let component = create_component_record(
        id,
        "Input",
        props,
        vec![], // no children
        input_state_default(),
        input_methods()
    );

    Ok(component)
}

pub fn button(args: &[Value]) -> Result<Value, String> {
    let props = extract_record(&args[0])?;
    // Similar structure
}

pub fn container(args: &[Value]) -> Result<Value, String> {
    let props = extract_record(&args[0])?;
    let children = extract_children(&props)?;
    // Container can have children
}
```

### 2.3 Style System (Tailwind-like)

```rust
// crates/achronyme-eval/src/function_modules/ui/styles.rs

pub fn register_style_module(registry: &mut ModuleRegistry) {
    let mut module = Module::new("ui/styles");

    // Spacing
    module.register("p", create_padding_generator, -1);  // p.4, p.x.4, p.y.2
    module.register("m", create_margin_generator, -1);
    module.register("gap", create_gap_generator, -1);

    // Layout
    module.register("flex", create_flex_record, 0);
    module.register("grid", create_grid_record, 0);

    // Colors
    module.register("bg", create_bg_color_generator, 0);
    module.register("text", create_text_color_generator, 0);
    module.register("border", create_border_color_generator, 0);

    // Typography
    module.register("font", create_font_generator, 0);

    // Effects
    module.register("shadow", create_shadow_generator, 0);
    module.register("rounded", create_rounded_generator, 0);

    registry.register_module(module);
}
```

Implementación de estilos como records anidados:

```javascript
// El usuario puede acceder así:
bg.blue.500   // { type: "bg", color: "blue", shade: 500 }
text.lg       // { type: "text", size: "lg" }
p.4           // { type: "padding", value: 4 }
p.x.4         // { type: "padding", axis: "x", value: 4 }

// Internamente se convierten a:
{
    background: "rgb(59, 130, 246)",  // blue-500
    fontSize: "1.125rem",              // text-lg
    padding: "1rem"                    // p-4
}
```

## 3. Reactive State Management

### 3.1 State Store

```rust
// crates/achronyme-eval/src/ui/state.rs

pub struct UIState {
    components: HashMap<String, ComponentState>,
    listeners: HashMap<String, Vec<Callback>>,
    dirty: HashSet<String>,
}

impl UIState {
    pub fn update(&mut self, component_id: &str, key: &str, value: Value) {
        if let Some(state) = self.components.get_mut(component_id) {
            state.values.insert(key.to_string(), value);
            self.dirty.insert(component_id.to_string());
            self.notify_listeners(component_id);
        }
    }

    pub fn subscribe(&mut self, component_id: &str, callback: Callback) {
        self.listeners
            .entry(component_id.to_string())
            .or_insert_with(Vec::new)
            .push(callback);
    }
}
```

### 3.2 Reactive Updates en Achronyme

```javascript
// Signal-like API
import { signal, computed, effect } from "ui/reactive"

let count = signal(0)
let doubled = computed(() => count.value * 2)

effect(() => {
    print("Count is:", count.value)
})

// En componentes
let counter = Container({
    state: {
        count: signal(0)
    },
    children: [
        Text({
            text: computed(() => "Count: " + state.count.value)
        }),
        Button({
            label: "Increment",
            onClick: () => state.count.value = state.count.value + 1
        })
    ]
})
```

## 4. Chart System Integration

### 4.1 Chart Component

```rust
// crates/achronyme-eval/src/function_modules/ui/charts.rs

pub fn chart(args: &[Value]) -> Result<Value, String> {
    let props = extract_record(&args[0])?;

    let chart_type = extract_string(&props, "type")?;
    let data = extract_record(&props, "data")?;
    let options = extract_record(&props, "options")?;

    match chart_type.as_str() {
        "line" => create_line_chart(data, options),
        "bar" => create_bar_chart(data, options),
        "scatter" => create_scatter_chart(data, options),
        "pie" => create_pie_chart(data, options),
        _ => Err(format!("Unknown chart type: {}", chart_type))
    }
}

fn create_line_chart(data: Record, options: Record) -> Result<Value, String> {
    let x_data = extract_array(&data, "x")?;
    let y_data = extract_array(&data, "y")?;

    // Usar plotters o similar para generar el chart
    let chart_component = record!{
        "id" => generate_id(),
        "type" => "LineChart",
        "data" => data,
        "options" => options,
        "render" => create_render_function(x_data, y_data)
    };

    Ok(chart_component)
}
```

### 4.2 Chart Usage

```javascript
import { Chart, Plot } from "ui"
import { linspace, sin, cos } from "math"

let x = linspace(0, 2*pi, 100)

let sinChart = Chart({
    type: "line",
    data: {
        x: x,
        y: map(sin, x),
        label: "sin(x)"
    },
    options: {
        title: "Trigonometric Functions",
        xAxis: { label: "x" },
        yAxis: { label: "y" },
        grid: true,
        legend: true
    },
    class: [w.full, h.96]
})

// Multi-series chart
let multiChart = Chart({
    type: "line",
    data: [
        { x: x, y: map(sin, x), label: "sin(x)", color: "blue" },
        { x: x, y: map(cos, x), label: "cos(x)", color: "red" }
    ],
    options: {
        title: "Sin vs Cos",
        animations: true
    }
})

// Reactive chart con datos que cambian
let dataSignal = signal([1, 2, 3, 4, 5])

let liveChart = Chart({
    type: "bar",
    data: {
        x: range(1, 6),
        y: computed(() => dataSignal.value)
    },
    options: {
        title: "Live Data",
        updateInterval: 1000  // Update every second
    }
})

// Actualizar datos
dataSignal.value = map(x => random() * 10, range(1, 6))
```

## 5. Form System

### 5.1 Form API

```javascript
import { Form, FormGroup, Input, Select, Radio, Checkbox, Button } from "ui"
import { required, email, minLength, maxLength, pattern } from "ui/validation"

let registrationForm = Form({
    title: "User Registration",
    fields: [
        FormGroup({
            label: "Full Name",
            name: "fullName",
            input: Input({
                type: "text",
                placeholder: "John Doe",
                validators: [
                    required("Name is required"),
                    minLength(3, "Name must be at least 3 characters")
                ]
            })
        }),
        FormGroup({
            label: "Email",
            name: "email",
            input: Input({
                type: "email",
                placeholder: "john@example.com",
                validators: [
                    required("Email is required"),
                    email("Invalid email format")
                ]
            })
        }),
        FormGroup({
            label: "Password",
            name: "password",
            input: Input({
                type: "password",
                validators: [
                    required("Password is required"),
                    minLength(8, "Password must be at least 8 characters"),
                    pattern("^(?=.*[A-Z])(?=.*[0-9])", "Must contain uppercase and number")
                ]
            })
        }),
        FormGroup({
            label: "Gender",
            name: "gender",
            input: Radio({
                options: ["Male", "Female", "Other", "Prefer not to say"],
                defaultValue: "Prefer not to say"
            })
        }),
        FormGroup({
            label: "Interests",
            name: "interests",
            input: Checkbox({
                options: [
                    "Programming",
                    "Mathematics",
                    "Science",
                    "Arts"
                ],
                multiple: true
            })
        }),
        FormGroup({
            label: "Country",
            name: "country",
            input: Select({
                options: ["USA", "UK", "Canada", "Mexico", "Other"],
                searchable: true
            })
        })
    ],
    actions: [
        Button({
            label: "Submit",
            type: "submit",
            class: [bg.blue.500, text.white, px.6, py.3, rounded.md]
        }),
        Button({
            label: "Reset",
            type: "reset",
            class: [bg.gray.300, px.6, py.3, rounded.md]
        })
    ],
    onSubmit: data => {
        print("Form data:", data)
        // Validar y enviar
        if (validateForm(data)) {
            submitToServer(data)
        }
    },
    onValidationError: errors => {
        print("Validation errors:", errors)
    },
    class: [max.w.md, mx.auto, p.8, bg.white, rounded.xl, shadow.lg]
})
```

### 5.2 Form Validation

```rust
// crates/achronyme-eval/src/function_modules/ui/validation.rs

pub fn required(args: &[Value]) -> Result<Value, String> {
    let message = extract_string(&args[0])?;

    Ok(create_validator_record(
        "required",
        move |value: &Value| {
            if is_empty(value) {
                Err(message.clone())
            } else {
                Ok(())
            }
        }
    ))
}

pub fn email(args: &[Value]) -> Result<Value, String> {
    let message = extract_string(&args[0])?;
    let email_regex = Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();

    Ok(create_validator_record(
        "email",
        move |value: &Value| {
            let str_value = value.as_string()?;
            if email_regex.is_match(&str_value) {
                Ok(())
            } else {
                Err(message.clone())
            }
        }
    ))
}
```

## 6. Layout System

### 6.1 Flexbox Layout

```javascript
import { Container, Row, Column } from "ui"
import { flex, justify, items, gap } from "ui/styles"

let dashboard = Container({
    children: [
        // Header
        Row({
            children: [
                Text({ text: "Dashboard", class: [text.xl, font.bold] }),
                Button({ label: "Logout", class: [ml.auto] })
            ],
            class: [flex.row, justify.between, items.center, p.4, bg.gray.100]
        }),

        // Main content
        Row({
            children: [
                // Sidebar
                Column({
                    children: [
                        NavLink({ text: "Home", href: "/" }),
                        NavLink({ text: "Analytics", href: "/analytics" }),
                        NavLink({ text: "Settings", href: "/settings" })
                    ],
                    class: [w.64, p.4, bg.gray.50]
                }),

                // Content area
                Column({
                    children: [
                        statsCards,
                        dataChart,
                        dataTable
                    ],
                    class: [flex.1, p.6, gap.6]
                })
            ],
            class: [flex.row, flex.1]
        })
    ],
    class: [flex.col, h.screen]
})
```

### 6.2 Grid Layout

```javascript
import { Grid, GridItem } from "ui"

let photoGallery = Grid({
    columns: 4,
    gap: 4,
    children: map(
        photo => GridItem({
            children: [
                Image({ src: photo.url, alt: photo.title }),
                Text({ text: photo.title, class: [text.center, mt.2] })
            ],
            class: [rounded.lg, overflow.hidden, shadow]
        }),
        photos
    ),
    class: [p.6]
})

// Responsive grid
let responsiveGrid = Grid({
    columns: {
        sm: 1,
        md: 2,
        lg: 3,
        xl: 4
    },
    gap: 4,
    children: items
})
```

## 7. Window & Dialog System

### 7.1 Window API

```javascript
import { Window, Dialog, Modal } from "ui"

let mainWindow = Window({
    title: "My Application",
    width: 1024,
    height: 768,
    resizable: true,
    minimizable: true,
    maximizable: true,
    content: dashboard,
    onClose: () => {
        if (hasUnsavedChanges()) {
            confirmDialog.show()
        } else {
            window.close()
        }
    }
})

let confirmDialog = Dialog({
    title: "Unsaved Changes",
    content: Text({
        text: "You have unsaved changes. Do you want to save before closing?"
    }),
    actions: [
        Button({
            label: "Save",
            onClick: () => {
                saveData()
                window.close()
            }
        }),
        Button({
            label: "Don't Save",
            onClick: () => window.close()
        }),
        Button({
            label: "Cancel",
            onClick: () => confirmDialog.hide()
        })
    ],
    modal: true,
    class: [w.96]
})

// Modal overlay
let settingsModal = Modal({
    title: "Settings",
    content: settingsForm,
    onClose: () => settingsModal.hide(),
    overlay: true,
    closeOnOverlayClick: true,
    class: [max.w.2xl]
})
```

## 8. Event System

### 8.1 Event Handlers

```javascript
let interactiveButton = Button({
    label: "Click Me",
    onClick: event => {
        print("Button clicked at:", event.position)
        print("Modifiers:", event.modifiers)
    },
    onMouseEnter: () => {
        button.state.hovered = true
    },
    onMouseLeave: () => {
        button.state.hovered = false
    },
    onFocus: () => {
        button.state.focused = true
    },
    class: computed(() => [
        px.4, py.2, rounded,
        button.state.hovered ? bg.blue.600 : bg.blue.500,
        button.state.focused ? ring.2, ring.blue.300 : []
    ])
})

// Keyboard events
let searchInput = Input({
    placeholder: "Search...",
    onKeyPress: event => {
        if (event.key == "Enter") {
            performSearch(searchInput.value)
        }
        if (event.key == "Escape") {
            searchInput.value = ""
        }
    },
    onInput: value => {
        // Live search as user types
        debouncedSearch(value)
    }
})
```

## 9. Backend Renderers

### 9.1 Web Renderer (Canvas/WebGL)

```rust
// crates/achronyme-ui/src/renderers/web.rs

pub struct WebRenderer {
    canvas: Canvas,
    context: CanvasRenderingContext2d,
    components: HashMap<String, RenderedComponent>,
}

impl Renderer for WebRenderer {
    fn render(&mut self, component: &Component) -> Result<(), String> {
        match component.type_name.as_str() {
            "Input" => self.render_input(component),
            "Button" => self.render_button(component),
            "Text" => self.render_text(component),
            "Container" => self.render_container(component),
            _ => Err(format!("Unknown component type: {}", component.type_name))
        }
    }

    fn handle_event(&mut self, event: Event) {
        // Route events to components
    }
}
```

### 9.2 Native Renderer (egui/tauri)

```rust
// crates/achronyme-ui/src/renderers/native.rs

pub struct NativeRenderer {
    egui_ctx: egui::Context,
    components: HashMap<String, RenderedComponent>,
}

impl Renderer for NativeRenderer {
    fn render(&mut self, component: &Component) -> Result<(), String> {
        egui::CentralPanel::default().show(&self.egui_ctx, |ui| {
            self.render_component_tree(ui, component);
        });
        Ok(())
    }

    fn render_component_tree(&self, ui: &mut egui::Ui, component: &Component) {
        match component.type_name.as_str() {
            "Input" => {
                let value = component.props.get("value").unwrap().as_string();
                ui.text_edit_singleline(&mut value);
            }
            "Button" => {
                let label = component.props.get("label").unwrap().as_string();
                if ui.button(label).clicked() {
                    self.trigger_event(component, "onClick");
                }
            }
            // ...
        }
    }
}
```

### 9.3 TUI Renderer (ratatui)

```rust
// crates/achronyme-ui/src/renderers/tui.rs

pub struct TuiRenderer {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    components: HashMap<String, RenderedComponent>,
}

impl Renderer for TuiRenderer {
    fn render(&mut self, component: &Component) -> Result<(), String> {
        self.terminal.draw(|frame| {
            self.render_component_tree(frame, component);
        })?;
        Ok(())
    }

    fn render_component_tree(&self, frame: &mut Frame, component: &Component) {
        match component.type_name.as_str() {
            "Input" => {
                let value = component.props.get("value").unwrap().as_string();
                let input_widget = Paragraph::new(value)
                    .block(Block::default().borders(Borders::ALL));
                frame.render_widget(input_widget, area);
            }
            // ...
        }
    }
}
```

## 10. Implementation Plan

### Phase 1: Core Infrastructure (2-3 weeks)
1. Create `crates/achronyme-ui` crate
2. Implement Component record structure
3. Implement basic UI state management
4. Create base Renderer trait

### Phase 2: Basic Components (2-3 weeks)
1. Text, Label
2. Input (text, number, password)
3. Button
4. Container, Row, Column
5. Basic event system

### Phase 3: Style System (1-2 weeks)
1. Implement Tailwind-like style records
2. Style composition and merging
3. Responsive breakpoints
4. Theme system

### Phase 4: Form System (2 weeks)
1. Form, FormGroup
2. Radio, Checkbox, Select
3. Validation framework
4. Form submission

### Phase 5: Advanced Components (2-3 weeks)
1. Chart system (integrate with existing math/stats)
2. Table/DataGrid
3. Tabs, Accordion
4. Dropdown, Menu

### Phase 6: Layout & Windows (1-2 weeks)
1. Flexbox layout engine
2. Grid layout engine
3. Window management
4. Modal/Dialog system

### Phase 7: Renderers (3-4 weeks)
1. Web renderer (Canvas-based)
2. Native renderer (egui/tauri)
3. TUI renderer (ratatui)
4. Renderer abstraction and swapping

### Phase 8: Reactivity (2 weeks)
1. Signal/computed values
2. Effect system
3. Component lifecycle
4. Performance optimization

## 11. Example Application

```javascript
// Complete example: Data Dashboard
import { Window, Container, Row, Column } from "ui"
import { Chart, Table, Card } from "ui"
import { Input, Select, Button } from "ui"
import { flex, grid, gap, p, m, rounded, shadow, bg, text } from "ui/styles"
import { signal, computed } from "ui/reactive"
import { mean, std, linspace, sin } from "stats"

// State
let selectedDataset = signal("dataset1")
let dateRange = signal({ start: "2024-01-01", end: "2024-12-31" })

// Computed values
let filteredData = computed(() =>
    filterByDateRange(datasets[selectedDataset.value], dateRange.value)
)

let stats = computed(() => ({
    mean: mean(filteredData.value),
    std: std(filteredData.value),
    count: len(filteredData.value)
}))

// Components
let dashboard = Container({
    children: [
        // Header
        Row({
            children: [
                Text({
                    text: "Analytics Dashboard",
                    class: [text.2xl, font.bold, text.gray.800]
                }),
                Select({
                    value: selectedDataset,
                    options: ["dataset1", "dataset2", "dataset3"],
                    class: [ml.auto, w.48]
                })
            ],
            class: [flex.row, justify.between, items.center, p.6, bg.white, border.b]
        }),

        // Stats Cards
        Row({
            children: [
                Card({
                    title: "Mean",
                    value: computed(() => format("{:.2f}", stats.value.mean)),
                    class: [flex.1]
                }),
                Card({
                    title: "Std Dev",
                    value: computed(() => format("{:.2f}", stats.value.std)),
                    class: [flex.1]
                }),
                Card({
                    title: "Count",
                    value: computed(() => str(stats.value.count)),
                    class: [flex.1]
                })
            ],
            class: [flex.row, gap.4, p.6]
        }),

        // Chart
        Chart({
            type: "line",
            data: computed(() => ({
                x: range(1, len(filteredData.value) + 1),
                y: filteredData.value
            })),
            options: {
                title: "Data Trend",
                responsive: true,
                animations: true
            },
            class: [w.full, h.96, p.6]
        }),

        // Data Table
        Table({
            data: computed(() => filteredData.value),
            columns: [
                { key: "date", label: "Date", sortable: true },
                { key: "value", label: "Value", sortable: true },
                { key: "category", label: "Category", filterable: true }
            ],
            pagination: { pageSize: 20 },
            class: [w.full, p.6]
        })
    ],
    class: [flex.col, h.screen, bg.gray.50]
})

let app = Window({
    title: "Analytics Dashboard",
    width: 1280,
    height: 800,
    content: dashboard
})

app.render()
```

## 12. Benefits of This Approach

1. **Declarative**: UI is data (records), easy to reason about
2. **Composable**: Components compose naturally with function composition
3. **Type-safe**: Leverage Achronyme's type system
4. **Reactive**: Automatic updates with signals/computed
5. **Flexible**: Multiple backend renderers (web, native, TUI)
6. **Familiar**: Tailwind-like styles, React-like component model
7. **Integrated**: Works seamlessly with Achronyme's math/stats functions
8. **Testable**: Components are pure records, easy to test

## 13. Next Steps

1. Create RFC document with community feedback
2. Implement Phase 1 (core infrastructure)
3. Create example applications
4. Build documentation and tutorials
5. Iterate based on real-world usage

---

**Status**: Design Proposal
**Author**: Claude
**Date**: 2025-01-13
**Version**: 1.0
