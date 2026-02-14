# vb6semantic

Semantic analysis and symbol table construction for Visual Basic 6 code.

## Overview

`vb6semantic` provides semantic analysis capabilities for VB6 code parsed by [vb6parse](../vb6parse). It builds symbol tables, performs type checking, resolves names, and validates semantic correctness of VB6 code.

## Features

- **Symbol Tables**: Build comprehensive symbol tables for VB6 projects
- **Scope Management**: Hierarchical scope management (global, class, procedure, block)
- **Name Resolution**: Resolve symbol references with proper scoping rules
- **Type Checking**: Validate type compatibility and assignments
- **Visibility Rules**: Enforce Public/Private/Friend visibility rules
- **Error Reporting**: Detailed semantic error messages with source locations

## Usage

### Basic Analysis

```rust
use vb6semantic::{SemanticAnalyzer, AnalysisResult};
use vb6parse::parsers::parse_project;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse VB6 project
    let project = parse_project("path/to/project.vbp")?;
    
    // Create semantic analyzer
    let mut analyzer = SemanticAnalyzer::new();
    
    // Analyze the project
    let result = analyzer.analyze_project(&project)?;
    
    // Check for errors
    if result.is_successful() {
        println!("Analysis successful!");
        println!("Warnings: {}", result.warning_count());
    } else {
        println!("Analysis failed with {} errors", result.error_count());
        for error in result.errors {
            eprintln!("Error: {}", error);
        }
    }
    
    Ok(())
}
```

### Symbol Lookup

```rust
use vb6semantic::{SemanticAnalyzer, SymbolKind};

let mut analyzer = SemanticAnalyzer::new();
// ... analyze code ...

// Look up a symbol
if let Some(symbol) = analyzer.lookup_symbol("MyFunction") {
    println!("Found: {} (kind: {:?})", symbol.name, symbol.kind);
    println!("Type: {}", symbol.type_info.to_string());
    println!("Visibility: {:?}", symbol.visibility);
}

// Get all functions
let scope_manager = analyzer.scope_manager();
// ... query scope manager ...
```

### Type Checking

```rust
use vb6semantic::{TypeChecker, TypeInfo, SourceLocation};
use std::path::PathBuf;

let checker = TypeChecker::new();

let int_type = TypeInfo::integer();
let string_type = TypeInfo::string();

let location = SourceLocation {
    file: PathBuf::from("test.bas"),
    line: 10,
    column: 5,
};

// Check if assignment is valid
match checker.check_assignment(&string_type, &int_type, &location) {
    Ok(()) => println!("Assignment valid"),
    Err(e) => eprintln!("Type mismatch: {}", e),
}
```

### Custom Symbol Creation

```rust
use vb6semantic::{Symbol, SymbolKind, Visibility, TypeInfo, SourceLocation};
use std::path::PathBuf;
use std::collections::HashMap;

let symbol = Symbol {
    name: "MyVariable".to_string(),
    kind: SymbolKind::Variable,
    type_info: TypeInfo::integer(),
    visibility: Visibility::Public,
    location: SourceLocation {
        file: PathBuf::from("module.bas"),
        line: 5,
        column: 10,
    },
    scope_id: 0,
    attributes: HashMap::new(),
};
```

## Architecture

### Core Components

1. **Symbol Table** (`symbols.rs`)
   - Manages symbols across scopes
   - Handles symbol definitions and lookups
   - Tracks symbol metadata

2. **Scope Manager** (`scope.rs`)
   - Hierarchical scope management
   - Parent-child scope relationships
   - Name resolution with scope walking

3. **Type System** (`types.rs`)
   - Type information representation
   - Type compatibility checking
   - Type promotion rules

4. **Semantic Analyzer** (`analyzer.rs`)
   - Main entry point for analysis
   - Coordinates symbol table building
   - Collects errors and warnings

5. **Name Resolver** (`resolution.rs`)
   - Resolves symbol references
   - Handles qualified names
   - Checks accessibility

### Symbol Kinds

