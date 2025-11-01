# Comparación Visual de Arquitecturas

## 🏗️ Arquitectura Actual (Stateless)

```
┌─────────────────────────────────────────────────────────────┐
│                    JavaScript Layer                         │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Llamada 1:  Module.eval("let x = 5")                      │
│       │                                                      │
│       ▼                                                      │
│  ┌────────────────────────────────────┐                     │
│  │ C++: eval(expression)              │                     │
│  │  ┌──────────────────────────────┐  │                     │
│  │  │ Evaluator eval₁              │  │                     │
│  │  │  └─ Environment env₁         │  │                     │
│  │  │      └─ variables: {x: 5}    │  │ ✓ x existe aquí    │
│  │  └──────────────────────────────┘  │                     │
│  │  return "5"                        │                     │
│  └────────────────────────────────────┘                     │
│       │                                                      │
│       ▼                                                      │
│  ❌ eval₁ se destruye → env₁ se destruye → x desaparece     │
│                                                             │
│  ─────────────────────────────────────────────────────────  │
│                                                             │
│  Llamada 2:  Module.eval("x + 10")                         │
│       │                                                      │
│       ▼                                                      │
│  ┌────────────────────────────────────┐                     │
│  │ C++: eval(expression)              │                     │
│  │  ┌──────────────────────────────┐  │                     │
│  │  │ Evaluator eval₂ (NUEVO)      │  │                     │
│  │  │  └─ Environment env₂ (VACÍO) │  │                     │
│  │  │      └─ variables: {}        │  │ ❌ x no existe     │
│  │  └──────────────────────────────┘  │                     │
│  │  return "Error: Undefined 'x'"     │                     │
│  └────────────────────────────────────┘                     │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## 🏗️ Arquitectura con Evaluador Global (Stateful)

```
┌─────────────────────────────────────────────────────────────┐
│                    JavaScript Layer                         │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Llamada 1:  Module.eval("let x = 5")                      │
│       │                                                      │
│       ▼                                                      │
│  ┌────────────────────────────────────┐                     │
│  │ C++: eval(expression)              │                     │
│  │                                    │                     │
│  │  Usa evaluador global ──────────┐ │                     │
│  │                                 │ │                     │
│  │  return "5"                     │ │                     │
│  └──────────────────────────────────│─┘                     │
│                                    │                        │
│  ════════════════════════════════════════════════════════   │
│  ║  MEMORIA GLOBAL (Persiste)      ▼                    ║   │
│  ║  ┌───────────────────────────────────────┐           ║   │
│  ║  │ static Evaluator globalEvaluator     │           ║   │
│  ║  │   └─ Environment env                 │           ║   │
│  ║  │       └─ variables: {x: 5} ◄─── Guardado aquí   ║   │
│  ║  └───────────────────────────────────────┘           ║   │
│  ║                                                       ║   │
│  ║  ✅ No se destruye entre llamadas                    ║   │
│  ════════════════════════════════════════════════════════   │
│                                    │                        │
│  Llamada 2:  Module.eval("x + 10") │                       │
│       │                            │                        │
│       ▼                            │                        │
│  ┌────────────────────────────────│┐                        │
│  │ C++: eval(expression)          │                        │
│  │                                │                        │
│  │  Usa evaluador global ◄────────┘                        │
│  │    env.get("x")  // ✓ Encuentra 5                      │
│  │                                                         │
│  │  return "15"                                            │
│  └────────────────────────────────────────────────────────┘ │
│                                                             │
│  ✅ Variables persisten entre llamadas                      │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## 🏗️ Arquitectura con Sesiones (Multi-Contexto)

```
┌─────────────────────────────────────────────────────────────┐
│                    JavaScript Layer                         │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Module.createSession("A")                                  │
│  Module.useSession("A")                                     │
│  Module.eval("let x = 5")                                   │
│       │                                                      │
│       ▼                                                      │
│  ════════════════════════════════════════════════════════   │
│  ║  MEMORIA GLOBAL - Mapa de Sesiones               ║      │
│  ║  ┌─────────────────────────────────────────────┐ ║      │
│  ║  │ sessions = {                                 │ ║      │
│  ║  │                                              │ ║      │
│  ║  │   "A": Evaluator ─┬─> Environment          │ ║      │
│  ║  │                   │    └─ variables: {x: 5} │ ║      │
│  ║  │                   │                          │ ║      │
│  ║  │   "B": Evaluator ─┼─> Environment          │ ║      │
│  ║  │                   │    └─ variables: {}     │ ║      │
│  ║  │                   │                          │ ║      │
│  ║  │   "default": Eval─┘─> Environment          │ ║      │
│  ║  │                        └─ variables: {}     │ ║      │
│  ║  │ }                                            │ ║      │
│  ║  │                                              │ ║      │
│  ║  │ currentSessionId = "A"                       │ ║      │
│  ║  └─────────────────────────────────────────────┘ ║      │
│  ════════════════════════════════════════════════════        │
│                                                             │
│  Module.useSession("B")                                     │
│  Module.eval("let x = 100")  // No afecta session "A"      │
│                                                             │
│  Module.useSession("A")                                     │
│  Module.eval("x + 10")       // → "15" (x=5 preservado)    │
│                                                             │
│  ✅ Múltiples contextos aislados                            │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## 📈 Flujo de Datos Comparado

### Stateless (Actual)
```
eval("let x=5")     eval("x+10")
     │                   │
     ▼                   ▼
  Evaluator₁         Evaluator₂
  env₁: {x:5}        env₂: {}
     │                   │
  Destruye            ❌ Error
     x
