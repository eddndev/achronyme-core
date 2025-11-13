# Native GUI System for Achronyme

## Vision

Create a native desktop GUI system for Achronyme using records as components, enabling scientists and engineers to build interactive data visualization and analysis tools with a simple, declarative API.

## Target Use Cases

1. **Scientific Computing**: Interactive parameter tuning, real-time visualization
2. **Data Analysis**: Charts, plots, statistical dashboards
3. **Numerical Tools**: Equation solvers with live feedback
4. **Educational Software**: Mathematical demonstrations, interactive learning
5. **Engineering Applications**: Signal processing, optimization tools

## Core Principles

1. **Native First**: Direct integration with OS GUI frameworks (no web)
2. **Records as Components**: UI elements are records with methods and mutable state
3. **Immediate Mode GUI**: Simple, direct rendering model (inspired by egui)
4. **Data Visualization**: First-class support for charts and plots
5. **Scientific Focus**: Built for math, science, and engineering workflows

## Native GUI Backend: egui

We'll use [egui](https://github.com/emilk/egui) - a Rust immediate mode GUI library that:
- ✅ Pure Rust (easy integration)
- ✅ Cross-platform (Windows, macOS, Linux)
- ✅ Immediate mode (simpler than retained mode)
- ✅ Good performance
- ✅ Built-in widgets (buttons, sliders, plots)
- ✅ Active development and community

### Alternative Considered: iced
- More complex (retained mode)
- Better for large applications
- Overkill for our use cases

## Architecture

```
┌──────────────────────────────────────────────────────┐
│           Achronyme Application Code                  │
│     (Records, Functions, State Management)            │
└──────────────────────────────────────────────────────┘
                       ↓
┌──────────────────────────────────────────────────────┐
│              GUI Module (Rust)                        │
│  • Component Bridge (Record → egui)                   │
│  • State Management (Signals)                         │
│  • Event Loop                                         │
│  • Layout Engine                                      │
└──────────────────────────────────────────────────────┘
                       ↓
┌──────────────────────────────────────────────────────┐
│                  egui Framework                       │
│  • Native Rendering                                   │
│  • Input Handling                                     │
│  • Widget Library                                     │
└──────────────────────────────────────────────────────┘
                       ↓
┌──────────────────────────────────────────────────────┐
│            OS Window Manager                          │
│        (Windows / macOS / Linux)                      │
└──────────────────────────────────────────────────────┘
```

## Achronyme GUI API

### Basic Window

```javascript
import { Window, run } from "gui"
import { createSignal } from "gui/state"

let app = () => {
    let counter = createSignal(0)

    {
        title: "My App",
        width: 400,
        height: 300,

        render: (ui) => {
            ui.label("Counter: " + str(counter.get()))

            if(ui.button("Increment")) {
                counter.set(counter.get() + 1)
            }

            if(ui.button("Reset")) {
                counter.set(0)
            }
        }
    }
}

run(app())
```

### Components as Records

```javascript
// Counter component
let Counter = (initialValue) => {
    let count = createSignal(initialValue)

    {
        value: count,

        increment: () => count.set(count.get() + 1),
        decrement: () => count.set(count.get() - 1),
        reset: () => count.set(initialValue),

        render: (ui) => {
            ui.horizontal(() => {
                if(ui.button("-")) {
                    self.decrement()
                }

                ui.label(str(count.get()))

                if(ui.button("+")) {
                    self.increment()
                }
            })
        }
    }
}

// Usage
let app = () => {
    let counter1 = Counter(0)
    let counter2 = Counter(10)

    {
        title: "Multi-Counter",

        render: (ui) => {
            ui.heading("Counter 1")
            counter1.render(ui)

            ui.separator()

            ui.heading("Counter 2")
            counter2.render(ui)
        }
    }
}
```

## Widget API

### Layout Widgets

```javascript
// Horizontal layout
ui.horizontal(() => {
    ui.button("Left")
    ui.button("Center")
    ui.button("Right")
})

// Vertical layout (default)
ui.vertical(() => {
    ui.button("Top")
    ui.button("Middle")
    ui.button("Bottom")
})

// Grid layout
ui.grid("my-grid", 2, () => {  // 2 columns
    ui.label("Name:")
    ui.textEdit(nameSignal)

    ui.label("Age:")
    ui.numberInput(ageSignal)
})

// Columns with proportional widths
ui.columns([0.3, 0.7], () => {
    // Left column (30%)
    ui.label("Sidebar")

    // Right column (70%)
    ui.label("Main content")
})
```

