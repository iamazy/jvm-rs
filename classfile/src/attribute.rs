#[derive(Debug, Clone)]
pub struct Attribute<'a> {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub attribute_type: AttributeType<'a>,
}

#[derive(Debug, Clone)]
pub enum AttributeType<'a> {
    ConstantValue {
        constant_value_index: u16,
    },
    Code {
        code: CodeAttribute<'a>,
    },
    StackMapTable {
        entries: Vec<StackMap>,
    },
    Exceptions {
        exception_index_table: Vec<u16>,
    },
    InnerClasses {
        classes: Vec<InnerClass>,
    },
    EnclosingMethod {
        class_index: u16,
        method_index: u16,
    },
    Synthetic,
    Signature {
        signature_index: u16,
    },
    SourceFile {
        sourcefile_index: u16,
    },
    SourceDebugExtension {
        debug_extension: Vec<u8>,
    },
    LineNumberTable {
        line_number_table: Vec<LineNumber>,
    },
    LocalVariableTable {
        local_variable_table: Vec<LocalVariable>,
    },
    LocalVariableTypeTable {
        local_variable_type_table: Vec<LocalVariableType>,
    },
    Deprecated,
    RuntimeVisibleAnnotations {
        annotations: Vec<Annotation>,
    },
    RuntimeInvisibleAnnotations {
        annotations: Vec<Annotation>,
    },
    RuntimeVisibleParameterAnnotations {
        parameter_annotations: Vec<ParameterAnnotation>,
    },
    RuntimeInvisibleParameterAnnotations {
        parameter_annotations: Vec<ParameterAnnotation>,
    },
    RuntimeVisibleTypeAnnotations {
        annotations: Vec<TypeAnnotation>,
    },
    RuntimeInvisibleTypeAnnotations {
        annotations: Vec<TypeAnnotation>,
    },
    AnnotationDefault {
        default_value: ElementValue,
    },
    BootstrapMethods {
        bootstrap_methods: Vec<BootstrapMethod>,
    },
    MethodParameters {
        parameters: Vec<MethodParameter>,
    },
    Module {
        module_name_index: u16,
        module_flags: u16,
        module_version_index: u16,
        requires: Vec<Require>,
        exports: Vec<Export>,
        opens: Vec<Open>,
        uses: Vec<u16>,
        provides: Vec<Provide>,
    },
    ModulePackages {
        package_index: Vec<u16>,
    },
    ModuleMainClass {
        main_class_index: u16,
    },
    NestHost {
        host_class_index: u16,
    },
    NestMembers {
        classes: Vec<u16>,
    },
    Record {
        components: Vec<RecordComponent<'a>>,
    },
    PermittedSubclasses {
        classes: Vec<u16>,
    },
    Unknown {
        data: &'a [u8],
    },
}

#[derive(Debug, Clone)]
pub enum AttributeTag {
    ConstantValue,
    Code,
    StackMapTable,
    Exceptions,
    InnerClasses,
    EnclosingMethod,
    Synthetic,
    Signature,
    SourceFile,
    SourceDebugExtension,
    LineNumberTable,
    LocalVariableTable,
    LocalVariableTypeTable,
    Deprecated,
    RuntimeVisibleAnnotations,
    RuntimeInvisibleAnnotations,
    RuntimeVisibleParameterAnnotations,
    RuntimeInvisibleParameterAnnotations,
    RuntimeVisibleTypeAnnotations,
    RuntimeInvisibleTypeAnnotations,
    AnnotationDefault,
    BootstrapMethods,
    MethodParameters,
    Module,
    ModulePackages,
    ModuleMainClass,
    NestHost,
    NestMembers,
    Record,
    PermittedSubclasses,
    Unknown,
}