```

### Global (Stateful)
```
eval("let x=5")     eval("x+10")
     │                   │
     └─────┬─────────────┘
           ▼
    GlobalEvaluator
    env: {x:5} ──> {x:5}
           │
        ✅ "15"
```

### Sesiones
```
Session A:              Session B:
eval("let x=5")         eval("let x=100")
     │                       │
     ▼                       ▼
Evaluator_A             Evaluator_B
env_A: {x:5}            env_B: {x:100}
     │                       │
     └───────────────────────┘
     Ambos coexisten sin conflicto
```

---

## 🔄 Ciclo de Vida de Variables

### Stateless
```
let x = 5
   ↓
   Nace en env₁
   ↓
   Retorna "5"
   ↓
   ❌ Muere (destructor)
   ↓
   Gone forever
```

### Global
```
let x = 5
   ↓
   Nace en globalEnv
   ↓
   Retorna "5"
   ↓
   ✅ Sigue vivo (static)
   ↓
   Disponible para próximas llamadas
   ↓
   Muere solo con reset() o exit
```

### Sesiones
```
Session A: let x = 5       Session B: let x = 100
     ↓                          ↓
   env_A: {x:5}              env_B: {x:100}
     ↓                          ↓
   Vive mientras              Vive mientras
   session existe             session existe
     ↓                          ↓
   deleteSession("A")         deleteSession("B")
     ↓                          ↓
   ❌ Muere                    ❌ Muere
```

---

## 💾 Uso de Memoria

```
┌────────────────────────────────────────────────────────┐
│                 Memory Usage Comparison                │
├────────────────────────────────────────────────────────┤
│                                                        │
│  Stateless:                                            │
│  │                                                     │
│  ├─ eval("let x=5")                                   │
│  │   Memory: [Evaluator(~100 bytes)] ─┐              │
│  │                                     │              │
│  │   Retorna                           ▼              │
│  └─> Memory: []  (freed)               │              │
│                                         │              │
│  ├─ eval("x+10")                       │              │
│  │   Memory: [Evaluator(~100 bytes)] ─┘              │
│  │                                                     │
│  └─> Memory: []  (freed)                              │
│                                                        │
│  Peak: ~100 bytes                                     │
│  Average: ~50 bytes (freed between calls)             │
│                                                        │
├────────────────────────────────────────────────────────┤
│                                                        │
│  Global:                                               │
│  │                                                     │
│  ├─ Startup                                           │
│  │   Memory: [GlobalEvaluator(~100 bytes)]           │
│  │            └─ env: {}                              │
│  │                                                     │
│  ├─ eval("let x=5")                                   │
│  │   Memory: [GlobalEvaluator(~100 bytes)]           │
│  │            └─ env: {x:5} (~50 bytes)               │
│  │                                                     │
│  ├─ eval("let y=[1,2,3]")                             │
│  │   Memory: [GlobalEvaluator(~100 bytes)]           │
│  │            └─ env: {x:5, y:vec} (~100 bytes)       │
│  │                                                     │
│  └─> Memory persiste                                  │
│                                                        │
│  Peak: ~200 bytes + (variables size)                  │
│  Average: ~100-500 bytes (depends on variables)       │
│                                                        │
├────────────────────────────────────────────────────────┤
│                                                        │
│  Sesiones (3 sessions):                                │
│  │                                                     │
│  ├─ Session A: eval("let x=5")                        │
│  │   Memory: [Evaluator_A(~100 bytes)]               │
│  │                                                     │
│  ├─ Session B: eval("let y=10")                       │
│  │   Memory: [Evaluator_A(~100 bytes)]               │
│  │           [Evaluator_B(~100 bytes)]               │
│  │                                                     │
│  ├─ Session C: eval("let z=[1..100]")                │
│  │   Memory: [Evaluator_A(~100 bytes)]               │
│  │           [Evaluator_B(~100 bytes)]               │
│  │           [Evaluator_C(~100 bytes + 800 bytes)]   │
│  │                                                     │
│  └─> Memory: ~1000+ bytes                             │
│                                                        │
│  Peak: N × 100 bytes + (all variables)                │
│  Average: ~300-3000 bytes (depends on N sessions)     │
│                                                        │
└────────────────────────────────────────────────────────┘
```

---

## 🎯 Decisión por Caso de Uso

```
Use Case: Interactive Calculator/REPL
├─ Need: Variables persist
├─ Users: 1
└─ Solution: ✅ GLOBAL

Use Case: Jupyter Notebook
├─ Need: Variables persist, can clear
├─ Users: 1
└─ Solution: ✅ GLOBAL (with reset())

Use Case: Automated Tests
├─ Need: Isolated tests
├─ Users: N parallel
└─ Solution: ✅ STATELESS or SESSIONS

Use Case: Multi-User Web App
├─ Need: Isolated per user
├─ Users: N concurrent
└─ Solution: ✅ SESSIONS

Use Case: One-off calculations
├─ Need: No persistence
├─ Users: Any
└─ Solution: ✅ STATELESS (current)
```
