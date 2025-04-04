// This file is generated by rust-protobuf 3.0.3. Do not edit
// .proto file is parsed by protoc 3.20.1
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
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `src/config/geoip.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_0_3;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:CIDR)
pub struct CIDR {
    // message fields
    // @@protoc_insertion_point(field:CIDR.ip)
    pub ip: ::bytes::Bytes,
    // @@protoc_insertion_point(field:CIDR.prefix)
    pub prefix: u32,
    // special fields
    // @@protoc_insertion_point(special_field:CIDR.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CIDR {
    fn default() -> &'a CIDR {
        <CIDR as ::protobuf::Message>::default_instance()
    }
}

impl CIDR {
    pub fn new() -> CIDR {
        ::std::default::Default::default()
    }

    // bytes ip = 1;

    pub fn ip(&self) -> &[u8] {
        &self.ip
    }

    // uint32 prefix = 2;

    pub fn prefix(&self) -> u32 {
        self.prefix
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ip",
            |m: &CIDR| { &m.ip },
            |m: &mut CIDR| { &mut m.ip },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "prefix",
            |m: &CIDR| { &m.prefix },
            |m: &mut CIDR| { &mut m.prefix },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CIDR>(
            "CIDR",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CIDR {
    const NAME: &'static str = "CIDR";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.ip = is.read_tokio_bytes()?;
                },
                16 => {
                    self.prefix = is.read_uint32()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.ip.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.ip);
        }
        if self.prefix != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.prefix);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.ip.is_empty() {
            os.write_bytes(1, &self.ip)?;
        }
        if self.prefix != 0 {
            os.write_uint32(2, self.prefix)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> CIDR {
        CIDR::new()
    }

    fn clear(&mut self) {
        self.ip.clear();
        self.prefix = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CIDR {
        static instance: CIDR = CIDR {
            ip: ::bytes::Bytes::new(),
            prefix: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CIDR {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CIDR").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CIDR {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CIDR {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:GeoIP)
pub struct GeoIP {
    // message fields
    // @@protoc_insertion_point(field:GeoIP.country_code)
    pub country_code: ::std::string::String,
    // @@protoc_insertion_point(field:GeoIP.cidr)
    pub cidr: ::std::vec::Vec<CIDR>,
    // special fields
    // @@protoc_insertion_point(special_field:GeoIP.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GeoIP {
    fn default() -> &'a GeoIP {
        <GeoIP as ::protobuf::Message>::default_instance()
    }
}

impl GeoIP {
    pub fn new() -> GeoIP {
        ::std::default::Default::default()
    }

    // string country_code = 1;

    pub fn country_code(&self) -> &str {
        &self.country_code
    }

    // repeated .CIDR cidr = 2;

    pub fn cidr(&self) -> &[CIDR] {
        &self.cidr
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "country_code",
            |m: &GeoIP| { &m.country_code },
            |m: &mut GeoIP| { &mut m.country_code },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "cidr",
            |m: &GeoIP| { &m.cidr },
            |m: &mut GeoIP| { &mut m.cidr },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GeoIP>(
            "GeoIP",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GeoIP {
    const NAME: &'static str = "GeoIP";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.country_code = is.read_string()?;
                },
                18 => {
                    self.cidr.push(is.read_message()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.country_code.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.country_code);
        }
        for value in &self.cidr {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.country_code.is_empty() {
            os.write_string(1, &self.country_code)?;
        }
        for v in &self.cidr {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> GeoIP {
        GeoIP::new()
    }

    fn clear(&mut self) {
        self.country_code.clear();
        self.cidr.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GeoIP {
        static instance: GeoIP = GeoIP {
            country_code: ::std::string::String::new(),
            cidr: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GeoIP {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GeoIP").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GeoIP {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GeoIP {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:GeoIPList)
pub struct GeoIPList {
    // message fields
    // @@protoc_insertion_point(field:GeoIPList.entry)
    pub entry: ::std::vec::Vec<GeoIP>,
    // special fields
    // @@protoc_insertion_point(special_field:GeoIPList.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GeoIPList {
    fn default() -> &'a GeoIPList {
        <GeoIPList as ::protobuf::Message>::default_instance()
    }
}

impl GeoIPList {
    pub fn new() -> GeoIPList {
        ::std::default::Default::default()
    }

    // repeated .GeoIP entry = 1;

    pub fn entry(&self) -> &[GeoIP] {
        &self.entry
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "entry",
            |m: &GeoIPList| { &m.entry },
            |m: &mut GeoIPList| { &mut m.entry },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GeoIPList>(
            "GeoIPList",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GeoIPList {
    const NAME: &'static str = "GeoIPList";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.entry.push(is.read_message()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        for value in &self.entry {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.entry {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> GeoIPList {
        GeoIPList::new()
    }

    fn clear(&mut self) {
        self.entry.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GeoIPList {
        static instance: GeoIPList = GeoIPList {
            entry: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GeoIPList {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GeoIPList").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GeoIPList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GeoIPList {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16src/config/geoip.proto\".\n\x04CIDR\x12\x0e\n\x02ip\x18\x01\x20\
    \x01(\x0cR\x02ip\x12\x16\n\x06prefix\x18\x02\x20\x01(\rR\x06prefix\"E\n\
    \x05GeoIP\x12!\n\x0ccountry_code\x18\x01\x20\x01(\tR\x0bcountryCode\x12\
    \x19\n\x04cidr\x18\x02\x20\x03(\x0b2\x05.CIDRR\x04cidr\")\n\tGeoIPList\
    \x12\x1c\n\x05entry\x18\x01\x20\x03(\x0b2\x06.GeoIPR\x05entryb\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(3);
            messages.push(CIDR::generated_message_descriptor_data());
            messages.push(GeoIP::generated_message_descriptor_data());
            messages.push(GeoIPList::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
