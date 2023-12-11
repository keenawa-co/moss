package ason

type serConf int

const (
	// CACHE_REF flag must be used when you carry out some manual
	// manipulations with the source AST tree. For example, you
	// duplicate nodes, which can create nodes that have the same
	// references to the original object in memory. In order to
	// reduce the number of checks that reduce performance, only
	// large nodes can be cached, such as specifications,
	// types and declarations.
	//
	// Use this flag when duplicating nodes containing many fields.
	CACHE_REF serConf = iota // can be used for [Serialize, Deserialize]

	// FILE_SCOPE enable serialization of `Scope` field in `*ast.File`.
	FILE_SCOPE // can be used for [Serialize]

	// PKG_SCOPE enable serialization of `Scope` field in `*ast.Package`.
	PKG_SCOPE // can be used for [Serialize]

	// IDENT_OBJ enable serialization of `Obj` field in `*ast.Ident`.
	IDENT_OBJ // can be used for [Serialize]

	// LOC allow to include `start` and `end` position for
	// all AST Nodes in the final serialization object.
	LOC // can be used for [Serialize]
)