### Input Widgets

```javascript
// Button - returns true when clicked
if(ui.button("Click me")) {
    print("Button clicked!")
}

// Text input - mutates signal
let text = createSignal("")
ui.textEdit(text)  // Automatically updates signal

// Number input with drag
let value = createSignal(0.0)
ui.dragValue(value, 0.1)  // Step of 0.1

// Slider
let slider = createSignal(50.0)
ui.slider(slider, 0.0, 100.0)  // min, max

// Checkbox
let checked = createSignal(false)
ui.checkbox("Enable feature", checked)

// Radio buttons
let selected = createSignal("option1")
ui.radioButton("Option 1", "option1", selected)
ui.radioButton("Option 2", "option2", selected)

// Combo box (dropdown)
let choice = createSignal("apple")
ui.comboBox("Select fruit", choice, ["apple", "banana", "orange"])
```

### Display Widgets

```javascript
// Label
ui.label("Hello, World!")

// Heading (different sizes)
ui.heading("Main Title")        // h1
ui.heading2("Subtitle")          // h2
ui.heading3("Section")           // h3

// Monospace text (for code/data)
ui.monospace("let x = 42")

// Colored text
ui.coloredLabel("Error!", "#ff0000")
ui.coloredLabel("Success!", "#00ff00")

// Hyperlink
if(ui.hyperlink("Documentation", "https://example.com")) {
    // Link was clicked
}

// Separator line
ui.separator()

// Spacing
ui.spacing(10)  // Add vertical space
```

### Data Visualization

```javascript
// Line plot
let data = createSignal([[0, 0], [1, 1], [2, 4], [3, 9]])

ui.plot("My Plot", {
    width: 400,
    height: 300,
    data: data.get(),
    xLabel: "Time (s)",
    yLabel: "Value",
    showGrid: true
})

// Real-time plotting
let points = createSignal([])

ui.plot("Live Data", {
    data: points.get(),
    autoScale: true
})

if(ui.button("Add Point")) {
    let current = points.get()
    let newX = len(current)
    let newY = sin(newX * 0.1) * 100
    points.set([...current, [newX, newY]])
}

// Multiple series
ui.plot("Comparison", {
    series: [
        { name: "sin(x)", data: sinData.get(), color: "#ff0000" },
        { name: "cos(x)", data: cosData.get(), color: "#0000ff" }
    ]
})

// Histogram
ui.histogram("Distribution", {
    data: samples.get(),
    bins: 20
})

// Scatter plot
ui.scatter("Correlation", {
    points: xyData.get(),
    pointSize: 3
})
```

### Containers

```javascript
// Collapsing header (accordion)
if(ui.collapsingHeader("Advanced Options")) {
    ui.checkbox("Debug mode", debugMode)
    ui.slider(threshold, 0.0, 1.0)
}

// Window (sub-window/dialog)
if(ui.window("Settings", settingsOpen, () => {
    ui.checkbox("Auto-save", autoSave)
    ui.button("Apply")
})) {
    // Window is open and visible
}

// Panel (bordered area)
ui.panel("Results", () => {
    ui.label("Mean: " + str(mean(data)))
    ui.label("Std Dev: " + str(std(data)))
})

// Scroll area
ui.scrollArea({height: 200}, () => {
    // Content that might be taller than 200px
    map((item) => ui.label(item), longList.get())
})
```

## Scientific Computing Integration

### Interactive Parameter Tuning

