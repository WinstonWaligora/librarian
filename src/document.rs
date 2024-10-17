// This file is generated by rust-protobuf 2.27.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `document.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_27_1;

#[derive(PartialEq,Clone,Default)]
pub struct Document {
    // message fields
    pub name: ::std::string::String,
    pub subject: ::std::string::String,
    pub content: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Document {
    fn default() -> &'a Document {
        <Document as ::protobuf::Message>::default_instance()
    }
}

impl Document {
    pub fn new() -> Document {
        ::std::default::Default::default()
    }

    // string name = 1;


    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    // string subject = 2;


    pub fn get_subject(&self) -> &str {
        &self.subject
    }
    pub fn clear_subject(&mut self) {
        self.subject.clear();
    }

    // Param is passed by value, moved
    pub fn set_subject(&mut self, v: ::std::string::String) {
        self.subject = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_subject(&mut self) -> &mut ::std::string::String {
        &mut self.subject
    }

    // Take field
    pub fn take_subject(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.subject, ::std::string::String::new())
    }

    // string content = 3;


    pub fn get_content(&self) -> &str {
        &self.content
    }
    pub fn clear_content(&mut self) {
        self.content.clear();
    }

    // Param is passed by value, moved
    pub fn set_content(&mut self, v: ::std::string::String) {
        self.content = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_content(&mut self) -> &mut ::std::string::String {
        &mut self.content
    }

    // Take field
    pub fn take_content(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.content, ::std::string::String::new())
    }
}

impl ::protobuf::Message for Document {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.subject)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.content)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if !self.subject.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.subject);
        }
        if !self.content.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.content);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if !self.subject.is_empty() {
            os.write_string(2, &self.subject)?;
        }
        if !self.content.is_empty() {
            os.write_string(3, &self.content)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Document {
        Document::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "name",
                |m: &Document| { &m.name },
                |m: &mut Document| { &mut m.name },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "subject",
                |m: &Document| { &m.subject },
                |m: &mut Document| { &mut m.subject },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "content",
                |m: &Document| { &m.content },
                |m: &mut Document| { &mut m.content },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Document>(
                "Document",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Document {
        static instance: ::protobuf::rt::LazyV2<Document> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Document::new)
    }
}

impl ::protobuf::Clear for Document {
    fn clear(&mut self) {
        self.name.clear();
        self.subject.clear();
        self.content.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Document {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Document {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Library {
    // message fields
    pub documents: ::protobuf::RepeatedField<Document>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Library {
    fn default() -> &'a Library {
        <Library as ::protobuf::Message>::default_instance()
    }
}

impl Library {
    pub fn new() -> Library {
        ::std::default::Default::default()
    }

    // repeated .Document documents = 1;


    pub fn get_documents(&self) -> &[Document] {
        &self.documents
    }
    pub fn clear_documents(&mut self) {
        self.documents.clear();
    }

    // Param is passed by value, moved
    pub fn set_documents(&mut self, v: ::protobuf::RepeatedField<Document>) {
        self.documents = v;
    }

    // Mutable pointer to the field.
    pub fn mut_documents(&mut self) -> &mut ::protobuf::RepeatedField<Document> {
        &mut self.documents
    }

    // Take field
    pub fn take_documents(&mut self) -> ::protobuf::RepeatedField<Document> {
        ::std::mem::replace(&mut self.documents, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for Library {
    fn is_initialized(&self) -> bool {
        for v in &self.documents {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.documents)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.documents {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.documents {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Library {
        Library::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Document>>(
                "documents",
                |m: &Library| { &m.documents },
                |m: &mut Library| { &mut m.documents },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Library>(
                "Library",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Library {
        static instance: ::protobuf::rt::LazyV2<Library> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Library::new)
    }
}

impl ::protobuf::Clear for Library {
    fn clear(&mut self) {
        self.documents.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Library {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Library {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0edocument.proto\"R\n\x08Document\x12\x12\n\x04name\x18\x01\x20\x01(\
    \tR\x04name\x12\x18\n\x07subject\x18\x02\x20\x01(\tR\x07subject\x12\x18\
    \n\x07content\x18\x03\x20\x01(\tR\x07content\"2\n\x07Library\x12'\n\tdoc\
    uments\x18\x01\x20\x03(\x0b2\t.DocumentR\tdocumentsJ\xac\x02\n\x06\x12\
    \x04\0\0\n\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\n\n\x02\x04\0\x12\x04\
    \x02\0\x06\x01\n\n\n\x03\x04\0\x01\x12\x03\x02\x08\x10\n\x0b\n\x04\x04\0\
    \x02\0\x12\x03\x03\x04\x14\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x03\x04\n\
    \n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x03\x0b\x0f\n\x0c\n\x05\x04\0\x02\0\
    \x03\x12\x03\x03\x12\x13\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x04\x04\x17\n\
    \x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x04\x04\n\n\x0c\n\x05\x04\0\x02\x01\
    \x01\x12\x03\x04\x0b\x12\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x04\x15\
    \x16\n\x0b\n\x04\x04\0\x02\x02\x12\x03\x05\x04\x17\n\x0c\n\x05\x04\0\x02\
    \x02\x05\x12\x03\x05\x04\n\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x05\x0b\
    \x12\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x05\x15\x16\n\n\n\x02\x04\x01\
    \x12\x04\x08\0\n\x01\n\n\n\x03\x04\x01\x01\x12\x03\x08\x08\x0f\n\x0b\n\
    \x04\x04\x01\x02\0\x12\x03\t\x04$\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03\
    \t\x04\x0c\n\x0c\n\x05\x04\x01\x02\0\x06\x12\x03\t\r\x15\n\x0c\n\x05\x04\
    \x01\x02\0\x01\x12\x03\t\x16\x1f\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\t\
    \"#b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
