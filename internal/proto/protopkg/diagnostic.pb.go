// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.32.0
// 	protoc        v4.25.3
// source: protopkg/diagnostic.proto

package protopkg

import (
	protoreflect "google.golang.org/protobuf/reflect/protoreflect"
	protoimpl "google.golang.org/protobuf/runtime/protoimpl"
	reflect "reflect"
	sync "sync"
)

const (
	// Verify that this generated code is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(20 - protoimpl.MinVersion)
	// Verify that runtime/protoimpl is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(protoimpl.MaxVersion - 20)
)

type Diagnostic_Severity int32

const (
	Diagnostic_INVALID Diagnostic_Severity = 0
	Diagnostic_ERR     Diagnostic_Severity = 1
	Diagnostic_WARN    Diagnostic_Severity = 2
)

// Enum value maps for Diagnostic_Severity.
var (
	Diagnostic_Severity_name = map[int32]string{
		0: "INVALID",
		1: "ERR",
		2: "WARN",
	}
	Diagnostic_Severity_value = map[string]int32{
		"INVALID": 0,
		"ERR":     1,
		"WARN":    2,
	}
)

func (x Diagnostic_Severity) Enum() *Diagnostic_Severity {
	p := new(Diagnostic_Severity)
	*p = x
	return p
}

func (x Diagnostic_Severity) String() string {
	return protoimpl.X.EnumStringOf(x.Descriptor(), protoreflect.EnumNumber(x))
}

func (Diagnostic_Severity) Descriptor() protoreflect.EnumDescriptor {
	return file_protopkg_diagnostic_proto_enumTypes[0].Descriptor()
}

func (Diagnostic_Severity) Type() protoreflect.EnumType {
	return &file_protopkg_diagnostic_proto_enumTypes[0]
}

func (x Diagnostic_Severity) Number() protoreflect.EnumNumber {
	return protoreflect.EnumNumber(x)
}

// Deprecated: Use Diagnostic_Severity.Descriptor instead.
func (Diagnostic_Severity) EnumDescriptor() ([]byte, []int) {
	return file_protopkg_diagnostic_proto_rawDescGZIP(), []int{0, 0}
}

type Diagnostic struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Severity Diagnostic_Severity `protobuf:"varint,1,opt,name=severity,proto3,enum=protopkg.Diagnostic_Severity" json:"severity,omitempty"`
	Summary  string              `protobuf:"bytes,2,opt,name=summary,proto3" json:"summary,omitempty"`
	Detail   string              `protobuf:"bytes,3,opt,name=detail,proto3" json:"detail,omitempty"`
}

func (x *Diagnostic) Reset() {
	*x = Diagnostic{}
	if protoimpl.UnsafeEnabled {
		mi := &file_protopkg_diagnostic_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *Diagnostic) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Diagnostic) ProtoMessage() {}