```javascript
import { integral, derivative } from "numerical"
import { createSignal, createComputed } from "gui/state"

let InteractiveIntegral = () => {
    // Parameters
    let a = createSignal(0.0)
    let b = createSignal(1.0)
    let n = createSignal(100)

    // Function definition
    let funcStr = createSignal("x^2")

    // Computed result
    let result = createComputed(() => {
        // Parse and evaluate integral
        let f = parseFunction(funcStr.get())
        integral(f, a.get(), b.get())
    }, [a, b, funcStr])

    {
        title: "Interactive Integration",
        width: 600,
        height: 400,

        render: (ui) => {
            ui.heading("Definite Integral Calculator")

            ui.horizontal(() => {
                ui.label("f(x) =")
                ui.textEdit(funcStr)
            })

            ui.horizontal(() => {
                ui.label("Lower bound:")
                ui.dragValue(a, 0.1)

                ui.label("Upper bound:")
                ui.dragValue(b, 0.1)
            })

            ui.separator()

            ui.heading2("Result")
            ui.monospace("∫ " + funcStr.get() + " dx from " +
                        str(a.get()) + " to " + str(b.get()))
            ui.heading3("= " + str(result.get()))

            // Visualization
            let plotData = generatePlotData(funcStr.get(), a.get(), b.get())
            ui.plot("Function", {
                data: plotData,
                fillUnder: true,
                xRange: [a.get() - 1, b.get() + 1]
            })
        }
    }
}

run(InteractiveIntegral())
```

### Data Analysis Dashboard

```javascript
import { mean, std, histogram } from "stats"
import { createSignal } from "gui/state"

let DataDashboard = () => {
    let data = createSignal([])
    let inputText = createSignal("")

    {
        title: "Data Analysis",
        width: 800,
        height: 600,

        render: (ui) => {
            // Left panel - input
            ui.columns([0.3, 0.7], () => {
                // Input panel
                ui.panel("Data Input", () => {
                    ui.label("Enter numbers (one per line):")
                    ui.textEditMultiline(inputText, 300)

                    if(ui.button("Load Data")) {
                        let lines = split(inputText.get(), "\n")
                        let numbers = map(parseFloat, lines)
                        data.set(filter(x => !isNaN(x), numbers))
                    }

                    if(ui.button("Generate Random")) {
                        data.set(map(() => random() * 100, range(0, 100)))
                    }
                })

                // Results panel
                if(len(data.get()) > 0) {
                    ui.panel("Statistics", () => {
                        let d = data.get()

                        ui.label("Count: " + str(len(d)))
                        ui.label("Mean: " + str(mean(d)))
                        ui.label("Std Dev: " + str(std(d)))
                        ui.label("Min: " + str(min(d)))
                        ui.label("Max: " + str(max(d)))
                    })

                    ui.separator()

                    // Histogram
                    ui.histogram("Distribution", {
                        data: data.get(),
                        bins: 20
                    })

                    // Raw data plot
                    let indexedData = map(
                        (val, idx) => [idx, val],
                        data.get()
                    )
                    ui.plot("Raw Data", {
                        data: indexedData,
                        type: "scatter"
                    })
                } else {
                    ui.label("No data loaded")
                }
            })
        }
    }
}
```

### Signal Processing Visualizer

```javascript
import { fft, ifft } from "dsp"
import { linspace } from "numerical"

let SignalProcessor = () => {
    // Signal parameters
    let frequency = createSignal(5.0)
    let amplitude = createSignal(1.0)
    let sampleRate = createSignal(100.0)

    // Computed signals
    let timeSignal = createComputed(() => {
        let t = linspace(0, 1, sampleRate.get())
        map(x => amplitude.get() * sin(2 * PI * frequency.get() * x), t)
    }, [frequency, amplitude, sampleRate])

    let spectrum = createComputed(() => {
        fft(timeSignal.get())
    }, [timeSignal])

    {
        title: "Signal Processor",
        width: 900,
        height: 700,

        render: (ui) => {
            ui.heading("Signal Processing Tool")

            // Controls
            ui.panel("Parameters", () => {
                ui.horizontal(() => {
                    ui.label("Frequency (Hz):")
                    ui.slider(frequency, 0.1, 50.0)
                    ui.label(str(frequency.get()))
                })

                ui.horizontal(() => {
                    ui.label("Amplitude:")
                    ui.slider(amplitude, 0.0, 2.0)
                    ui.label(str(amplitude.get()))
                })

                ui.horizontal(() => {
                    ui.label("Sample Rate:")
                    ui.slider(sampleRate, 10.0, 1000.0)
                    ui.label(str(sampleRate.get()))
                })
            })

            ui.separator()

            // Visualization
            ui.columns([0.5, 0.5], () => {
                // Time domain
                ui.panel("Time Domain", () => {
                    let plotData = map(
                        (val, idx) => [idx / sampleRate.get(), val],
                        timeSignal.get()
                    )
                    ui.plot("Signal", {
                        data: plotData,
                        xLabel: "Time (s)",
                        yLabel: "Amplitude"
                    })
                })

                // Frequency domain
                ui.panel("Frequency Domain", () => {
                    let spec = spectrum.get()
                    let magnitudes = map(
                        (c, idx) => [idx * sampleRate.get() / len(spec), abs(c)],
                        spec
                    )
                    ui.plot("Spectrum", {
                        data: magnitudes,
                        xLabel: "Frequency (Hz)",
                        yLabel: "Magnitude"
                    })
                })
            })
        }
    }
}
```

