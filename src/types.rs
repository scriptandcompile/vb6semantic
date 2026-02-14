use crate::error::{Result, SourceLocation};
use serde::{Deserialize, Serialize};

/// Type information for VB6 symbols
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypeInfo {
    /// The kind of type
    pub kind: TypeKind,

    /// Whether this is a reference type (ByRef)
    pub is_reference: bool,

    /// Whether this is an array
    pub is_array: bool,

    /// Array dimensions (if is_array is true)
    pub array_dimensions: Option<Vec<ArrayBound>>,

    /// For Object and Class types, the class name
    pub class_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TypeKind {
    // Primitive types
    Integer,
    Long,
    Single,
    Double,
    Currency,
    String,
    Boolean,
    Byte,
    Date,

    // Complex types
    Variant,
    Object,
    Class(String),
    UserType(String),
    Enum(String),

    // Special types
    Nothing,
    Empty,
    Null,

    // Function types
    Sub,
    Function { return_type: Box<TypeInfo> },

    // Unknown/unresolved
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArrayBound {
    pub lower: Option<i32>,
    pub upper: Option<i32>,
}

impl TypeInfo {
    pub fn new(kind: TypeKind) -> Self {
        Self {
            kind,
            is_reference: false,
            is_array: false,
            array_dimensions: None,
            class_name: None,
        }
    }

    pub fn integer() -> Self {
        Self::new(TypeKind::Integer)
    }

    pub fn long() -> Self {
        Self::new(TypeKind::Long)
    }

    pub fn string() -> Self {
        Self::new(TypeKind::String)
    }

    pub fn boolean() -> Self {
        Self::new(TypeKind::Boolean)
    }

    pub fn variant() -> Self {
        Self::new(TypeKind::Variant)
    }

    pub fn object() -> Self {
        Self::new(TypeKind::Object)
    }

    pub fn unknown() -> Self {
        Self::new(TypeKind::Unknown)
    }

    /// Check if this type is compatible with another type
    pub fn is_compatible_with(&self, other: &TypeInfo) -> bool {
        // Variant is compatible with everything
        if matches!(self.kind, TypeKind::Variant) || matches!(other.kind, TypeKind::Variant) {
            return true;
        }

        // Same types are compatible
        if self.kind == other.kind {
            return true;
        }

        // Numeric types have some compatibility
        match (&self.kind, &other.kind) {
            (TypeKind::Integer, TypeKind::Long) |
            (TypeKind::Long, TypeKind::Integer) |
            (TypeKind::Single, TypeKind::Double) |
            (TypeKind::Double, TypeKind::Single) => true,

            (TypeKind::Object, TypeKind::Class(_)) |
            (TypeKind::Class(_), TypeKind::Object) => true,

            _ => false,
        }
    }

    /// Check if this type can be assigned to another type
    pub fn can_assign_to(&self, other: &TypeInfo) -> bool {
        // Variant can be assigned to anything
        if matches!(other.kind, TypeKind::Variant) {
            return true;
        }

        // Same types can be assigned
        if self.kind == other.kind {
            return true;
        }

        // Smaller numeric types can be assigned to larger
        match (&self.kind, &other.kind) {
            (TypeKind::Integer, TypeKind::Long) |
            (TypeKind::Integer, TypeKind::Double) |
            (TypeKind::Long, TypeKind::Double) |
            (TypeKind::Single, TypeKind::Double) => true,

            _ => false,
        }
    }

    /// Get a string representation of this type
    pub fn to_string(&self) -> String {
        let base = match &self.kind {
            TypeKind::Integer => "Integer",
            TypeKind::Long => "Long",
            TypeKind::Single => "Single",
            TypeKind::Double => "Double",
            TypeKind::Currency => "Currency",
            TypeKind::String => "String",
            TypeKind::Boolean => "Boolean",
            TypeKind::Byte => "Byte",
            TypeKind::Date => "Date",
            TypeKind::Variant => "Variant",
            TypeKind::Object => "Object",
            TypeKind::Class(name) => return name.clone(),
            TypeKind::UserType(name) => return name.clone(),
            TypeKind::Enum(name) => return name.clone(),
            TypeKind::Nothing => "Nothing",
            TypeKind::Empty => "Empty",
            TypeKind::Null => "Null",
            TypeKind::Sub => "Sub",
            TypeKind::Function { return_type } => return format!("Function -> {}", return_type.to_string()),
            TypeKind::Unknown => "Unknown",
        };

        if self.is_array {
            format!("{}()", base)
        } else {
            base.to_string()
        }
    }
}

/// Type checker for VB6 code
pub struct TypeChecker {
    // Future: store type relationships, conversion rules, etc.
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {}
    }

    /// Check if an assignment is valid
    pub fn check_assignment(
        &self,
        target_type: &TypeInfo,
        source_type: &TypeInfo,
        _location: &SourceLocation,
    ) -> Result<()> {
        if source_type.can_assign_to(target_type) {
            Ok(())
        } else {
            Err(crate::error::SemanticError::TypeMismatch {
                expected: target_type.to_string(),
                found: source_type.to_string(),
                location: _location.clone(),
            })
        }
    }

    /// Check if two types are compatible for an operation
    pub fn check_operation(
        &self,
        left_type: &TypeInfo,
        right_type: &TypeInfo,
        _operation: &str,
        _location: &SourceLocation,
    ) -> Result<TypeInfo> {
        // Variant propagates
        if matches!(left_type.kind, TypeKind::Variant) || matches!(right_type.kind, TypeKind::Variant) {
            return Ok(TypeInfo::variant());
        }

        // Numeric operations
        if self.is_numeric(&left_type.kind) && self.is_numeric(&right_type.kind) {
            return Ok(self.promote_numeric_types(left_type, right_type));
        }

        // String concatenation
        if matches!(left_type.kind, TypeKind::String) || matches!(right_type.kind, TypeKind::String) {
            return Ok(TypeInfo::string());
        }

        // Default to variant for unknown operations
        Ok(TypeInfo::variant())
    }

    fn is_numeric(&self, kind: &TypeKind) -> bool {
        matches!(kind, 
            TypeKind::Integer | 
            TypeKind::Long | 
            TypeKind::Single | 
            TypeKind::Double | 
            TypeKind::Currency |
            TypeKind::Byte
        )
    }

    fn promote_numeric_types(&self, left: &TypeInfo, right: &TypeInfo) -> TypeInfo {
        // Promotion rules: Byte < Integer < Long < Single < Double < Currency
        use TypeKind::*;
        
        let promoted = match (&left.kind, &right.kind) {
            (Double, _) | (_, Double) => Double,
            (Currency, _) | (_, Currency) => Currency,
            (Single, _) | (_, Single) => Single,
            (Long, _) | (_, Long) => Long,
            (Integer, _) | (_, Integer) => Integer,
            (Byte, Byte) => Byte,
            _ => Variant,
        };

        TypeInfo::new(promoted)
    }
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}