func (x *Diagnostic) ProtoReflect() protoreflect.Message {
	mi := &file_protopkg_diagnostic_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Diagnostic.ProtoReflect.Descriptor instead.
func (*Diagnostic) Descriptor() ([]byte, []int) {
	return file_protopkg_diagnostic_proto_rawDescGZIP(), []int{0}
}

func (x *Diagnostic) GetSeverity() Diagnostic_Severity {
	if x != nil {
		return x.Severity
	}
	return Diagnostic_INVALID
}

func (x *Diagnostic) GetSummary() string {
	if x != nil {
		return x.Summary
	}
	return ""
}

func (x *Diagnostic) GetDetail() string {
	if x != nil {
		return x.Detail
	}
	return ""
}

var File_protopkg_diagnostic_proto protoreflect.FileDescriptor

var file_protopkg_diagnostic_proto_rawDesc = []byte{
	0x0a, 0x19, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x70, 0x6b, 0x67, 0x2f, 0x64, 0x69, 0x61, 0x67, 0x6e,
	0x6f, 0x73, 0x74, 0x69, 0x63, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x08, 0x70, 0x72, 0x6f,
	0x74, 0x6f, 0x70, 0x6b, 0x67, 0x22, 0xa5, 0x01, 0x0a, 0x0a, 0x44, 0x69, 0x61, 0x67, 0x6e, 0x6f,
	0x73, 0x74, 0x69, 0x63, 0x12, 0x39, 0x0a, 0x08, 0x73, 0x65, 0x76, 0x65, 0x72, 0x69, 0x74, 0x79,
	0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1d, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x70, 0x6b,
	0x67, 0x2e, 0x44, 0x69, 0x61, 0x67, 0x6e, 0x6f, 0x73, 0x74, 0x69, 0x63, 0x2e, 0x53, 0x65, 0x76,
	0x65, 0x72, 0x69, 0x74, 0x79, 0x52, 0x08, 0x73, 0x65, 0x76, 0x65, 0x72, 0x69, 0x74, 0x79, 0x12,
	0x18, 0x0a, 0x07, 0x73, 0x75, 0x6d, 0x6d, 0x61, 0x72, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09,
	0x52, 0x07, 0x73, 0x75, 0x6d, 0x6d, 0x61, 0x72, 0x79, 0x12, 0x16, 0x0a, 0x06, 0x64, 0x65, 0x74,
	0x61, 0x69, 0x6c, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x64, 0x65, 0x74, 0x61, 0x69,
	0x6c, 0x22, 0x2a, 0x0a, 0x08, 0x53, 0x65, 0x76, 0x65, 0x72, 0x69, 0x74, 0x79, 0x12, 0x0b, 0x0a,
	0x07, 0x49, 0x4e, 0x56, 0x41, 0x4c, 0x49, 0x44, 0x10, 0x00, 0x12, 0x07, 0x0a, 0x03, 0x45, 0x52,
	0x52, 0x10, 0x01, 0x12, 0x08, 0x0a, 0x04, 0x57, 0x41, 0x52, 0x4e, 0x10, 0x02, 0x42, 0x32, 0x5a,
	0x30, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x34, 0x72, 0x63, 0x68,
	0x72, 0x34, 0x79, 0x2f, 0x67, 0x6f, 0x72, 0x61, 0x79, 0x2f, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e,
	0x61, 0x6c, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x70, 0x6b,
	0x67, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_protopkg_diagnostic_proto_rawDescOnce sync.Once
	file_protopkg_diagnostic_proto_rawDescData = file_protopkg_diagnostic_proto_rawDesc
)

func file_protopkg_diagnostic_proto_rawDescGZIP() []byte {
	file_protopkg_diagnostic_proto_rawDescOnce.Do(func() {
		file_protopkg_diagnostic_proto_rawDescData = protoimpl.X.CompressGZIP(file_protopkg_diagnostic_proto_rawDescData)
	})
	return file_protopkg_diagnostic_proto_rawDescData
}

var file_protopkg_diagnostic_proto_enumTypes = make([]protoimpl.EnumInfo, 1)
var file_protopkg_diagnostic_proto_msgTypes = make([]protoimpl.MessageInfo, 1)
var file_protopkg_diagnostic_proto_goTypes = []interface{}{
	(Diagnostic_Severity)(0), // 0: protopkg.Diagnostic.Severity
	(*Diagnostic)(nil),       // 1: protopkg.Diagnostic
}
var file_protopkg_diagnostic_proto_depIdxs = []int32{
	0, // 0: protopkg.Diagnostic.severity:type_name -> protopkg.Diagnostic.Severity
	1, // [1:1] is the sub-list for method output_type
	1, // [1:1] is the sub-list for method input_type
	1, // [1:1] is the sub-list for extension type_name
	1, // [1:1] is the sub-list for extension extendee
	0, // [0:1] is the sub-list for field type_name
}

func init() { file_protopkg_diagnostic_proto_init() }
func file_protopkg_diagnostic_proto_init() {
	if File_protopkg_diagnostic_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_protopkg_diagnostic_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*Diagnostic); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_protopkg_diagnostic_proto_rawDesc,
			NumEnums:      1,
			NumMessages:   1,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_protopkg_diagnostic_proto_goTypes,
		DependencyIndexes: file_protopkg_diagnostic_proto_depIdxs,
		EnumInfos:         file_protopkg_diagnostic_proto_enumTypes,
		MessageInfos:      file_protopkg_diagnostic_proto_msgTypes,
	}.Build()
	File_protopkg_diagnostic_proto = out.File
	file_protopkg_diagnostic_proto_rawDesc = nil
	file_protopkg_diagnostic_proto_goTypes = nil
	file_protopkg_diagnostic_proto_depIdxs = nil
}