## State Management

### Signals (Reactive Values)

```javascript
let createSignal = (initialValue) => {
    mut value = initialValue
    mut subscribers = []

    {
        get: () => value,

        set: (newValue) => do {
            value = newValue;
            map(fn => fn(newValue), subscribers)
        },

        subscribe: (fn) => do {
            subscribers = [...subscribers, fn];
            () => do {
                subscribers = filter(f => f != fn, subscribers)
            }
        }
    }
}
```

### Computed Values

```javascript
let createComputed = (computation, dependencies) => {
    mut value = computation()

    let update = () => do {
        let newValue = computation();
        if(newValue != value) {
            value = newValue
        }
    }

    // Subscribe to all dependencies
    map(dep => dep.subscribe(update), dependencies)

    {
        get: () => value
    }
}
```

## Implementation Plan

### Phase 1: Core GUI Infrastructure (3-4 weeks)

**Week 1-2: Rust Integration**
- [ ] Add egui dependency to Cargo.toml
- [ ] Create gui module in Rust
- [ ] Implement event loop
- [ ] Basic window creation
- [ ] Bridge Achronyme values to Rust

**Week 3-4: Basic Widgets**
- [ ] Button
- [ ] Label
- [ ] TextEdit
- [ ] Slider
- [ ] Checkbox
- [ ] Basic layouts (horizontal, vertical)

**Deliverable**: Simple counter app that compiles and runs

### Phase 2: State Management (1-2 weeks)

**Week 5-6:**
- [ ] Implement createSignal in Achronyme
- [ ] Implement createComputed
- [ ] Auto-update GUI on state changes
- [ ] Memory management for signals

**Deliverable**: Interactive calculator with reactive state

### Phase 3: Data Visualization (2-3 weeks)

**Week 7-9:**
- [ ] Integrate plotting library (egui_plot)
- [ ] Line plots
- [ ] Scatter plots
- [ ] Histograms
- [ ] Real-time data updates
- [ ] Multiple series support

**Deliverable**: Live signal visualizer

### Phase 4: Advanced Widgets (2 weeks)

**Week 10-11:**
- [ ] Combo boxes
- [ ] Radio buttons
- [ ] Multi-line text edit
- [ ] Collapsing headers
- [ ] Scroll areas
- [ ] Panels/containers

**Deliverable**: Data analysis dashboard

### Phase 5: Scientific Integration (2 weeks)

**Week 12-13:**
- [ ] Integration with numerical functions
- [ ] Integration with DSP functions
- [ ] Integration with stats functions
- [ ] Integration with graph theory
- [ ] Interactive parameter tuning examples

**Deliverable**: Complete scientific toolkit GUI

### Phase 6: Polish & Documentation (1 week)

**Week 14:**
- [ ] Error handling
- [ ] Performance optimization
- [ ] Complete documentation
- [ ] Tutorial examples
- [ ] Best practices guide

## File Structure

```
crates/
├── achronyme-gui/
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs              # Main GUI module
│   │   ├── bridge.rs           # Achronyme ↔ Rust bridge
│   │   ├── widgets/
│   │   │   ├── mod.rs
│   │   │   ├── button.rs
│   │   │   ├── text.rs
│   │   │   ├── input.rs
│   │   │   └── plot.rs
│   │   ├── layout.rs           # Layout engine
│   │   ├── state.rs            # Signal management
│   │   └── event_loop.rs       # GUI event loop
│   └── examples/
│       ├── counter.soc
│       ├── calculator.soc
│       └── signal_processor.soc

examples/
└── gui/
    ├── 01-hello-gui.soc
    ├── 02-interactive-plot.soc
    ├── 03-data-analysis.soc
    ├── 04-signal-processing.soc
    └── 05-optimization-tool.soc
```

## Built-in Functions API