impl From<&[u8]> for AttributeTag {
    fn from(tag: &[u8]) -> Self {
        match tag {
            b"ConstantValue" => AttributeTag::ConstantValue,
            b"Code" => AttributeTag::Code,
            b"StackMapTable" => AttributeTag::StackMapTable,
            b"Exceptions" => AttributeTag::Exceptions,
            b"InnerClasses" => AttributeTag::InnerClasses,
            b"EnclosingMethod" => AttributeTag::EnclosingMethod,
            b"Synthetic" => AttributeTag::Synthetic,
            b"Signature" => AttributeTag::Signature,
            b"SourceFile" => AttributeTag::SourceFile,
            b"SourceDebugExtension" => AttributeTag::SourceDebugExtension,
            b"LineNumberTable" => AttributeTag::LineNumberTable,
            b"LocalVariableTable" => AttributeTag::LocalVariableTable,
            b"LocalVariableTypeTable" => AttributeTag::LocalVariableTypeTable,
            b"Deprecated" => AttributeTag::Deprecated,
            b"RuntimeVisibleAnnotations" => AttributeTag::RuntimeVisibleAnnotations,
            b"RuntimeInvisibleAnnotations" => AttributeTag::RuntimeInvisibleAnnotations,
            b"RuntimeVisibleParameterAnnotations" => {
                AttributeTag::RuntimeVisibleParameterAnnotations
            }
            b"RuntimeInvisibleParameterAnnotations" => {
                AttributeTag::RuntimeInvisibleParameterAnnotations
            }
            b"RuntimeVisibleTypeAnnotations" => AttributeTag::RuntimeVisibleTypeAnnotations,
            b"RuntimeInvisibleTypeAnnotations" => AttributeTag::RuntimeInvisibleTypeAnnotations,
            b"AnnotationDefault" => AttributeTag::AnnotationDefault,
            b"BootstrapMethods" => AttributeTag::BootstrapMethods,
            b"MethodParameters" => AttributeTag::MethodParameters,
            b"Module" => AttributeTag::Module,
            b"ModulePackages" => AttributeTag::ModulePackages,
            b"ModuleMainClass" => AttributeTag::ModuleMainClass,
            b"NestHost" => AttributeTag::NestHost,
            b"NestMembers" => AttributeTag::NestMembers,
            b"Record" => AttributeTag::Record,
            b"PermittedSubclasses" => AttributeTag::PermittedSubclasses,
            _ => AttributeTag::Unknown,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CodeAttribute<'a> {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: &'a [u8],
    pub exception_table: Vec<Exception>,
    pub attributes: Vec<Attribute<'a>>,
}

#[derive(Debug, Clone)]
pub struct Exception {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

#[derive(Debug, Clone)]
pub struct StackMap {
    pub frame_type: u8,
    pub frame: StackMapFrame,
}

#[derive(Debug, Clone)]
pub enum StackMapFrame {
    SameFrame,
    SameLocals1StackItemFrame {
        stack: VerificationTypeInfo,
    },
    SameLocals1StackItemFrameExtended {
        offset_delta: u16,
        stack: VerificationTypeInfo,
    },
    ChopFrame {
        offset_delta: u16,
    },
    SameFrameExtended {
        offset_delta: u16,
    },
    AppendFrame {
        offset_delta: u16,
        locals: Vec<VerificationTypeInfo>,
    },
    FullFrame {
        offset_delta: u16,
        locals: Vec<VerificationTypeInfo>,
        stack: Vec<VerificationTypeInfo>,
    },
}

#[derive(Debug, Clone)]
pub enum VerificationTypeInfo {
    Top,
    Integer,
    Float,
    Long,
    Double,
    Null,
    UninitializedThis,
    Object { cpool_index: u16 },
    Uninitialized { offset: u16 },
}

#[derive(Debug, Clone)]
pub struct InnerClass {
    pub inner_class_info_index: u16,
    pub outer_class_info_index: u16,
    pub inner_name_index: u16,
    pub inner_class_access_flags: u16,
}

#[derive(Debug, Clone)]
pub struct LineNumber {
    pub start_pc: u16,
    pub line_number: u16,
}

#[derive(Debug, Clone)]
pub struct LocalVariable {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub index: u16,
}

#[derive(Debug, Clone)]
pub struct LocalVariableType {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub signature_index: u16,
    pub index: u16,
}

#[derive(Debug, Clone)]
pub struct Annotation {
    pub type_index: u16,
    pub element_value_pairs: Vec<ElementValuePair>,
}

#[derive(Debug, Clone)]
pub struct ElementValuePair {
    pub element_name_index: u16,
    pub value: ElementValue,
}

#[derive(Debug, Clone)]
pub struct ElementValue {
    pub tag: u8,
    pub value: Element,
}

#[derive(Debug, Clone)]
pub enum Element {
    ConstValueIndex(u16),
    EnumConstValue {
        type_name_index: u16,
        const_name_index: u16,
    },
    ClassInfoIndex(u16),
    AnnotationValue(Annotation),
    ArrayValue(Vec<ElementValue>),
}

#[derive(Debug, Clone)]
pub struct ParameterAnnotation {
    pub annotations: Vec<Annotation>,
}

#[derive(Debug, Clone)]
pub struct TypeAnnotation {
    pub target_type: u8,
    pub target_info: TargetInfo,
    pub type_path: TypePath,
    pub type_index: u16,
    pub element_value_pairs: Vec<(u16, ElementValue)>,
}

#[derive(Debug, Clone)]
pub enum TargetInfo {
    TypeParameterTarget {
        type_parameter_index: u8,
    },
    SupertypeTarget {
        supertype_index: u16,
    },
    TypeParameterBoundTarget {
        type_parameter_index: u8,
        bound_index: u8,
    },
    EmptyTarget,
    FormalParameterTarget {
        formal_parameter_index: u8,
    },
    ThrowTarget {
        throws_type_index: u16,
    },
    LocalVarTarget(Vec<LocalVar>),
    CatchTarget {
        exception_table_index: u16,
    },
    OffsetTarget {
        offset: u16,
    },
    TypeArgumentTarget {
        offset: u16,
        type_argument_index: u8,
    },
}

#[derive(Debug, Clone)]
pub struct TypePath {
    pub path: Vec<TypePathPair>,
}

#[derive(Debug, Clone)]
pub struct TypePathPair {
    pub type_path_kind: u8,
    pub type_argument_index: u8,
}

#[derive(Debug, Clone)]
pub struct LocalVar {
    pub start_pc: u16,
    pub length: u16,
    pub index: u16,
}

#[derive(Debug, Clone)]
pub struct BootstrapMethod {
    pub bootstrap_method_ref: u16,
    pub bootstrap_arguments: Vec<u16>,
}

#[derive(Debug, Clone)]
pub struct MethodParameter {
    pub name_index: u16,
    pub access_flags: u16,
}

#[derive(Debug, Clone)]
pub struct Require {
    pub require_index: u16,
    pub require_flags: u16,
    pub require_version_index: u16,
}

#[derive(Debug, Clone)]
pub struct Export {
    pub export_index: u16,
    pub export_flags: u16,
    pub export_to_index: Vec<u16>,
}

#[derive(Debug, Clone)]
pub struct Open {
    pub open_index: u16,
    pub open_flags: u16,
    pub open_to_index: Vec<u16>,
}

#[derive(Debug, Clone)]
pub struct Provide {
    pub provide_index: u16,
    pub provide_with_index: Vec<u16>,
}

#[derive(Debug, Clone)]
pub struct RecordComponent<'a> {
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<Attribute<'a>>,
}