The following symbol kinds are supported:

- **Variable**: Module/local variables
- **Constant**: Constant declarations
- **SubProcedure**: Sub procedures
- **Function**: Functions
- **PropertyGet/Let/Set**: Property accessors
- **Class**: Class definitions
- **Module**: Module files
- **Form**: Form files
- **Control**: Form controls
- **Enum**: Enumeration types
- **EnumMember**: Enumeration values
- **UserType**: User-defined types
- **TypeMember**: Type fields
- **Parameter**: Procedure parameters
- **Label**: GoTo labels

### Scope Hierarchy

```
Global Scope (Module/Form)
  ├─ Class Scope
  │   ├─ Property Scope
  │   │   └─ Block Scope
  │   └─ Procedure Scope
  │       └─ Block Scope
  └─ Procedure Scope
      └─ Block Scope
```

### Type System

Supported VB6 types:

**Primitive Types:**
- Integer, Long, Byte
- Single, Double, Currency
- String, Boolean, Date

**Complex Types:**
- Variant (universal type)
- Object (generic object)
- Class (specific class)
- UserType (custom type)
- Enum (enumeration)

**Special Types:**
- Nothing, Empty, Null
- Sub, Function

**Type Rules:**
- Variant is compatible with all types
- Numeric promotions (Byte → Integer → Long → Single → Double)
- Object-Class relationships

## Integration with vb6parse

This library works on the output of vb6parse:

```rust
// VB6 code is parsed into structures
let project = vb6parse::parsers::parse_project("project.vbp")?;
let module = vb6parse::parsers::parse_module("module.bas")?;
let class = vb6parse::parsers::parse_class("class.cls")?;
let form = vb6parse::parsers::parse_form("form.frm")?;

// Then analyzed for semantics
let mut analyzer = SemanticAnalyzer::new();
analyzer.analyze_project(&project)?;
analyzer.analyze_module(&module)?;
analyzer.analyze_class(&class)?;
analyzer.analyze_form(&form)?;
```

## Use Cases

### IDE Support
- Code completion (symbol suggestions)
- Go to definition
- Find references
- Rename refactoring
- Type information on hover

### Static Analysis
- Find unused variables
- Detect undefined symbols
- Check type safety
- Validate visibility rules
- Detect dead code

### Code Transformation
- Provide symbol information for converters
- Enable semantic-aware refactoring
- Support automated migration

### Documentation Generation
- Extract symbol information
- Generate API documentation
- Create cross-references

## Implementation Status

### ✅ Completed
- Core type system
- Symbol representation
- Scope management framework
- Basic error types
- Type compatibility checking

### 🚧 In Progress
- Full VB6 project analysis
- Complete name resolution
- All type checking rules

### 📋 Planned
- Control flow analysis
- Dead code detection
- Advanced type inference
- Cross-reference tracking
- Performance optimizations

## Examples

See [docs/EXAMPLES.md](docs/EXAMPLES.md) for detailed examples.

## Testing

```bash
# Run tests
cargo test -p vb6semantic

# Run with output
cargo test -p vb6semantic -- --nocapture

# Test specific module
cargo test -p vb6semantic --lib symbols
```

## Documentation

- [DESIGN.md](docs/DESIGN.md) - Detailed design documentation
- [SYMBOL_TABLES.md](docs/SYMBOL_TABLES.md) - Symbol table design
- [TYPE_SYSTEM.md](docs/TYPE_SYSTEM.md) - Type system documentation
- [SCOPING.md](docs/SCOPING.md) - Scoping rules
- [API.md](docs/API.md) - API reference

## Contributing

Contributions welcome! Areas that need work:

- Complete implementation of analysis passes
- Additional type checking rules
- Performance improvements
- More comprehensive tests
- Better error messages

## License

MIT License - see [LICENSE](LICENSE) for details.

## Related Projects

- [vb6parse](../vb6parse) - VB6 parser library
- [vb6convert](../vb6convert) - VB6 conversion framework
- [aspen](../aspen) - VB6 project tools