```javascript
// In Rust, register these functions:
// module: "gui"

// Window management
gui.run(appRecord)                 // Start GUI event loop
gui.createWindow(title, w, h)      // Create window

// State management
gui.createSignal(initialValue)
gui.createComputed(fn, deps)

// Widgets (called from render callback)
ui.label(text)
ui.button(text) -> bool
ui.checkbox(text, signal) -> bool
ui.slider(signal, min, max)
ui.textEdit(signal)
ui.plot(name, options)

// Layout
ui.horizontal(callback)
ui.vertical(callback)
ui.columns(widths, callback)
ui.grid(name, cols, callback)
```

## Example: Complete Scientific App

```javascript
import { createSignal, createComputed } from "gui/state"
import { integral, derivative } from "numerical"
import { mean, std } from "stats"

let ScientificWorkbench = () => {
    // Function definition
    let funcStr = createSignal("sin(x)")
    let x = createSignal(0.0)

    // Analysis parameters
    let a = createSignal(-PI)
    let b = createSignal(PI)
    let samples = createSignal(100)

    // Computed values
    let func = createComputed(() => parseFunction(funcStr.get()), [funcStr])
    let value = createComputed(() => func.get()(x.get()), [func, x])
    let deriv = createComputed(() => derivative(func.get(), x.get()), [func, x])
    let integralVal = createComputed(() =>
        integral(func.get(), a.get(), b.get()),
        [func, a, b]
    )

    {
        title: "Scientific Workbench",
        width: 1000,
        height: 700,

        render: (ui) => {
            ui.heading("Function Analysis Tool")

            // Function input
            ui.horizontal(() => {
                ui.label("f(x) =")
                ui.textEdit(funcStr)
            })

            ui.separator()

            // Three-column layout
            ui.columns([0.25, 0.5, 0.25], () => {
                // Left: Controls
                ui.panel("Controls", () => {
                    ui.heading3("Evaluation")
                    ui.horizontal(() => {
                        ui.label("x =")
                        ui.dragValue(x, 0.1)
                    })
                    ui.label("f(x) = " + str(value.get()))
                    ui.label("f'(x) = " + str(deriv.get()))

                    ui.separator()

                    ui.heading3("Integration")
                    ui.horizontal(() => {
                        ui.label("from")
                        ui.dragValue(a, 0.1)
                        ui.label("to")
                        ui.dragValue(b, 0.1)
                    })
                    ui.label("∫f(x)dx = " + str(integralVal.get()))
                })

                // Center: Plots
                ui.panel("Visualization", () => {
                    // Generate plot data
                    let xs = linspace(a.get(), b.get(), samples.get())
                    let ys = map(func.get(), xs)
                    let plotData = map((x, i) => [x, ys[i]], xs)

                    ui.plot("Function", {
                        data: plotData,
                        xLabel: "x",
                        yLabel: "f(x)",
                        markers: [[x.get(), value.get()]]
                    })

                    // Derivative plot
                    let derivYs = map(x => derivative(func.get(), x), xs)
                    let derivData = map((x, i) => [x, derivYs[i]], xs)

                    ui.plot("Derivative", {
                        data: derivData,
                        xLabel: "x",
                        yLabel: "f'(x)"
                    })
                })

                // Right: Statistics
                ui.panel("Statistics", () => {
                    let xs = linspace(a.get(), b.get(), samples.get())
                    let ys = map(func.get(), xs)

                    ui.label("Samples: " + str(len(ys)))
                    ui.label("Mean: " + str(mean(ys)))
                    ui.label("Std: " + str(std(ys)))
                    ui.label("Min: " + str(min(ys)))
                    ui.label("Max: " + str(max(ys)))
                })
            })
        }
    }
}

run(ScientificWorkbench())
```

## Next Steps

1. **Set up egui integration**: Add dependencies and create basic bridge
2. **Implement core widgets**: Button, Label, TextEdit as proof of concept
3. **Create simple example**: Counter app to validate architecture
4. **Iterate on API**: Refine based on usage patterns
5. **Add plotting**: Integrate data visualization
6. **Build real app**: Scientific calculator or data analyzer

## Questions to Resolve

1. How to handle window closing and cleanup?
2. Should we support multiple windows?
3. How to handle file dialogs (open/save)?
4. Menu bar API design?
5. Keyboard shortcuts system?
6. Theme/styling customization?
